use sqlite::{State};
use std::vec::Vec;

use crate::datamanager::db::Database;

pub fn add_tasks(db: &Database, title: String, desc: String, status: i64, category: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("insert into tasks(TITLE, DESCRIPTION, STATUS, CATEGORY) values(?, ?, ?, ?);")?;
    stmt.bind(1, title.as_str())?;
    stmt.bind(2, desc.as_str())?;
    stmt.bind(3, status)?;
    stmt.bind(4, category)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn change_task_status(db: &Database, id: i64, status: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("update tasks set STATUS=? where ID=?;")?;
    stmt.bind(1, status)?;
    stmt.bind(2, id)?;
    stmt.next()?;
    Ok(String::from("Successs"))
}

pub fn change_task_category(db: &Database, id: i64, category: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("update tasks set CATEGORY=? where ID=?;")?;
    stmt.bind(1, category)?;
    stmt.bind(2, id)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn remove_task(db: &Database, id: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("delete from tasks where ID=?;")?;
    stmt.bind(1, id)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn get_tasks_by_category(db: &Database, category: i64) -> sqlite::Result<Vec<String>, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TASK from tasks where CATEGORY=?;")?;
    stmt.bind(1, category)?;
    let mut results: Vec<String> = Vec::new();
    while let State::Row = stmt.next()? {
        let mut s = String::new();
        let id: i64 = stmt.read::<i64>(0)?;
        // s.push(id);
        // s.push(stmt.read::<String>(1)?);
        let r = format!("{}. {}", id.to_string().as_str(), stmt.read::<String>(1)?);
        results.push(r);
    }
    Ok(results)
}

pub fn get_task_by_status(db: &Database, status: i64) -> sqlite::Result<Vec<String>, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TITLE from tasks where STATUS=?;")?;
    stmt.bind(1, status)?;
    let mut results: Vec<String> = Vec::new();
    while let State::Row = stmt.next()? {
        let mut s = String::new();
        let id: i64 = stmt.read::<i64>(0)?;
        // s.push(id);
        // s.push(stmt.read::<String>(1)?);
        let r = format!("{}. {}", id.to_string().as_str(), stmt.read::<String>(1)?);
        results.push(r);
    }
    Ok(results)
}

pub fn get_task_title(db: &Database, title: String) -> sqlite::Result<Vec<String>, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TASK from tasks where Title=?;")?;
    stmt.bind(1, title.as_str())?;
    let mut results: Vec<String> = Vec::new();
    while let State::Row = stmt.next()? {
        let mut s = String::new();
        let id: i64 = stmt.read::<i64>(0)?;
        // s.push(id);
        // s.push(stmt.read::<String>(1)?);
        let r = format!("{}. {}", id.to_string().as_str(), stmt.read::<String>(1)?);
        results.push(r);
    }
    Ok(results)
}

pub fn get_task_list(db: &Database) -> sqlite::Result<Vec<String>, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, TASK from tasks;")?;
    let mut results: Vec<String> = Vec::new();
    while let State::Row = stmt.next()? {
        let mut s = String::new();
        let id: i64 = stmt.read::<i64>(0)?;
        // s.push(id);
        // s.push(stmt.read::<String>(1)?);
        let r = format!("{}. {}", id.to_string().as_str(), stmt.read::<String>(1)?);
        results.push(r);
    }
    Ok(results)
}
