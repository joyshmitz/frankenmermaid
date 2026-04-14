# FNX Compatibility Matrix (bd-ml2r.7.2)

This document tracks required fnx capabilities by phase and their availability status.

---

## Phase 1: Undirected Structural Intelligence

### Required Capabilities

| Capability | fnx Module | Status | Test Coverage |
|------------|-----------|--------|---------------|
| Cycle detection (undirected) | `fnx-algorithms` | ✓ Available | `fnx_baseline_invariants.rs` |
| Connectivity analysis | `fnx-algorithms` | ✓ Available | `fnx_differential_report.rs` |
| Degree centrality | `fnx-algorithms` | ✓ Available | `fnx_e2e_scenarios.rs` |
| Graph construction | `fnx-classes` | ✓ Available | `fnx_baseline_invariants.rs` |
| Node/edge iteration | `fnx-views` | ✓ Available | All fnx tests |

### Feature Flag

```toml
fnx-integration  # Enables Phase-1 undirected capabilities
```

---

## Phase 2: Directed Graph Intelligence

### Required Capabilities

| Capability | fnx Module | Status | Test Coverage |
|------------|-----------|--------|---------------|
| SCC detection (Tarjan) | `fm-layout` native | ✓ Implemented | `fnx_directed::tests` |
| WCC detection | `fm-layout` native | ✓ Implemented | `fnx_directed::tests` |
| Directed cycle detection | `fm-layout` native | ✓ Implemented | `fnx_directed::tests` |
| Reachability analysis | `fm-layout` native | ✓ Implemented | `fnx_directed::tests` |
| DAG detection | Pending | ⏳ Requires fnx upstream | - |
| Topological ordering | Pending | ⏳ Requires fnx upstream | - |
| Directed centrality | Pending | ⏳ Requires fnx upstream | - |

### Feature Flag

```toml
fnx-experimental-directed  # Enables Phase-2 directed capabilities
```

### Note on Native Implementations

To unblock Phase-2 enablement without waiting for fnx upstream directed APIs,
frankenmermaid implements core directed algorithms natively in `fm-layout/src/fnx_directed.rs`:

- `compute_scc()` - Tarjan's SCC with deterministic ordering
- `compute_wcc()` - Weakly connected components via BFS
- `detect_directed_cycles()` - DFS-based cycle detection
- `compute_reachability()` - Source/sink identification

These operate directly on `MermaidDiagramIr` and maintain determinism guarantees.

---

## Version Pinning Policy

### Current Pin

```toml
# workspace Cargo.toml
fnx-runtime = { git = "...", rev = "cb8bdb59..." }
fnx-classes = { git = "...", rev = "cb8bdb59..." }
fnx-algorithms = { git = "...", rev = "cb8bdb59..." }
fnx-views = { git = "...", rev = "cb8bdb59..." }
```

### Upgrade Workflow

1. Identify target fnx commit
2. Update `rev` in `Cargo.toml`
3. Run `cargo update -p fnx-runtime -p fnx-classes -p fnx-algorithms -p fnx-views`
4. Run `cargo check --workspace --features fnx-integration`
5. Run `cargo test --workspace --features fnx-integration`
6. If tests pass, commit with `bd-ml2r.7.2: bump fnx to <rev>`

### Breaking Change Policy

- Pinned revision guarantees reproducible builds
- fnx-off mode always works as fallback
- Breaking API changes require explicit compatibility layer or feature guard

---

## CI Validation

CI enforces compatibility via:

1. **Build matrix**: Both `fnx-off` and `fnx-on` tested
2. **Baseline tests**: `fnx_baseline_invariants.rs` catches output drift
3. **Differential tests**: `fnx_differential_report.rs` catches quality regressions
4. **API smoke tests**: `fnx_capability_checks.rs` validates required APIs

See `.github/workflows/ci.yml` for configuration.

---

## References

- [FNX Integration Architecture](./FNX_INTEGRATION.md)
- Bead: bd-ml2r.7.2 (this document)
- Parent: bd-ml2r.7 (Directed capability gap-closure)
