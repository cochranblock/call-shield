<!-- Unlicense — cochranblock.org -->

# Accessibility — Section 508 / WCAG Compliance

*Call Shield is a CLI application. This document covers CLI accessibility.*

## CLI Accessibility

| Criterion | Status | Evidence |
|-----------|--------|----------|
| `--help` completeness | Done | `call-shield --help` shows all commands, usage syntax, and examples |
| `-h` short form | Done | Alias for `--help` |
| `--version` flag | Done | `call-shield --version` prints version |
| `-V` short form | Done | Alias for `--version` |
| Exit codes | Done | 0 = success, 1 = error (bad input, missing args) |
| Error messages to stderr | Done | Errors print to stderr, not stdout |
| Human-readable errors | Done | Errors include what went wrong and how to fix it |
| No color-dependent output | Done | Output is plain text, no ANSI color codes |
| Screen reader compatible | Done | Plain text output, no cursor manipulation, no progress bars |
| Pipe-friendly output | Done | Structured output (input/verdict/score on separate lines) |

## Error Message Examples

```
$ call-shield foobar
unknown command: foobar
run 'call-shield --help' for usage

$ call-shield classify
usage: call-shield classify <text>
  provide caller transcript text to classify
```

Both errors: print to stderr, exit code 1, tell the user what to do next.

## What's Not Applicable

- **WCAG color contrast:** No visual UI (CLI only)
- **Keyboard navigation:** Terminal handles this
- **ARIA labels:** No web interface
- **Touch targets:** No mobile UI yet
- **Font sizes:** Controlled by user's terminal

## Planned (GUI/Mobile)

When egui GUI or mobile interface is added:
- Touch targets >= 44x44px
- Font size >= 14pt default
- High contrast mode
- Screen reader labels on all interactive elements

---

*Last updated: 2026-03-27*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
