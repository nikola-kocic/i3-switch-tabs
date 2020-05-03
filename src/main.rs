#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate i3ipc;
use i3ipc::reply::{Node, NodeLayout, NodeType};
use i3ipc::I3Connection;

// Return the list of nodes related to node fulfilling condition.
// First element of list is the node fulfilling condition, second is its parent node, etc.
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

fn get_current_tab<'a>(nodes: &[&'a Node]) -> Option<&'a Node> {
    let tab_node = nodes
        .iter()
        .rev()
        .skip_while(|&&n| n.nodetype != NodeType::Workspace)
        .skip_while(|&&n| n.layout != NodeLayout::Tabbed) // "Root container" for tabs
        .nth(1); // current tab

    tab_node.copied()
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
    let nodes = get_nodes(&tree, &|n: &Node| n.focused)
        .expect("Can not find focused window. Maybe focused window is floating?");
    if let Some(current_tab) = get_current_tab(&nodes) {
        if current_tab.id != nodes[0].id {
            let focus_tab_msg = format!("[con_id=\"{}\"] focus", current_tab.id);
            c.run_command(&focus_tab_msg).unwrap();
        }
        let focus_direction_msg = "focus ".to_string() + direction;
        c.run_command(&focus_direction_msg).unwrap();
        while focus_child(c) {}
    }
}

fn check_i3_version(c: &mut I3Connection) -> bool {
    let i3_version = c.get_version().unwrap();
    let valid_version = i3_version.major > 4 || (i3_version.major == 4 && i3_version.minor >= 8);
    if !valid_version {
        eprintln!(
            "Error: Your i3wm version is too old, 4.8 or newer is required. You are running {}",
            i3_version.human_readable
        );
    }
    valid_version
}

fn real_main() -> i32 {
    let matches = App::new("i3 Switch Tabs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("direction")
                .required(true)
                .help("left|right|down|up")
                .takes_value(false),
        )
        .get_matches();

    let direction = matches.value_of("direction").unwrap();

    let mut connection = I3Connection::connect().unwrap();

    if !check_i3_version(&mut connection) {
        return 1;
    }

    superfocus(&mut connection, &direction);
    0
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}
