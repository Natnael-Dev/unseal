# Unseal development context

## 69. Context index and working instructions

Start with the product and experience. Follow one learner from preview to reading, one sponsor from contribution to receipt or refund, and one creator through release or cancellation. Then review the technical contracts in the same order as the data flows. The chapter numbers match the original 70-item inventory.

| Read | Chapters | Use this for |
|---|---|---|
| [Product and publication](product.md) | 1–13 | Requirements, all 35 decisions, content outline, rights, fixed terms, real evidence and pilot responsibilities |
| [Screen and journey review](design-experience.md) | 14–21 | Navigation, wireframes, layouts, state changes, prototype and recovery |
| [Design system](design-system.md) | 22–29 | Copy, accessibility, responsiveness, offline interactions, styles and assets |
| [Architecture](technical-architecture.md) | 30–37 | Stack, boundaries, storage and interfaces |
| [Settlement](technical-program.md) | 38–44 | Authorities, transactions, deadlines, liabilities and cancelled campaigns |
| [Publication and reader](technical-publication.md) | 45–48 | Precise bytes, commitment, reader and mirror contracts |
| [Security and engineering](technical-operations.md) | 49–56 | Threats, credentials, reproducible setup, repository rules and dependencies |
| [Delivery and verification](delivery.md) | 57–63 | Backlog, dependencies, fixtures, acceptance tests, budgets, support and rollback |
| [Hackathon handoff](submission.md) | 64–68 | Rubric, demo, release checklist, prework and risks |
| This index and [readiness](readiness.md) | 69–70 | Reading order, source precedence, audit and remaining gates |

The 24 November decision log governs intended behavior. The actual Rust code and historical [Phase 1 test evidence](../phase-1-status.md) govern statements about existing behavior. New program rules, reader behavior, wire formats and screen designs describe work to implement. When there is a discrepancy, record the affected decision number and test before changing either contract. Publish a protocol revision for breaking byte-level changes. Never correct a published pack by overwriting immutable bytes.

Use a focused branch and one Conventional Commit per meaningful change. Include actual commands, exit codes and counts in the review. Do not commit credentials or user data. Protect main and require Dexter's single-action approval for production deployment or other protected operations. No spending or external outreach is implied by a delivery task. Record each gate with a dated pass, fail or UNVERIFIED and an evidence link. Start the next implementation slice after product and design review.
