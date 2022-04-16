use std::io;

use crate::datamanager::db::Database;
use crate::datamanager::*;

pub struct Menu {
    db: Database,
}

impl Menu {

    pub fn new() -> Menu {
        Menu {
            db: Menu::get_database(),
        }
    }

    pub fn run(&self) {
        self.banner();

        loop {
            self.display_main_menu();
            let input = Menu::get_number_option();
            let choice = match input {
                Some(i) => {
                    if i > 4 || i < 0 {
                        continue;
                    }
                    i
                }
                None => continue,
            };

            if choice == 1 {
                self.display_tasks_menu();
            } else if choice == 2 {
                self.manage_tasks();
            } else if choice == 3 {
                self.display_category_menu();
            } else if choice == 4 {
                return;
            }
        }
    }

    fn banner(&self) {
        println!("Phanes task manager v0.0.0\n");
    }

    fn display_main_menu(&self) {
        println!("Main Menu:");
        println!("\t1. See Tasks");
        println!("\t2. Manage Tasks");
        println!("\t3. Manage Categories");
        println!("\t4. quit");
        println!("Select an option [0-4]: ");
    }

    fn display_tasks_menu(&self) {
        println!("Options for displaying tasks:");
        println!("\t1. See all opened tasks");
        println!("\t2. See all in-progress tasks");
        println!("\t3. See all closed tasks");
        println!("\t4. See all information for a task by ID");
        println!("\t5. Back to main menu");

        let input = Menu::get_number_option();
        let choice = match input {
            Some(i) => {
                if i > 5 {
                    return;
                }
                if i < 0 {
                    return;
                }
                i
            }
            None => return,
        };

        // TODO: Need to work out returning query results
        match choice {
            1 => {
                // see all opened tasks
                match task::get_task_by_status(&self.db, 1) {
                    Ok(results) => {
                        for i in results.iter() {
                            println!("{i}");
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
            2 => match task::get_task_by_status(&self.db, 2) {
                Ok(results) => {
                    for i in results.iter() {
                        println!("{i}");
                    }
                }
                Err(e) => println!("{:?}", e),
            }, // See all in-progress tasks
            3 => match task::get_task_by_status(&self.db, 3) {
                Ok(result) => {
                    for i in result.iter() {
                        println!("{i}");
                    }
                }
                Err(e) => println!("{:?}", e),
            }, // See all closed tasks
            4 => {
                let id = match Menu::ask_question_number("Enter a task ID: ") {
                    Some(i) => i,
                    None => return,
                };
                match task::get_task_by_id(&self.db, id) {
                    Ok(t) => {
                        t.print_task();
                    },
                    Err(_) => {
                        println!("An error occured fetch task with id: {}", id);
                        return;
                    }
                };

            },
            _ => {
                // return
                println!("Not a valid option");
                return;
            }
        };
    }

    fn display_category_menu(&self) {
        println!("Options for managing categores:");
        println!("\t1. Add a category");
        println!("\t2. Delete a category");
        println!("\t3. List categories");
        println!("\t4. Return to main menu");

        let choice: i64 = match Menu::ask_question_number("Select an option [0-4]") {
            Some(i) => {
                if i < 1 {
                    println!("Error: Invalid number");
                    return;
                } else if i > 4 {
                    println!("Error: Invalid number");
                    return;
                }
                i
            }
            None => {
                println!("Error: Not a valid input");
                return;
            }
        };

        match choice {
            1 => match Menu::ask_question("Enter a name for category: ") {
                Some(title) => match category::add_category(&self.db, title) {
                    Ok(_) => println!("Category added."),
                    Err(e) => println!("{:?}", e),
                },
                None => println!("Not a valid input"),
            },
            2 => {}
            3 => match category::get_all_categories(&self.db) {
                Ok(results) => {
                    for i in results.iter() {
                        i.print_category();
                    }
                }
                Err(_) => println!("Error: Could not get all categories"),
            },
            _ => return,
        }
    }

    fn manage_tasks(&self) {
        println!("Options for managing tasks");
        println!("\t1. Add a task");
        println!("\t2. Delete a task");
        println!("\t3. Move open task to in-progress");
        println!("\t4. Move in-progress task to closed");
        println!("\t5. Assign task a category");
        println!("\t6. Return to main menu");
        println!("Enter choice");

        let number: i64 = match Menu::get_number_option() {
            Some(i) => {
                if i > 6 {
                    println!("Not a valid option");
                    return;
                } else if i < 1 {
                    println!("Not a valid option");
                }
                i
            }
            None => {
                println!("Not a valid option");
                return;
            }
        };

        match number {
            1 => self.add_task(), // add a task
            2 => {
                // Delete a task
                match Menu::ask_question_number("Enter ID of task to delete:") {
                    Some(i) => {
                        match task::remove_task(&self.db, i) {
                            Ok(_) => println!("Success, task removed!"),
                            Err(_) => println!("Task not able to be removed"),
                        }
                    }
                    None => return,
                };
            }
            3 => {
                // Move open task to in progress
                match Menu::ask_question_number("Enter ID of task to move to in-progress:") {
                    Some(i) => match task::change_task_status(&self.db, i, 2) {
                        Ok(_) => println!("Success, task moved to in-progress"),
                        Err(e) => println!("{:?}", e),
                    },
                    None => return,
                };
            }
            4 => {
                // Move task to closed
                match Menu::ask_question_number("Enter ID of task to move to closed:") {
                    Some(i) => match task::change_task_status(&self.db, i, 3) {
                        Ok(_) => println!("Success, task moved to closed"),
                        Err(e) => println!("{:?}", e),
                    },
                    None => return,
                };
            }
            5 => {
                // asign task a category
                let task_id = match Menu::ask_question_number("Enter ID of a task to assign a category:")
                {
                    Some(i) => i,
                    None => {
                        println!("Not a valid input");
                        return;
                    }
                };
                let category_id = match Menu::ask_question_number("Enter a category to assign to:") {
                    Some(i) => i,
                    None => {
                        println!("Not a valid input");
                        return;
                    }
                };
                match task::change_task_category(&self.db, task_id, category_id) {
                    Ok(_) => println!("Operation successful"),
                    Err(e) => println!("{:?}", e),
                }
            } // return
            _ => {
                return;
            }
        };
    }

    fn add_task(&self) {
        let title = match Menu::ask_question("What is the task title? ") {
            Some(s) => s,
            None => return,
        };
        let desc = match Menu::ask_question("Describe the task: ") {
            Some(s) => s,
            None => return,
        };
        let status: i64 = 1;
        // Eventually handle category better
        let category = 1;
        match task::add_tasks(&self.db, title, desc, status, category) {
            Ok(_) => println!("Success, task added!"),
            Err(e) => println!("{:?}", e),
        };
    }

    fn get_database() -> Database {
        match Menu::ask_question("Do you wish to create a new databse? [y/n]") {
            Some(answer) => match answer.as_str() {
                "y" => match Menu::ask_question("Provide a new path:") {
                    Some(path) => match Database::create_new_db(path) {
                        Ok(db) => return db,
                        Err(e) => panic!("{:?}", e),
                    },
                    None => {
                        panic!()
                    }
                },
                "n" => match Menu::ask_question("Provide path to database:") {
                    Some(path) => {
                        return Database::new(path);
                    }
                    None => panic!(),
                },
                _ => panic!(),
            },
            None => panic!(),
        }
    }

    fn ask_question_number(question: &str) -> Option<i64> {
        println!("{question}");
        match Menu::get_number_option() {
            Some(i) => return Some(i),
            None => {
                println!("Not a valid option");
                return None;
            }
        }
    }

    fn ask_question(question: &str) -> Option<String> {
        println!("{question}");
        Menu::get_string_input()
    }

    fn get_string_input() -> Option<String> {
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => return Some(input.trim().to_string()),
            Err(_e) => {
                println!("Input invalid.");
                return None;
            }
        }
    }

    fn get_number_option() -> Option<i64> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_s) => match input.trim().parse::<i64>() {
                Ok(i) => Some(i),
                Err(_e) => None,
            },
            Err(_e) => None,
        }
    }
}
