# Phase 1 implementation status

24 November 2026, EAT. Work on feat/campaign-core, draft PR #2, dependent on product-plan PR #1.

## Implemented

Original Anchor 1.2.2 program defines Campaign, Receipt, program-controlled classic-token vault, capped contributions, fixed-creator release and individually claimable expiry refunds. Commitment binds program, campaign and a 32-byte publication value. Receipt and campaign accounts persist. Relayers can trigger release/refund but cannot redirect the fixed beneficiary. Host validation covers ordered deadlines, positive amounts, overfill, overflow, terminal states and allocated serialization space.

## Recorded local checks

| Command | Result |
|---|---|
| cargo +stable test --workspace --offline | Exit 0, 4 tests passed, 0 failed, 0 doc tests. Windows Rust 1.98.0. |
| cargo +stable clippy --workspace --all-targets --locked --offline -- -D warnings | Exit 0, no warnings. |
| cargo +stable fmt --all -- --check | Exit 0. |
| git diff --check | Exit 0. |
| & "$env:TEMP/dev3pack-intel-svm-20261122/Scripts/python.exe" tests/program_vm.py | Exit 0, 5 scenarios passed against the actual compiled SBF. Two sponsors, overfill, fixed payout, wrong material/payee, unauthorized source, replay, and both timeout paths exercised. |

The +stable override uses the already-installed host toolchain, observed at 1.98.0. rust-toolchain.toml pins 1.98.0 for clean installations and CI. Local named-toolchain installation encountered a clippy download timeout; its existing stable alias runs checks successfully. Initial compile exposed Anchor feature/API differences; source now uses its current CPI program-ID API and required account-initialization features. Token-2022 modules are enabled to satisfy generated initialization code; program accounts still constrain to the classic Token program.

## Pending gate

GitHub Actions run 45266242671 succeeded for both host checks and pinned Linux SBF build, exit 0. Downloaded unseal.so is 230,408 bytes. That binary executed in local LiteSVM via solders 0.29.0; five real program scenarios passed with actual PDA-authorized classic-token transfers. These tests use ephemeral local signers and seeded mint/source fixtures. Public devnet deployment and browser wallet confirmations remain UNVERIFIED. The source currently declares a development example program ID and needs a dedicated deployment identity before deploy. Keep the PR in draft until remaining gates are resolved.

Linux setup was attempted via official rustup and Anza endpoints. WSL Rust download timed out, Anza installer connection reset and a GitHub binary download reset. Native dependency download recovered and all host checks now pass. CI's pinned Agave 4.4.1 SBF build succeeded in its independent network/build environment. Run 45267519730 also passed host formatting/tests/lint and all five compiled-program VM scenarios, with both jobs successful and gh run watch exit 0. Public devnet RPC getVersion responded successfully; publishing a dedicated program and browser wallet confirmations are still pending.
