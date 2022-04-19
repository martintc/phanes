use std::borrow::Borrow;

use cursive::views::{Button, Dialog, LinearLayout, SelectView};
use cursive::{Cursive, CursiveExt};

use crate::datamanager::db::*;
use crate::datamanager::task::Task;
use crate::datamanager::*;

pub fn run_app() {
    let mut app = Cursive::default();

    app.set_user_data(Database {
        path: String::from("/Users/toddmartin/d.db"),
    });

    app.add_global_callback('q', Cursive::quit);

    app.add_layer(
        Dialog::new().title("Phanes - Main Menu").content(
            LinearLayout::vertical()
                /*
                .child(Button::new("View Tasks Menu", |a| {
                    view_tasks_menu(a, &db.clone());
                }))
                */
                .child(Button::new("View Tasks menu", |a| {
                    view_tasks_menu(a);
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

fn view_tasks_menu(app: &mut Cursive) {
    app.add_layer(
        Dialog::new()
            .title("Phanes - View Tasks Menu")
            .content(
                LinearLayout::vertical()
                    .child(Button::new("View all Open Tasks", |a| {
                        view_tasks_lists(a, 1);
                    }))
                    .child(Button::new("View all In-Process Tasks", |a| {
                        view_tasks_lists(a, 2);
                    }))
                    .child(Button::new("View all closed Tasks", |a| {
                        view_tasks_lists(a, 3);
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

fn view_tasks_lists(app: &mut Cursive, status: i64) {
    let d: &Database = app.user_data::<Database>().unwrap();
    let results: Vec<Task> = match task::get_task_by_status(d, status) {
        Ok(r) => r,
        Err(_) => panic!(),
    };

    let mut selection = SelectView::new();
    for task in results {
        selection.add_item(task.get_task_title(), task.get_task_id());
    }
    app.add_layer(Dialog::around(selection).title("Tasks")
        .button("Return", |a| {
            a.pop_layer();
        }));
}
