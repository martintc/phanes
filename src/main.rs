use menu::Menu;

mod datamanager;
mod menu;

fn main() {
    let menu = Menu::new();
    menu.run();
}
