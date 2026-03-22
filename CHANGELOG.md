# Changelog

All notable changes to **frankenmermaid** are documented here.

> frankenmermaid is a Rust-first, Mermaid-compatible diagram engine with
> intent-aware parsing, 10+ layout algorithms, and SVG / terminal / WASM
> rendering from a single intermediate representation.
>
> Repository: <https://github.com/Dicklesworthstone/frankenmermaid>
> Live demo: <https://dicklesworthstone.github.io/frankenmermaid/>

There are no tagged releases yet. The sections below are organized
chronologically and grouped by capability area. Every commit link points to
the canonical GitHub history.

---

## 2026-03-21 — Class diagram generics, classDef/style pipeline, Gantt rendering, new node shapes

### Class diagrams

- **Generic type parameters** (`<T, U>`) on class diagram nodes, parsed and
  rendered across SVG, terminal, and canvas backends
  ([4ed63f2](https://github.com/Dicklesworthstone/frankenmermaid/commit/4ed63f212a1e447abb872946b3877b24406b6866),
  [9da93f6](https://github.com/Dicklesworthstone/frankenmermaid/commit/9da93f68e6212f9ec8ae19faa32f7a811df537f9),
  [05b4b03](https://github.com/Dicklesworthstone/frankenmermaid/commit/05b4b039f42de74c590895453a4bbcce87b32912))
- **UML three-compartment** class box rendering in terminal and canvas backends
  ([aa7d624](https://github.com/Dicklesworthstone/frankenmermaid/commit/aa7d6246ae58cb23e79f35df290a235bbd0ee7df),
  [d5fe116](https://github.com/Dicklesworthstone/frankenmermaid/commit/d5fe116eeceb60db481d2cb21ea4abec5d2aa69f))
- Dedicated **class diagram layout engine** with parser improvements
  ([a673510](https://github.com/Dicklesworthstone/frankenmermaid/commit/a673510c33abf84eaab9658e61dfb54a718ed64c))

### Flowchart styling: classDef / style / linkStyle

- `IrStyleTarget` and `IrStyleRef` core types for style directives
  ([ccfc3da](https://github.com/Dicklesworthstone/frankenmermaid/commit/ccfc3da939b2157b02d15ea8f0f2d00cba32799f))
- Parser extraction of `classDef`, `style`, and `linkStyle` for flowcharts
  ([a71819a](https://github.com/Dicklesworthstone/frankenmermaid/commit/a71819a03a3dc36268bbf41633f2d0ab0bb77ee1))
- End-to-end pipeline wiring through core, parser, and SVG renderer
  ([d2dfc92](https://github.com/Dicklesworthstone/frankenmermaid/commit/d2dfc9293888bc7ae3183b38c447b30490d22ddb))

### Gantt chart rendering

- Gantt chart IR types and `--font-size` CLI flag
  ([c540ea4](https://github.com/Dicklesworthstone/frankenmermaid/commit/c540ea409952654b18a07013566002437c46cfb9))
- Font-size passthrough for SVG rendering and Gantt IR metadata
  ([d663c5b](https://github.com/Dicklesworthstone/frankenmermaid/commit/d663c5bfef77dc996a5d08654171b88ea2e008c4))
- Section-aware Gantt layout with proper timeline positioning
  ([17c374e](https://github.com/Dicklesworthstone/frankenmermaid/commit/17c374e19f4f43ce42fe99585d83aa0479b92437))
- Band/axis-tick SVG rendering and serde tests
  ([dc702b8](https://github.com/Dicklesworthstone/frankenmermaid/commit/dc702b87c97f7b8b719f44920b739af6e07631f6))

### New node shapes and arrow types

- `FilledCircle` and `HorizontalBar` node shapes
  ([dbfe983](https://github.com/Dicklesworthstone/frankenmermaid/commit/dbfe9832d59493c1d0e0d0a3bf876e07889ae2ba))
- New arrow types and inline edge styles in SVG and terminal renderers
  ([e6ef6ad](https://github.com/Dicklesworthstone/frankenmermaid/commit/e6ef6ad03ea4f2330e843c3e03ca81249d7e53a3))
- Layout engine and renderer improvements for new shapes
  ([d2d233e](https://github.com/Dicklesworthstone/frankenmermaid/commit/d2d233e49c46b31d10e687660681834a0f23c31d))
- Cluster dividers in SVG output
  ([05b4b03](https://github.com/Dicklesworthstone/frankenmermaid/commit/05b4b039f42de74c590895453a4bbcce87b32912))

### Parser improvements

- Expanded Mermaid parser coverage and multi-renderer output refinement
  ([f8a3423](https://github.com/Dicklesworthstone/frankenmermaid/commit/f8a342341580cde0bb1826513a409a92b0b34008),
  [d05f694](https://github.com/Dicklesworthstone/frankenmermaid/commit/d05f694a9e3769cfbe7ee13accd2d44b58a4fbbf))
- Enhanced DOT parser with shape mapping and default attribute support
  ([94c26e2](https://github.com/Dicklesworthstone/frankenmermaid/commit/94c26e21150c1eb76ed9bb8e251051d83b6767a9))

### Testing

- E2E replay determinism and ledger trace continuity tests
  ([c32a75a](https://github.com/Dicklesworthstone/frankenmermaid/commit/c32a75a36c0d16edf6f067b7a33be7e212a3514a))

### Fixes

- Gantt axis tick count and LayoutRect construction fixes
  ([f4e8873](https://github.com/Dicklesworthstone/frankenmermaid/commit/f4e8873433888f60d3983939a8b3bd411c8d8a27))
- Explicit scale factor passthrough in rendering pipeline; golden SVGs updated
  ([018d96d](https://github.com/Dicklesworthstone/frankenmermaid/commit/018d96daf7781629b52d39049ec77b54509edc2b))
- Improved diagram detection heuristics and ANSI-aware truncation
  ([77c32cb](https://github.com/Dicklesworthstone/frankenmermaid/commit/77c32cb45d05012107328c7fc33836c9ee156a53))
- Use `is_none_or` for keyword check; suppress Clippy `too_many_arguments`
  ([842ee63](https://github.com/Dicklesworthstone/frankenmermaid/commit/842ee63259d2f3a77c33c634f43e78b9c3b806d4))
- Subgraph key stability, ANSI-aware diff widths, WASM API updates
  ([7de04c8](https://github.com/Dicklesworthstone/frankenmermaid/commit/7de04c893fa6fb00202f4ef590fd03c9676a136a))

---

## 2026-03-20 — Layout decision ledger, custom font metrics, parser hardening

### Observability: MermaidLayoutDecisionLedger

- New `MermaidLayoutDecisionLedger` type wired into CLI output for full
  pipeline introspection
  ([52c202d](https://github.com/Dicklesworthstone/frankenmermaid/commit/52c202d9c02b2d31c6ad7360e00746e984529235))
- Tracing field enforcement tests and observability output format tests
  ([9d8e89c](https://github.com/Dicklesworthstone/frankenmermaid/commit/9d8e89ca6a6f703071e647767160a9ba013d29a9),
  [85984df](https://github.com/Dicklesworthstone/frankenmermaid/commit/85984dfe6bcce6edc6ef40cc9ac3bbbfd8e7d1e6))

### Layout: LayoutConfig and custom font metrics

- `LayoutConfig` with pluggable font metrics, expanded Mermaid parser coverage,
  refactored SVG text rendering
  ([57b5d24](https://github.com/Dicklesworthstone/frankenmermaid/commit/57b5d2426303e63bc0e0c9c80286736f611d1ebf))

### Parser and IR refinement

- Cluster, subgraph, and label deduplication in IR builder
  ([acd301c](https://github.com/Dicklesworthstone/frankenmermaid/commit/acd301c88e59938ad8d5d81b68695728821871f6))
- Simplified cluster title backfill with let-chains; removed dead `Subgraph`
  AST variant
  ([423202e](https://github.com/Dicklesworthstone/frankenmermaid/commit/423202e48904004cfe5056575fe46b27301bb00a))
- DOT parser attribute handling simplified; terminal diff rendering added
  ([72f114e](https://github.com/Dicklesworthstone/frankenmermaid/commit/72f114e6825c7e0de2b999ba1d02ec3605c9ae3c))

### Testing

- Stress and fuzzy-recovery fixtures with resilience suite validation
  ([2c5c063](https://github.com/Dicklesworthstone/frankenmermaid/commit/2c5c063aa0b55f87cb7f5de20d6c831f3d7c3364))

### Fixes

- Quoted identifiers with spaces; hash function stabilization
  ([15823ef](https://github.com/Dicklesworthstone/frankenmermaid/commit/15823ef2db78f8effcb3efe1ee7dcd91ab37b4e4))
- Simplified synthetic_dag edge generation
  ([f3871ad](https://github.com/Dicklesworthstone/frankenmermaid/commit/f3871ad83d3d9f88d275b97cd72e0ab839c169e7))
- Three bugs found in deep code review
  ([02a8fdc](https://github.com/Dicklesworthstone/frankenmermaid/commit/02a8fdccafb8d95c1bb1f2842799bc8e2fc3c705))

---

## 2026-03-19 — Auto algorithm selection, orthogonal edge routing, fuzz testing, test infrastructure

### Layout: auto algorithm selection

- **Graph-metrics-based automatic layout algorithm selection** -- inspects
  density, branching factor, and cycle presence to pick Sugiyama vs.
  force-directed vs. tree vs. radial
  ([927dd7b](https://github.com/Dicklesworthstone/frankenmermaid/commit/927dd7bb4dadb12b0cc2745779c4701ee36031e0))

### Layout: orthogonal edge routing

- Node-aware orthogonal edge routing with bend minimization
  ([bc91f77](https://github.com/Dicklesworthstone/frankenmermaid/commit/bc91f77a1806e85bd8bcf355f80dd7ed258cf51f))

### SVG arrowhead markers

- Proper SVG `<marker>` definitions for arrowheads; parallel edge diff fix
  ([27228a6](https://github.com/Dicklesworthstone/frankenmermaid/commit/27228a6bf35b63495c4edd58ee24528dfd2113fd))

### Structured tracing

- Pipeline decision tracing with structured spans throughout the layout engine
  ([307bfcf](https://github.com/Dicklesworthstone/frankenmermaid/commit/307bfcf18f6c2e21026c1d911355812ec73bd31a))

### Fuzz testing infrastructure

- cargo-fuzz harness for parser and full pipeline
  ([0154b56](https://github.com/Dicklesworthstone/frankenmermaid/commit/0154b56fb996eeb1c3af27e5910865f45159dd85))
- Parser and detect fuzz corpora with tracing dependency
  ([473d258](https://github.com/Dicklesworthstone/frankenmermaid/commit/473d258696e950a2e44a7712a12d6ec9e471088c))

### Test infrastructure expansion

- E2E pipeline tests for all 24 diagram types
  ([e46dc88](https://github.com/Dicklesworthstone/frankenmermaid/commit/e46dc88a8a42baea4c3d7ecc6c18b3aad6bce59d))
- Golden layout checksum infrastructure for determinism verification
  ([e8e298b](https://github.com/Dicklesworthstone/frankenmermaid/commit/e8e298bb87fb05206b54666c2257825c04c6d2ce))
- Property-based roundtrip invariant tests for parser
  ([76efddc](https://github.com/Dicklesworthstone/frankenmermaid/commit/76efddcba8e760af83b917322e1f3e017a2309dc))
- Adversarial input security hardening tests
  ([bf06fcd](https://github.com/Dicklesworthstone/frankenmermaid/commit/bf06fcd1071e550a0e8fb75cf52ed9622fc71ece))
- Performance baseline tests for all layout algorithms
  ([d2d614b](https://github.com/Dicklesworthstone/frankenmermaid/commit/d2d614b11aae7f5413148a8873e1f2edde6d1443))
- Layout dispatch capability parity and fallback tests
  ([223b3b1](https://github.com/Dicklesworthstone/frankenmermaid/commit/223b3b15f5f34cceaf05b4f1ad8d75ab14667feb))
- Graph IR operations unit tests
  ([c05eea3](https://github.com/Dicklesworthstone/frankenmermaid/commit/c05eea3d0e11bbcbe42561a4f8d55e2f91d81cef))

### Refactoring

- Simplified BK algorithm guard clauses
  ([ed6fa7e](https://github.com/Dicklesworthstone/frankenmermaid/commit/ed6fa7e3e52273564ad685373e2f7ee77456dd8d))
- Optimized parser lookups, fixed multi-line text, added edge markers
  ([00f9d43](https://github.com/Dicklesworthstone/frankenmermaid/commit/00f9d43d44807b8d0600c3c50dd79e5327ac62ca))

### Fixes

- Guard `force_temperature` against zero `max_iterations`
  ([c060121](https://github.com/Dicklesworthstone/frankenmermaid/commit/c06012183beb259189ef0f0934fb47fee4b74b64))
- Guard `f32`-to-`i32` cast in SVG attribute formatting
  ([16b99a8](https://github.com/Dicklesworthstone/frankenmermaid/commit/16b99a882effad3b11159275527b3311fb5d0a9f))
- Use `INFINITY`/`NEG_INFINITY` for bounding box initialization
  ([8339f3f](https://github.com/Dicklesworthstone/frankenmermaid/commit/8339f3f74d601bdd778da37c4b33fe8ec0c244d5))
- Fix cluster CSS test by adding member nodes
  ([2517af4](https://github.com/Dicklesworthstone/frankenmermaid/commit/2517af443d43e77a7550fe4ec1cf80ca475cbec4))

---

## 2026-03-18 — Sequence/class/state IR, observability pipeline, Brandes-Kopf fixes

### Diagram-specific IR and parsing

- **Sequence diagram**: comprehensive IR with lifeline, activation, loop/alt
  fragments, and participant ordering
  ([cd9d35f](https://github.com/Dicklesworthstone/frankenmermaid/commit/cd9d35f4aa80aaec2036d590ef9b66bd848670b9))
- **Class diagram**: IR types and member (field/method) parsing
  ([b6adce8](https://github.com/Dicklesworthstone/frankenmermaid/commit/b6adce862ac054a4a999a0123c3454bb7357d497))
- **State diagram**: composite states and pseudo-states (fork, join, choice)
  ([d3665eb](https://github.com/Dicklesworthstone/frankenmermaid/commit/d3665eb708d456acd7280aa17fe5ae4571519705))

### Observability and pressure reporting

- Observability infrastructure, pressure reporting, parser improvements, and
  layout optimizations
  ([9f1b1ea](https://github.com/Dicklesworthstone/frankenmermaid/commit/9f1b1ea2cfa7119dc760f12601c74717f32cd1df))
- Capability matrix automation, BLESS mode for golden test updates, and
  security hardening
  ([ba90204](https://github.com/Dicklesworthstone/frankenmermaid/commit/ba90204dadbe305dffdcf084c3949e6a79ec7917))
- Budget event tracing and precomputed layout rendering
  ([c61e209](https://github.com/Dicklesworthstone/frankenmermaid/commit/c61e20924f5792446bfcdc0cde575ac38dc0753e))

### Layout engine improvements

- Expanded layout algorithms, parser robustness, and SVG rendering
  ([2c9dc54](https://github.com/Dicklesworthstone/frankenmermaid/commit/2c9dc54f605f612fcb3fbb9ee8a6d8e4bc903905))
- Fixed 4 bugs in Brandes-Kopf coordinate assignment
  ([3c5a2ac](https://github.com/Dicklesworthstone/frankenmermaid/commit/3c5a2ac50ee594ab543f1a098b2d3e27e5e9b1d2))
- Fixed BK compaction double-shift and improved kanban indent detection
  ([745f203](https://github.com/Dicklesworthstone/frankenmermaid/commit/745f203bbf98e6de5d105de701d7843a115830cf))

### Documentation

- Major README expansion with comprehensive feature documentation (+1,008
  lines) and diagram type coverage documentation (+573 lines)
  ([562b248](https://github.com/Dicklesworthstone/frankenmermaid/commit/562b248e296b3150c8a95c5211877caab169f79e),
  [eb3eeda](https://github.com/Dicklesworthstone/frankenmermaid/commit/eb3eeda341b62ee4aabca297da17464f790e55c7),
  [3ff59a9](https://github.com/Dicklesworthstone/frankenmermaid/commit/3ff59a9ef05d1ceb22d0762cfbb1611156f44034))

### Testing

- Updated integration tests, golden SVGs, and observability evidence
  ([8f0aa85](https://github.com/Dicklesworthstone/frankenmermaid/commit/8f0aa85ca176f9a5a0df3a2386e987841e46d535))

### Fixes

- Compact tier test updated for layout dimension changes
  ([7364b8d](https://github.com/Dicklesworthstone/frankenmermaid/commit/7364b8d00f2938aadaad722c46fa2efcc836e941))

---

## 2026-03-17 — Major parser expansion and layout improvements

### Parser and layout

- Major parser expansion and layout improvements (+681 lines) covering
  additional diagram types, edge cases, and IR builder refinements
  ([23bc3fc](https://github.com/Dicklesworthstone/frankenmermaid/commit/23bc3fc2c8a37d828e0d5ad7e76aab553154152c))

---

## 2026-03-16 — SVG visual polish, GitHub Pages showcase, WASM production rebuild

### GitHub Pages showcase

- Standalone browser showcase with live WASM rendering
  ([e07b519](https://github.com/Dicklesworthstone/frankenmermaid/commit/e07b5194a22823a36017d8e04addb7a90fbd5fc9))
- GitHub Pages publishing workflow
  ([3e9d98c](https://github.com/Dicklesworthstone/frankenmermaid/commit/3e9d98c6770de9a8127df029e5ed4ee267d4def6))
- Expanded to 80 realistic gallery samples
  ([70f92f5](https://github.com/Dicklesworthstone/frankenmermaid/commit/70f92f580d2e68b8f69428840146ef4a0cc5b863))
- Major expansion with additional diagram examples (+534 lines)
  ([b7d00e1](https://github.com/Dicklesworthstone/frankenmermaid/commit/b7d00e152e2477f9bdcae52cc53629f32efd08db))
- Mermaid.js fallback, mobile layout, diagnostics collapse
  ([d52d71c](https://github.com/Dicklesworthstone/frankenmermaid/commit/d52d71cebaa63bfed997188f31d87d6778e3bf36))

### SVG rendering polish

- Refined SVG theme system with regenerated golden snapshots
  ([b171e18](https://github.com/Dicklesworthstone/frankenmermaid/commit/b171e184341d186f2b858d2eb37f49639596d357))
- Refined SVG rendering with regenerated golden snapshots
  ([5804927](https://github.com/Dicklesworthstone/frankenmermaid/commit/5804927ba6323e57ca3efdbcc98d8da1d75c6ec6))
- Refined SVG/terminal rendering and refreshed golden snapshots
  ([a62d78d](https://github.com/Dicklesworthstone/frankenmermaid/commit/a62d78d5cbe6890ccb750e783f7b20eb28f13e17),
  [01695fe](https://github.com/Dicklesworthstone/frankenmermaid/commit/01695fea3b3df66bc90b0791e9d222cadcd908b9))

### WASM production rebuild

- Larger nodes, refined arrows, rebuilt WASM for production use
  ([ca53913](https://github.com/Dicklesworthstone/frankenmermaid/commit/ca53913e16a2aba6101c701b815183db347b36dd))

---

## 2026-03-15 — Rendering pipeline expansion, terminal minimap, diagram type coverage

### Rendering pipeline

- Extended rendering pipeline and WASM API (+317 lines)
  ([d0bf676](https://github.com/Dicklesworthstone/frankenmermaid/commit/d0bf6766626244f9c43e74f1d000b161a966eeef))
- Refactored WASM bindings and improved SVG rendering (+153 lines)
  ([53c46a6](https://github.com/Dicklesworthstone/frankenmermaid/commit/53c46a6c31b642c931a6a924d10fc23cf74ad115))
- Extended WASM API and layout algorithms (+145 lines)
  ([8880e7e](https://github.com/Dicklesworthstone/frankenmermaid/commit/8880e7e8655aa640582aa6129361dbbec9d60609))

### Terminal rendering

- Terminal minimap and diff rendering (+818 lines)
  ([56daaaf](https://github.com/Dicklesworthstone/frankenmermaid/commit/56daaaf01520c545afa64f44def22305bb02dcf5))

### Layout engine

- Major layout engine expansion with edge routing and cluster placement (+424
  lines)
  ([e8d6816](https://github.com/Dicklesworthstone/frankenmermaid/commit/e8d68169d0e6d00a30ca2aa1d88fffbd97ad5dce))

### Diagram type coverage

- Expanded diagram type coverage and updated capability matrix
  ([990e164](https://github.com/Dicklesworthstone/frankenmermaid/commit/990e164aa82d77ba327300c7af0dbc97af4714b1))
- Broadened diagram parsing and expanded capability evidence (+494 lines)
  ([ebaf8d6](https://github.com/Dicklesworthstone/frankenmermaid/commit/ebaf8d6b003429d26144f80c0c27c66489ffb28d))
- Expanded parser module API and diagram type support (+415 lines)
  ([51a1396](https://github.com/Dicklesworthstone/frankenmermaid/commit/51a139603b8bdd9da307d5635ccd5fb0d63801ee))
- Extended mermaid parser with additional diagram support (+96 lines)
  ([b37949b](https://github.com/Dicklesworthstone/frankenmermaid/commit/b37949b12db095da7e61f525a1ef407b6027dbb0))
- Expanded mermaid parser with additional diagram type handling (+121 lines)
  ([6b63e36](https://github.com/Dicklesworthstone/frankenmermaid/commit/6b63e36c0485230209dd6d526fe9a94fc3904446))

### Fixes

- Refined mermaid parser edge case handling
  ([402851f](https://github.com/Dicklesworthstone/frankenmermaid/commit/402851f0c05788d1d25ffbaf8fd8560d99053408))

---

## 2026-03-14 — Block-beta and gitGraph refinement, capability matrix, parser architecture

### Block-beta diagram support

- Two-phase block-beta parsing and centralized support metadata
  ([d349a49](https://github.com/Dicklesworthstone/frankenmermaid/commit/d349a49f3d236a10c47554c9d640f5efcf662ae9))
- Validated zero-span in block-beta groups and blocks
  ([ab69a17](https://github.com/Dicklesworthstone/frankenmermaid/commit/ab69a17085e9af940ccbe76d69d61a082febfd0e))

### gitGraph parser architecture

- Two-phase parse/lower architecture for gitGraph command parsing
  ([5f7bf74](https://github.com/Dicklesworthstone/frankenmermaid/commit/5f7bf74b8d9c9e750feda3d009ca79e280e318ec))
- Improved gitGraph command parsing robustness
  ([b309c8d](https://github.com/Dicklesworthstone/frankenmermaid/commit/b309c8d28683847ae2bc8114e84b1e78c4129d3f))

### Capability matrix

- Comprehensive diagram capability matrix and detection evidence in CLI
  ([6e6f22f](https://github.com/Dicklesworthstone/frankenmermaid/commit/6e6f22f64b3b2de08dac8bf066d801897c39f973))

### Parser and layout expansion

- Expanded mermaid parser with improved diagram support
  ([b789d5d](https://github.com/Dicklesworthstone/frankenmermaid/commit/b789d5d707e118af76b6c9b230102f39348d51b8),
  [4148577](https://github.com/Dicklesworthstone/frankenmermaid/commit/41485774a33450272fdbea6905c84d84cff598f8))
- Expanded parser coverage and CLI improvements with integration tests
  ([72c5c25](https://github.com/Dicklesworthstone/frankenmermaid/commit/72c5c258304a4b559d874de147d3c2df89d071ad))
- Extended layout algorithm and CLI integration tests
  ([f8c33da](https://github.com/Dicklesworthstone/frankenmermaid/commit/f8c33da0c8667528e567bcac6b9dca34ece74f48))
- Expanded layout engine with advanced placement strategies
  ([dccff1d](https://github.com/Dicklesworthstone/frankenmermaid/commit/dccff1d96125b9f3a5fbe3995910aab16b596d31))

### WASM

- Expanded WASM bindings and updated capability matrix with README refresh
  ([bb6013a](https://github.com/Dicklesworthstone/frankenmermaid/commit/bb6013a26df601c5f2285ac5f28721a447c906f2))

---

## 2026-03-13 — Block-beta grid layout, grid_span, diagram engine expansion

### Block-beta layout

- **Grouped block-beta grid placement** with subgraph-aware layout
  ([866b339](https://github.com/Dicklesworthstone/frankenmermaid/commit/866b3399f1e3d134e0fce55d384beb58ba2a237b))
- `grid_span` support for block-beta clusters and subgraphs
  ([7cfde1c](https://github.com/Dicklesworthstone/frankenmermaid/commit/7cfde1c76a07bb48fd87b8fc905c50cddd43a9a6))
- Promoted block-beta to basic support and added `block` alias
  ([45c2d7f](https://github.com/Dicklesworthstone/frankenmermaid/commit/45c2d7f9f64f634b4aa25ad974fd3daeac1caf36))

### Layout and rendering

- Expanded diagram layout engine and rendering support
  ([fe832f3](https://github.com/Dicklesworthstone/frankenmermaid/commit/fe832f30823ac88c3538147220fb6a29f50b45f0))

---

## 2026-03-12 — Graph-level IR, subgraph hierarchy, block-beta parsing, flowchart AST

### Graph-level IR

- **Graph-level IR** with subgraphs, typed nodes, and typed edges
  ([c65a835](https://github.com/Dicklesworthstone/frankenmermaid/commit/c65a8353bef0fd206004ccad0005392e7aa54e4a))
- Traversal helpers for subgraph hierarchy and node membership
  ([4570612](https://github.com/Dicklesworthstone/frankenmermaid/commit/45706123ee777b7098eb19608c3c1b5bebdc398c))
- Endpoint resolution, graph adjacency helpers, and `leaf_subgraphs` query
  ([26f0081](https://github.com/Dicklesworthstone/frankenmermaid/commit/26f0081083710c3fad24fddeead723febdea0c37))

### Flowchart parser architecture

- Document-level AST for flowchart parsing
  ([7a051b5](https://github.com/Dicklesworthstone/frankenmermaid/commit/7a051b56a31c8762aeb9103a62a997eea0d39992))
- Flowchart header direction propagation to IR builder
  ([991cf8f](https://github.com/Dicklesworthstone/frankenmermaid/commit/991cf8f6091620dbe2d69cdb89cc804157947289))

### New diagram type: block-beta

- Block-beta diagram parsing support
  ([a3c913e](https://github.com/Dicklesworthstone/frankenmermaid/commit/a3c913e9c2172c5b4aa700acd0f5d547d99645ca))

### Renderer improvements

- Accept pre-computed `DiagramLayout` in SVG, canvas, and WASM renderers
  ([e1e913b](https://github.com/Dicklesworthstone/frankenmermaid/commit/e1e913bc24f64c8482abec2a42e4acf62b99dfa4))

### Support level promotions

- Promoted gitGraph support level to basic
  ([e93f411](https://github.com/Dicklesworthstone/frankenmermaid/commit/e93f411a398d57167d016625e652882cb0b7f8c9))

### Fixes

- Allow duplicate subgraph and cluster keys instead of merging
  ([177f3e8](https://github.com/Dicklesworthstone/frankenmermaid/commit/177f3e8fefc169065ac1edbe47e4ff174c29c11d))
- Ignore nested flowchart headers inside subgraphs
  ([9faaef7](https://github.com/Dicklesworthstone/frankenmermaid/commit/9faaef732db72ef5a67350e08476de18f3f12f06))

---

## 2026-02-27 — Tree and radial layout, adaptive SVG detail tiers, render scene IR, diagnostics

### New layout algorithms: tree and radial

- **Tree layout** (Reingold-Tilford) and **radial layout** with bounds
  computation fix
  ([71505a8](https://github.com/Dicklesworthstone/frankenmermaid/commit/71505a8babd9ed06e6ed5f57691b40df991db302))
- Major layout engine expansion with force-directed improvements and new
  algorithms
  ([69dceec](https://github.com/Dicklesworthstone/frankenmermaid/commit/69dceec4039186f66e30182001e4887452c29c68))

### SVG rendering

- **Adaptive detail tiers** (compact, normal, rich), print-optimized CSS, and
  label truncation
  ([0675004](https://github.com/Dicklesworthstone/frankenmermaid/commit/06750042a767e8c0cdd2bfdcd01001d51c25fd65))
- Major SVG rendering expansion with `<defs>` module and golden tests
  ([843468f](https://github.com/Dicklesworthstone/frankenmermaid/commit/843468f37da7ff45be78c10e819c16ecce060988))

### Render scene IR

- Target-agnostic render scene IR and backend implementations
  ([a7141c8](https://github.com/Dicklesworthstone/frankenmermaid/commit/a7141c8711928c4ff7ff34a4b68b32aad5ebcb20))

### Parser configuration

- **YAML front-matter config** support, unified `%%{init}` directive handling,
  and DOT comment stripping fix
  ([e8b6997](https://github.com/Dicklesworthstone/frankenmermaid/commit/e8b6997c6d0a5a5f48e4a7637b48137382e324a3))
- Mermaid.js config adapter, structured diagnostics, and init config extensions
  ([07532f4](https://github.com/Dicklesworthstone/frankenmermaid/commit/07532f4f9374ea07eb2f7add10b869ce6d88658c))

### CLI: structured validate command

- Overhauled `validate` command with structured diagnostics pipeline
  ([d25408d](https://github.com/Dicklesworthstone/frankenmermaid/commit/d25408ddfa456174ff447b0da737475835cc4138))

### Fixes

- Fixed off-by-one in terminal diagram block boundary detection
  ([c6b8537](https://github.com/Dicklesworthstone/frankenmermaid/commit/c6b8537bdf12c3f0743f31fcb9361e9becacb341))
- Improved DOT edge attribute parsing and fixed SVG detail tier selection
  ([0f8ec9a](https://github.com/Dicklesworthstone/frankenmermaid/commit/0f8ec9a2473c0b13c92a1039798a6295a64a3a44))
- Corrected force-directed physics and Tarjan SCC, added proptest coverage
  across all crates
  ([007ebb5](https://github.com/Dicklesworthstone/frankenmermaid/commit/007ebb54e001bdce5820f6e6a7743be14fae49b9))

---

## 2026-02-26 — Subgraph/cluster parsing, visual design overhaul, security hardening

### Subgraph and cluster support

- **Subgraph/cluster parsing** and compact disconnected component layout
  ([55d08b7](https://github.com/Dicklesworthstone/frankenmermaid/commit/55d08b7d3fb9036bd62a2b730cc740484f835b83))
- Hardened subgraph parsing; prevented isolated nodes from exploding layout
  width
  ([3a988c8](https://github.com/Dicklesworthstone/frankenmermaid/commit/3a988c8f240292724963a12ee94a33cb2824d494))

### Visual design overhaul

- Overhauled visual design to modern aesthetic, added hyperlink support and
  font-aware node sizing
  ([4f08f5f](https://github.com/Dicklesworthstone/frankenmermaid/commit/4f08f5f1ab1a1f69e7a428752b41a6ffff9d6290))

### Security and robustness

- Hardened parsers against edge cases, added **SVG XSS prevention**, and fixed
  terminal renderer underflows
  ([03c6d23](https://github.com/Dicklesworthstone/frankenmermaid/commit/03c6d23d5e089fd328add7d3b9ea4e7582156267))
- Replaced `unwrap()` in `fuzzy_keyword_match` with safe pattern match
  ([1420f51](https://github.com/Dicklesworthstone/frankenmermaid/commit/1420f51d1cb3b3a9221d29b3b8960c4adfff2158))
- Preserved valid edge prefix when chain has malformed trailing segment
  ([2cc5a67](https://github.com/Dicklesworthstone/frankenmermaid/commit/2cc5a67e25a090285b5f46427add45631297722e))

---

## 2026-02-21 — Force-directed layout, cycle handling, crossing refinement, edge routing, node shapes

### Force-directed layout

- **Fruchterman-Reingold force-directed layout** algorithm
  ([a982da5](https://github.com/Dicklesworthstone/frankenmermaid/commit/a982da56e85bdf5a8d3ec37fb300597a1f4c7d00))

### Sugiyama cycle handling

- Complete cycle handling: **SCC collapse**, quality metrics, and comprehensive
  tests
  ([8148819](https://github.com/Dicklesworthstone/frankenmermaid/commit/81488199e4ff46cb25f1a4d338db1d7e674b3f51))

### Crossing minimization refinement

- **Transpose and sifting** heuristics added to Sugiyama crossing minimization
  pipeline
  ([fb8dd86](https://github.com/Dicklesworthstone/frankenmermaid/commit/fb8dd86d5f636a092dac3b0c211ceac03af23664),
  [fb2aef5](https://github.com/Dicklesworthstone/frankenmermaid/commit/fb2aef5efada742d3d5092ccf73836e54b6883b6))

### Edge routing

- **Self-loop routing**, parallel edge offsets, and `EdgeRouting` enum
  ([1257eae](https://github.com/Dicklesworthstone/frankenmermaid/commit/1257eae2e8556c39ce3f3e44270b418b975874a5))

### Node shapes

- **Parallelogram and inverse parallelogram** node shapes with Mermaid syntax
  and full renderer support
  ([f50afca](https://github.com/Dicklesworthstone/frankenmermaid/commit/f50afca3645451ee5bd73f60fde73f23a20077ce))

### Licensing

- Updated license to MIT with OpenAI/Anthropic Rider
  ([ecf2b2d](https://github.com/Dicklesworthstone/frankenmermaid/commit/ecf2b2db0811dce42085d2fed6582893dff14175))

---

## 2026-02-20 — Multi-line labels, theme overrides, Gantt fixes, Mermaid parser expansion

### Multi-line labels

- **Multi-line label rendering** in SVG and terminal; improved text measurement
  in WASM; DOT parser robustness fixes
  ([02f5081](https://github.com/Dicklesworthstone/frankenmermaid/commit/02f5081ed8642d622fd8c9542a4bcc2d948aa731))

### Layout and rendering

- Fixed layout coordinate assignment for reversed ranks, added Mermaid node
  shapes, and supported theme overrides in SVG
  ([95d679c](https://github.com/Dicklesworthstone/frankenmermaid/commit/95d679c9f05b3ad9828d0a7c88d6561c317853dd))
- Expanded Mermaid parser coverage and upgraded dependencies
  ([3f8c6d7](https://github.com/Dicklesworthstone/frankenmermaid/commit/3f8c6d7908e28b1c303d077bfc056137c4b18606))

### Fixes

- Fixed Gantt task ID collisions, improved edge label positioning, added
  multi-line support
  ([a5a4a03](https://github.com/Dicklesworthstone/frankenmermaid/commit/a5a4a035a54ae85b2dc3098a44c7010f9b103fe1))

---

## 2026-02-13 — Mindmap shape parsing, timeline rewrite

### Parser: mindmap and timeline

- Enhanced **mindmap shape parsing** and rewrote timeline as period-event model
  ([73a9e45](https://github.com/Dicklesworthstone/frankenmermaid/commit/73a9e45c89c2137e7b1d8d94e77736ceffc3c2a3))

---

## 2026-02-12 — Initial feature build: workspace, parsers, layout, all three renderers, WASM, CLI

This date represents the initial burst of development that stood up the
complete pipeline from parse through render across all backends.

### Workspace architecture

- **Scaffolded 8-crate Rust workspace** (`fm-core`, `fm-parser`, `fm-layout`,
  `fm-render-svg`, `fm-render-term`, `fm-render-canvas`, `fm-wasm`, `fm-cli`)
  ([328e84f](https://github.com/Dicklesworthstone/frankenmermaid/commit/328e84fef3fb7755ef585009218cc75235dbc23c))

### Parser

- **Modularized fm-parser** into `dot_parser`, `ir_builder`, `mermaid_parser`
  ([5d84e76](https://github.com/Dicklesworthstone/frankenmermaid/commit/5d84e767c2caca83baaba09d405362da48c45bd3))
- Expanded DOT/Mermaid parsers with subgraph, attribute, and diagram type
  support
  ([4833fd4](https://github.com/Dicklesworthstone/frankenmermaid/commit/4833fd4dd71c2d8a38d4426d5845f6e906f82c34))
- Comprehensive **25-type diagram detection** and rendering enhancements
  ([7837e1f](https://github.com/Dicklesworthstone/frankenmermaid/commit/7837e1f27e33d8ebf440acdc763f83a6b0289ae7))
- Enhanced Mermaid parser capabilities
  ([83fb575](https://github.com/Dicklesworthstone/frankenmermaid/commit/83fb575163c843802be4962af10ebba50f5b14d9))
- Comprehensive diagram type parsers and expanded SVG layout engine
  ([2f5b869](https://github.com/Dicklesworthstone/frankenmermaid/commit/2f5b869420c43779d222e0e4702f3d99aabd3e97))

### Core IR

- **ER diagram** entity attribute support
  ([3caf7f8](https://github.com/Dicklesworthstone/frankenmermaid/commit/3caf7f8a2f1d919d4856e1f2795359f21af99238))
- Font metrics, canvas renderer/shapes, SVG accessibility, and theming modules
  ([dcde402](https://github.com/Dicklesworthstone/frankenmermaid/commit/dcde402909155eccc5cb829d4d62c4724e36556e))

### Layout engine

- **Sugiyama layout** with proper cycle removal and crossing minimization
  ([303def5](https://github.com/Dicklesworthstone/frankenmermaid/commit/303def539030c62edfe6e1d51933f2b52150f1eb))
- Fixed rank coordinate assignment, added extended shapes, improved parser
  routing
  ([ed0c64b](https://github.com/Dicklesworthstone/frankenmermaid/commit/ed0c64b4bb393b98e67a39765a7c4a035ab31008))

### SVG renderer

- **Complete SVG generation core** with node rendering, edge paths, and
  viewBox calculation
  ([5feb20b](https://github.com/Dicklesworthstone/frankenmermaid/commit/5feb20bb52ab97d6474cff7b0bae7e29491ecbc0))
- Theming, accessibility (ARIA labels), and diamond arrowhead support
  ([94141fb](https://github.com/Dicklesworthstone/frankenmermaid/commit/94141fb874aa9e6d7e40af5d9ddb88b2cb0b8f54))

### Terminal renderer

- `TermRenderConfig` for terminal rendering options
  ([f381faf](https://github.com/Dicklesworthstone/frankenmermaid/commit/f381faf01ff32fe740ce9bb3436c2a96cb5636ef))
- Canvas and glyph modules for terminal rendering
  ([743150a](https://github.com/Dicklesworthstone/frankenmermaid/commit/743150a226051d2130ba1a668c41b6425608af9d))
- Core terminal diagram renderer
  ([3fdbebc](https://github.com/Dicklesworthstone/frankenmermaid/commit/3fdbebcf5109076b081129fadc85178bb85f7fea))
- Diagram diff and minimap modules
  ([fcaf9b3](https://github.com/Dicklesworthstone/frankenmermaid/commit/fcaf9b31b21ee0e1b54897d100ec0915f2d5174e))
- **ASCII art renderer** for text-only terminal output
  ([e22f404](https://github.com/Dicklesworthstone/frankenmermaid/commit/e22f40436a87ef3da73459e5bc5d9631b0d20352))
- Expanded ASCII renderer and integrated terminal rendering modules
  ([f4a4c44](https://github.com/Dicklesworthstone/frankenmermaid/commit/f4a4c44e71c84a321bb78ca63bb17ad8bbe5af78))
- Polished ASCII renderer and minimap visualization
  ([ba7502d](https://github.com/Dicklesworthstone/frankenmermaid/commit/ba7502d1f946f8f595d387ad0cb5d08fbbfc76e6))

### WASM bindings

- Complete WASM bindings with runtime config
  ([788f81a](https://github.com/Dicklesworthstone/frankenmermaid/commit/788f81ae745311ba6560004f4941ddbfc4b6f37c))
- `Serialize` derive to `ParseResult`, restructured WASM crate
  ([463316b](https://github.com/Dicklesworthstone/frankenmermaid/commit/463316b0c42db2042a646b3092669800378e8a5e))
- Simplified `RuntimeConfig` with derive `Default`
  ([593046e](https://github.com/Dicklesworthstone/frankenmermaid/commit/593046e80eb3cae5795f893a14f0e730930794bd))

### CLI

- **Comprehensive CLI rewrite** with full command suite (`render`, `parse`,
  `detect`, `validate`, `diff`, `capabilities`)
  ([b83e409](https://github.com/Dicklesworthstone/frankenmermaid/commit/b83e4091daf7375edca0f449c7dcbc07a4c4de9d))
- Integration test suite and dependency updates
  ([274d89c](https://github.com/Dicklesworthstone/frankenmermaid/commit/274d89ca705a6c80c3ea9043e8228d69d076c8da))

### Fixes

- Corrected right-border alignment for Unicode content in terminal renderer
  ([808f6e7](https://github.com/Dicklesworthstone/frankenmermaid/commit/808f6e776f04392bebfc089baae5ca23b4967071))

---

## 2026-02-11 — Project inception

### Foundation

- Initial commit with AGENTS.md
  ([a487793](https://github.com/Dicklesworthstone/frankenmermaid/commit/a4877939105eb405add227d48e0f5f5d054fcfec))
- Project foundation: README, `.gitignore`, illustration assets, and legacy
  reference code
  ([d6e1921](https://github.com/Dicklesworthstone/frankenmermaid/commit/d6e1921b47f26067cff1c5d808b1c2cc4ba7f826))
- Comprehensive bead set for project planning
  ([f3b28a0](https://github.com/Dicklesworthstone/frankenmermaid/commit/f3b28a0bec843e35a319ba1e0f5e56e78a91b408))
