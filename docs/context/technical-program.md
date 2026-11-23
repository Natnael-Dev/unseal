# Solana funding and settlement

The current four instructions and account layouts below come from [the Rust program](../../programs/unseal/src/lib.rs) and [the compiled-program scenarios](../../tests/program_vm.py). Cancellation is an approved pilot requirement awaiting implementation.

## 38. Program contract and data layout

Use program-derived addresses under the pinned program ID. Campaign seeds = UTF-8 `campaign`, creator public key (32 raw bytes), nonce (unsigned 64-bit little endian). Vault seeds = UTF-8 `vault`, campaign public key. Receipt seeds = UTF-8 `pledge`, campaign public key, sponsor public key. The vault is a derived classic SPL token account. Do not derive it as an associated token account. The UI can use canonical associated token accounts for convenience as payout and refund destinations; the current program allows any valid account of the required mint and owner.

| Instruction | Required signer and data | Effect |
|---|---|---|
| create_campaign | Creator, terms serialized in Rust field order. Positive target and now < fund_by < reveal_by, classic mint with no freeze authority. | Creates campaign and vault atomically. Creator pays account rent. Publish only configured pilot mint campaigns in catalog. |
| pledge | Sponsor, u64 amount in base units; sponsor-owned source and matching mint. | Transfer checked; initialize sponsor-paid receipt once, aggregate on later pledges. Set Filled on exact cap. |
| release | Caller, 32 raw bytes of publication material. | Check commitment and Filled window, transfer exactly target to creator-owned account, persist release material and Released in same transaction. |
| refund | Any signer as caller, correct sponsor receipt and sponsor-owned receiver. | Refund full cumulative receipt once. First timeout claim changes campaign to Refunding. No sponsor list iteration. |
| cancel_campaign (planned) | Creator signer matching campaign.creator, writable derived campaign. No amount, recipient or release material. | Move eligible Funding or Filled campaign to Cancelled; leave balances, receipts and material untouched. Refund each receipt independently. Never transfer directly to the creator. |

Current account data after the 8-byte Anchor discriminator: Campaign has version u8, bump u8, creator[32], mint[32], terms (nonce u64, target u64, fund_by i64, reveal_by i64, three digests[32] in order manifest/ciphertext/commitment), pledged u64, refunded u64, state enum u8 and material[32]. Size 251 bytes including discriminator. Receipt has campaign[32], sponsor[32], amount u64 and refunded bool (81 bytes including discriminator). Integer encoding is little endian. Validate observed size and IDL in compatibility tests. Do not rely on raw byte offsets in the browser.

## 39. Authorities, recipient rules and admission

Creator signs creation, commits immutable terms, pays rent and may request cancellation while eligible. A release can be relayed by someone who possesses the material, even though the normal interface is for the creator. A relayer can pay fees but cannot replace the recipient owner. Lost creator material leads to refund eligibility at the release deadline if the target fills. Losing a sponsor wallet does not permit redirecting a refund to a new owner. A new token account owned by the original sponsor is an acceptable destination, and another signer can call refund to it.

Operator curates official campaign addresses, publishes test configuration and manually admits official creators and invites pilot participants. Those powers do not include signing for sponsors, changing deadlines or withdrawing escrow. Existing program checks each campaign's mint and does not restrict the set of possible external creators or freezeless classic mints. For the pilot, implement Q10 by adding a read-only curator signer account to create_campaign, constrained to the dedicated public curator key pinned in the deployment build. Require both creator and curator signatures on exactly the same transaction message. The creator can also be the curator; verify duplicate account-key handling in a compiled test. With distinct wallets the curator reviews the fixed transaction message, program, mint, campaign and terms offline from the sponsor flow; combine partial signatures before the blockhash expires. If either party declines or the message expires, abandon it and request fresh review from both. No signing server. Pin and disclose the public curator identity at deployment. Key loss blocks new campaigns only. Curator replacement requires a separately reviewed deployment change. Existing accounts remain readable and settle without curator involvement. Test missing/wrong curator, changed terms after signing, separate and same signer, and normal claims while curator is unavailable. The current create ABI is historical; publish a new IDL before enabling the pilot. Use two separate configuration lists. The active catalog admits official creators and campaigns for discovery, creation and new pledges. A durable historical network profile preserves every previously admitted campaign's genesis hash, program ID, ABI version and mint. Delisting removes discovery and new sponsorship in the official app. It must never remove the historical profile or make an existing receipt ineligible. A direct-link refund checks that profile, receipt, fixed sponsor destination and finalized eligibility regardless of current catalog membership. A previously admitted creator can still settle or cancel an existing campaign under the published rules. The reader can still open a released campaign independently. Separate admission from user authorization: an arbitrary external campaign does not become an official Unseal publication by sharing the same program. A curator cannot revoke public-chain access or unilaterally freeze an existing campaign. Upgrade authority can alter program behavior. Identify the actual controller and disclose it before the demo.

## 40. State machine, including proposed cancellation

| Recorded state | Allowed transitions |
|---|---|
| Funding (0) | Before fund_by, positive pledge up to remaining, then Funding or Filled. Creator can cancel before fund_by including with zero pledges. At/after fund_by, underfilled campaign is refund-eligible immediately without a keeper; first successful refund sets Refunding. |
| Filled (1) | Release immediately, strictly before reveal_by; no need to wait until fund_by. Creator can cancel only before reveal_by and before any successful release. At/after reveal_by, first successful refund sets Refunding. |
| Released (2) | Terminal with durable material. No cancellation, pledge, repeat release or refund. |
| Refunding (3) | Any remaining positive, unrefunded receipt can claim in any order. Terminal for release/funding. |
| Cancelled (4, planned append-only) | All unrefunded receipts can claim immediately, regardless of deadlines. Preserve Cancelled across claims so reason remains visible. With no pledges, no transfer is needed. |

The display can derive 'deadline passed', 'awaiting claim' and 'all refunds claimed' without changing storage. For cancellation pay particular attention to the existing unconditional assignment to Refunding inside refund. Preserve existing numeric values, verify the length still fits, update IDL, old-account fixture and reader/client version gates. Enable cancellation in the official UI only against the matching upgraded pilot deployment; older deployments reject it. Old clients must surface an unsupported state, never default to Funding.

## 41. Money and integrity invariants

- Initial pledged/refunded = 0. At all times 0 ≤ refunded ≤ pledged ≤ target; every recorded receipt amount equals that wallet's successful debits. Sum of receipts equals pledged. Sum of refunded receipts equals refunded.
- While Funding, Filled, Refunding or Cancelled, liability = pledged − refunded. In Released liability = 0 and exactly target has been paid once to the creator. Unexpected deposits do not affect progress, payout or refund amounts. No recovery instruction, dust sweep, closing accounts or fee deduction in this release.
- Store immutable nonce, mint, beneficiary owner, target, deadlines, manifest/ciphertext digests and reveal commitment. Never allow editing a live campaign. Fix errors by starting a new campaign with a fresh nonce and files. Announce prior cancellation/refund rights.
- Keep amounts in bigint in the browser. Parse decimal input as a string with at most mint.decimals fractional digits; reject exponent notation, negative, ambiguous separators, overprecision, zero and u64 overflow. Offer configurable suggested amounts and allow the remaining positive balance down to one base unit. Display token units and fee/rent in SOL separately. A pilot copy limit does not change the program's one-base-unit minimum.
- Failure can cost a network fee. An instruction that fails must leave token principal, account balances for transfers, receipts, material and campaign counters unchanged. Prove this with before/after snapshots in VM and in the devnet rehearsal.

## 42. Signing, pending transactions and recovery

1. Verify selected cluster, deployment, mint, wallet, current receipt and remaining need. Simulate key-free steps, including destination existence and balance for fees. Use idempotent associated-token-account creation if a required owner/mint destination is absent. Disclose who pays rent. Show exact amount, deadlines, no early withdrawal and final recipient before signature.
2. Journal action summary, signature and last valid block height as soon as available. Catch rejection, transport disconnect, blockhash expiry, returned signature without confirmation, and browser refresh. Treat 'sent' and 'finalized' as separate UI states. When possible rebroadcast only the same signed transaction while valid. Never silently produce a freshly signed top-up.
3. Recover by signature status/history plus fresh finalized campaign and receipt. A wallet's repeated equal top-ups are valid and indistinguishable from each other by amount. A higher receipt alone cannot establish which signature finalized. If unknown, keep the signature, show observed balance/receipt and explain double-pledge risk. Only offer a newly reviewed retry after expiry, trustworthy status check and explicit user intent.
4. Map known outcomes: overfill → refresh amount without rounding or automatic partial acceptance; lost signing prompt → no second prompt before status check; wrong cluster → choose devnet; insufficient SOL → display fee/rent need separately; AlreadyRefunded → fetch final receipt; WrongState/Deadline → refresh state and present the eligible next action.
5. Prepare release locally and check decryption, commitments and destination before exposing material. If a wallet or RPC preflight receives material, treat it as potentially public even on rejection. Never promise secrecy after release submission, log release arguments or permit an automated release retry after a cancel/expiry race.

## 43. Time, concurrency and boundary tests

Chain Clock decides eligibility, using epoch seconds. Show EAT and explicit absolute dates as well as countdowns. Device time only estimates. When connectivity is lost or the chain time is stale, display the last observation and defer signing. Close eligibility exactly at the applicable deadline; refunds open at equality. Funding closes at fund_by, even when zero receipts exist.

| Competing operations | Acceptance condition to exercise |
|---|---|
| Two final pledges for the same remaining amount | At most one fits; other fails with no principal debit. Preserve winning receipt, refresh losing sponsor's UI. |
| Last pledge and creator cancel | Pledge first → receipt refundable if cancel wins next; cancel first → pledge rejected. No stranded tokens. |
| Release and creator cancel from Filled | First valid transition wins. Released pays creator and makes refunds impossible; Cancelled pays nobody and makes receipts claimable. A losing reveal may still disclose the key. |
| Two claims of same receipt | One succeeds once; other reports already refunded. Concurrent claims of different receipts both eventually succeed without waiting for every sponsor. |
| Deadline and stale sign prompt | Evaluate at execution: accept before, reject at/after cutoff. Test fund_by−1, fund_by, reveal_by−1 and reveal_by. Cancel after applicable expiry must leave timeout reason and claims intact. |
| Cancellation of zero-pledge campaign | Terminal Cancelled with no transfer, no receipts and no fabricated 'claim available' button. |

## 44. Deployment, ABI evolution and rollback

Before devnet deployment replace the public placeholder identity consistently in Rust, VM fixtures, generated IDL, browser config and evidence. Obtain a reproducible compiled binary, record SHA-256, git commit, tool versions, program ID, network genesis hash and actual upgrade authority address. Store only public evidence. Test existing four-instruction fixtures before enabling the new cancellation state. Test decoding older v1 accounts and unknown future versions. Account field order/discriminators are compatibility commitments.

Deploy pilot with test funds and verify executable program owner, loader, deployed address, upgrade authority and a successful finalized read against the release profile. A new ID changes campaign PDAs, commitments and authenticated file context; repack and create fresh campaigns. Never reinterpret old addresses under a new profile. On a suspected regression disable new signatures in the official UI, retain read access and documented direct refund instructions for existing campaigns. Preserve old reader builds/profiles. Upgrades cannot erase a previously exposed key. Follow Dexter's single-action approval for production deployment.
