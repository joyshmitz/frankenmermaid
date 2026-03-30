# `bd-2u0.5.6.4` Compatibility and Runtime Evidence

This bundle captures the compatibility/performance hardening pass for the standalone and static `/web` showcase surfaces.

## Hardening focus

- fallback UUID generation when `crypto.randomUUID()` is unavailable
- clipboard fallback when `navigator.clipboard` is unavailable
- gallery render fallback when `IntersectionObserver` is unavailable
- CSS fallback when `backdrop-filter` is unsupported
- reduced-motion gating for scripted animation and smooth scroll paths

## Validation commands

```bash
python3 -m unittest tests/test_showcase_harness.py tests/test_static_web_e2e.py -v
python3 scripts/showcase_harness.py validate-showcase-compatibility --entry frankenmermaid_demo_showcase.html
python3 scripts/run_static_web_e2e.py --bead-id bd-2u0.5.6.4 --repo-root . --output-root evidence/runs/web/bd-2u0.5.6.4 --repeat 1
```

## Key artifacts

- `showcase-compatibility/2026-03-30T00-05-00Z__evidence__log.json`
- `2026-03-30T00-05-24Z__determinism__summary.json`

The compatibility guardrail log passed, and the static `/web` replay remained green across:

- `desktop-default`
- `desktop-reduced-motion`
- `mobile-narrow`
