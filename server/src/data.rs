use crate::error::ServerError;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, NO_PARAMS};
use shared::{Account, DbVersion};
use shared::Operation;
use std::path::Path;
use std::fs;
use std::fs::DirEntry;

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

    pub fn current_db_version(&self) -> Result<DbVersion, ServerError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT VERSION, DEPLOY_AT FROM DB_VERSION ORDER BY VERSION DESC")?;
        let db_version = stmt.query_row(NO_PARAMS, |row| {
            Ok(DbVersion {
                version: row.get(0)?,
                deploy_at: row.get(1)?,
            })
        })?;

        return Ok(db_version);
    }

    pub fn execute(&self, sql: &str) -> Result<(), ServerError> {
        let conn = self.pool.get()?;
        conn.execute_batch(sql)?;

        return Ok(());
    }

    pub fn migrate<P: AsRef<Path>>(&self, path: P) -> Result<(), ServerError> {
        let mut migrations: Vec<DirEntry> = fs::read_dir(path)?
            .map(|dir| dir.unwrap())
            .collect();

        migrations.sort_by_key(|dir| dir.file_name());

        let current_version = match self.current_db_version() {
            Ok(version) => Some(version),
            Err(_) => {
                info!("No database version found. Initializing...");
                None
            }
        };

        for migration in migrations {
            let file_string = migration.file_name().into_string()?;
            let file_name = file_string.split('.').nth(0);
            if match (file_name, &current_version) {
                (Some(file_name), Some(version)) => file_name > &version.version[..],
                (Some(_), None) => true,
                (None, _) => false,
            } {
                info!("Applying database migration {:?}.", migration.file_name());
                let sql = fs::read_to_string(migration.path())?;
                self.execute(&sql)?;
            }
        }

        info!("The database is now the latest version.");

        Ok(())
    }
}
