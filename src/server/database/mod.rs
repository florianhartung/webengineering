use std::error::Error as StdError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

pub mod user;
pub mod schema;

#[derive(Debug)]
pub enum DatabaseError {
    UniqueKeyViolation,
    Other(Box<dyn StdError>)
}

impl From<DieselError> for DatabaseError {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => Self::UniqueKeyViolation,
            err => Self::Other(Box::new(err))
        }
    }
}