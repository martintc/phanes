use sqlite::State;
use std::vec::Vec;

use crate::datamanager::db::Database;

pub struct Task {
    id: i64,
    title: String,
    description: String,
    status: i64,
    category: i64,
}

impl Task {
    pub fn print(&self) {
        println!("Id: {}", self.id);
        println!("\tTitle: {}", self.title);
        println!("\tDescription: {}", self.description);
        println!("\tStatus: {}", self.status);
        println!("\tCategory: {}", self.category);
    }

    pub fn get_task_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_task_id(&self) -> i64 {
        self.id
    }

    pub fn get_task_desc(&self) -> &str {
        self.description.as_str()
    }

    pub fn get_task_status_number(&self) -> i64 {
        self.status
    }

    pub fn get_task_category_number(&self) -> i64 {
        self.category
    }
}

pub fn add_tasks(
    db: &Database,
    title: String,
    desc: String,
    status: i64,
    category: i64,
) -> sqlite::Result<String> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("insert into tasks(TITLE, DESCRIPTION, STATUS, CATEGORY) values(?, ?, ?, ?);")?;
    stmt.bind(1, title.as_str())?;
    stmt.bind(2, desc.as_str())?;
    stmt.bind(3, status)?;
    stmt.bind(4, category)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn change_task_status(db: &Database, id: i64, status: i64) -> sqlite::Result<String> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("update tasks set STATUS=? where ID=?;")?;
    stmt.bind(1, status)?;
    stmt.bind(2, id)?;
    stmt.next()?;
    Ok(String::from("Successs"))
}

pub fn change_task_category(db: &Database, id: i64, category: i64) -> sqlite::Result<String> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("update tasks set CATEGORY=? where ID=?;")?;
    stmt.bind(1, category)?;
    stmt.bind(2, id)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn remove_task(db: &Database, id: i64) -> sqlite::Result<String> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("delete from tasks where ID=?;")?;
    stmt.bind(1, id)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn get_tasks_by_category(db: &Database, category: i64) -> sqlite::Result<Vec<Task>> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TITLE, DESCRIPTION, STATUS, CATEGORY from tasks where CATEGORY=?;")?;
    stmt.bind(1, category)?;
    let mut results: Vec<Task> = Vec::new();
    while let State::Row = stmt.next()? {
        // let id: i64 = stmt.read::<i64>(0)?;
        // // s.push(id);
        // // s.push(stmt.read::<String>(1)?);
        // let r = format!("{}. {}", id.to_string().as_str(), stmt.read::<String>(1)?);
        let task = Task {
            id: stmt.read::<i64>(0)?,
            title: stmt.read::<String>(1)?,
            description: stmt.read::<String>(2)?,
            status: stmt.read::<i64>(3)?,
            category: stmt.read::<i64>(4)?,
        };
        results.push(task);
    }
    Ok(results)
}

pub fn get_task_by_status(db: &Database, status: i64) -> sqlite::Result<Vec<Task>> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TITLE, DESCRIPTION, STATUS, CATEGORY from tasks where STATUS=?;")?;
    stmt.bind(1, status)?;
    let mut results: Vec<Task> = Vec::new();
    while let State::Row = stmt.next()? {
        let task = Task {
            id: stmt.read::<i64>(0)?,
            title: stmt.read::<String>(1)?,
            description: stmt.read::<String>(2)?,
            status: stmt.read::<i64>(3)?,
            category: stmt.read::<i64>(4)?,
        };
        results.push(task);
    }
    Ok(results)
}

pub fn get_task_title(db: &Database, title: String) -> sqlite::Result<Vec<Task>> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TITLE, DESCRIPTION, STATUS, CATEGORY from tasks where Title=?;")?;
    stmt.bind(1, title.as_str())?;
    let mut results: Vec<Task> = Vec::new();
    while let State::Row = stmt.next()? {
        let task = Task {
            id: stmt.read::<i64>(0)?,
            title: stmt.read::<String>(1)?,
            description: stmt.read::<String>(2)?,
            status: stmt.read::<i64>(3)?,
            category: stmt.read::<i64>(4)?,
        };

        results.push(task);
    }
    Ok(results)
}

pub fn get_task_list(db: &Database) -> sqlite::Result<Vec<Task>> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("select ID, TITLE, DESCRIPTION, STATUS, CATEGORY from tasks;")?;
    let mut results: Vec<Task> = Vec::new();
    while let State::Row = stmt.next()? {
        let task = Task {
            id: stmt.read::<i64>(0)?,
            title: stmt.read::<String>(1)?,
            description: stmt.read::<String>(2)?,
            status: stmt.read::<i64>(3)?,
            category: stmt.read::<i64>(4)?,
        };
        results.push(task);
    }
    Ok(results)
}

pub fn get_task_by_id(db: &Database, id: i64) -> sqlite::Result<Task> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TITLE, DESCRIPTION, STATUS, CATEGORY from tasks where tasks.ID=?")?;
    stmt.bind(1, id)?;
    stmt.next()?;
    let task: Task = Task {
        id: stmt.read::<i64>(0)?,
        title: stmt.read::<String>(1)?,
        description: stmt.read::<String>(2)?,
        status: stmt.read::<i64>(3)?,
        category: stmt.read::<i64>(4)?,
    };
    Ok(task)
}
