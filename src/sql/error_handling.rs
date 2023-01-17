use sqlx::sqlite::SqliteQueryResult;

#[derive(Debug)]
enum SqliteError {
    ItemAlreadyExists,
    Unknown(String),
}
pub enum SqlResult {
    QuerySuccess(SqliteQueryResult),
    AcceptableError(String),
    UnacceptableError(String),
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

pub fn sqlite_error_handler(
    query_result: Result<SqliteQueryResult, sqlx::Error>,
) -> Result<SqlResult, sqlx::Error> {
    match query_result {
        Ok(res) => Ok(SqlResult::QuerySuccess(res)),
        Err(err) => match SqliteError::from(err) {
            SqliteError::ItemAlreadyExists => Ok(SqlResult::AcceptableError(format!(
                "Duplication: Item already exists"
            ))),
            SqliteError::Unknown(code) => Err(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error code: {}", code),
            ))),
        },
    }
}
