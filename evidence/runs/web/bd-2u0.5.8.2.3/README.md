# Static `/web` Replay

Use the local headless Chromium runner to reproduce the current static showcase smoke journeys:

```bash
python3 scripts/run_static_web_e2e.py \
  --repo-root . \
  --output-root evidence/runs/web/bd-2u0.5.8.2.3 \
  --chromium /snap/bin/chromium
```

What it does:

- serves the repository root over a local HTTP server
- opens `/web` in headless Chromium with query-restored scenarios
- captures post-JavaScript DOM dumps as reviewable HTML artifacts
- emits schema-valid JSON logs for each scenario under `evidence/runs/web/bd-2u0.5.8.2.3/`

Current scenarios:

- `static-web-compare-export`
- `static-web-diagnostics-recovery`

The runner is intentionally local and deterministic enough for defect isolation. Downstream broader browser-matrix work belongs in the later release-grade suite bead.
