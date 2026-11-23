# Delivery work packages and verification

## 57. Implementation plan and backlog

These are solo-engineer effort estimates for sequencing. Actual elapsed time depends on the first vertical slice and devnet availability. Work in order within a dependency chain. Keep a working recorded campaign available for rehearsal.

| Order | Depends on | Slice and estimate | Observable exit |
|---|---|---|---|
| A | Context audit | Walk through the design with Dexter, 30–60 min | A reader, a sponsor and a creator can each find their next action. Record any corrections before implementing the affected screens. |
| B | A | Finish and review the illustrated lesson, 3–5 h | Named publisher checks each factual claim, preview, credits and reusable checklist. Store the exact reviewed revision. |
| C | A | Close differences between current program and approved rules, 4–8 h | Effective creator admission boundary is disclosed and tested. Creator cancellation opens refunds immediately. Concurrent release/cancel has one final outcome. Historical account compatibility has a recorded decision. |
| D | B, technical contract | Build packager and separate reader, 5–8 h | Known-answer artifact verifies from either copy. A phone reopens a saved lesson in airplane mode with campaign host unavailable. |
| E | C | Pin client/IDL, provision dedicated devnet identity and test mint, 3–5 h | Two wallets fund a campaign, settlement changes balances once, separate cancellation and timeout cases refund. Save public signatures and balance deltas. |
| F | C, D, E | Build curated entry, campaign, review, receipt and creator flows, 6–10 h | Open a deep link on mobile and desktop, switch wallets, reload a pending transaction, finish via finalized state. |
| G | B, D, F | Invited usability sessions, verification and rehearsal, 3–5 h | Report counts, failures and fixes. Rehearse with a clean profile on a second device. |
| H | E–G | Submission package, 2–4 h | Judge can independently open each link; actual submission confirmation is recorded separately. |

Highest schedule risk is the combined browser-wallet and offline path. Exercise a single wallet transaction and install the reader shell early while the final content is in review. Cut additional campaigns, recognition embellishments, downloads and creator conveniences before cutting settlement, independent reading or refunds. If a required gate fails, record the exact missing capability and adjust the public claims.

## 58. Acceptance matrix

| ID | Setup and action | Required result | Evidence level |
|---|---|---|---|
| F1 | Two sponsors pledge to a fresh campaign; last pledge fills it | Aggregate and receipts equal target, vault principal moves once on release, creators receive target, UI shows finalized | Compiled program + devnet balances |
| F2 | Competing pledges for the last units; retry from stale tab | At most the amount remaining is accepted; losing wallet sees refreshed remaining amount and reviews a new quote | VM + browser |
| F3 | At funding deadline for underfilled or reveal deadline for filled campaign | Pledge or reveal respectively fails at equality; claim works at equality; other sponsor can claim separately | Controlled-clock VM, both branches |
| F4 | Creator cancels before release after partial and full funding; race cancellation with release | One terminal outcome, no payout after cancellation; principal paid once per sponsor even if claim is relayed | VM both orderings, UI + public case |
| F5 | Wrong signer, beneficiary, mint, signer authority, campaign, receipt, duplicate release or claim | Reject unauthorized transfer; compare all affected balances before/after; no mutation on failure apart from applicable network fees | VM |
| F6 | Send unrecorded tokens directly to vault | Progress, payout and claim liability continue to follow receipts; disclose leftover balance | VM |
| W1 | User rejects approval; user changes wallet or network on confirmation; close tab after submission | Bind draft to wallet/network/terms. Invalidate unsigned review on change. Restore sent signature separately and reconcile before any new signature request | Browser with test wallet |
| R1 | Independently opened reader with released campaign and no wallet or campaign site | Verify using configured chain/program and exact artifact, then read full lesson | Fresh-profile browser |
| R2 | Corrupt first copy, serve valid second copy; then corrupt both | First case opens verified copy; second gives recovery choices and displays zero unverified lesson content | Artifact + browser |
| R3 | Reopen saved resource offline; then clear storage, or refuse quota | Cached complete lesson opens offline; missing/partial data shows recovery. App only claims saved after read-back | Real mobile browser, desktop |
| R4 | Supply wrong campaign, truncated ciphertext, altered metadata, unsupported version or malformed plaintext | Fail closed, keep last known good cache, no unsafe rendering | Parser fixtures |
| U1 | Keyboard-only and mobile zoom walkthrough from preview to receipt, reader and refund | Visible focus, meaningful heading/order/status, no trapped focus or hidden primary action; amount and deadlines comprehensible | Manual device pass |
| P1 | Disable recognition, refuse optional feedback, visit as a guest | Public amounts still visible, no name displayed, guest reads without sign-in | Browser, storage/log check |
| P2 | Try unpublished or unauthorised listing; withdraw an admitted creator from future publishing | Curated discovery reflects admission, existing refunds and independently released content remain accessible | Integration |
| J1 | Open all submission links from logged-out browser and follow demo | Program ID, signature, live reader, video and documented commands match the submitted revision | Rehearsal |

For each run record commit, cluster, command or manual steps, start time, versions, expected and actual outcome, pass/fail/skip counts and links. A screenshot of an enabled action cannot establish a transfer. Retain counterexamples and reproduce them after fixing the cause.

## 59. Fixtures and environments

Use a deterministic local clock, synthetic creator and two distinct sponsor signers recreated per test. Seed one classic test mint with zero decimals, no freeze authority, target five base units and source balances greater than five; fund fee/rent separately. T0 funding cutoff and T1 release cutoff are Unix seconds with T0 < T1. Cover T0−1, T0 and T1−1, T1. Local VM balances can be seeded; public demo funds must be identifiable devnet test units. Never check signer bytes into fixtures.

Construct separate campaign nonces for partial (2/5), filled (2+3), released, cancelled with two receipts, underfunded expired and filled unreleased expired. Do not reuse one terminal campaign to demonstrate a different outcome. Create an empty receipt case and a sponsor with multiple pledges. Pair each with the expected finalized state, accrued amounts and allowed actions. For the reader include a tiny redistributable original lesson, deterministic test-only encryption vector, wrong key, truncated artifact and cache eviction case. Real publishing always uses fresh random material. Use local fixtures for deterministic tests, devnet for settlement evidence, an explicitly labeled interactive design for product review.

## 60. Performance and reliability budgets

Proposed engineering targets measured against a cold midrange Android browser at 1.6 Mbps/150 ms RTT and a desktop baseline. Record browser, device and network profile. Homepage text and preview CTA should be usable within 4 s, initial compressed campaign JS ≤250 KiB excluding lazy wallet code. Cache the reader shell separately from the pack. Target ≤2 MiB for the first illustrated lesson, measure actual download bytes and label them before save. Reopen a saved lesson within 2 s offline. Measure three runs, report median and worst; record exceptions with asset breakdown.

Serialize each real transaction including signatures before signing; legacy packet must fit 1232 bytes. Simulate with actual account list and compiled binary, log measured units per instruction against the requested compute limit with headroom. No invented compute figures. Bound host requests and retry one mirror at a time. Never turn a slow RPC into a failed-payment message. Performance failures affecting the core journey block the demo until reproduced and resolved or explicitly disclosed.

## 61. CI and release checks

The existing Rust workflow runs formatting, locked host tests, clippy with warnings denied, compiled SBF build, and Python VM scenarios. Existing commands and dated counts are in [Phase 1 status](../phase-1-status.md). New handlers require new compiled scenarios. Add client type checking, production builds for both origins, protocol vectors and clean-browser tests when those modules exist. Verify generated interfaces match the compiled revision. Run dependency/license review when adding libraries. A failed required job holds its feature PR; a skipped live check remains UNVERIFIED. Keep devnet signing credentials out of PR jobs.

## 62. Diagnostics and support

A support report can contain app version, cluster, campaign, action, public transaction signature voluntarily shared, approximate time, browser version and a short error category. Default to local diagnostics without tracking scripts. Do not include wallet backup phrases, unreleased publication material, full browsing history, personal feedback or private contact details. On a pending transfer show a copyable signature, explorer link and last refresh. Distinguish user rejection, stale campaign terms, RPC timeout, finalized program error, artifact corruption and local storage full. During the invited pilot Dexter handles reports through the invitation channel. Choose a project contact before wider participation. For a reported rights issue pause new promotion, preserve settlement/refund links, and record the publisher's response. Released copies may persist.

## 63. Release and rollback runbook

1. Record source revision, toolchain, binary digest and generated IDL. Verify program ID agrees in source, built client, configured cluster and explorer. Compare deployed executable and upgrade authority to release record.
2. Create test-only mint and validate decimals, authority and accounts. Publish mirrored immutable files, fetch both from the reader origin and verify expected bytes before creating a campaign. Check actual network time and show deadlines in EAT plus local time.
3. Serve versioned static assets over HTTPS. Check deep links on reload, mobile wallet return, both origins, cross-origin fetch policy and service-worker scope. Publish the reader shell independently. Use free-tier services only. A production deploy requires Dexter's approval for that specific action.
4. Rehearse one live success and a separate refund. Capture transaction links, program version, actual time, token deltas and test counts. Test a previously saved reader offline. Perform a logged-out judge walkthrough.
5. On UI regression revert static assets to the previous compatible build and keep historical artifact URLs alive. On program mismatch disable new writes in the client, retain lookup and claim instructions, and investigate before any upgrade. A frontend rollback cannot reverse finalized transactions, erase released keys or refund automatically. Publish a dated incident note with affected campaigns and recovery evidence.
