#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate i3ipc;
use i3ipc::reply::{Node, NodeLayout, NodeType};
use i3ipc::{I3Connection, MessageError};

// Return the list of nodes related to node fulfilling condition.
// First element of list is the node fulfilling condition, second is its parent node, etc.
fn get_nodes<'a, F>(tree: &'a Node, condition: &F) -> Option<Vec<&'a Node>>
where
    F: Fn(&Node) -> bool,
{
    for node in &tree.nodes {
        if condition(node) {
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

fn get_node<'a, F>(tree: &'a Node, condition: &F) -> Option<&'a Node>
where
    F: Fn(&Node) -> bool,
{
    for node in &tree.nodes {
        if condition(node) {
            return Some(node);
        }
        if let Some(n) = get_node(node, condition) {
            return Some(n);
        }
    }
    None
}

fn get_focused_node_id(c: &mut I3Connection) -> Result<Option<i64>, MessageError> {
    let tree = c.get_tree()?;
    Ok(get_node(&tree, &|n: &Node| n.focused).map(|n| n.id))
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

fn focus_child(c: &mut I3Connection) -> Result<bool, MessageError> {
    let mut focused_node_id = get_focused_node_id(c)?;
    loop {
        let r = c.run_command("focus child")?;
        for o in r.outcomes {
            if !o.success {
                return Ok(false);
            }
        }
        let new_focused_node_id = get_focused_node_id(c)?;
        if new_focused_node_id == focused_node_id {
            return Ok(false);
        }
        focused_node_id = new_focused_node_id;
    }
}

fn superfocus(c: &mut I3Connection, direction: &str) -> Result<(), MessageError> {
    let tree = c.get_tree()?;
    let nodes = get_nodes(&tree, &|n: &Node| n.focused)
        .expect("Can not find focused window. Maybe focused window is floating?");
    if let Some(current_tab) = get_current_tab(&nodes) {
        if current_tab.id != nodes[0].id {
            let focus_tab_msg = format!("[con_id=\"{}\"] focus", current_tab.id);
            c.run_command(&focus_tab_msg)?;
        }
        let focus_direction_msg = "focus ".to_string() + direction;
        c.run_command(&focus_direction_msg)?;
        while focus_child(c)? {}
    }
    Ok(())
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

    match I3Connection::connect() {
        Ok(mut connection) => match superfocus(&mut connection, &direction) {
            Ok(()) => 0,
            Err(MessageError::JsonCouldntParse(e)) => {
                eprintln!("Error: {}", e);
                let i3_version = connection.get_version().unwrap();
                eprintln!(
                    "Your WM version might be too old, i3wm 4.8 or newer or \
                        sway 1.0 or newer is required. You are running version {}",
                    i3_version.human_readable
                );
                1
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                1
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}
