use menu::Menu;
use tui::*;

mod datamanager;
mod menu;
mod tui;

fn main() {
    // let menu = Menu::new();
    // menu.run();
    let tui: TUI_Instance;
    tui.run_app();
}
