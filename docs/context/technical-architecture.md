# Architecture, interfaces and storage

Planning baseline for the English, invited devnet pilot. Code references describe the feature branch at c383318. Future paths and contracts below are implementation targets.

## 30. System architecture

A creator opens a local packager, selects the finished guide and public sample, derives a fresh campaign address and prepares the immutable files. The creator inspects an actual round-trip decryption, verifies both hosts and signs a creation transaction. A curated catalog advertises that campaign. The sponsor browser reads its terms from Solana and requests a wallet signature only after review. The Rust program controls token movement. A separately hosted reader can discover the same campaign through its address and trusted network configuration, read its public release material and verify the files without a wallet or the campaign website.

Keep the catalog, public preview, wallet application and wallet-free reader deployable as static files. Make the reader's shell, fonts, manifest discovery and RPC configuration independent of the campaign origin. A shared package can implement the byte format. A second decoder and fixed vectors must independently check that package before relying on it. The chain commits to bytes. The creator remains accountable for the content.

## 31. Technology choices

| Layer | Implementation and lock point |
|---|---|
| Funding | Rust 2021, Anchor Lang/SPL =1.2.2, pinned Solana SHA-256 3.1.0. Follow the checked-in Cargo.lock and workflow. Use classic SPL Token at runtime. The crate feature list also includes Token-2022, but existing account constraints accept the classic program ID. |
| Host and execution checks | Rust 1.98.0 host, Agave 4.4.1 build tools, solders 0.29.0 local VM. Record actual tools and binary hash per build. |
| Browser | React, TypeScript, Vite, generated IDL, compatible Anchor TS client and web3.js v1. Build separate reader and campaign entry points. Verify actual package exports, generated IDL, wallet support and toolchain together before locking JS versions. No desktop-only wallet assumption on Android. |
| Files | Native Web Crypto, raw CIDv1 SHA-256, static HTTPS mirrors and IndexedDB for reader-owned cached bytes. Candidate multiformats 14.0.6 from prior research. Check size, offline operation and exact-version compatibility before adding a dependency. |
| Database and operations | Solana is authoritative for funding and final release. Versioned static JSON provides discovery. Browser storage is disposable. Zero paid services, server-held wallet credentials, fee relay service or hosted SQL required for pilot. |

## 32. Architecture decisions and reconsideration triggers

| Decision | Tradeoff and revisit trigger |
|---|---|
| Fixed creator, test mint, exact cap, individual refunds | No oversubscription, pro-rata accounting or batch payout. Revisit only with a new program/product decision. |
| Admitted creators on the pilot deployment, separately curated catalog | Baseline creation is permissionless. Before deploying the pilot, require the configured curator to co-sign creation with the creator. No curator signature for release, cancellation, pledges or refunds. A separate deployment can have its own rules. |
| Sponsor invitation handled operationally | Distribute official pilot links and test units to invitees. The public reader needs neither. Possession of a copied link or compatible devnet tokens is possible. Do not describe this as enforceable identity or spending control. |
| Two independent copies + local import | Creator supplies both copies before opening sponsorship. Replacing a host preserves terms when bytes match. If both are lost an uncached reader cannot recover the guide from the chain. |
| Browser decryption and opt-in local caching | No wallet or app account for readers. First verification needs connectivity. Private browsing, storage eviction and device loss can erase files. |
| No early administrative retrieval | Fees, unsolicited tokens and account rent are distinct from principal. Keeping campaign/receipt accounts costs rent but preserves replay protection and independent evidence. |

## 33. Repository and module boundaries

Existing: `programs/unseal/src/lib.rs` owns instructions/account validation, `tests/program_vm.py` exercises compiled transfers, `.github/workflows/` builds and runs checks, `docs/design/` holds concept visuals. Planned: `apps/campaign/` for catalog, creator, sponsor and receipts; `apps/reader/` for standalone checking and offline reading; `packages/artifact/` for the manifest and envelope; `packages/chain/` for program IDL, PDA derivation and network-bound account reads. Reserve `fixtures/public/` for synthetic cross-language vectors and `content/` for publishable assets and rights records. Create paths only when their first consumer exists.

Keep unpublished lesson source, pre-release material, private signup lists and personal data outside public git history and preview builds. A public repo containing the final unencrypted lesson before release would undermine the demonstration. At release the creator can publish the complete licensed source separately. The two browser bundles must build from a clean checkout. Inspect generated static assets for private inputs before publishing.

## 34. Domain model

| Entity | Identity, relationships and authority |
|---|---|
| Campaign | Tuple of network genesis hash, program ID and campaign address. One creator and one mint. Immutable terms bind target, two timestamps and three 32-byte commitments. |
| Sponsor receipt | Campaign and sponsoring wallet. More pledges accumulate on the same receipt. One full-principal refund consumes its right. |
| Vault | Derived token account of a single campaign, token authority = campaign PDA. A token-account balance can contain unsolicited units. |
| Resource | Exact manifest bytes, public preview and encrypted guide bytes. Creator signature binds their identities through campaign terms. Titles and aliases never identify financial recipients. |
| Recognition | Optional public alias with explicit consent. Omission is default. Keep live aliases in curated display data, separately from immutable manifest and funding accounts, so removal from the official page is possible. |
| Reader cache | Network/program/campaign + manifest and ciphertext digests + prior finalized release and verified bytes. Never count a cached signature or cached balance as live funding evidence. |

## 35. Persistence, indexing and lifecycle

One catalog file lists approved campaign addresses and mirror locators. Build receipt lookups directly from the connected sponsor and address; no global sponsor index or scraped leaderboard. Cache public manifests/envelopes by digest across refreshes. Treat all disk imports as untrusted. In the reader, commit verified blobs and the matching release record together; a quota failure rolls back or leaves the previous intact version. Show size before saving, provide export/remove, and distinguish 'opened once' from 'saved on this device'. Deleting site data never deletes chain records.

The campaign client may journal network ID, signature, expiry block height, intended action and pre-action receipt amount while confirmation is unresolved. Use per-tab action locks and re-evaluate when another tab changes the journal. Never journal signed transaction bytes, unpublished material or wallet credentials. Browser-saved links, dossiers and timestamps are hints. Refresh authoritative state before the next signing prompt.

## 36. Browser and RPC interface contract

| Operation | Inputs and output contract | Failure handling |
|---|---|---|
| Read campaign | Fixed devnet config + parsed address. Fetch finalized account with context slot, verify owner, discriminator, account version, derived campaign address and mint. Return bigint amounts, UTC seconds, recorded enum and derived display state. | Distinguish unknown address, unsupported version/network, malformed account and temporary RPC failure. Never render a zero balance as a fallback. |
| Read my receipt | Same campaign and wallet; derive receipt and validate owner, discriminator, sponsor, campaign. | Missing account on a healthy finalized read means no recorded pledge at that slot. A failed RPC read means unknown. |
| Prepare action | Read current account/receipt/mint and caller. Return exact message summary, amount/base units, cluster, mint, addresses, estimated fee/rent and two deadlines. | Invalid decimal precision, account mismatch, wrong network, closed campaign or insufficient balance blocks signing. |
| Submit/recover | Wallet returns signed transaction/signature; save public recovery information, submit and watch signature with its last valid block height. | See transaction policy. An ambiguous response leaves a recoverable pending state. |
| Read artifact | Trusted config + finalized campaign + bounded byte inputs. Return decoded metadata and verified plaintext only after all integrity checks. | Offer alternate mirror or import on network failure; discard invalid bytes. See independent reader. |

No application write API or shared login is required. Stable domain errors should map to one plain-language recovery instruction with optional diagnostic code. Rebuild the generated IDL when the ABI changes and check in the generated public interface and provenance.

## 37. Integration settings and rollout

Publish an explicit network profile containing devnet genesis hash, RPC URL(s), program ID, test mint, decimals, program interface version, historical campaign profiles and a separately updateable active catalog, reader origin and mirror roots. Ship only public values. Verify the RPC's genesis hash before trusting reads or requesting a signature. Verify account owner and discriminator on each decoded address; never accept a program ID injected through a campaign URL. Run read-after-write on a finalized context at least as recent as the accepted signature where supported. Fallback to a second configured RPC if the first is unavailable; distinguish RPC trust from independent on-chain verification.

Use wallet capabilities detection on a real phone and desktop. On mobile return from the wallet with the same action summary and network, handle app restarts and reconnect explicitly. Opening a judge's URL in a wallet-free browser must allow preview/reading. Keep endpoints, chain identifiers, mirror origins and wallet packages **UNVERIFIED** until the integration run records actual values and test devices.
