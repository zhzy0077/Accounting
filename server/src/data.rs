use crate::error::ServerError;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, NO_PARAMS};
use shared::Account;
use shared::Operation;
use std::path::Path;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager).unwrap();

        Database { pool }
    }

    pub fn insert_operation(&self, operation: &Operation) -> Result<(), ServerError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("INSERT INTO OPERATION(FROM_ACCOUNT, TO_ACCOUNT, COMMENT, AMOUNT, DATETIME) VALUES (?, ?, ?, ?, ?);")?;
        let rows_updated = stmt.execute(params![
            operation.from,
            operation.to,
            operation.comment,
            operation.amount,
            operation.datetime
        ])?;

        if rows_updated != 1 {
            return Err(ServerError::DBError(
                "Wrong number of lines inserted.".to_owned(),
            ));
        }

        return Ok(());
    }

    pub fn update_balance(&self, account_name: &str, delta: i64) -> Result<(), ServerError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("UPDATE ACCOUNT SET BALANCE = BALANCE + ? WHERE NAME = ?;")?;
        let rows_updated = stmt.execute(params![delta, account_name])?;

        if rows_updated != 1 {
            return Err(ServerError::DBError(
                "Wrong number of lines updated.".to_owned(),
            ));
        }

        return Ok(());
    }

    pub fn get_secret(&self) -> Result<String, ServerError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT TOKEN FROM SECRET LIMIT 1")?;
        let secret = stmt.query_row(NO_PARAMS, |row| Ok(row.get(0)?))?;

        return Ok(secret);
    }

    pub fn get_accounts(&self) -> Result<Vec<Account>, ServerError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT NAME, BALANCE FROM ACCOUNT")?;
        let accounts = stmt
            .query_map(NO_PARAMS, |row| {
                Ok(Account {
                    name: row.get(0)?,
                    balance: row.get(1)?,
                })
            })
            .and_then(Iterator::collect)?;

        Ok(accounts)
    }

    pub fn add_account(&self, account: &Account) -> Result<(), ServerError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("INSERT INTO ACCOUNT (NAME, BALANCE) VALUES (?, ?)")?;
        let rows_updated = stmt.execute(params![account.name, account.balance])?;

        if rows_updated != 1 {
            Err(ServerError::DBError(
                "Wrong number of lines inserted.".to_owned(),
            ))
        } else {
            Ok(())
        }
    }
}
