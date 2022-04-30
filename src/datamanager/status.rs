use sqlite::State;
use std::vec::Vec;

use crate::datamanager::db::Database;

pub struct Status {
    id: i64,
    name: String,
}

impl Status {
    pub fn print(&self) {
        println!("ID: {}", self.id);
        println!("\tName: {}", self.name);
    }
}

pub fn get_status_name(db: &Database, id: i64) -> sqlite::Result<String> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("select STATUS from status where ID=?")?;
    stmt.bind(1, id)?;
    stmt.next()?;
    Ok(stmt.read::<String>(0))?
}

pub fn get_status_id(db: &Database, name: String) -> sqlite::Result<i64> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("select ID from status where ID=?")?;
    stmt.bind(1, name.as_str())?;
    stmt.next()?;
    Ok(stmt.read::<i64>(0))?
}

pub fn get_all_status(db: &Database) -> sqlite::Result<Vec<Status>> {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection.prepare("select ID, STATUS from status")?;
    let mut result: Vec<Status> = Vec::new();
    while let State::Row = stmt.next()? {
        let s = Status {
            id: stmt.read::<i64>(0)?,
            name: stmt.read::<String>(1)?,
        };
        result.push(s);
    }
    Ok(result)
}
