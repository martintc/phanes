use cursive::views::{Button, Dialog, LinearLayout, TextView};
use cursive::Cursive;

use crate::datamanager::db::*;
use crate::datamanager::*;
use crate::datamanager::task::Task;

pub struct TUI_Instance {
    db: Option<Database>,
}

impl TUI_Instance {
    pub fn run_app(&mut self) {
        self.db = Some(Database {
            path: String::from("/Users/toddmartin/d.db"),
        });
        let mut app = cursive::default();

        app.add_layer(
            Dialog::new().title("Phanes - Main Menu").content(
                LinearLayout::vertical()
                    .child(Button::new("View Tasks Menu", |a| {
                        self.view_tasks_menu(a)
                    }))
                    .child(Button::new("Manage Tasks Menu", |a| {
                        println!("Manage Tasks")
                    }))
                    .child(Button::new("Manage Categories", |a| {
                        println!("manage Categories")
                    }))
                    .child(Button::new("Quit", |a| a.quit())),
            ),
        );

        app.run();
    }

    fn view_tasks_menu(&self, app: &mut Cursive) {
        app.add_layer(
            Dialog::new()
                .title("Phanes - View Tasks Menu")
                .content(
                    LinearLayout::vertical()
                        .child(Button::new("View all Open Tasks", |a| {
                            self.view_tasks_lists(a, 1)
                        }))
                        .child(Button::new("View all In-Process Tasks", |a| {
                            println!("View all in progress")
                        }))
                        .child(Button::new("View all closed Tasks", |a| {
                            println!("View all closed")
                        }))
                        .child(Button::new("View Task Information", |a| {
                            println!("View tasks information")
                        })),
                )
                .button("Return to Main Menu", |a| {
                    a.pop_layer();
                }),
        );
    }

    fn view_tasks_lists(&self, app: &mut Cursive, status: i64) {
        let temp = match &self.db {
            Some(s) => s,
            None => panic!(),
        };
        let results: Vec<Task> = match task::get_task_by_status(&temp, status) {
            Ok(r) => r,
            Err(_) => panic!(),
        };

    }
}
