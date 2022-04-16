use sqlite::{State};
use std::vec::Vec;

use crate::datamanager::db::Database;

pub struct Category {
    id: i64,
    name: String,
}

impl Category {
    pub fn print_category(&self) {
        println!("ID: {}", self.id);
        println!("\tName: {}", self.id);
    }
}

pub fn add_category(db: &Database, name: String) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("insert into category(NAME) values(?);")?;
    stmt.bind(1, name.as_str())?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn get_category_id(db: &Database, name: String) -> sqlite::Result<i64, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID from categoery where NAME=(?);")?;
    stmt.bind(1, name.as_str())?;
    stmt.next()?;
    Ok(stmt.read::<i64>(0))?
}

pub fn get_category_name(db: &Database, id: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select NAME from category where ID=?")?;
    stmt.bind(1, id)?;
    stmt
        .next()?;
    Ok(stmt.read::<String>(0))?
}

pub fn remove_category(db: &Database, id: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("delete from category where ID=?")?;
    stmt.bind(1, id)?;
    stmt.next()?;
    Ok(String::from("Success"))
}

pub fn get_all_categories(db: &Database) -> sqlite::Result<Vec<Category>, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID, NAME from category;")?;
    let mut results: Vec<Category> = Vec::new();
    while let State::Row = stmt.next()? {
        // let id: i64 = stmt.read::<i64>(0)?;
        // let s: String = format!("{}. {}", id.to_string().as_str(), stmt.read::<String>(1)?);
        // results.push(s);
        let cat: Category = Category {
            id: stmt.read::<i64>(0)?,
            name: stmt.read::<String>(1)?,
        };
        results.push(cat);
    }
    Ok(results)
}
