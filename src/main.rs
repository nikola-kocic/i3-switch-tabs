extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply::{Node, NodeType};

fn get_nodes<'a, F>(tree: &'a Node, condition: &F) -> Option<Vec<&'a Node>>
    where F: Fn(&Node) -> bool
{
    for node in &tree.nodes {
        if node.focused {
            let r = vec![node];
            return Some(r);
        }
        if let Some(mut n) = get_nodes(&node, condition) {
            n.push(node);
            return Some(n);
        }
    }
    return None;
}

fn get_current_tab<'a>(nodes: &[&'a Node]) -> &'a Node {
    let i = nodes.iter().position(|&n| {
        match n.nodetype { NodeType::Con => false, _ => true }
    }).unwrap();
    // i is Workspace
    // i - 1 is "Root container" for workspace tabs
    // i - 2 is current tab
    nodes[i - 2]
}

fn superfocus(c: &mut I3Connection, direction: &str) {
    let tree = c.get_tree().unwrap();
    let nodes = get_nodes(&tree, &|n: &Node| { return n.focused; }).unwrap();
    let current_tab = get_current_tab(&nodes);
}

fn main() {
    let mut connection = I3Connection::connect().unwrap();
    superfocus(&mut connection, "right");
}
