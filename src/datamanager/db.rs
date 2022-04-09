pub struct Database {
    path: String,
}

impl Database {
    pub fn create_new_db(path: String) -> sqlite::Result<String, > {
        let connection = sqlite::open(path)?;
        let mut stmt = connection
            .prepare("CREATE TABLE tasks(ID INTEGER PRIMARY KEY, TITLE TEXT, DESCRIPTION TEXT, STATUS INTEGER, CATEGORY INTEGER);")?;
        stmt.next()?;
        stmt = connection
            .prepare("CREATE TABLE category(ID INTEGER PRIMARY KEY, NAME TEXT UNIQUE);")?;
        stmt.next()?;
        stmt = connection
            .prepare("CREATE TABLE status(ID INTEGER KEY, STATUS TEXT);")?;
        stmt.next()?;
        Ok(String::from("Database  created"))
    }

    pub fn new(path: String) -> Self {
        Database {
            path: path,
        }
    }

    pub fn get_path(&self) -> &str {
        self.path.as_ref()
    }
}
