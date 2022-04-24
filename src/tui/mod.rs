use std::borrow::Borrow;

use cursive::traits::*;
use cursive::views::{Button, Dialog, LinearLayout, SelectView, EditView, TextView, PaddedView, TextArea};
use cursive::{Cursive, CursiveExt};

use languages_rs::{Config, Languages, LanguageTexts, load, Value};

use sys_locale::get_locale;

use crate::datamanager::db::*;
use crate::datamanager::task::Task;
use crate::datamanager::status;
use crate::datamanager::*;

struct Session {
    pub db: Database,
    pub locale: LanguageTexts,
}

// initial entry point of graphicalk TUI application
pub fn run_app() {
    let locale = set_up_locale();
    let mut app = Cursive::default();

    let db = Database {
        path: String::from("/users/toddmartin/d.db"),
    };

    app.set_user_data(Session {
        db: db,
        locale: locale.clone(),
    });

    app.add_global_callback('q', Cursive::quit);

    app.add_layer(
        Dialog::new()
            .title(locale.try_get_text("main_title").unwrap().get_string().unwrap())
            .content(
            LinearLayout::vertical()
                .child(Button::new(locale.try_get_text("view_task_menu").unwrap().get_string().unwrap(), |a| {
                    view_tasks_menu(a);
                }))
                .child(Button::new(locale.try_get_text("manage_task").unwrap().get_string().unwrap(), |a| {
                    view_task_manager(a);
                }))
                .child(Button::new(locale.try_get_text("manage_categories").unwrap().get_string().unwrap(), |a| {
                    println!("manage Categories")
                }))
                .child(Button::new(locale.try_get_text("quit").unwrap().get_string().unwrap(), |a| a.quit())),
        ),
    );

    app.run();
}

fn set_up_locale() -> LanguageTexts {
    let locale = get_locale().unwrap();
    let mut configuration: Config = Config::default().unwrap();
    configuration.add_language(locale.clone()).unwrap();
    let mut texts: Languages = load(configuration).unwrap();
    let locale_text : LanguageTexts = match texts.try_get_language(locale.as_str()) {
        Ok(loc) => loc,
        Err(_) => texts.try_get_language("en-US").unwrap(),
    };
    locale_text
}

fn view_tasks_menu(app: &mut Cursive) {
    let locale: &LanguageTexts = &app.user_data::<Session>().unwrap().locale.clone();
    let view_open_label: String = locale.try_get_text("view_all_open").unwrap().get_string().unwrap();
    let view_inprogress_label: String = locale.try_get_text("view_all_in_progress").unwrap().get_string().unwrap();
    let view_closed_label: String = locale.try_get_text("view_all_closed").unwrap().get_string().unwrap();
    app.add_layer(
        Dialog::new()
            .title(locale.try_get_text("view_tasks_title").unwrap().get_string().unwrap())
            .content(
                LinearLayout::vertical()
                    .child(Button::new(view_open_label, |a| {
                        view_tasks_lists(a, 1, "Open Tasks");
                    }))
                    .child(Button::new(view_inprogress_label, |a| {
                        view_tasks_lists(a, 2, "In-Progress Tasks");
                    }))
                    .child(Button::new(view_closed_label, |a| {
                        view_tasks_lists(a, 3, "Closed Tasks");
                    }))
            )
            .button("Return to main menu", |a| {
                a.pop_layer();
            }),
    );
}

fn view_tasks_lists(app: &mut Cursive, status: i64, title: &str) {
    let d: &Database = match app.user_data::<Session>() {
        Some(d) => &d.db,
        None => panic!(),
    };
    let results: Vec<Task> = match task::get_task_by_status(d, status) {
        Ok(r) => r,
        Err(_) => panic!("Error is occuring here in view tasks list"),
    };

    let mut selection = SelectView::new();
    for task in results {
        selection.add_item(task.get_task_title(), task.get_task_id());
    }

    selection.set_on_submit(|a, task| {
        view_task(a, &task);
    });

    app.add_layer(
        Dialog::new()
            .title(title)
            .content(selection)
            .button("Return", |a| {
                a.pop_layer();
            }),
    );
}

fn view_task(app: &mut Cursive, id: &i64) {
    // let locale: &LanguageTexts = &app.user_data::<LanguageTexts>().unwrap().clone();
    let locale: &LanguageTexts = &app.user_data::<Session>().unwrap().locale.clone();
    let d: &Database = &app.user_data::<Session>().unwrap().db;
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

    let title: String = locale.try_get_text("title").unwrap().get_string().unwrap();
    let desc: String = locale.try_get_text("description").unwrap().get_string().unwrap();
    let status: String = locale.try_get_text("status").unwrap().get_string().unwrap();
    let category: String = locale.try_get_text("category").unwrap().get_string().unwrap();

    app.add_layer(
        Dialog::new()
            .title(task.get_task_title())
            .content(
            LinearLayout::vertical()
                    .child(
                    LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 7, 0, 1, TextView::new(title)))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(task.get_task_title()))),
                    )
                    .child(
                        LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(desc)))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(task.get_task_desc())))
                    )
                    .child(LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 6, 0, 1, TextView::new(status)))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(status_name)))
                    )
                    .child(LinearLayout::horizontal()
                        .child(PaddedView::lrtb(1, 5, 0, 1, TextView::new(category)))
                        .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(cat_name)))
                    )
            )
            .button("Close display", |a| {
                a.pop_layer();
            }),
    );
}

// TODO: Use locales
fn view_task_manager(app: &mut Cursive) {
    let locale: &LanguageTexts = &app.user_data::<Session>().unwrap().locale.clone();
    app.add_layer(
        Dialog::new()
            .title("Phanes - View Tasks Menu")
            .content(
                LinearLayout::vertical()
                    .child(Button::new("Add a Task", |a| {
                        add_task_ui(a);
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
            .button(locale.try_get_text("return_menu").unwrap().get_string().unwrap(), |a| {
                a.pop_layer();
            }),
    );
}

fn add_task_ui(app: &mut Cursive) {
    let locale: &LanguageTexts = &app.user_data::<Session>().unwrap().locale.clone();
    let title: String = locale.try_get_text("title").unwrap().get_string().unwrap();
    let desc: String = locale.try_get_text("description").unwrap().get_string().unwrap();

    app.add_layer(
        Dialog::new()
            .title("Add a task")
            .content(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(PaddedView::lrtb(1, 7, 0, 1, TextView::new(title)))
                            .child(PaddedView::lrtb(1, 1, 0, 1, EditView::new().with_name("title_entry").fixed_width(20)))
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(desc)))
                            .child(PaddedView::lrtb(1, 1, 0, 1, EditView::new().with_name("desc_entry").fixed_width(20)))
                    )
            )
            .button("Return", |a| {
                a.pop_layer();
            })
            .button("Submit", |a| {
                let title = a.call_on_name("title_entry", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                let desc = a.call_on_name("desc_entry", |view: &mut EditView| {
                   view.get_content()
                }).unwrap();
                let db: &Database = &a.user_data::<Session>().unwrap().db;
                task::add_tasks(db, title.to_string(), desc.to_string(), 1, 1);
                a.pop_layer();
            }),
    )
}

// TODO: Finish implementation
fn delete_task_ui(app: &mut Cursive) {
    let locale: &LanguageTexts = &app.user_data::<Session>().unwrap().locale.clone();
    let db: &Database = &app.user_data::<Session>().unwrap().db;
    let tasks: Vec<Task> = match task::get_task_list(db) {
        Ok(list) => list,
        Err(_) => panic!("Error fetching list of tasks"),
    };

}


