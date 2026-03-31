# `bd-2u0.5.7.1` Presenter Mode Evidence

This bundle captures the guided presenter-tour pass for the standalone/static showcase.

## What landed

- a presenter control rail in the standalone showcase
- a fixed guided tour sequence for the strongest runtime, resilience, determinism, and support-evidence moments
- next/prev/skip/reset controls
- URL-restorable presenter state via `tour` and `tour_step`
- safe reset back to the pre-tour showcase state

## Validation commands

```bash
python3 -m unittest tests/test_showcase_harness.py tests/test_static_web_e2e.py -v
python3 scripts/run_static_web_e2e.py --bead-id bd-2u0.5.7.1 --repo-root . --output-root evidence/runs/web/bd-2u0.5.7.1 --repeat 1
```

## Key artifact

- `2026-03-30T02-16-26Z__determinism__summary.json`

The replay suite now includes `static-web-presenter-tour`, which restored the guided tour at a concrete step across:

- `desktop-default`
- `desktop-reduced-motion`
- `mobile-narrow`
