extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply::Node;

fn get_node<'a, F>(tree: &'a Node, condition: &F) -> Option<&'a Node>
    where F: Fn(&Node) -> bool
{
    for node in &tree.nodes {
        if node.focused {
            return Some(node);
        }
        if let Some(n) = get_node(&node, condition) {
            return Some(n);
        }
    }
    return None;
}

fn superfocus(c: &mut I3Connection, direction: &str) {}

fn main() {
    let mut connection = I3Connection::connect().unwrap();
    let tree = connection.get_tree().unwrap();
    let node = get_node(&tree,
                        &|n: &Node| {
                            return n.focused;
                        });
    println!("node: {:?}", node);
}
