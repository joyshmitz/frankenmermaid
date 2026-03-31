#[test]
fn test_tree_overflow() {
    let mut input = String::from("mindmap\n");
    for i in 0..10000 {
        input.push_str(&format!("{}A{i}\n", "  ".repeat(i)));
    }
    let res = fm_parser::parse(&input);
    let _layout = fm_layout::layout_diagram(&res.ir);
}
