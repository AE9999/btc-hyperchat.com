use std::fmt;

#[derive(Clone, Debug)]
pub enum DbEvent {
    DbQueryFailure,
    DbConsistencyFailure,
    DbEntityCreation,
    DbEntityUpdate,
}

impl fmt::Display for DbEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbEvent::DbQueryFailure => write!(f, "DbQueryFailure"),
            DbEvent::DbConsistencyFailure=> write!(f, "DbConsistencyFailure"),
            DbEvent::DbEntityCreation => write!(f, "DbEntityCreation"),
            DbEvent::DbEntityUpdate => write!(f, "DbEntityUpdate"),
        }
    }
}

