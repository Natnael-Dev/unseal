# Portable publication and independent reader

The following v1 wire format is a design contract for new code. Existing research vectors demonstrate feasibility and use different header literals. Generate new public vectors against this document before shipping.

## 45. Manifest, preview and lesson schemas

A publication consists of three byte streams named `manifest.json`, `preview.json` and `lesson.unseal`. Locators carry no authority. Terms in the creator-signed campaign commit to SHA-256 of the exact manifest and envelope bytes. Hash before parsing or normalizing line endings. All JSON uses strict UTF-8 without BOM, objects with unique keys, no unknown fields in v1. Reject invalid characters, invalid lengths, impossible types, unsupported versions and unsafe numeric values. Use a duplicate-key rejecting decoder or validation pass before ordinary JSON object construction. Public keys use canonical base58 of exactly 32 bytes. Digests use 64 lowercase hex characters.

| Required top-level manifest field | Rule |
|---|---|
| version, program, campaign | Integer 1 and two canonical public keys matching the verified deployment/account. |
| title, author, bio, rights | Plain text only; lengths 1–120, 1–100, 1–400 and 1–600 Unicode code points, respectively. Rights declares authorship/permission and attribution. Optional profile URL is separate curated metadata; it never enters the renderer automatically. |
| license | Exact string `CC-BY-4.0` for this pilot. Display title, named author, license link and source campaign; preserve modification notices on derivatives. |
| language, mediaType | Exact strings `en` and `application/vnd.unseal.lesson+json`. |
| previewSha256, previewBytes | SHA-256 of the public preview; integer byte count 1–524288. |
| ciphertextSha256, ciphertextBytes, ciphertextCid | SHA-256 of complete envelope, byte count 29–5242908, and canonical CIDv1 raw sha2-256 encoded base32 lowercase with b prefix, all mutually consistent and matching campaign terms. |
| plaintextSha256, plaintextBytes | SHA-256 of the final lesson JSON bytes; size 1–5242880. Require ciphertextBytes = plaintextBytes + 28. |

Bound manifest at 16384 bytes. The public preview is a JSON object with `version:1`, `cover` (an illustration as defined below), `contents` (1–24 nonempty section titles, each ≤120 code points), and `sample` (one complete section). Preview and full text use UTF-8 and the same block renderer. The lesson is an object with `version:1` and `sections` (1–24 sections). A section has `title` (1–120 code points) and `blocks` (1–60 blocks). Each block matches exactly one form:

- Paragraph: `{"type":"paragraph","text":"..."}` with 1–4000 code points.
- Checklist: `{"type":"checklist","items":["..."]}` with 1–24 items, each 1–320 code points. Checkbox state, if offered, is local and resettable; never expose a student's private answers on-chain.
- Illustration: `{"type":"image","alt":"...","png":"..."}` with meaningful alt text 1–300 code points and base64 padded RFC 4648 PNG bytes. Validate PNG signature, enforce ≤256 KiB decoded per image and dimensions ≤1600×1600 pixels from header before decoding. Reject animation and embedded external resources. Render decoded image only as a local blob with text alternative. Release object URLs when no longer needed.

Resource-specific approval: every contents entry matches the corresponding final section title, and the public sample reproduces one genuine final section. Check at packaging while both inputs are available. Escape as text in semantic HTML. No dynamic markup, scripts, remote images, arbitrary files, links with active schemes or executable content. Keep public source citations in plain text blocks or a reviewed static editorial credits page. Keep funding, deadlines and amounts out of authored manifest metadata; bind those independently through campaign terms.

## 46. Envelope, authentication and publication material

1. Fix actual program ID, creator, nonce and derive the campaign PDA. Generate fresh cryptographically random 32-byte K and 12-byte IV. K is publication material specific to this campaign and unrelated to any wallet. Treat it as confidential until intended release. Never reuse a K/IV pair. Re-encrypting a revision creates fresh material/IV, digests and terms.
2. AAD = UTF-8 bytes of `unseal/aes-v1` followed immediately by 32 raw program-ID bytes and 32 raw campaign-PDA bytes. No NUL, length prefix, separator or text encoding of keys. Encrypt exact lesson JSON bytes using AES-256-GCM with K, IV, this AAD and a 128-bit tag. Envelope = IV (12 bytes) || ciphertext || tag (16 bytes). Mirrors store these exact bytes independently of file name.
3. Ciphertext digest = SHA256(envelope). CIDv1 raw digest bytes = hex `01 55 12 20` || digest; base32-lower multibase when displayed. Commitment = SHA256(UTF8(`unseal/reveal-v1`) || rawProgramID || rawCampaignPDA || K), matching the current Rust code. Manifest contains neither K nor a hash of itself. Terms store SHA256(exact manifest file bytes), ciphertext digest and commitment as raw bytes.
4. Independently reopen both mirror copies, inspect content and compare SHA-256 before creation. Keep an offline recovery copy of the private publication material under creator control, labeled with program/campaign; exercise a restore before any pledge. Never use a public fixture key for a pilot release. Lost K has no replacement for that campaign. Treat any pre-release disclosure as irreversible and record whether the campaign is still cancellable/refundable.
5. Commit a deterministic *synthetic* vector with public test program ID, campaign, material, IV, AAD, manifest bytes, envelope bytes, expected digests/CID and decrypted lesson. Two independent implementations must compute the same bytes, then reject a one-byte change in IV, tag, program, campaign, manifest or plaintext claim. Production creation uses platform randomness regardless of test fixture inputs.

## 47. Wallet-free independent reader

A reader URL may carry the campaign address. The reader opens its own bundled network profile, never an untrusted program ID or arbitrary remote hostname from the URL. Fetch a finalized campaign with context slot, compare genesis hash, account owner, discriminator, version, length and PDA derived from embedded creator/nonce. Display creator, campaign and network identity. Refuse to unlock from a pasted key, arbitrary JSON 'receipt', unfinalized signature, or state other than Released.

Load bounded manifest and envelope from either trusted mirror, disk import or prior local cache. Compare both raw digests to chain terms, then parse manifest. Verify campaign/program identity, CID and byte counts. Recompute reveal commitment from recorded material. Decrypt with exact AAD, check plaintext size/hash, validate lesson schema and render semantic section headings, lists and alt text. Treat malformed UTF-8, wrong schema, failed authentication and altered bytes as distinct from 'release pending'. Never show unverified content as a successful release. No wallet prompt, approved creator API, tracking endpoint, campaign-origin cookie or access token required.

Offer two save operations: pre-release encrypted files for later verification, or post-release saved reading with finalized slot, network/program/address, exact account bytes and verified manifest/envelope. Save the reader shell on its own origin and make cache completion explicit. On offline reopening, use only that reader's previously verified release record and cached files, rerun byte checks and show 'Saved on this device' with slot/last verification time. Fresh device or imported files without prior trusted release evidence require one online finalized read. Offline mode assumes the integrity of that browser/device; local storage is not an independently authenticated proof of chain state. Exported public lesson includes author/license/source. Site data deletion and cache eviction require re-import or redownload.

Acceptance rehearsal: in a clean profile on reader origin, with campaign origin blocked and no wallet installed, open the released lesson via second mirror or file import and independent RPC. Save; close the tab, block all network, reopen reader and inspect one illustration and checklist. Uncached profile offline must explain what is missing; no success placeholder.

## 48. Mirrors and availability

Configure two independently reachable HTTPS origins and fixed path templates such as `/packs/<campaign>/manifest.json`, `preview.json` and `lesson.unseal`. Bundle both roots into the reader or a pinned reader build configuration independent of the campaign site. Serve exact bytes with cross-origin read access for static public files, no cookies, no server-side transformations or HTML soft-404s. Verify from a real reader origin; a local HTTP 200 cannot establish readable cross-origin access.

Bound actual bytes while streaming even when Content-Length is absent or false. Cancel downloads on cap, user cancellation or timeout. Try each mirror at most once per action after network/size/integrity failure. A valid second copy may be used with an availability diagnostic. Reject redirects to unknown origins or insecure transport. Failed copies must never replace a verified cache entry. Preview must pass manifest digest/size validation before being described as publisher-committed. If both files are unavailable, offer local import and retain claim/release account visibility. Before funding opens compare manifest, preview and envelope at both hosts; before signing a pledge require at least one full verified copy available with honest status of the other mirror. Recheck both before live demonstration.
