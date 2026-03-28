# Compression Map — Call Shield

> P13 tokenization. Every public symbol gets a compressed identifier.

## Functions

| Token | Name | Location |
|-------|------|----------|
| f0 | main — entry point, CLI dispatch | src/main.rs |
| f1 | help — print usage | src/main.rs |
| f2 | version — print version | src/main.rs |
| f3 | classify — intent classification on text input | src/main.rs |
| f4 | score — pattern-match classifier | src/main.rs |
| f5 | govdocs — print embedded compliance docs | src/main.rs |
| f6 | govdocs_all — dump all compliance docs | src/main.rs |
| f7 | screen — interactive call screening session | src/main.rs |
| f8 | decide — routing decision from session state | src/main.rs |
| f9 | print_stats — display session statistics | src/main.rs |
| f10 | sbom_spdx — machine-readable SPDX SBOM | src/main.rs |

## Types

| Token | Name | Location |
|-------|------|----------|
| t0 | ClassifyResult — classification output | src/main.rs |
| t1 | SessionStats — running stats for screening session | src/main.rs |

## Fields

| Token | Name | Parent |
|-------|------|--------|
| s0 | spam_patterns — phrases indicating spam | f4 |
| s1 | legit_patterns — phrases indicating legitimate caller | f4 |

## Error Variants

| Token | Name | Parent |
|-------|------|--------|

## CLI Commands

| Token | Name | Description |
|-------|------|-------------|
| classify | classify | Classify caller transcript text |
| screen | screen | Interactive call screening session |
| govdocs | govdocs | Print embedded federal compliance docs |
| --sbom | sbom | Machine-readable SPDX SBOM |

---

*Updated as symbols are added. Token numbers are append-only — never reuse a retired token.*
