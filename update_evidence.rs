use fm_core::*;
use std::fs;

fn main() {
    let matrix_json = capability_matrix_json_pretty().unwrap();
    fs::write("evidence/capability_matrix.json", matrix_json).unwrap();
    
    let mut readme = fs::read_to_string("README.md").unwrap();
    
    // Update diagram types
    let block = capability_readme_supported_diagram_types_markdown();
    let start = "<!-- BEGIN GENERATED: supported-diagram-types -->\n";
    let end = "\n<!-- END GENERATED: supported-diagram-types -->";
    let s = readme.find(start).unwrap() + start.len();
    let e = readme.find(end).unwrap();
    readme.replace_range(s..e, &block);
    
    // Update surface
    let block = capability_readme_surface_markdown();
    let start = "<!-- BEGIN GENERATED: runtime-capability-metadata -->\n";
    let end = "\n<!-- END GENERATED: runtime-capability-metadata -->";
    if let Some(s_idx) = readme.find(start) {
        let s = s_idx + start.len();
        let e = readme.find(end).unwrap();
        readme.replace_range(s..e, &block);
    } else {
        println!("Could not find runtime-capability-metadata, adding it");
        readme.push_str("\n");
        readme.push_str(start);
        readme.push_str(&block);
        readme.push_str(end);
        readme.push_str("\n");
    }
    
    fs::write("README.md", readme).unwrap();
}
