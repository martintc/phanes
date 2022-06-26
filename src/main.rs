use menu::Menu;
use tui::*;
use tui::Session;

mod datamanager;
mod menu;
mod tui;

fn main() {
    // let menu = Menu::new();
    // menu.run();
    let mut session: Session = tui::Session::new();
    session.run();
}
