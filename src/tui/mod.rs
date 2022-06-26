use std::borrow::Borrow;
use std::sync::Arc;

use cursive::traits::*;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, PaddedView, RadioGroup, SelectView, TextArea, TextView,
};
use cursive::{Cursive, CursiveExt};

use languages_rs::{load, Config, LanguageTexts, Languages, Value};

use sys_locale::get_locale;

use crate::datamanager::category::{get_all_categories, Category};
use crate::datamanager::db::*;
use crate::datamanager::status;
use crate::datamanager::task::{change_task_category, Task};
use crate::datamanager::*;

pub struct Session {
    app: Cursive,
    pub db: Arc<Database>,
    pub locale: Arc<LanguageTexts>,
}

impl Session {
    pub fn new() -> Self {
        Session { 
            app: Cursive::default(), 
            db: Arc::new(Database {
            path: String::from(""),
            }),
            locale: Arc::new(set_up_locale()), 
        }
    }

    pub fn run(&mut self) {
        self.app.add_global_callback('q', Cursive::quit);

        self.get_db_tui();

    }

    fn get_db_tui(&mut self) {
        //let session: Session = app.take_user_data::<Session>().unwrap();
        //let locale: LanguageTexts = session.locale.clone();
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale.try_get_text("db_path").unwrap().get_string().unwrap().clone(),
                )
                .content(EditView::new().with_name("path")
                )
                .button(self.locale.try_get_text("existing_db").unwrap().get_string().unwrap(), |a| {
                    let p = a.call_on_name("path", |p: &mut EditView| {
                        p.get_content()
                    }).unwrap();
                    let db: Database = Database::new(p.to_string());
                    self.db = Arc::new(db);
                    self.app.pop_layer();
                    self.main_menu_tui();
    
                })
                .button(self.locale.try_get_text("new_db").unwrap().get_string().unwrap(),|a| {
                    let p = a.call_on_name("path", |p: &mut EditView| {
                        p.get_content()
                    }).unwrap();
                    let db: Database = match Database::create_new_db(p.to_string()) {
                        Ok(db) => db,
                        Err(_) => panic!("Can not create the database as requested"),
                    };
                    self.db = Arc::new(db);
                    self.app.pop_layer();
                    self.main_menu_tui();
    
                })
            .button(self.locale.try_get_text("quit").unwrap().get_string().unwrap(), |a| {
            a.quit()
            })
        )
    }

    fn main_menu_tui(&mut self) {
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("main_title")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(
                    LinearLayout::vertical()
                        .child(Button::new(
                            self.locale
                                .try_get_text("view_task_menu")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.view_tasks_menu();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("manage_task")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.view_task_manager();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("manage_categories")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.view_category_manager();
                            },
                        ))
                        .child(Button::new(
                            self.locale.try_get_text("quit").unwrap().get_string().unwrap(),
                            |a| a.quit(),
                        )),
                ),
        );
    }

    fn view_task_manager(&mut self) {
        self.app.add_layer(
            Dialog::new()
                .title("Phanes - View Tasks Menu")
                .content(
                    LinearLayout::vertical()
                        .child(Button::new(
                            self.locale
                                .try_get_text("add_task")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.add_task_ui();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("del_task")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.delete_task_ui();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("move_in_prog")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.move_in_prog_task_ui();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("move_to_closed")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.move_closed_task_ui();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("assign_task_cat")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.assign_task_category_ui();
                            },
                        )),
                )
                .button(
                    self.locale
                        .try_get_text("return_menu")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }

    fn view_tasks_menu(&mut self) {
        let view_open_label: String = self.locale
            .try_get_text("view_all_open")
            .unwrap()
            .get_string()
            .unwrap();
        let view_inprogress_label: String = self.locale
            .try_get_text("view_all_in_progress")
            .unwrap()
            .get_string()
            .unwrap();
        let view_closed_label: String = self.locale
            .try_get_text("view_all_closed")
            .unwrap()
            .get_string()
            .unwrap();
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("view_tasks_title")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(
                    LinearLayout::vertical()
                        .child(Button::new(view_open_label, |a| {
                            self.view_tasks_lists(1, "Open Tasks");
                        }))
                        .child(Button::new(view_inprogress_label, |a| {
                            self.view_tasks_lists(2, "In-Progress Tasks");
                        }))
                        .child(Button::new(view_closed_label, |a| {
                            self.view_tasks_lists(3, "Closed Tasks");
                        })),
                )
                .button("Return to main menu", |a| {
                    a.pop_layer();
                }),
        );
    }

    fn view_tasks_lists(&mut self, status: i64, title: &str) {
        let d: &Database = self.db.as_ref();
        let results: Vec<Task> = match task::get_task_by_status(d, status) {
            Ok(r) => r,
            Err(_) => panic!("Error is occuring here in view tasks list"),
        };
    
        let mut selection = SelectView::new();
        for task in results {
            selection.add_item(task.get_task_title(), task.get_task_id());
        }
    
        selection.set_on_submit(|a, task| {
            self.view_task(&task);
        });
    
        self.app.add_layer(
            Dialog::new()
                .title(title)
                .content(selection)
                .button("Return", |a| {
                    a.pop_layer();
                }),
        );
    }
    
    fn view_task(&mut self, id: &i64) {
        let d: &Database = self.db.as_ref();
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
    
        let title: String = self.locale.try_get_text("title").unwrap().get_string().unwrap();
        let desc: String = self.locale
            .try_get_text("description")
            .unwrap()
            .get_string()
            .unwrap();
        let status: String = self.locale.try_get_text("status").unwrap().get_string().unwrap();
        let category: String = self.locale
            .try_get_text("category")
            .unwrap()
            .get_string()
            .unwrap();
    
        self.app.add_layer(
            Dialog::new()
                .title(task.get_task_title())
                .content(
                    LinearLayout::vertical()
                        .child(
                            LinearLayout::horizontal()
                                .child(PaddedView::lrtb(1, 7, 0, 1, TextView::new(title)))
                                .child(PaddedView::lrtb(
                                    1,
                                    1,
                                    0,
                                    1,
                                    TextView::new(task.get_task_title()),
                                )),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(desc)))
                                .child(PaddedView::lrtb(
                                    1,
                                    1,
                                    0,
                                    1,
                                    TextView::new(task.get_task_desc()),
                                )),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(PaddedView::lrtb(1, 6, 0, 1, TextView::new(status)))
                                .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(status_name))),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(PaddedView::lrtb(1, 5, 0, 1, TextView::new(category)))
                                .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(cat_name))),
                        ),
                )
                .button("Close display", |a| {
                    a.pop_layer();
                }),
        );
    }
    
    fn add_task_ui(&mut self) {
        let title: String = self.locale.try_get_text("title").unwrap().get_string().unwrap();
        let desc: String = self.locale
            .try_get_text("description")
            .unwrap()
            .get_string()
            .unwrap();
    
        self.app.add_layer(
            Dialog::new()
                .title("Add a task")
                .content(
                    LinearLayout::vertical()
                        .child(
                            LinearLayout::horizontal()
                                .child(PaddedView::lrtb(1, 7, 0, 1, TextView::new(title)))
                                .child(PaddedView::lrtb(
                                    1,
                                    1,
                                    0,
                                    1,
                                    EditView::new().with_name("title_entry").fixed_width(20),
                                )),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(PaddedView::lrtb(1, 1, 0, 1, TextView::new(desc)))
                                .child(PaddedView::lrtb(
                                    1,
                                    1,
                                    0,
                                    1,
                                    EditView::new().with_name("desc_entry").fixed_width(20),
                                )),
                        ),
                )
                .button("Return", |a| {
                    a.pop_layer();
                })
                .button("Submit", |a| {
                    let title = a
                        .call_on_name("title_entry", |view: &mut EditView| view.get_content())
                        .unwrap();
                    let desc = a
                        .call_on_name("desc_entry", |view: &mut EditView| view.get_content())
                        .unwrap();
                    let db: &Database = self.db.as_ref();
                    match task::add_tasks(db, title.to_string(), desc.to_string(), 1, 1) {
                        Ok(_) => {
                            self.success_pop_up();
                        },
                        Err(_) => {
                            self.failure_pop_up();
                        },
                    }
                }),
        )
    }
    
    fn delete_task_ui(&mut self) {
        let d: &Database = self.db.as_ref();
        let tasks: Vec<Task> = match task::get_task_list(d) {
            Ok(list) => list,
            Err(_) => panic!("Error fetching list of tasks"),
        };
    
        let mut selection = SelectView::new();
        for task in tasks.iter() {
            selection.add_item(task.get_task_title(), task.get_task_id());
        }
    
        selection.set_on_submit(|a, task| {
            self.del_task(task);
            a.pop_layer();
        });
    
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("del_task")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(selection)
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }
    
    fn del_task(&mut self, id: &i64) {
        let db: &Database = self.db.as_ref();
        match task::remove_task(db, *id) {
            Ok(_) => {
                self.success_pop_up();
            }
            Err(_) => {
                self.failure_pop_up();
            }
        }
    }
    
    fn move_in_prog_task_ui(&mut self) {
        let d: &Database = self.db.as_ref();
        let tasks: Vec<Task> = match task::get_task_by_status(d, 1) {
            Ok(list) => list,
            Err(_) => panic!("Error fetching list of tasks"),
        };
    
        let mut selection = SelectView::new();
        for task in tasks.iter() {
            selection.add_item(task.get_task_title(), task.get_task_id());
        }
    
        selection.set_on_submit(|a, task| {
            self.move_in_prog(task);
        });
    
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("move_in_prog")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(selection)
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }
    
    fn move_in_prog(&mut self, id: &i64) {
        let db: &Database = self.db.as_ref();
        match task::change_task_status(db, *id, 2) {
            Ok(_) => {
                self.success_pop_up();
            }
            Err(_) => {
                self.failure_pop_up();
            }
        }
    }
    
    fn move_closed_task_ui(&mut self) {
        let d: &Database = self.db.as_ref();
        let tasks: Vec<Task> = match task::get_task_by_status(d, 2) {
            Ok(list) => list,
            Err(_) => panic!("Error fetching list of tasks"),
        };
    
        let mut selection = SelectView::new();
        for task in tasks.iter() {
            selection.add_item(task.get_task_title(), task.get_task_id());
        }
    
        selection.set_on_submit(|a, task| {
            self.move_closed(task);
        });
    
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("move_to_closed")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(selection)
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }
    
    fn move_closed(&mut self, id: &i64) {
        let db: &Database = self.db.as_ref();
        match task::change_task_status(db, *id, 3) {
            Ok(_) => {
                self.success_pop_up();
            }
            Err(_) => {
                self.failure_pop_up();
            }
        }
    }
    
    fn assign_task_category_ui(&mut self) {
        let d: &Database = self.db.as_ref();
    
        let tasks: Vec<Task> = task::get_tasks_by_category(d, 1).unwrap();
        let categories: Vec<Category> = category::get_all_categories(d).unwrap();
    
        let mut select_task: RadioGroup<i64> = RadioGroup::new();
        let mut select_cat: RadioGroup<i64> = RadioGroup::new();
    
        // for task in tasks.iter() {
        //     select_task.button(task.get_task_id(), task.get_task_title());
        // }
    
        let mut task_layout = LinearLayout::vertical();
        for task in tasks.iter() {
            task_layout.add_child(select_task.button(task.get_task_id(), task.get_task_title()));
        }
    
        let mut cat_layout = LinearLayout::vertical();
        for cat in categories.iter() {
            cat_layout.add_child(select_cat.button(cat.get_id(), cat.get_name()));
        }
    
        self.app.add_layer(
            Dialog::new()
                .content(
                    LinearLayout::horizontal()
                        .child(task_layout)
                        .child(cat_layout),
                )
                .button(
                    self.locale.try_get_text("select").unwrap().get_string().unwrap(),
                     |a| {
                        let d: &Database = self.db.as_ref();
                        let task = select_task.selection().clone();
                        let cat = select_cat.selection().clone();
                        match task::change_task_category(d, *task, *cat) {
                            Ok(_) => {
                                self.success_pop_up();
                            }
                            Err(_) => {
                                self.failure_pop_up();
                            }
                        }
                    },
                )
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }
    
    fn view_category_manager(&mut self) {
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("manage_categories")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(
                    LinearLayout::vertical()
                        .child(Button::new(
                            self.locale
                                .try_get_text("add_cat")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.add_category_tui();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("del_cat")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.del_category_tui();
                            },
                        ))
                        .child(Button::new(
                            self.locale
                                .try_get_text("list_cat")
                                .unwrap()
                                .get_string()
                                .unwrap(),
                            |a| {
                                self.view_categories_tui();
                            },
                        )),
                )
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }
    
    fn add_category_tui(&mut self) {
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("add_cat")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(EditView::new().with_name("name"))
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                )
                .button(
                    self.locale.try_get_text("submit").unwrap().get_string().unwrap(),
                    |a| {
                        let name = a
                            .call_on_name("name", |view: &mut EditView| view.get_content())
                            .unwrap();
                        self.add_cat(&name);
                    },
                ),
        );
    }
    
    fn add_cat(&mut self, name: &str) {
        let db: &Database = self.db.as_ref();
        match category::add_category(db, name.to_string()) {
            Ok(_) => {
                self.success_pop_up();
            }
            Err(_) => {
                self.failure_pop_up();
            }
        }
    }
    
    fn del_category_tui(&mut self) {
        let d: &Database = self.db.as_ref();
        let categories = match category::get_all_categories(d) {
            Ok(list) => list,
            Err(_) => panic!("Error fetching list of tasks"),
        };
    
        let mut selection = SelectView::new();
        for cat in categories.iter() {
            selection.add_item(cat.get_name(), cat.get_id());
        }
    
        selection.set_on_submit(|a, cat| {
            self.del_cat(cat);
        });
    
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("del_cat")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(selection)
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        );
    }
    
    fn del_cat(&mut self, id: &i64) {
        let db: &Database = self.db.as_ref();
        match category::remove_category(db, *id) {
            Ok(_) => {
                self.success_pop_up();
            }
            Err(_) => {
                self.failure_pop_up();
            }
        }
    }
    
    fn view_categories_tui(&mut self) {
        let db: &Database = self.db.as_ref();
    
        let categories = match get_all_categories(db) {
            Ok(list) => list,
            Err(_) => panic!("Issue getting a list of categories from database"),
        };
    
        let mut cat_list = SelectView::new();
        for cat in categories.iter() {
            cat_list.add_item(cat.get_name(), cat.get_id());
        }
    
        self.app.add_layer(
            Dialog::new()
                .title(
                    self.locale
                        .try_get_text("list_cat")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                )
                .content(cat_list)
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                    },
                ),
        )
    }
    
    fn success_pop_up(&mut self) {
        self.app.add_layer(
            Dialog::new()
                .content(TextView::new(
                    self.locale
                        .try_get_text("success")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                ))
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                        a.pop_layer();
                    },
                ),
        )
    }
    
    fn failure_pop_up(&mut self) {
        self.app.add_layer(
            Dialog::new()
                .content(TextView::new(
                    self.locale
                        .try_get_text("failure")
                        .unwrap()
                        .get_string()
                        .unwrap(),
                ))
                .button(
                    self.locale.try_get_text("return").unwrap().get_string().unwrap(),
                    |a| {
                        a.pop_layer();
                        a.pop_layer();
                    },
                ),
        )
    }
    

}

fn set_up_locale() -> LanguageTexts {
    let locale = get_locale().unwrap();
    let mut configuration: Config = Config::default().unwrap();
    configuration.add_language(locale.clone()).unwrap();
    let mut texts: Languages = load(configuration).unwrap();
    let locale_text: LanguageTexts = match texts.try_get_language(locale.as_str()) {
        Ok(loc) => loc,
        Err(_) => texts.try_get_language("en-US").unwrap(),
    };
    locale_text
}