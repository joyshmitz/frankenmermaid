#[test]
fn test_tree_overflow() {
    use std::fmt::Write;
    use std::time::Instant;
    let mut input = String::with_capacity(100_000_000);
    input.push_str("mindmap\n");
    let mut indent = String::new();
    for i in 0..10000 {
        writeln!(input, "{}A{i}", indent).unwrap();
        indent.push_str("  ");
    }

    let start = Instant::now();
    println!("Starting parse...");
    let res = fm_parser::parse(&input);
    println!("Parse took {:?}", start.elapsed());

    let start2 = Instant::now();
    println!("Starting layout...");
    let _layout = fm_layout::layout_diagram(&res.ir);
    println!("Layout took {:?}", start2.elapsed());
}
