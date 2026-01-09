# Book Style Guide
**Why**: Keep the book consistent and easy to skim.
**How it fits**: Use this before adding or refactoring pages.
**Learn more**: See [Linking Style](linking_style.md) and [docs.rs Alignment](docsrs_alignment.md).

## Chapters & structure
How the book is organized and what belongs where.
| Area | Scope |
| --- | --- |
| Top-level | workspace overview, feature flags, build/dev workflow. |
| Per-crate | `10_crates/<crate>`: overview, public API, lifecycle/data flow, module map, error model, performance notes, examples, design review. |
| Cross-crate | canonical flows, integration contracts, docs.rs alignment, linking style, quality gates, changelog, reader’s guide. |

## Naming conventions
Consistent naming and placement rules for new pages.
| Convention | Guidance |
| --- | --- |
| Paths | `10_crates/<crate>/0X_*.md` for crate pages; `00_workspace/*.md` for workspace-wide topics. |
| Headings | Title Case; keep consistent order per crate. |
| Templates | reuse `_templates/crate_template.md` when creating new crate pages. |

## Cross-link style
Linking rules to keep references stable and readable.
| Pattern | Example |
| --- | --- |
| Relative links | `../<crate>/01_overview.md` for intra-book references. |
| Source links | `crate/path.rs:L123` format when pointing to GitHub code. |
| Cross-references | Mention related crates/flows inline; avoid duplicate content by linking to existing sections. |

## Auto-generated vs manual
Which sections are machine-derived versus hand-curated.
| Type | Items |
| --- | --- |
| Auto-generated | feature flags list, pub API tables, module maps, canonical flow diagrams. |
| Manual curation | lifecycle narratives, design reviews, performance notes, examples, error model nuances, integration contracts. |

## Examples & code blocks
Rules for concise, copyable examples.
1) Prefer small, focused snippets; mark non-runnable blocks with `ignore`/`no_run`.
2) Include inputs/outputs when helpful; tie examples to public API.

## Diagrams
When and how to use Mermaid.
- Use Mermaid for dependency graphs and flows; place inline in relevant pages.
    - Keep diagrams minimal and update when APIs/flows change.

## Quality
Baseline expectations for every page.
1) Each page should answer: what is this, how to use/extend it, boundaries, failure modes.
2) Keep prose concise; favor tables/diagrams over long text.
3) Note NdArray default and feature-gated GPU paths where relevant.
