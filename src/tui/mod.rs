use cursive::traits::Resizable;
use cursive::views::{Button, Dialog, LinearLayout, SelectView, TextView, PaddedView, TextArea};
use cursive::{Cursive, CursiveExt};

use crate::datamanager::db::*;
use crate::datamanager::task::Task;
use crate::datamanager::status;
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
                    view_task_manager(a);
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
                        view_tasks_lists(a, 1, "Open Tasks");
                    }))
                    .child(Button::new("View all In-Process Tasks", |a| {
                        view_tasks_lists(a, 2, "In-Progress Tasks");
                    }))
                    .child(Button::new("View all closed Tasks", |a| {
                        view_tasks_lists(a, 3, "Closed Tasks");
                    }))
            )
            .button("Return to Main Menu", |a| {
                a.pop_layer();
            }),
    );
}

fn view_tasks_lists(app: &mut Cursive, status: i64, title: &str) {
    let d: &Database = app.user_data::<Database>().unwrap();
    let results: Vec<Task> = match task::get_task_by_status(d, status) {
        Ok(r) => r,
        Err(_) => panic!(),
    };

    let mut selection = SelectView::new();
    for task in results {
        selection.add_item(task.get_task_title(), task.get_task_id());
    }


    selection.set_on_submit(|a, task| {
        view_task(a, &task);
    });

    app.add_layer(
        Dialog::around(selection)
            .title(title)
            .button("Return", |a| {
                a.pop_layer();
            }),
    );
}

fn view_task(app: &mut Cursive, id: &i64) {
    let d: &Database = app.user_data::<Database>().unwrap();
    let task = match task::get_task_by_id(d, *id) {
        Ok(task) => task,
        Err(_) => panic!("An error occured"),
    };

    let cat_name = match category::get_category_name(d, *id) {
        Ok(i) => i,
        Err(_) => "None found".to_string(),
    };

    let status_name = match status::get_status_name(d, *id) {
        Ok(i) => i,
        Err(_) => "None designated".to_string(),
    };

    app.add_layer(
        Dialog::new()
            .title(task.get_task_title())
            .content(
            LinearLayout::vertical()
                    .child(
                    LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 7, 0, 1, TextView::new("Title:")))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(task.get_task_title()))),
                    )
                    .child(
                        LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new("Description:")))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(task.get_task_desc())))
                    )
                    .child(LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 6, 0, 1, TextView::new("Status:")))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(status_name)))
                    )
                    .child(LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 5, 0, 1, TextView::new("Category")))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(cat_name)))
                    )
            )
            .button("Close display", |a| {
                a.pop_layer();
            }),
    );
}

fn view_task_manager(app: &mut Cursive) {
    app.add_layer(
        Dialog::new()
            .title("Phanes - View Tasks Menu")
            .content(
                LinearLayout::vertical()
                    .child(Button::new("Add a Task", |a| {
                        add_task(a);
                    }))
                    .child(Button::new("Delete a Task", |a| {
                        println!("Delete a task");
                    }))
                    .child(Button::new("Move task to in-progress", |a| {
                        println!("Move to in-progress");
                    }))
                    .child(Button::new("Move task to closed", |a| {
                        println!("Move to closed");
                    }))
                    .child(Button::new("Move task to closed", |a| {
                        println!("Assign task a category");
                    }))
            )
            .button("Return to Main Menu", |a| {
                a.pop_layer();
            }),
    );
}

fn add_task(app: &mut Cursive) {
    let mut title_area: TextArea = TextArea::new();
    let mut desc_area: TextArea = TextArea::new();

    app.add_layer(
        Dialog::new()
            .title("Add a task")
            .content(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(PaddedView::lrtb(1, 7, 0, 1, TextView::new("Title:")))
                            .child(PaddedView::lrtb(1, 1, 0, 1, title_area))
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new("Description:")))
                            .child(PaddedView::lrtb(1, 1, 0, 1, desc_area))
                    )
            )
            .button("Return", |a| {
                a.pop_layer();
            })
            .button("Submit", |a| {
                println!("TODO");
            }),
    )
}
