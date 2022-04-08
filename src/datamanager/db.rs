pub struct Database {
    path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        Database {
            path: path,
        }
    }

    pub fn get_path(&self) -> &str {
        self.path.as_ref()
    }
}
