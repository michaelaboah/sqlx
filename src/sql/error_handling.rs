#[derive(Debug)]
enum SqliteError {
    ItemAlreadyExists,
    Unknown(String),
}

impl std::convert::From<sqlx::Error> for SqliteError {
    fn from(err: sqlx::Error) -> Self {
        let code = err
            .into_database_error()
            .unwrap()
            .code()
            .unwrap()
            .into_owned();
        match code.as_str() {
            "1555" => SqliteError::ItemAlreadyExists,
            // other matches for other error codes
            c => SqliteError::Unknown(c.to_string()),
        }
    }
}

pub fn sqlite_error_handler(err: sqlx::Error) {
    match SqliteError::from(err) {
        SqliteError::ItemAlreadyExists => println!("Item already exists"),
        SqliteError::Unknown(code) => println!("Unknown error code: {}", code),
    }
}
