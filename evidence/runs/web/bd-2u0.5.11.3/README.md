# Static `/web` Release-Grade Replay Suite

This bundle extends the narrower `bd-2u0.5.8.2.3` smoke runner into repeated profile-based replay.

Run it locally:

```bash
python3 scripts/run_static_web_e2e.py \
  --repo-root . \
  --output-root evidence/runs/web/bd-2u0.5.11.3 \
  --repeat 5
```

Profiles:

- `desktop-default`
- `desktop-reduced-motion`
- `mobile-narrow`

Scenarios:

- `static-web-compare-export`
- `static-web-diagnostics-recovery`

Artifacts emitted:

- per-run DOM dumps: `.../__e2e__html.html`
- per-run schema-valid logs: `.../__e2e__log.json`
- suite summary: `.../__determinism__summary.json`

Interpretation guidance:

- `stable_normalized_log: true` means the replayed runs preserved the same semantic outcome for diagnostics, degradation tier, runtime mode, and other normalized fields.
- `stable_output_hash: false` means the full post-JavaScript DOM is not currently byte-stable across repeated runs, even when the normalized outcome is stable.

That distinction matters for release review:

- normalized stability is currently good enough for semantic regression triage
- full DOM hash instability remains a real determinism gap for stricter snapshot-style gating
