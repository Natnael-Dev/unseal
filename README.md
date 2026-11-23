# Unseal

Collectively fund the public release of useful learning and creative resources. Sponsors fund a finished, previewable pack. Successful release pays its fixed creator, and anyone can open a verified copy without a wallet. An expired campaign provides individual contribution refunds.

Development is in Phase 1. Original Rust source, four host tests and five compiled-program VM scenarios pass locally. Linux CI built the SBF artifact successfully. Public devnet deployment and the browser application remain unverified. Only devnet test funds are planned for the hackathon demonstration.

## Start here

- [Product decisions and phases](docs/product-plan.md)
- [Architecture, stack and judging evidence](docs/architecture.md)
- [Phase 1 verification and remaining gates](docs/phase-1-status.md)
- [Screen concept](docs/design/README.md)

## Host checks

Rust 1.98.0 and Cargo are pinned in rust-toolchain.toml. Run from the repository root:

```sh
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo fmt --all -- --check
```

Linux CI builds the SBF binary using Agave 4.4.1 and executes tests/program_vm.py using solders 0.29.0. To reproduce the VM checks, install that package in an isolated Python environment, build or download target/deploy/unseal.so, then run python tests/program_vm.py. A local VM result cannot establish a public devnet transaction. The development example program ID must be replaced before deployment.

## How contributions become a release

1. Creator fixes target, mint, funding/release deadlines and file commitments.
2. Sponsors contribute up to the exact target through the program.
3. Successful release records the unlocking material and transfers the target to the fixed creator.
4. Independent reader verifies and opens a cached copy.
5. Funding failure or creator default opens individual principal-refund claims.

Creator remains responsible for quality and rights. Publishing unlocking material can disclose the file even if the transaction fails. File verification cannot guarantee quality or continuing hosting. Development work before the event and upstream dependencies will be documented in the submission.

## Contribution workflow

Use a short-lived branch, one logical change per Conventional Commit and a pull request with exact verification evidence. Keep dependent PRs linked. Never commit wallet files, credentials, generated build output or real user data. Changes to main and production deployment follow the owner's action-specific approval rules.
Collectively fund the public release of useful learning and creative resources.
