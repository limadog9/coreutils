# GNU parser corrections: independent review evidence

Implementation commit: **`8165b5aeaebf9a7af2559fc6549f923905a852fe`**

Code branch: [feature/gnu-parser-ls-cat-rm](https://github.com/limadog9/coreutils/tree/feature/gnu-parser-ls-cat-rm).
Original upstream base: `dd74d76072c69ae9ef1effb50599d49d6f360898`. Previously reviewed implementation: `7d19d6347a35f8b476e9f5b76cefb19b2145d6db`.
This evidence-only branch has independent history and contains no implementation files, workflows, binaries, caches or bulk logs. It must not be merged into the implementation or its eventual upstream PR.

## Findings and disposition

All **211** original flagged outcomes are reconciled without omissions: **2 introduced rm advice regressions, corrected; 115 GNU corrections with another mismatch remaining; 84 diagnostic/extension decisions; 10 checker presentation misclassifications**. No unexplained group remains. These assessments are not maintainer approval.

The runtime correction retains existing dash-filename advice alongside the primary scanner error. Tests failed before correction and passed afterward. The checker now distinguishes reachable help/version requests from ordinary banner-shaped output and checks each previously GNU-matching component independently. A proven inherited directory-atime fixture race has a narrowly controlled year display.

**The strict gate still exits 1. All 201 changed diagnostic/interface outcomes remain failures pending explicit review. No allowlist or approved exception was added.** The ten former presentation cases now classify as exact GNU semantic fixes. Only the two repaired rm cases changed runtime output; the other 209 original cases reproduced byte-for-byte.

| Fresh seed | Cases | Match | Fixed | Baseline difference | Changed difference | Extension match | Exit |
|---|---:|---:|---:|---:|---:|---:|---:|
| 4254 | 1516 | 886 | 95 | 301 | 201 | 33 | 1 |
| 1729 | 1516 | 891 | 96 | 295 | 201 | 33 | 1 |
| 20261017 | 1516 | 890 | 94 | 298 | 201 | 33 | 1 |

All remaining classification counts, health failures and reference verification errors were zero. GNU reference version is pinned to **9.12**, with binary hashes in the differential summaries. Additional seeds were selected before execution.

## Files

- [Complete original-211 reconciliation](original-211-reconciliation.json): all case IDs, raw argv, exact reference/base/final streams and rm snapshots, previous candidate overrides, 13 root-cause groups and source locations. Large JSON: use GitHub's Raw/Download view if needed.
- [Three fresh differential summaries](differential-summaries.json): complete category tables, per-utility counts, health, pinned references, actual exit status and seed/fixture disclosure.
- [Exact build configurations and verification limits](build-configurations.json): compiler, features, release settings, build arguments, binary identifiers and explicit unverified checks.

These are existing validation results, prepared for publication without rebuilding or rerunning broad development work. Fresh validation included 63 Python CI tests (29 checker tests), 108 uucore tests, 38 utility unit tests, 409/401 enabled integration tests with/without diagnostics (including confined root cases), 905 adapter comparisons, 492 tooling/localization processes, and 47 passing GNU tests with one gdb skip. Help/version/manual/completion bytes were unchanged. Formatting, Clippy, Rustdoc, Ruff and spelling checks passed. Existing ignores and platform limits remain disclosed in the configuration file.

Controlled test inputs are included where necessary to interpret cases; process environment dumps, credentials, personal emails, local validation archives and bulk logs are excluded. No PR, issue or upstream contact is part of this publication.
