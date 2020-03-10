use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub balance: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub from: String,
    pub to: String,
    pub comment: String,
    pub amount: i64,
    pub datetime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub datetime: i64,
    pub from: String,
    pub to: String,
    pub amount: i64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginChallenge {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbVersion {
    pub version: String,
    pub deploy_at: i64,
}
