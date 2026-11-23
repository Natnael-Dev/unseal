# How people move through Unseal

This is the build brief for a one-resource, English-language pilot. A learner can find out what the guide offers and read a released copy without an account. Signing is reserved for actions that change funds or campaign terms. The independent reader must have its own address and work when the campaign website is unavailable.

## 14. Information architecture

| Destination | Entry and purpose | Required exits |
|---|---|---|
| Curated home | Public root, one featured guide with cover, audience, useful sample and current availability | View guide, learn how sponsoring works |
| Campaign | Stable shareable link keyed by campaign address. Show title, actual creator, preview, license, terms and live funding outcome | Sponsor when open, open reader when released, view own receipt, copy public link |
| Sponsor review | Enter from campaign. Show an amount field and the exact terms that will appear in the wallet flow | Back to campaign, connect or review signature |
| My receipt | Stable campaign-scoped route. Connect the contributing wallet or switch wallets to look up a personal receipt | Claim when eligible, see transaction when known, return to guide |
| Creator workspace | Invited publisher opens setup or manages the specific campaign. An invitation controls the pilot interface. The public program's capabilities are documented separately. | Preview, publish fixed terms, release, or confirm cancellation when supported |
| Independent reader | Direct link includes the campaign identity. Use a separate origin, its own shell and explicit network configuration. Opens in a regular browser. | Read, move between units, save locally, inspect verification, open attribution |

Use stable links that survive reload and direct entry. Preserve the guide when a user returns from a wallet. Unknown addresses, the wrong network, and an unavailable network each get a distinct explanation. A valid but unlisted campaign is never silently replaced by the featured campaign. Never place unpublished material, sponsor nicknames, or wallet session data in share links.

## 15. User journeys

| Person and starting point | Steps and decision | Observable finish |
|---|---|---|
| Student on a phone receives a guide link | Read audience and sample. Before release, inspect contents and share. After release, open reader, choose a unit, save a copy | Can read a full verified unit after disconnecting, without creating an account |
| Alumni sponsor discovers the curated guide | Read creator credit, sample and intended outcome. Inspect two deadlines, public transfer, no early withdrawal, fees and refund conditions. Choose an amount | Finalized receipt shows cumulative contribution and where to return |
| Club representative wants credit | Read public recognition policy. Use an optional display name only with express consent and a stated publication location. Wallet address and amount are publicly observable regardless | Can sponsor without a public display name. If recognition is managed manually, the publisher records opt-in before posting |
| Dexter publishes the finished guide | Check preview, attributed sources, license, encrypted file copies and recoverable release material. Review immutable terms and fixed payee, then sign | Live campaign opens only after the actual creation is finalized |
| Sponsor returns after failure | Open campaign receipt using original wallet. Check eligible balance and fees. Claim independently and inspect result | Refunded receipt remains readable. Other sponsors need take no action |

## 16. Task flows and guards

1. **Sponsor.** Read preview → enter suggested or custom amount → show exact test units and destination → connect wallet → validate network, remaining target, funds and fee balance → review → sign once → reconcile transaction and receipt. A changed amount requires fresh consent. If another sponsor fills the last units, keep the typed amount visible and explain why signing is unavailable.
2. **Publish.** Select finished artifact and public sample → verify rights, credit and mirrors → prepare package → back up release material privately → review title, target, deadlines, payee, mint, network and commitments → confirm immutability → sign → link to live campaign. Allow corrections before signing. On restore, load the previously created account before offering to publish again.
3. **Release.** Open filled campaign → recheck outcome and deadline → verify locally that release material belongs to this campaign → warn that sending it may make the file readable even if payment fails → sign → verify on-chain release and payout → open the independent reader. A missing key leads to recovery instructions and the refund deadline.
4. **Cancel.** Creator reviews exact campaign, amount pledged and effect on each sponsor → confirm explicitly → sign → reconcile finalized state → expose claim links. Cancellation before release makes outstanding contributions individually refundable. Enable this flow only after the implementing program and its tests exist. A competing release or refund requires reloading the actual outcome.
5. **Read.** Open independent link → validate network and release → verify manifest and encrypted file → authenticate decryption → render the guide → save offline → reopen offline. Keep preview distinct from the full document. A failed verification offers another mirror or a matching local file with the same checks.
6. **Claim.** Connect original sponsor wallet → derive receipt for that wallet and campaign → verify status → review exact amount, destination and required fees → sign → await finality → show claimed. A second device should recover the same result from the chain.

## 17. Screen states and precedence

Never infer success from clicking a button, opening a wallet, a returned signature, a local timer, or a vault balance. Use finalized account data for the public outcome. Show observation time when data may be stale. The program clock decides deadline eligibility. Refresh before any write.

| Observed condition | Campaign and reader | Available action |
|---|---|---|
| Loading, no prior data | Neutral “Checking campaign” | Wait or retry on failure |
| Network unavailable, cached data | Last checked time and cached status; disable spending | Read already saved material, retry network |
| Open, before funding deadline | Exact recorded amount and remaining amount, both deadlines | Sponsor |
| Target reached before release deadline | “Fully funded. Waiting for release.” No full-reader success claim | Creator can release immediately; others can view preview |
| Below target at funding deadline | “Funding ended below the target.” | Claim own outstanding amount |
| Filled at release deadline, unreleased | “Release deadline passed.” | Claim own outstanding amount |
| Cancellation confirmed | “Creator cancelled this release.” Where there are contributions, explain individual claims | View receipt and claim if eligible |
| Refunding with partial or complete claims | Continue showing original totals and personal receipt outcome | Each unclaimed sponsor may claim, already claimed sponsors see the existing receipt |
| Released | Fixed payee, public reader link, no further claims | Open reader. File retrieval or verification can still fail independently |
| Unknown version, unexpected account or wrong network | Explain which campaign/network the page expected | Read supported public info, return or reconnect. Disable signing |

Receipt independently supports no wallet, wrong wallet/no receipt, contributed, eligible, pending claim, claimed and unknown. A missing receipt after an unavailable lookup is an error, never a zero balance. Reader independently supports never saved, waiting, verifying, readable, saving, saved, storage full, missing cached bytes, verification failed and offline without a usable copy.

## 18. Wireframes and reading order

| Screen | Wide layout in reading order | Narrow layout in reading order |
|---|---|---|
| Home | Wordmark and navigation; single publication feature with cover beside title, audience and availability; sample; how sponsorship works | Title and status before a restrained cover, sample, view guide, explanation |
| Campaign | Book cover on left; title, author, summary, sample, funding and primary action on right; below are contents, creator, license, both deadlines and receipts | Back/home, title, author, compact cover, sample, funding state, action, contents and full terms |
| Sponsor | Compact review column with amount, remaining units, fixed payee, fees and two dated terms; signature control at end | One column, amount first, wallet summary and signature action after terms. Amount remains visible with software keyboard open |
| Creator | Guide identity beside publish/release status, checklist, fixed terms, primary action; separate cancellation area below | Identity, prerequisites, fixed terms, primary action, cancellation disclosure |
| Receipt | Campaign link, connected wallet, cumulative amount, status, claim or recovery instruction and transaction details | Same order. Full address copy beside abbreviated display; wrapping details below |
| Reader | Guide title, contents navigation, article column, verification and offline controls alongside article | Title, status, contents disclosure, article, attribution and offline controls; reachable save action near top |

The DOM follows reading order. Use one main heading per route and one primary action per decision. Native links handle navigation and open-in-new-tab. A desktop side panel becomes an in-flow section on narrow screens.

## 19. High-fidelity screen handoff

The [four-panel concept board](../design/unseal-concept.png) sets the cover-led campaign, compact mobile sponsorship, creator's funded release panel, and text-led independent reader. The [responsive walkthrough](../design/walkthrough.html) adds home, receipt, campaign terms, release review, cancellation, draft publication, and offline and refund scenes. Open the HTML file locally with the PNG beside it. Keep the board's book proportions, left alignment and typographic hierarchy. The concept's sample amounts, dates, copy, byline and labels must be replaced by verified content and runtime data. Its generic “Guide creator” label becomes the actual published identity plus inspectable fixed address. Show both real deadlines with zone and retain all reviewed terms before signing.

For home, enlarge the book feature and surface the useful sample without a wallet prompt. For receipt, lead with the personal amount and whether anything is actionable. Reuse the campaign terms and status components. Build separate desktop and narrow views of open, filled, released and refund states using labeled fixtures during review. Capture each real route at the narrow and wide test sizes before claiming visual parity.

## 20. Prototype rehearsal

Use the walkthrough's “Review scene” selector to choose a labeled fixture. Under “Funding open,” contribute two example units, open Creator, review release, and follow the independent reader link. Under “Deadline missed,” open Receipt, use the example wallet and claim. The prototype runs locally and resets on reload. Use a separate browser profile for the unrelated learner when testing the implemented product. Real wallet operations, persistent offline recovery, concurrency and a live end-to-end recording require separate evidence.

| Scene | Reviewer action | What the reviewer must see |
|---|---|---|
| Discover | Open curated home on a phone, follow the guide link, sample a unit | Realistic learning task, author and license visible. No wallet request |
| Sponsor | Type an invalid amount, correct it, dismiss wallet, then contribute | Field explains correction; dismissed action preserves input; only finalized result changes funding |
| Release | View a filled guide as creator, review fixed recipient and send release | Irreversible key exposure disclosed before signing; independent reader link after finality |
| Read | Open independent link in clean wallet-free browser, save, disconnect, reload | Verified full lesson and checklist remain readable on that device |
| Refund | On a separate failed campaign, reclaim one of two sponsors' contributions, refresh | One receipt claimed, second still eligible. A repeated claim offers its recorded result |
| Recover | Disable campaign origin, corrupt one mirror, interrupt confirmation, change wallet | Independent reader still operates with a valid source; stale wallet and pending states never authorize a second payment |

## 21. Interaction rules

- Show precise review details before asking for a signature. Disable duplicate clicks while wallet approval or confirmation is unresolved. Switching accounts or network clears the old approval and rechecks the receipt.
- Store the submitted signature with campaign, cluster, wallet and operation so reload can resume lookup. A timeout may mean the transaction is still pending. Reconcile signature and chain state before offering a fresh attempt. Never automatically resubmit a payment.
- Keep amount strings exact through display and parsing. Reject negative, zero, excess precision, exponent notation, more than the remaining target or unavailable funds. Use mint decimals for conversion and do arithmetic in integer base units. Never silently round, clamp, or change the last sponsor's amount.
- Show precise timestamps with an explicit zone, including the date and year, alongside any countdown. Stop offering the time-sensitive action while rechecking at the boundary. Use explicit text for canceled wallet approvals, transaction failures and uncertain confirmations.
- “Copy” briefly confirms success in place and offers selectable text if clipboard permission fails. Use visible back navigation from every multi-step flow. A wallet approval or destructive confirmation never auto-opens when a page loads.
