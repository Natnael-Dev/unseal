# Security, privacy and engineering operations

## 49. Threat model

| Asset and failure | Control and remaining trust | Acceptance evidence |
|---|---|---|
| Sponsor principal misdirected by a modified UI | Program enforces signer/owner/mint/PDA and fixed recipient owner per campaign. Official UI pins network/program/mint, validates terms and displays campaign + human-readable transaction summary. Malicious websites can still ask wallets to sign unrelated instructions. | Inspect a normal wallet preview and negative VM tests with incorrect payee, receipt, signer and mint. |
| Wrong file or poisoned mirror | Hash exact bytes against finalized terms; authenticate envelope and validate safe content schema; try second mirror or import. Identity says nothing about educational quality or rights. | Corrupt manifest, envelope and claimed plaintext digest separately. Verify no rendering on failure and recovery through a matching copy. |
| Publication material disclosed before payment | Creator local packaging/recovery, no logs, no analytics; disclose unavoidable release-input leakage through submission/simulation. Cancellation and release compete on-chain; a losing reveal may still open the guide. | Exercise release/cancel ordering with public fixtures. Describe partial disclosure without claiming an atomic secrecy guarantee. |
| Malicious or stale RPC | Trusted configured endpoints, verified genesis, owner, PDA and finalized reads; cross-check explorer/second operator for demo. RPCs can lie or censor. Ordinary browser reads are not a light client. | Wrong cluster/program rejected; disagreement or stale slots show uncertainty, blocking financial actions. |
| Wallet/RPC/mirror outage or disappearing frontend | Persistent receipts, individual direct claims, independent reader/mirror and local exports. Missing all copies prevents uncached reading. | Practice refund with official page unavailable, mirror failover and offline read after first verification. |
| Unreviewed creator content or misleading recognition | Manual admission, rights review, explicit consent, fixed publisher identity and meaningful preview. Require manual curator co-signature at pilot creation. Unrelated deployments and historical delisted campaigns can exist outside the catalog. | Unlisted campaign is never labeled approved; no display alias inferred from wallet ownership. |
| Stolen upgrade authority or compromised static host | Disclose authority, minimize access, pin releases, use independently published hashes and stop new signatures on incident. Already-finalized public state and leaked artifacts cannot be recalled. | Rehearse read-only incident mode and verify operator's public list of affected addresses/releases. |

## 50. Security implementation boundaries

Schema-validate every untrusted import, manifest, RPC account and URL parameter. Browser CSP should allow only required local assets and configured endpoints; disallow object embedding, dynamic code execution and third-party tracking. Render text through normal DOM text nodes; bounded PNGs via blob URLs. Service worker scope belongs to the reader origin and should cache only intended public resources. Never cache wallet RPC POSTs or pending signature responses generically. Enforce timeouts, caps, sanitized filenames and clear cache quotas. Ensure compromised or missing mirrors cannot select RPC/program/asset authority.

Transaction builder must target the pinned deployed program, classic token program and matching mint. Validate owner for any token account chosen; never trust a destination supplied by a link. Tests include rejected wrong mint/account owner, wrong vault/receipt seeds, noncreator cancellation, overflow, stale blockhash, replay and duplicate top-up intent. Separate host assertions from SBF/VM evidence. For review, record risk, scenario, expected failure, and actual unchanged principal/state. Scope technical review to defensive validation. No mainnet, value-bearing assets, new guardian/admin override or hidden refund exceptions.

## 51. Privacy and data retention

No login/email, default public sponsor directory, tracking pixels or analytics by default. Chain permanently exposes wallet addresses, public balances/pledge amounts, timestamps/signatures and recipient relationships. A pseudonym may be linked to wallet activity. Say this before first signature. Optional recognition needs explicit opt-in and approval of exact display name; keep it outside immutable chain/manifest. Offer removal through the operator's published contact. Removing an official label cannot erase public chain evidence or copies others saved.

The lesson contains fictional application examples without real students' résumés, emails or phone numbers. Keep club invitations and interview notes in a private owner-controlled location, collect only what is needed with consent, and publish quotes only with specific permission. Pilot proposal: keep identifiable session notes only through judging and clean them up afterward; confirm owner and deletion date when collecting any entry. Offline data belongs to the device and may be visible to other local users. Provide a clear saved-data removal action and explain any installed shell or exported files that survive it. Public RPCs/hosts can observe IP and request metadata; avoid anonymity claims.

## 52. Credential and publication-material handling

Wallet seed phrases and private signing keys never enter the app, repository, examples, logs or support instructions. Use wallet-provider signing. Separate test wallets and operator access from daily accounts. Record public deployment identity and upgrade authority only. Creator's fresh random 32-byte publication material is intentionally public after release; before release it requires private custody, a restore check and deliberate consent before transmission. Never collect it via a website analytics event, include it in a query string, commit it in build output, paste it into an issue or send it through shared chat. Use only synthetic explicitly public values in tests.

Use public-only browser config. A service that demands a confidential server key has no place in the static client. Use scoped hosting credentials through the provider's secure UI outside repository files if the operator deploys. On accidental disclosure of a hosting credential revoke and rotate it through the provider; on premature guide disclosure fix user-facing claims and assess cancellation eligibility. Rotation cannot undo a public guide leak or change immutable campaign terms.

## 53. Local setup and reproducibility

Start from the reviewed branch in a clean checkout. Use the repository's rust-toolchain file, Cargo.lock, and CI workflow as the authoritative build recipe. On Windows use host Rust checks locally if installed. Use a supported Linux/WSL environment for the SBF job. Produce the binary before running its VM scenarios. No wallet seed, RPC, devnet balance or deployment is needed for host and VM checks.

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo-build-sbf --manifest-path programs/unseal/Cargo.toml --sbf-out-dir target/deploy -- --locked
python -m venv .venv
.venv/bin/python -m pip install solders==0.29.0
.venv/bin/python tests/program_vm.py
```

Commands are a Linux-environment recipe; run the last two with the corresponding virtual-environment interpreter on Windows. Put the environment outside tracked files. When the browser workspace lands, pin runtime/package-manager versions and add exact clean install, build, two-origin local serve, offline test and preview commands in the README in the same PR as the first usable app. A repo without those apps cannot claim browser readiness. Record command, platform, exit status and actual counts, with local VM results distinct from public devnet transactions.

## 54. Coding and interface standards

Preserve program account layouts, enum ordinals and instruction discriminators unless accompanied by a compatibility decision and fixtures. Keep Rust guard logic pure when practical and test both boundary conditions and compiled transfer behavior. Checked integer arithmetic; no floating point currency. Validate wallet and account inputs adjacent to irreversible signing/transfer logic. In TypeScript use explicit unions for pending/finalized/unknown, byte arrays for cryptographic inputs and branded or well-labeled identifiers for chain/network/amounts. Surface unsupported versions explicitly.

Treat the reader package as usable without wallet code or campaign origin. Enforce module boundaries with an import or build check. Pair implementation of each user-visible state with an actionable label, accessible focus and test evidence. Document nonobvious byte ordering and trust assumptions near the contract. Keep one source of truth per invariant and compare cross-language test vectors against independent expected outputs.

## 55. Branches, commits and review

Short-lived branches off the correct dependency base. State the base PR on stacked work. Stage named files. Each Conventional Commit under 73 characters should describe one independently reviewable change. Keep generated IDL/fixtures and their contract change in the same logical commit where separation would leave callers broken. Review the staged diff for unexpected secret material and private lesson contents without opening secret files.

Open a PR with objective, affected user journeys/terms, test commands + exit/count evidence, migration implications and outstanding **UNVERIFIED** items. Check dependency PRs and CI on the exact head before integration. Respect the owner's single-action rules for main/master pushes, force pushes, remote deletion, history rewrites and production deployment. Preserve public tests and links; no AI attribution in commits or PRs. Audit findings need a resolution and a focused rerun before declaring a gate met.

## 56. Dependencies, licenses and third-party notices

MIT applies to original code after adding the approved license text and verified copyright credit. CC BY 4.0 applies to the cleared guide, illustrations and its public sample; use explicit scope in the root README and attribution in exports. Third-party components, fonts and third-party assets retain their own terms. An MIT root license cannot automatically relicense a dependency or another author's illustration. Require rights records and reviewed usage/attribution for each external asset before publication. Original illustrations avoid extra rights ambiguity.

Use native APIs where sufficient. Every direct dependency needs a concrete use, pinned compatible version/lockfile, source URL, license expression, redistribution obligations and update owner. Inspect transitive notices in shipping artifacts; preserve font and library licenses where redistribution requires them. Record dependency verification date and unresolved restrictions; replace anything with unclear rights rather than guessing. Before changing Anchor/Agave/web3.js regenerate IDL, compare account layout, build the compiled program and rerun VM, browser wallet and byte-vector checks. Purchase/paid hosting remains outside the zero-spend pilot.

Initial direct-dependency inventory, inspected against the package manifests and lockfile on 24 November. Dexter owns the release notice inventory. Capture upstream LICENSE and any applicable NOTICE alongside the distribution, preserve copyright notices and identify modifications. Recheck transitive components against the actual build before distribution.

| Dependency | Version / purpose | Provenance and release action |
|---|---|---|
| anchor-lang | 1.2.2, account constraints and instruction entry points | Apache-2.0 in installed crate manifest. [Upstream](https://github.com/otter-sec/anchor) |
| anchor-spl | 1.2.2, classic token transfers and vault initialization | Apache-2.0 in installed crate manifest. [Upstream](https://github.com/otter-sec/anchor) |
| solana-sha256-hasher | 3.1.0, publication commitment | Apache-2.0 in installed crate manifest. [Upstream](https://github.com/anza-xyz/solana-sdk) |
| solders | 0.29.0, Python compiled-program test harness; test environment only | Record exact distribution notice before redistributing any test environment. [Package](https://pypi.org/project/solders/0.29.0/) |
| Browser libraries and fonts | Candidates in sections 27 and 31 | Exact browser dependency set and font files remain **UNVERIFIED** until locked. Record each source, version, file license, size and notice in the first browser dependency PR. |
