# `bd-2u0.5.6.3` Accessibility Evidence

This bundle captures the standalone showcase accessibility hardening pass and a static `/web` replay check after the changes landed.

## What changed

- skip navigation to the main landmark
- shared focus-visible treatment
- polite live-region summaries for key operator/evidence panels
- keyboard-focusable zoom targets for spotlight and gallery renders
- reduced-motion and higher-contrast CSS branches
- smooth-scroll/intro-animation behavior that now respects reduced-motion

## Validation commands

```bash
python3 -m unittest tests/test_showcase_harness.py tests/test_static_web_e2e.py -v
python3 scripts/showcase_harness.py validate-showcase-accessibility --entry frankenmermaid_demo_showcase.html
python3 scripts/run_static_web_e2e.py --bead-id bd-2u0.5.6.3 --repo-root . --output-root evidence/runs/web/bd-2u0.5.6.3 --repeat 1
```

## Key artifacts

- `showcase-accessibility/2026-03-29T23-51-00Z__evidence__log.json`
- `2026-03-29T23-54-01Z__determinism__summary.json`

The accessibility guardrail log passed, and the static `/web` replay remained green across:

- `desktop-default`
- `desktop-reduced-motion`
- `mobile-narrow`
