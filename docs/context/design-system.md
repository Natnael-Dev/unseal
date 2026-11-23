# Unseal interface and publishing standards

Use these rules with [the screen and journey specifications](design-experience.md). They define the targets for implementation and acceptance. They do not establish that accessibility, offline behavior or wallet integration has passed testing.

## 22. Copy and terminology

| Situation | User-facing copy pattern |
|---|---|
| Funding | “{pledged} of {target} test units funded.” Display “Solana devnet” and “Test units have no cash value.” |
| Sponsorship | “Sponsor this release.” Review says “If the guide is released, {publisher} receives the funded amount. You can read it whether or not you contribute.” |
| Public data | “Your wallet address and contribution are public. Displaying a name is optional.” Put this beside review, ahead of signing |
| Fees | “Unseal charges no platform fee. Your wallet needs devnet SOL for network fees and any new accounts.” Show estimates with their units or explain why an estimate is unavailable |
| Deadlines | “Funding closes {date, time, zone}. If the target is reached, the creator can release the guide until {date, time, zone}.” Explain individual claims for missed deadlines or creator cancellation |
| Pending | “Waiting for confirmation. You can reopen this receipt to check.” Include the signature when available |
| Wrong mint or network | “This campaign uses {mint} on Solana devnet. Switch to the campaign network to continue.” Show inspectable mint address |
| Wallet declines | “Signing was cancelled. Review your amount and try again when ready.” |
| Verified file | “This copy matches the published release.” Keep identity, publication rights and editorial review in their own credits |
| Failed verification | “This copy does not match the published file. Try another copy.” Withhold content and success indicator |
| Offline | “Saved on this device” only after durable writes succeed and can be read back. “Storage is full. Free space and try again.” when applicable |

Use “guide” for the publication, “sponsor” for the contributor, “claim refund” for the action, and “creator” or the actual byline for the payee. No promise of a job, fixed asset price, instant finality, automatic refunds, quality refunds, permanent hosting, or cross-device offline access.

## 23. Accessibility acceptance

- Set English language metadata, landmark navigation and skip link. Heading order follows the article. Link text identifies its destination. Form controls have persistent labels and visible instructions; bind field errors to the relevant control.
- Reach every action with Tab, Shift+Tab and keyboard activation. Opening a confirmation moves focus to its heading or safe control; Escape closes optional overlays; closing restores the trigger. Failed review focuses the first invalid field. Navigation updates the title and moves focus to the new page heading.
- Use visible text together with every status icon. Announce user-triggered status changes politely once; reserve alerts for blocking failures. Avoid live countdown announcements. Amount progress has an accessible label and exact amount text.
- Target at least 4.5:1 for normal text, 3:1 for large text and applicable interface boundaries. Verify focus on both panel and primary-action backgrounds. Keep actionable surfaces at least 44 by 44 CSS pixels. Respect reduced motion and forced-color preferences.
- At 200% text size and 320 CSS pixels wide, verify reading order, input errors, focus visibility, address wrapping and the ability to reach all controls. Test keyboard-only navigation, a desktop screen reader and a phone screen reader on the full read and sponsor flows. Record actual browser, device and failures.
- The guide renders as structured HTML with headings, lists, meaningful illustration descriptions and a text checklist. A reader can understand each example with images disabled. Content inspection must reject unsafe markup rather than inject uploaded HTML.

## 24. Responsive behavior

Use intrinsic wrapping first. Start with one column, 16 px page gutters, full-width review controls and 16 px minimum input text. Increase gutters at wider sizes. Introduce side-by-side panels only when the main reading column and secondary controls both fit. Around 800 CSS pixels is a starting point to validate with actual content. Cap overall layout around 1160 px, reader measure around 65 characters and review form around 36 rem.

Test 320, 390, 768 and 1280 CSS pixels; repeat in landscape, with the mobile keyboard visible, large text and long creator names. Render long wallet addresses in a wrapping details panel. Avoid fixed bottom controls covering the last paragraph, input errors or phone safe areas. Keep wallet approval usable after returning from another application. Network or account switching invalidates the prior review.

## 25. Low-bandwidth and offline experience

1. Campaign terms, sample text and actions arrive ahead of large cover artwork. Set image dimensions to avoid layout jumps and request the size needed for the viewport. A failed image leaves title, attribution and sample available.
2. A first visit to the independent reader needs connectivity or a separately supported verified import including release evidence. Tell a first-time offline visitor what must be completed online.
3. Saving includes the reader shell, validated release evidence, immutable verified lesson bytes and illustrations. Bind the saved record to cluster, program, campaign, format and digests. Only show “Saved on this device” after complete persistence and an integrity check. A partial write does not replace the last working copy.
4. On offline reopen, read the saved evidence, check the file identity again, and state when release was last verified online. Private browsing, storage eviction and clearing site data can remove copies. If bytes are unavailable or corrupt, keep the shell usable and offer recovery once connected.
5. Test a clean browser, throttled connection, abandoned download, mirror failure, storage denial, offline reload, missing asset, and clearing saved data. Chain writes always require a current network connection. The campaign origin can be offline while the independent reader is online.

## 26. Brand direction

Present one usable publication, with a clear cover, real byline, generous article measure and a meaningful excerpt. Keep the approved ivory paper, charcoal ink, forest-green actions and restrained terracotta details. The book cover carries the visual emphasis. Show funding in exact numbers next to a continuous meter and never imply that readers have to buy the lesson. Use the Unseal wordmark as text. Reserve colored status marks for states that also have a written label.

## 27. Design tokens

| Token | Initial design value | Use |
|---|---|---|
| Paper / raised paper | #FAF5EB / #FFFCF5 | Page and reading surface |
| Ink / secondary ink | #202520 / #4D584E | Headings, paragraphs and explanatory text |
| Forest / on forest | #12432D / #FFFFFF | Primary actions and their text |
| Clay | #913F22 | Small emphasis or errors accompanied by a label |
| Rule / strong control outline | #D7CEBF / #716A5E | Section dividers / visible form boundaries |
| Focus | #185EB1 with contrasting outer gap | Keyboard focus on both pale and dark surfaces |
| Display / body | Instrument Serif, Georgia, serif / Instrument Sans, system-ui, sans-serif | Titles / controls and body. Bundle licensed local font files or use fallbacks offline |
| Type scale | 14, 16, 18, 24, 32, 48 px equivalents in rem | Metadata, controls, body, section, compact title, wide title |
| Spacing | 4, 8, 12, 16, 24, 32, 48, 64 px | Internal padding through page sections |
| Geometry | 44 px minimum action height, 6 px control radius | Primary controls and inputs |

Use approximately 60% surface, 30% type and structure, 10% accent as a visual hierarchy. Verification still requires measuring actual pairs and states. Avoid using the decorative rule token as the sole input boundary.

## 28. Component contracts

| Component | Inputs and behavior | Acceptance check |
|---|---|---|
| Publication identity | Title, credit, safe cover, descriptive sample and rights | Unavailable cover still leaves meaningful text |
| Funding summary | Recorded pledged, target, decimals, network, state, last checked time | Text and progress agree; stale or unsupported values disable writes |
| Terms sheet | Fixed payee, mint, target, deadlines, cancellation/refund policy, fees | Full addresses inspectable; approved review matches the eventual wallet action |
| Amount input | Exact decimal string, suggested amounts and remaining cap | Type custom value, correct an error, review again after cap changes |
| Wallet action | Account, cluster, operation, approval/confirmation stages | Reject, change accounts, reload mid-flight; never duplicate a transfer |
| Receipt | Campaign plus connected wallet, cumulative contribution/refund state | Using another wallet never exposes the prior personal receipt as its own |
| Creator controls | Verified terms, admission, live funding state, release preflight | Release and cancel have distinct confirmations and outcomes |
| Reader toolbar | Version, locally verified status, save progress, persisted state | Offline and integrity states are independent; failed save preserves readable content |
| Status panel | State-specific title, detail, recovery action | Error remains readable without color and retry is explicit |

Use conventional buttons for actions and anchors for navigation. Pair icons with text. Only render working controls; give an explanation when a necessary control is unavailable.

## 29. Illustration and asset handoff

The existing [concept artwork](../design/unseal-concept.png) is a design reference. The published guide needs its own attributable cover and interior art, with rights checked asset by asset. Use an original campus or student-work illustration, avoiding implied institutional endorsement. The cover conveys subject and audience. Interior art should clarify a specific step such as converting a project into an application example.

Track asset identifier, creator, source, license or permission, required credit, where used, alternative text, dimensions, derivative edits and publisher approval in the resource inventory. Retain source art outside the immutable package. Export web-sized cover variations and bounded PNG illustrations for the verified lesson format. Strip unnecessary personal metadata. Check small-screen crops, lossless readability of essential details and an image-disabled version. Repeat text drawn on the cover in real HTML. Use empty alternative text for a redundant decorative cover beside an identical title; describe any unique information conveyed by the image.
