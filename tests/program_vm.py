"""Execute the compiled Unseal SBF program with ephemeral local accounts.

Run with solders==0.29.0 after downloading/building target/deploy/unseal.so.
No wallet files, secrets, RPC or public funds are involved.
"""
import hashlib
from pathlib import Path
from struct import pack, unpack_from

from solders.account import Account
from solders.instruction import AccountMeta, Instruction
from solders.keypair import Keypair
from solders.litesvm import LiteSVM
from solders.pubkey import Pubkey
from solders.system_program import ID as SYSTEM
from solders.token import ID as TOKEN
from solders.token.state import Mint, TokenAccount, TokenAccountState
from solders.transaction import Transaction
from solders.transaction_metadata import FailedTransactionMetadata

PROGRAM = Pubkey.from_string("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMQhgYwR5Xh")
vm = LiteSVM()
vm.add_program_from_file(PROGRAM, str(Path(__file__).resolve().parents[1] / "target/deploy/unseal.so"))
creator, alice, bob, relay = [Keypair() for _ in range(4)]
for signer in [creator, alice, bob, relay]:
    vm.airdrop(signer.pubkey(), 100_000_000)
mint = Pubkey.new_unique()


def seed(address, state):
    data = bytes(state)
    vm.set_account(address, Account(vm.minimum_balance_for_rent_exemption(len(data)), data, TOKEN))


def token_account(owner, amount=0):
    address = Pubkey.new_unique()
    seed(address, TokenAccount(mint, owner, amount, None, TokenAccountState.Initialized, None, 0))
    return address


seed(mint, Mint(None, 40, 0, True))
alice_source = token_account(alice.pubkey(), 20)
bob_source = token_account(bob.pubkey(), 20)
creator_destination = token_account(creator.pubkey())
attacker_destination = token_account(relay.pubkey())


def balance(address):
    return TokenAccount.from_bytes(vm.get_account(address).data).amount


def meta(address, writable=False, signer=False):
    return AccountMeta(address, signer, writable)


def send(name, data, accounts, signer):
    discriminator = hashlib.sha256(("global:" + name).encode()).digest()[:8]
    vm.expire_blockhash()  # Each retry is a fresh transaction, not duplicate-signature rejection.
    tx = Transaction.new_signed_with_payer(
        [Instruction(PROGRAM, discriminator + data, accounts)], signer.pubkey(), [signer], vm.latest_blockhash()
    )
    return vm.send_transaction(tx)


def success(result):
    assert not isinstance(result, FailedTransactionMetadata), str(result)


def failure(result):
    assert isinstance(result, FailedTransactionMetadata), "Unexpected successful instruction"


now = vm.get_clock().unix_timestamp
fund_by, reveal_by = now + 100, now + 200


def create(nonce):
    campaign, _ = Pubkey.find_program_address([b"campaign", bytes(creator.pubkey()), pack("<Q", nonce)], PROGRAM)
    vault, _ = Pubkey.find_program_address([b"vault", bytes(campaign)], PROGRAM)
    material = hashlib.sha256(b"public synthetic lesson " + pack("<Q", nonce)).digest()
    commitment = hashlib.sha256(b"unseal/reveal-v1" + bytes(PROGRAM) + bytes(campaign) + material).digest()
    terms = pack("<QQqq", nonce, 5, fund_by, reveal_by) + bytes([1]) * 32 + bytes([2]) * 32 + commitment
    success(send("create_campaign", terms, [meta(creator.pubkey(), True, True), meta(mint),
        meta(campaign, True), meta(vault, True), meta(TOKEN), meta(SYSTEM)], creator))
    return campaign, vault, material


def pledge(campaign, vault, sponsor, source, amount):
    receipt, _ = Pubkey.find_program_address([b"pledge", bytes(campaign), bytes(sponsor.pubkey())], PROGRAM)
    result = send("pledge", pack("<Q", amount), [meta(sponsor.pubkey(), True, True), meta(campaign, True),
        meta(mint), meta(receipt, True), meta(source, True), meta(vault, True), meta(TOKEN), meta(SYSTEM)], sponsor)
    return result, receipt


def release(campaign, vault, material, destination):
    return send("release", material, [meta(relay.pubkey(), False, True), meta(campaign, True),
        meta(mint), meta(vault, True), meta(destination, True), meta(TOKEN)], relay)


def refund(campaign, vault, receipt, destination):
    return send("refund", b"", [meta(relay.pubkey(), False, True), meta(campaign, True), meta(mint),
        meta(receipt, True), meta(vault, True), meta(destination, True), meta(TOKEN)], relay)


c, vault, material = create(1)
result, receipt = pledge(c, vault, alice, alice_source, 2)
success(result)
failure(pledge(c, vault, bob, bob_source, 4)[0])
assert balance(vault) == 2 and balance(bob_source) == 20
success(pledge(c, vault, bob, bob_source, 3)[0])
assert balance(vault) == 5 and balance(creator_destination) == 0
print("PASS two independent sponsors fill exactly five; overfill rolls back")

failure(release(c, vault, bytes(32), creator_destination))
failure(release(c, vault, material, attacker_destination))
assert balance(vault) == 5 and balance(attacker_destination) == 0
success(release(c, vault, material, creator_destination))
assert balance(vault) == 0 and balance(creator_destination) == 5
data = vm.get_account(c).data
assert data[218] == 2 and data[219:251] == material
failure(release(c, vault, material, creator_destination))
failure(refund(c, vault, receipt, alice_source))
print("PASS real PDA payout records material; wrong material, wrong payee and repeats fail")

c2, vault2, _ = create(2)
success_result, receipt2 = pledge(c2, vault2, alice, alice_source, 2)
success(success_result)
failure(refund(c2, vault2, receipt2, alice_source))
c3, vault3, material3 = create(3)
failure(pledge(c3, vault3, bob, alice_source, 1)[0])
assert balance(vault3) == 0
result, alice_receipt3 = pledge(c3, vault3, alice, alice_source, 3)
success(result)
result, bob_receipt3 = pledge(c3, vault3, bob, bob_source, 2)
success(result)
print("PASS sponsor cannot debit another owner's source account")
clock = vm.get_clock()
clock.unix_timestamp = fund_by
vm.set_clock(clock)
before = balance(alice_source)
failure(refund(c2, vault2, receipt2, attacker_destination))
success(refund(c2, vault2, receipt2, alice_source))
assert balance(alice_source) == before + 2 and balance(vault2) == 0
failure(refund(c2, vault2, receipt2, alice_source))
assert unpack_from("<Q", vm.get_account(receipt2).data, 72)[0] == 2
print("PASS underfilled campaign refunds exact principal once to the recorded sponsor")
failure(refund(c3, vault3, alice_receipt3, alice_source))
clock.unix_timestamp = reveal_by
vm.set_clock(clock)
failure(release(c3, vault3, material3, creator_destination))
alice_before, bob_before = balance(alice_source), balance(bob_source)
success(refund(c3, vault3, bob_receipt3, bob_source))
success(refund(c3, vault3, alice_receipt3, alice_source))
assert balance(alice_source) == alice_before + 3
assert balance(bob_source) == bob_before + 2 and balance(vault3) == 0
failure(refund(c3, vault3, bob_receipt3, bob_source))
assert balance(creator_destination) == 5
print("PASS funded default locks until release deadline then refunds both sponsors in either order")
print("5 compiled-program scenarios passed; local VM; public deployment remains unverified")
