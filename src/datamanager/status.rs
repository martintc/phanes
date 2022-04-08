use sqlite;

use crate::datamanager::db::Database;

pub fn get_status_name(db: &Database, id: i64) -> sqlite::Result<String, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt= connection
        .prepare("select NAME from status where ID=?")?;
    stmt.bind(1, id)?;
    stmt.next()?;
    Ok(stmt.read::<String>(0))?
}

pub fn get_status_id(db: &Database, name: String) -> sqlite::Result<i64, > {
    let connection = sqlite::open(db.get_path())?;
    let mut stmt = connection
        .prepare("select ID from status where ID=?")?;
    stmt.bind(1, name.as_str())?;
    stmt.next()?;
    Ok(stmt.read::<i64>(0))?
}
