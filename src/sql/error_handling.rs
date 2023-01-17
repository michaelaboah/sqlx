use sqlx::sqlite::SqliteQueryResult;

#[derive(Debug)]
enum SqliteErrorKind {
    ItemAlreadyExists,
    ConstraintViolation,
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum SqlResult {
    QuerySuccess(SqliteQueryResult),
    AcceptableError(String),
    UnacceptableError(String),
}

#[derive(Debug)]
pub struct SqliteCustomError {
    pub code: u16,
    pub message: String,
    error_kind: SqliteErrorKind,
}

impl std::convert::From<sqlx::Error> for SqliteCustomError {
    fn from(err: sqlx::Error) -> Self {
        let db_err = err.into_database_error().unwrap();
        let cow_str = db_err.code().unwrap();
        let code = cow_str.as_ref();
        match code {
            "1555" => SqliteCustomError {
                code: code.parse::<u16>().unwrap_or(1555),
                message: format!("This item already exists in the database."),
                error_kind: SqliteErrorKind::ItemAlreadyExists,
            },
            "19" => SqliteCustomError {
                code: code.parse::<u16>().unwrap_or(19),
                message: format!("A constraint violation has occured."),
                error_kind: SqliteErrorKind::ConstraintViolation,
            },
            "1" => SqliteCustomError {
                code: code.parse::<u16>().unwrap_or(1),
                message: format!("This item was not found in the database."),
                error_kind: SqliteErrorKind::NotFound,
            },
            // other matches for other error codes
            c => SqliteCustomError {
                code: code.parse::<u16>().unwrap(),
                message: format!("Unknown, should better implement the err code"),
                error_kind: SqliteErrorKind::Unknown(c.to_string()),
            },
        }
    }
}

pub fn sqlite_error_handler(
    query_result: Result<SqliteQueryResult, sqlx::Error>,
) -> Result<SqlResult, SqliteCustomError> {
    match query_result {
        Ok(res) => Ok(SqlResult::QuerySuccess(res)),
        Err(err) => match SqliteCustomError::from(err) {
            e if e.code == 1555 => Ok(SqlResult::AcceptableError(e.message)),
            e if e.code == 19 => Ok(SqlResult::UnacceptableError(e.message)),
            e if e.code == 1 => Ok(SqlResult::AcceptableError(e.message)),
            e => Err(e),
        },
    }
}
