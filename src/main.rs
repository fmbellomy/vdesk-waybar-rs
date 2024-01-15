use hyprland::data::{Monitors, Workspaces};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;
use std::env;
use std::sync::OnceLock;
static ARGS: OnceLock<i32> = OnceLock::new();
fn main() -> hyprland::Result<()> {
    // naked unwraps because i do not care enough to provide good errors
    let args: Vec<String> = env::args().collect();
    ARGS.set(args[1].parse().unwrap()).unwrap();
    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(|id, _| {
        let monitors = Monitors::get().unwrap().to_vec();
        let target_monitor = monitors
            .get(monitors.len() - ((*ARGS.get().unwrap()) as usize))
            .unwrap()
            .id;
        let target_workspace = match &id {
            WorkspaceType::Regular(data) => Some(data.parse::<i32>().unwrap()),

            WorkspaceType::Special(_) => None,
        }
        .unwrap();
        if target_workspace as usize % Monitors::get().unwrap().count() != target_monitor as usize {
            return;
        }

        let mut workspaces = Workspaces::get().unwrap().to_vec();
        workspaces.sort_by(|a, b| a.id.cmp(&b.id));
        for workspace in workspaces {
            if target_monitor as usize != workspace.id as usize % Monitors::get().unwrap().count() {
                continue;
            }
            if id == WorkspaceType::Regular(workspace.id.to_string()) {
                print!("active ");
                continue;
            } else {
                print!("{} ", workspace.id);
            }
        }
        println!();
    });
    event_listener.start_listener()
}
