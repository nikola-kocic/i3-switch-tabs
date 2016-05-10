extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply::Node;

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

fn superfocus(c: &mut I3Connection, direction: &str) {}

fn main() {
    let mut connection = I3Connection::connect().unwrap();
    let tree = connection.get_tree().unwrap();
    let nodes = get_nodes(&tree, &|n: &Node| { return n.focused; }).unwrap();
}
