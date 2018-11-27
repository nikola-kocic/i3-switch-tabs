#[macro_use]
extern crate clap;
use clap::{Arg,App};

extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply::{Node, NodeType};

fn get_nodes<'a, F>(tree: &'a Node, condition: &F) -> Option<Vec<&'a Node>>
where
    F: Fn(&Node) -> bool,
{
    for node in &tree.nodes {
        if node.focused {
            let r = vec![node];
            return Some(r);
        }
        if let Some(mut n) = get_nodes(node, condition) {
            n.push(node);
            return Some(n);
        }
    }
    None
}

fn get_current_tab<'a>(nodes: &[&'a Node]) -> &'a Node {
    let i = nodes
        .iter()
        .position(|&n| match n.nodetype {
            NodeType::Con => false,
            _ => true,
        })
        .unwrap();
    // i is Workspace
    // i - 1 is "Root container" for workspace tabs
    // i - 2 is current tab
    nodes[i - 2]
}

fn focus_child(c: &mut I3Connection) -> bool {
    let r = c.run_command("focus child").unwrap();
    for o in r.outcomes {
        if !o.success {
            return false;
        }
    }
    true
}

fn superfocus(c: &mut I3Connection, direction: &str) {
    let tree = c.get_tree().unwrap();
    let nodes = get_nodes(&tree, &|n: &Node| { n.focused }).expect(
        "Can not find focused window. Maybe focused window is floating?",
    );
    let current_tab = get_current_tab(&nodes);
    if current_tab.id != nodes[0].id {
        let focus_tab_msg = format!("[con_id=\"{}\"] focus", current_tab.id);
        c.run_command(&focus_tab_msg).unwrap();
    }
    let focus_direction_msg = "focus ".to_string() + direction;
    c.run_command(&focus_direction_msg).unwrap();
    while focus_child(c) {}
}

fn main() {
    let matches = App::new("i3 Switch Tabs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("direction")
                .help("left|right|down|up")
                .takes_value(false),
        )
        .get_matches();

    let direction = matches.value_of("direction").unwrap();
    let mut connection = I3Connection::connect().unwrap();
    superfocus(&mut connection, &direction);
}
