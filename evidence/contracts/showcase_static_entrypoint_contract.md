# Showcase Static Entrypoint Contract

## Purpose

This contract defines how the standalone [frankenmermaid_demo_showcase.html](/data/projects/frankenmermaid/frankenmermaid_demo_showcase.html) behavior must map onto the future static `/web` host.

The standalone HTML remains the behavioral reference implementation. `/web` may reorganize packaging and hosting, but it must preserve route meaning, asset expectations, bootstrap honesty, and deep-link restoration semantics.

## Canonical Entrypoint Meaning

- `frankenmermaid_demo_showcase.html` is the semantic prototype for the static showcase entry.
- `/web` is the production route prefix for that same experience.
- A visitor opening `/web` or a deep link beneath `/web` must land in the same logical showcase state that the standalone HTML would reconstruct from equivalent URL parameters.

## Route And URL Semantics

The static host must preserve these URL-level guarantees:

1. `/web` is the canonical route root for the showcase shell.
2. Search parameters carry the restorable showcase snapshot:
   - `sample`
   - `spotlight`
   - `filter`
   - `compare`
   - `lab`
   - `studio`
   - `q`
   - `source`
   - `shells`
3. Deep links under the `/web` prefix must round-trip without losing valid state.
4. Invalid or malformed query values must degrade to canonical defaults, never to blank or undefined state.
5. Route prefixing must not leak into scenario identity. `/web?...` and the standalone file URL must resolve the same logical sample, compare set, lab focus, and studio settings.

## Asset Resolution Rules

The static host must treat asset loading as relative to the entry document, not hard-coded to a deployment-specific absolute origin.

### Required Relative Assets

- `./pkg/frankenmermaid.js`
- `./pkg/frankenmermaid_bg.wasm`
- `./evidence/capability_scenario_matrix.json`

Hosted `/web` deployments should therefore be packageable without rewriting the shared-core logic. When `/web` is served as a file-style route, the browser may resolve those relative references to root-level `/pkg/...` and `/evidence/...`; deployments may also choose an equivalent route-local asset layout if the same runtime/data semantics are preserved.

## Bootstrap Contract

The static host bootstrap sequence must preserve this order of meaning:

1. Restore URL state.
2. Apply shell snapshot and portable shared state.
3. Load the capability artifact.
4. Probe/runtime-load the frankenmermaid browser package.
5. Render the initial spotlight, gallery, playground, and support evidence using the restored state.

Required runtime-loading behavior:

- In hosted mode, probe same-origin relative assets first.
- If same-origin browser artifacts are unavailable, probe the current GitHub-backed fallback endpoints.
- If all runtime candidates fail, surface an honest runtime-unavailable state without pretending that the browser runtime is live.

## File Mode Versus Hosted Mode

The standalone file and the hosted `/web` entry must share semantics but may differ in transport constraints.

### `file:` mode

- Must skip same-origin fetch assumptions that do not work for local standalone execution.
- Must use inline capability-matrix fallback when fetching `./evidence/capability_scenario_matrix.json` is not possible.
- Must continue to restore URL state and render non-runtime evidence honestly.

### Hosted `/web` mode

- Must attempt same-origin relative asset loading first.
- Must treat missing same-origin assets as a packaging/publishing gap.
- Must keep GitHub-backed fallback probing as an explicit recovery path, not as silent substitution for broken primary packaging.

## Deep-Link Restoration Guarantees

The static entrypoint must preserve the same restore semantics already encoded in the standalone HTML:

- `compare` de-duplicates scenario ids and truncates to two entries.
- `lab` restores only known layout-lab focuses and otherwise falls back to `overview`.
- `studio` restores only normalized runtime-backed style controls.
- `source` is an override for editor text, not a new scenario identity.
- `shells` is best-effort restored; malformed shell JSON is ignored in favor of defaults.
- The restored URL should be canonicalized with the validated state snapshot after bootstrap.

## Runtime Honesty Requirements

The static host must make deployment state reviewable:

- If the runtime package is missing, the UI must say the runtime artifact is missing.
- If the capability matrix is fetched successfully, the UI must identify the checked-in artifact path.
- If file-mode fallback is active, the UI must say that inline fallback is active because of standalone execution constraints.
- The host must not claim “live runtime” unless the imported module exposes the expected bindings and `init()` succeeds.

## Cloudflare `/web` Hosting Expectations

Cloudflare-hosted static deployments must satisfy these assumptions:

- The `/web` route serves the showcase entry document.
- Relative references from that document must still resolve to the expected runtime/data artifacts, whether that means root-level `/pkg/...` and `/evidence/...` under file-style `/web` hosting or an equivalent route-local static asset layout.
- Direct navigation to deep links with query state must return the showcase entry rather than a 404 or unrelated route shell.
- Asset caching policy may vary, but bootstrap must tolerate `no-store` validation probes without changing semantics.

## Validation Checklist

This contract is considered satisfied only when the static host demonstrates all of the following:

- A deep link containing `sample`, `compare`, `lab`, `studio`, and `source` restores the expected state under `/web`.
- Invalid `compare`, `lab`, `studio`, or `shells` query values degrade to canonical defaults.
- Same-origin relative asset resolution is used for hosted mode.
- File-mode fallback behavior is documented and remains honest.
- Runtime-unavailable UI remains distinct from live-runtime UI.
- Capability artifact loading remains semantically equivalent between fetched and fallback modes.

## Downstream Consequences

- `bd-2u0.5.8.2.2` should implement the `/web` host container around this route/bootstrap contract rather than inventing new entry semantics.
- `/web_react` may use a different composition model, but it should preserve the same route-prefix, asset-resolution, and deep-link guarantees when it introduces its own entry surface.
