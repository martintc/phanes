#[derive(Clone)]
pub struct Database {
    pub path: String,
}

impl Database {
    pub fn create_new_db(path: String) -> sqlite::Result<Database> {
        let db = Database { path: path };
        let connection = sqlite::open(&db.path)?;
        let mut stmt = connection
            .prepare("CREATE TABLE tasks(ID INTEGER PRIMARY KEY, TITLE TEXT, DESCRIPTION TEXT, STATUS INTEGER, CATEGORY INTEGER);")?;
        stmt.next()?;
        stmt = connection
            .prepare("CREATE TABLE category(ID INTEGER PRIMARY KEY, NAME TEXT UNIQUE);")?;
        stmt.next()?;
        stmt = connection.prepare("CREATE TABLE status(ID INTEGER KEY, STATUS TEXT);")?;
        stmt.next()?;
        stmt = connection.prepare("insert into status(ID, STATUS) values(1, 'open')")?;
        stmt.next()?;
        stmt = connection.prepare("insert into status(ID, STATUS) values(2, 'in-progress')")?;
        stmt.next()?;
        stmt = connection.prepare("insert into status(ID, STATUS) values(3, 'closed')")?;
        stmt.next()?;
        stmt = connection.prepare("insert into category(NAME) values('none')")?;
        stmt.next()?;
        Ok(db)
    }

    pub fn new(path: String) -> Self {
        Database { path: path }
    }

    pub fn get_path(&self) -> &str {
        self.path.as_ref()
    }
}
