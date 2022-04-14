use menu::Menu;

mod menu;
mod datamanager;

fn main() {
    let menu = Menu::new();
    menu.run();
}
