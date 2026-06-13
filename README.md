# bsmell

For agents that take shortcuts. bsmell scans an agent's session transcript or staged diff for known deflection patterns: mock-instead-of-fix, comment-out-instead-of-debug, rename-test-instead-of-resolve. Matches emit a SMELL-DETECTED directive describing the shortcut and the substantive fix. The smell taxonomy is closed (12 patterns at v0.1); the prompt library evolves continuously via empirical-lift evaluation, so the same `bsmell verify` invocation gets stricter at catching shortcuts as the corpus matures.


Prompt lookup tool. Agent names a smell category from a fixed list; bsmell returns the prompt for that smell category. The prompt tells the agent how to check the session for that smell.

Built for agentic loops. Scans a session transcript or a diff buffer, matches against a closed 15-category smell taxonomy, writes a verdict on stdout, exits with a discriminating code so the calling agent can branch.

```
bsmell scan                       scan a session or diff against the smell taxonomy; exit 0 / 1 / 2 / 64
bsmell categories                 list the 15 supported smell-category identifiers
bsmell init                       scaffold a manifest in the current directory
bsmell update                     self-update to the latest published version
bsmell tail                       stream recent verdict transcripts
bsmell explain                   print taxonomy + exit-code reference
```

Exit code contract: `0` clean (no smell), `1` smell detected, `2` internal error, `64` malformed input.

## Install

```sh
cargo install --git https://github.com/bvasilenko/bSmell
```

## Use

```sh
bsmell scan --session ./session.jsonl
# stdout: CLEAN
# exit: 0

bsmell scan --diff ./PR-feature-x.diff
# stdout: SMELL-DETECTED red-herring at chunk 3
# exit: 1
```

Optional flags: `--session <path-or-fd>`, `--diff <path>`, `--manifest <path>`, `--json`, `--quiet`, `--reason <text>`. Subcommands consume the same flag set; defaults are sane.

## Smell taxonomy

Closed 15-variant `SmellCategory` enum. The taxonomy is fixed at this version; widening lands in a later version.

| Category | Variants |
|---|---|
| Engineering | `try-except-swallow`, `hardcoded-value`, `mock-instead-of-fix`, `silent-success`, `unaudited-edit`, `acceptable-degradation` |
| Scope | `scope-creep`, `scope-shrink` |
| Behavioral | `pride-defense`, `time-sink`, `red-herring`, `symptom-mute` |
| CMS edge | `schema-violation-silent-coerce`, `regulated-term-omission`, `synonym-hallucination` |

`bsmell categories` prints the full list.

## License

MIT.
