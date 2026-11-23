# Judging, demonstration and submission

## 64. Judging criteria and evidence

The authenticated organizer pages were inspected on 22 November 2026. Entry targets the overall track. The organizer describes Product and Technical review and further inspection for shortlisted teams. No official scoring weights were found. The design below ties each reported signal to something a judge can inspect. Recheck authenticated requirements at kickoff and before submission. [Organizer FAQ](https://hack.dev3pack.xyz/faq) · [Member qualification panel](https://app.dev3pack.xyz/hackathon/member/dashboard/prizes)

| Signal | What the judge should see | Evidence required before claim |
|---|---|---|
| Product clarity | A student can preview exactly what will become public; a club can sponsor the remaining amount | Actual licensed sample and understandable terms |
| Impact | One student uses the released checklist to prepare an application | Consented session count, task outcome and observed difficulty. No extrapolated beneficiary count |
| Founder-market fit | Named local publisher describes what a learner corrected and the change made | Real feedback and revision, with permission for any quote |
| Demo quality | Watch a sponsor fund the final units, then follow a public reading link | Under-three-minute public video of real flows and a repeatable walkthrough |
| Solana depth and code inspection | Custody, fixed beneficiary, cap, deadlines and individual refunds work independently of the campaign UI | Original program source, compiled tests, rejection paths, receipts and token balance deltas |
| Deployment | Open a dedicated program address on devnet and compare against client config | Public address in README and submission; successful finalized transactions |
| Repository quality | Clone from clean checkout, build pinned revision, run tests, identify original and upstream work | Setup, notices/license, prework record and run output |
| Live verification | Fresh browser reads the released lesson without an account; eligible test participant can exercise signing | Public read-only path; supply invited test access for an interactive judging session |
| Eligibility and handoff | Judge can reach every required entry asset | Check against section 66 and the authenticated form on submission day |

Record actual evidence links in the submission record at release. A planned feature, CI-only simulation or design prototype cannot substitute for a live product link.

## 65. Demo storyboard and rehearsal

| Time | Action | Proof on screen |
|---|---|---|
| 0:00–0:18 | Open the internship guide preview as a student | Cover, contents, sample, author and public permission |
| 0:18–0:48 | Switch to an invited alumni sponsor, contribute the remaining test units | Network label, fixed terms, signed transaction, finalized receipt |
| 0:48–1:12 | Creator releases finished resource | Successful transaction, correct recipient and balance change |
| 1:12–1:48 | Open the independent reader in a fresh browser with no wallet, then save and reopen offline | Readable checklist, verified state and campaign site unavailable |
| 1:48–2:16 | Open a separate eligible receipt and claim its principal | Reason for refund, claim signature and returned units |
| 2:16–2:35 | Show program source, deployment and one real learning from testing if available | Public links and accurate evidence status |

Use distinct browser profiles and pre-seeded devnet cases. Capture one continuous live rehearsal with a clock, then trim only waiting and explicitly label time skips. Prestage a previously finalized case if RPC is slow; disclose when showing that case. Record a local copy of the real successful rehearsal as a fallback. If every live service is down, show that recording as a recording. Do not imply that the design prototype moved funds.

## 66. Submission checklist

Dexter owns the actual submission. Based on the prior authenticated inspection, prepare name (≤50 characters), one-liner (≤120), description (≤300 words), technology selection, matching Solana program address, public source with setup, accessible live product URL, public video under three minutes, screenshots, accurate team information and final review. Verify media bounds in the live wizard. [Project form](https://app.dev3pack.xyz/hackathon/member/dashboard/project) · [Schedule](https://app.dev3pack.xyz/hackathon/member/dashboard)

Draft one-liner: “Sponsor a finished learning guide so every student can read it free, even after the campaign site goes offline.” Test this wording against actual release and offline behavior. Prepare one real campaign link, reader link, refund proof and commit permalink. Check them on a logged-out phone. In the release record store final field text, clip duration, revision, submission time in EAT and confirmation/receipt. Keep private roster details out of public documentation. Target ready by **29 November 2026, 08:00 EAT**. Exact lock minute and the final wizard's current constraints are **UNVERIFIED**; verify from the authenticated organizer page. Entry status remains unsubmitted unless confirmed.

## 67. Prework and originality

| Work | Dated baseline | How to describe it |
|---|---|---|
| Research, alternatives and feasibility experiments | 22–23 Nov, supporting Dev3Pack research workspace | Prior investigation and isolated probes. Do not present as hackathon-window implementation |
| Original plan and concept | Product-plan revision cf0e443; see design board | Pre-event product choices and illustrative design |
| Original Rust program and checks | Revision c383318 and its history; Phase 1 status | Pre-event campaign logic and existing runtime tests. Report dates candidly |
| Planning pack | This branch and commit history | Pre-event specification, design, verification strategy and independent review |
| Event work | Record starting SHA/time at official kickoff; list later commits separately | Attribute actual changes and contributors. Never represent an unchanged dependency as original work |

Read the official prework policy at kickoff. If the policy limits earlier implementation, disclose the baseline and ask the organizer about eligibility before making a claim. Maintain separate entries for third-party libraries, fonts and illustrations with notices and permission evidence. Do not backdate work.

## 68. Risk, assumptions and unresolved evidence

| Risk or assumption | Trigger / owner | Response and fallback |
|---|---|---|
| No publisher-reviewed final pack or asset rights | Dexter, before live campaign | Use only cleared original assets, replace disputed content, obtain actual review before publishing |
| Students or alumni may have little use for this or dislike wallet friction | Dexter, before pitch | Run consented tasks with students first. If unavailable report zero participants and a hypothesis |
| Reveal leaks if broadcast but fails, including competing cancellation | Program owner, before release UI | Explain irreversible disclosure before signing. Simulate, refresh state and avoid simultaneous actions; never promise secrecy after broadcast |
| App invitation mistaken for protocol-wide access control | Product owner, before inviting users | Explain exact boundary, list only reviewed campaigns, keep write actions limited in official pilot; test direct-client behavior |
| Key lost before release or files disappear | Creator, before funding | Verify recoverable private material and two working copies before launch. If recovery fails use cancellation/eligible refund; never reconstruct lost secrets from the server |
| Wallet rejects, devnet resets, RPC rate limits or free hosting unavailable | Release owner, each rehearsal | Restore from signature if possible; never resubmit blindly. Use prior verified case or labeled recording. No paid fallback by default |
| Immutable harmful, incorrect or mislicensed publication | Publisher, before release | Review finished work; remove disputed listing and stop promotion if needed. Public chain history and downloaded copies persist |
| Program, upgrade key, client or IDL differs | Release owner, before any signing | Block writes until pinned identity matches; preserve existing recovery and read paths |
| Free tier, sponsor or schedule fails | Dexter, daily through deadline | Protect one real journey and its evidence. Defer optional reach, extra campaigns and cosmetic polish |
| Organizer changes qualification fields or timing | Dexter, kickoff and final morning | Recheck dashboard and record date; do not infer eligibility or scoring from a successful build |
