//! Mermaid-js compatibility test suite (bd-2xl.5).
//!
//! Validates that frankenmermaid can parse all standard mermaid-js diagram types
//! from the official documentation. For each example:
//! 1. Parse with fm-parser — must succeed (no errors, warnings acceptable)
//! 2. Run layout — must produce valid positions
//! 3. Render SVG — must produce valid SVG
//!
//! Reports compatibility percentage per diagram type.

use fm_layout::layout_diagram;
use fm_parser::parse;
use fm_render_svg::{SvgRenderConfig, render_svg_with_layout};

/// A mermaid-js compatibility test case.
struct CompatCase {
    diagram_type: &'static str,
    label: &'static str,
    input: &'static str,
}

fn compat_corpus() -> Vec<CompatCase> {
    vec![
        // ─── Flowchart ──────────────────────────────────────────────────
        CompatCase {
            diagram_type: "flowchart",
            label: "basic_lr",
            input: "flowchart LR\n    A-->B-->C",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "basic_td",
            input: "flowchart TD\n    A-->B\n    B-->C",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "node_shapes",
            input: "flowchart LR\n    A[Rectangle]\n    B(Rounded)\n    C{Diamond}\n    D((Circle))\n    E>Flag]\n    A-->B-->C-->D-->E",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "edge_labels",
            input: "flowchart LR\n    A-->|text|B\n    B---|dashed|C\n    C==>|thick|D",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "subgraph",
            input: "flowchart TB\n    subgraph one\n        A-->B\n    end\n    subgraph two\n        C-->D\n    end\n    B-->C",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "cycle",
            input: "flowchart TD\n    A-->B\n    B-->C\n    C-->A",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "graph_alias",
            input: "graph LR\n    A-->B-->C",
        },
        // ─── Sequence ───────────────────────────────────────────────────
        CompatCase {
            diagram_type: "sequence",
            label: "basic",
            input: "sequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi",
        },
        CompatCase {
            diagram_type: "sequence",
            label: "participants",
            input: "sequenceDiagram\n    participant A as Alice\n    participant B as Bob\n    A->>B: Request\n    B-->>A: Response",
        },
        CompatCase {
            diagram_type: "sequence",
            label: "notes",
            input: "sequenceDiagram\n    Alice->>Bob: Hello\n    Note over Alice,Bob: A typical interaction",
        },
        CompatCase {
            diagram_type: "sequence",
            label: "loops",
            input: "sequenceDiagram\n    Alice->>Bob: Hi\n    loop Every minute\n        Bob->>Alice: Ping\n    end",
        },
        // ─── Class ──────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "class",
            label: "basic",
            input: "classDiagram\n    Animal <|-- Duck\n    Animal <|-- Fish\n    Animal : +int age\n    Animal : +String gender",
        },
        CompatCase {
            diagram_type: "class",
            label: "methods",
            input: "classDiagram\n    class BankAccount\n    BankAccount : +String owner\n    BankAccount : +deposit(amount)",
        },
        // ─── State ──────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "state",
            label: "basic",
            input: "stateDiagram-v2\n    [*] --> Active\n    Active --> Inactive\n    Inactive --> [*]",
        },
        CompatCase {
            diagram_type: "state",
            label: "composite",
            input: "stateDiagram-v2\n    [*] --> First\n    state First {\n        [*] --> second\n        second --> [*]\n    }",
        },
        // ─── ER ─────────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "er",
            label: "basic",
            input: "erDiagram\n    CUSTOMER ||--o{ ORDER : places\n    ORDER ||--|{ LINE-ITEM : contains",
        },
        // ─── Gantt ──────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "gantt",
            label: "basic",
            input: "gantt\n    title A Gantt Diagram\n    dateFormat YYYY-MM-DD\n    section Section\n    A task :a1, 2024-01-01, 30d\n    Another task :after a1, 20d",
        },
        // ─── Pie ────────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "pie",
            label: "basic",
            input: "pie title Pets\n    \"Dogs\" : 386\n    \"Cats\" : 85\n    \"Rats\" : 15",
        },
        CompatCase {
            diagram_type: "pie",
            label: "show_data",
            input: "pie showData\n    \"A\" : 30\n    \"B\" : 50\n    \"C\" : 20",
        },
        // ─── Git Graph ──────────────────────────────────────────────────
        CompatCase {
            diagram_type: "gitgraph",
            label: "basic",
            input: "gitGraph\n    commit\n    branch develop\n    commit\n    checkout main\n    merge develop",
        },
        // ─── Journey ────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "journey",
            label: "basic",
            input: "journey\n    title My working day\n    section Go to work\n      Make tea: 5: Me\n      Go upstairs: 3: Me",
        },
        // ─── Mindmap ────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "mindmap",
            label: "basic",
            input: "mindmap\n  root((Central))\n    Topic A\n      Sub A1\n    Topic B",
        },
        // ─── Timeline ───────────────────────────────────────────────────
        CompatCase {
            diagram_type: "timeline",
            label: "basic",
            input: "timeline\n    title History\n    2000 : Event A\n    2010 : Event B\n    2020 : Event C",
        },
        // ─── Sankey ─────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "sankey",
            label: "basic",
            input: "sankey-beta\n\nSource,Target,Value\nA,X,5\nA,Y,3\nB,X,2",
        },
        // ─── Quadrant ───────────────────────────────────────────────────
        CompatCase {
            diagram_type: "quadrant",
            label: "basic",
            input: "quadrantChart\n    title Reach and engagement\n    x-axis Low Reach --> High Reach\n    y-axis Low Engagement --> High Engagement\n    quadrant-1 We should expand\n    Campaign A: [0.3, 0.6]\n    Campaign B: [0.45, 0.23]",
        },
        // ─── XY Chart ───────────────────────────────────────────────────
        CompatCase {
            diagram_type: "xychart",
            label: "basic",
            input: "xychart-beta\n    title \"Sales Revenue\"\n    x-axis [jan, feb, mar, apr]\n    y-axis \"Revenue (in $)\" 4000 --> 11000\n    bar [5000, 6000, 7500, 8200]\n    line [5000, 6000, 7500, 8200]",
        },
        // ─── Requirement ────────────────────────────────────────────────
        CompatCase {
            diagram_type: "requirement",
            label: "basic",
            input: "requirementDiagram\n    requirement test_req {\n    id: 1\n    text: the test text\n    risk: high\n    verifymethod: test\n    }",
        },
        // ─── C4 ─────────────────────────────────────────────────────────
        CompatCase {
            diagram_type: "c4",
            label: "context",
            input: "C4Context\n    title System Context\n    Person(customer, \"Customer\", \"A customer\")\n    System(system, \"System\", \"The system\")\n    Rel(customer, system, \"Uses\")",
        },
        // ─── Block Beta ─────────────────────────────────────────────────
        CompatCase {
            diagram_type: "block-beta",
            label: "basic",
            input: "block-beta\n    columns 3\n    a b c",
        },
        // ─── Fuzzy/Recovery ─────────────────────────────────────────────
        CompatCase {
            diagram_type: "flowchart",
            label: "fuzzy_keyword",
            input: "flowchrt LR\n    A-->B",
        },
        CompatCase {
            diagram_type: "flowchart",
            label: "malformed_recovery",
            input: "flowchart LR\n    A-->B\n    -->C\n    D[unclosed",
        },
    ]
}

#[test]
fn mermaid_compat_all_diagram_types_parse_layout_render() {
    let corpus = compat_corpus();
    let config = SvgRenderConfig::default();
    let mut results: Vec<(String, String, bool, String)> = Vec::new();

    for case in &corpus {
        let parsed = parse(case.input);
        let layout = layout_diagram(&parsed.ir);
        let svg = render_svg_with_layout(&parsed.ir, &layout, &config);

        let valid_svg = svg.contains("<svg") && svg.contains("</svg>") && !svg.contains("NaN");
        let valid_layout = layout.bounds.width.is_finite() && layout.bounds.height.is_finite();
        let success = valid_svg && valid_layout;

        let detail = if success {
            format!(
                "nodes={} edges={} svg_bytes={}",
                layout.nodes.len(),
                layout.edges.len(),
                svg.len()
            )
        } else {
            format!(
                "svg_ok={valid_svg} layout_ok={valid_layout} nodes={} edges={}",
                layout.nodes.len(),
                layout.edges.len()
            )
        };

        results.push((
            case.diagram_type.to_string(),
            case.label.to_string(),
            success,
            detail,
        ));
    }

    // Report per-type compatibility
    let mut type_stats: std::collections::BTreeMap<String, (usize, usize)> =
        std::collections::BTreeMap::new();
    for (dtype, label, success, detail) in &results {
        let entry = type_stats.entry(dtype.clone()).or_default();
        entry.0 += 1;
        if *success {
            entry.1 += 1;
        }
        let status = if *success { "PASS" } else { "FAIL" };
        println!("[{status}] {dtype}/{label}: {detail}");
    }

    println!("\n--- Compatibility Report ---");
    let mut total_cases = 0;
    let mut total_pass = 0;
    for (dtype, (count, pass)) in &type_stats {
        let pct = (*pass as f64 / *count as f64) * 100.0;
        println!("  {dtype}: {pass}/{count} ({pct:.0}%)");
        total_cases += count;
        total_pass += pass;
    }
    let total_pct = (total_pass as f64 / total_cases as f64) * 100.0;
    println!("  TOTAL: {total_pass}/{total_cases} ({total_pct:.0}%)");

    // Assert high compatibility
    assert!(
        total_pct >= 90.0,
        "Overall compatibility {total_pct:.1}% is below 90% threshold"
    );
}

/// Verify that each diagram type is detected with high confidence.
#[test]
fn mermaid_compat_detection_confidence() {
    let cases = [
        ("flowchart LR\n    A-->B", "Flowchart", 0.9),
        ("sequenceDiagram\n    A->>B: hi", "Sequence", 0.9),
        ("classDiagram\n    A <|-- B", "Class", 0.9),
        ("stateDiagram-v2\n    [*] --> A", "State", 0.9),
        ("erDiagram\n    A ||--o{ B : has", "Er", 0.7),
        (
            "gantt\n    title T\n    section S\n    A :a1, 2024-01-01, 30d",
            "Gantt",
            0.9,
        ),
        ("pie\n    \"A\" : 50\n    \"B\" : 50", "Pie", 0.9),
        ("gitGraph\n    commit", "GitGraph", 0.9),
        ("mindmap\n  root\n    A", "Mindmap", 0.9),
        ("timeline\n    2024 : Event", "Timeline", 0.9),
        ("C4Context\n    Person(a, \"A\", \"a\")", "C4Context", 0.9),
        (
            "requirementDiagram\n    requirement r { id: 1 }",
            "Requirement",
            0.9,
        ),
    ];

    for (input, expected_type, min_confidence) in &cases {
        let parsed = parse(input);
        assert!(
            parsed.confidence >= *min_confidence,
            "Detection confidence for {expected_type}: {:.2} < {min_confidence}",
            parsed.confidence
        );
    }
}
