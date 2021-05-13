use tree_sitter::{Parser, Language, Tree, Node};
// extern "C" { fn tree_sitter_c() -> Language; }
// extern "C" { fn tree_sitter_rust() -> Language; }

extern "C" { fn tree_sitter_javascript() -> Language; }

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_javascript() };
    parser.set_language(language).unwrap();

    let source_code = "import {sayHi} from './say.js';
const x = 1;
console.log(x);";
    let tree = parser.parse(source_code, None).unwrap();
    let mut nodes_before = get_all_nodes(&tree);
    for node in &nodes_before {
        match node.kind() {
            "import_statement" => {
                println!("{:?}", node.child(0));
                println!("{:?}", node.child(1));
                println!("{:?}", node.child(2));
                println!("{:?}", node.child(3));
            }
            _ => {}
        }
    }
}

fn get_all_nodes(tree: &Tree) -> Vec<Node> {
    let mut result = Vec::new();
    let mut visited_children = false;
    let mut cursor = tree.walk();
    loop {
        result.push(cursor.node());
        if !visited_children && cursor.goto_first_child() {
            continue;
        } else if cursor.goto_next_sibling() {
            visited_children = false;
        } else if cursor.goto_parent() {
            visited_children = true;
        } else {
            break;
        }
    }
    return result;
}
