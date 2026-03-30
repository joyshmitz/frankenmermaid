# Showcase React Embedding Contract

## Purpose

This contract defines how the future `/web_react` host must expose the showcase as a reusable React embedding surface without forking the semantics already fixed by the shared-core and static-entry contracts.

The standalone [frankenmermaid_demo_showcase.html](/data/projects/frankenmermaid/frankenmermaid_demo_showcase.html) remains the behavioral reference. The React host may change composition mechanics, rendering lifecycles, and host-framework integration, but it must not change scenario identity, URL state meaning, diagnostics severity, fallback behavior, or runtime honesty.

## Required Upstream Contracts

`/web_react` must treat these artifacts as normative prerequisites:

- `evidence/contracts/showcase_shared_core_contract.md`
- `evidence/contracts/showcase_static_entrypoint_contract.md`
- `evidence/contracts/showcase_test_taxonomy_and_logging_schema.md`
- `evidence/demo_strategy.md`

This contract is additive. It does not redefine shared-core or route semantics; it defines how React components must consume and expose them.

## React Host Role

The React host owns:

- route integration for `/web_react`
- component composition
- hook-based state propagation
- host-level layout/presentation concerns
- framework-specific loading boundaries around runtime/module availability

The React host must not own:

- alternate sample IDs
- alternate URL schema
- alternate diagnostics taxonomy
- alternate fallback policy
- alternate capability-matrix meaning

## Top-Level Component Model

The React embedding surface should decompose into three layers:

### 1. `ShowcaseCoreProvider`

Responsibilities:

- initialize shared-core modules
- own the portable showcase snapshot
- expose normalized actions/selectors to child React components
- bridge host services such as history, clipboard, and runtime loading

Inputs:

- scenario catalog source
- capability artifact source
- runtime loader
- initial URL snapshot or equivalent host-provided route state

Outputs:

- React context with state, derived view models, and actions

### 2. `ShowcaseShell`

Responsibilities:

- preserve the canonical section order:
  - Runtime
  - Spotlight
  - Gallery
  - Playground
  - Support Evidence
- compose shell-level panels and route section navigation
- map shell balance/rail state to the portable `shells` snapshot

This layer is the React analogue of the standalone page structure, not a new information architecture.

### 3. Feature Modules

Feature modules should be separately mountable components backed by the shared provider:

- `RuntimeStatusPanel`
- `SpotlightPanel`
- `GalleryPanel`
- `PlaygroundPanel`
- `DiagnosticsPanel`
- `ArtifactLabPanel`
- `FallbackPreviewPanel`
- `LayoutLabPanel`
- `SupportEvidencePanel`

The host may split or group these for composition, but the underlying contract surfaces must remain available.

## Minimum Component API

At minimum, the React host must define a single embeddable root with an API equivalent to:

```ts
type ShowcaseHostKind = "react-web";

type ShowcaseInitialState = {
  sample?: string;
  spotlight?: string;
  filter?: string;
  compare?: string[];
  lab?: "overview" | "cycles" | "crossings" | "budget" | "legibility";
  studio?: {
    preset?: string;
    fontSize?: number;
    padding?: number;
    radius?: number;
    shadows?: boolean;
    embedCss?: boolean;
  };
  q?: string;
  source?: string;
  shells?: Record<string, unknown>;
};

type ShowcaseRouteAdapter = {
  read(): ShowcaseInitialState;
  write(next: ShowcaseInitialState, mode: "replace" | "push"): void;
};

type ShowcaseHostServices = {
  route: ShowcaseRouteAdapter;
  copyText(text: string): Promise<void>;
  loadRuntime(): Promise<unknown>;
  loadCapabilityArtifact(): Promise<unknown>;
};

type ShowcaseRootProps = {
  hostKind?: ShowcaseHostKind;
  services: ShowcaseHostServices;
  initialState?: ShowcaseInitialState;
  onStateChange?: (next: ShowcaseInitialState) => void;
  onTelemetry?: (event: ShowcaseTelemetryEvent) => void;
};
```

Equivalent naming is acceptable, but the responsibilities must map cleanly onto this shape.

## Event Contract

The React host must expose stable callbacks or internal event types for:

- state snapshot changes
- runtime status changes
- diagnostics updates
- fallback activation/deactivation
- artifact export attempts
- compare-mode changes
- spotlight/editor scenario changes

At minimum, the telemetry/event payloads must carry enough information to map back to:

- scenario ID
- host kind
- degradation tier
- revision or render generation
- active renderer/runtime mode when relevant

## Ownership Boundaries

### Shared Core Owns

- canonical scenario resolution
- URL snapshot normalization rules
- latest-edit-wins render scheduling
- diagnostics projection
- fallback preservation
- capability-artifact semantics

### React Adapter Owns

- component tree organization
- suspense/loading/error boundaries
- hook subscriptions/selectors
- focus handoff and scroll orchestration
- framework-local memoization or transitions

### Host Route Layer Owns

- mapping browser/framework routing into the shared snapshot
- `/web_react` route mounting
- route transitions outside the showcase snapshot itself

## Compatibility Constraints

- `/web_react` must consume the same portable snapshot fields used by `/web`.
- The React host may add ephemeral local state, but it must not serialize host-only fields into the shared route contract.
- Invalid route values must degrade the same way they do on the static host.
- Runtime-unavailable states must remain honest even if React suspense boundaries are used.
- React re-renders must not break latest-edit-wins semantics or resurrect stale revisions.

## Versioning Expectations

This contract should be treated as versioned by semantic meaning rather than arbitrary prop churn.

Breaking changes include:

- changing the meaning of any serialized snapshot field
- changing required event semantics for diagnostics/fallback/runtime honesty
- changing section-order or proof-surface obligations without an explicit contract update

Non-breaking changes include:

- adding optional props
- adding optional telemetry fields
- splitting internal components while preserving the shared provider API

## Validation Checklist

The React embedding contract is only satisfied when:

- the root component API clearly distinguishes shared-core state from host services
- route integration is defined as an adapter, not hard-coded component logic
- shell, compare, lab, and studio state remain portable
- diagnostics/fallback/runtime truth are preserved across component boundaries
- the contract points downstream React implementation work at reusable tests/logging rather than ad hoc host-local checks

## Downstream Consequences

- `bd-2u0.5.8.3.2` should implement the `/web_react` route against this component/service boundary.
- `bd-2u0.5.8.3.3` should use this API to drive React-host E2E and structured log capture.
- `bd-2u0.5.8.4` should compare `/web` and `/web_react` using the shared snapshot semantics defined here rather than DOM-shape coincidence.
