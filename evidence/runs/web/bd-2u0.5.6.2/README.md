# `bd-2u0.5.6.2` Static Web Determinism Evidence

This bundle captures the standalone `/web` replay run after adding the in-app determinism checker to `frankenmermaid_demo_showcase.html`.

## What was exercised

- `static-web-compare-export`
- `static-web-diagnostics-recovery`
- `static-web-determinism-check`

Profiles:

- `desktop-default`
- `desktop-reduced-motion`
- `mobile-narrow`

Command used:

```bash
python3 scripts/run_static_web_e2e.py --bead-id bd-2u0.5.6.2 --repo-root . --output-root evidence/runs/web/bd-2u0.5.6.2 --repeat 1
```

## Key result

The new in-page determinism checker reported `stable` for the checked scenario across all three profiles, with the same normalized output hash:

- `sha256:db3c964ed90d6368297ca10441e854bb7415fab99c8931f20eae52464ffc6100`

See:

- `2026-03-29T22-35-54Z__determinism__summary.json`
- `static-web-determinism-check/*/*__e2e__log.json`
- `static-web-determinism-check/*/*__e2e__html.html`
