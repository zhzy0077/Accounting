use crate::Parser;
use crate::Result;
use chrono::{NaiveDate, NaiveDateTime};
use csv::{StringRecord, Trim};
use entities::Transaction;

pub trait CsvParser {
    fn map_row(&self, record: StringRecord) -> Option<Transaction>;
}

impl<T: CsvParser> Parser for T {
    fn parse(&self, content: String) -> Result<Vec<Transaction>> {
        let mut transactions = Vec::new();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .trim(Trim::All)
            .from_reader(content.as_bytes());
        for result in reader.records() {
            let record = result?;
            if let Some(transaction) = self.map_row(record) {
                transactions.push(transaction);
            }
        }
        Ok(transactions)
    }
}

pub(crate) struct CgbCreditParser {
    pub account_name: String,
}

pub(crate) struct AlipayParser {
    pub account_name: String,
}

pub(crate) struct WeChatPayParser {
    pub account_name: String,
}

pub(crate) struct CmbDebitParser {
    pub account_name: String,
}

impl CsvParser for CgbCreditParser {
    fn map_row(&self, record: StringRecord) -> Option<Transaction> {
        let date = NaiveDate::parse_from_str(record.get(0)?, "%Y-%m-%d").ok()?;
        let description = record.get(1)?;
        let amount = record.get(3)?.parse::<f64>().ok()?;

        let timestamp = date.and_hms(0, 0, 0).timestamp();

        if amount > 0.0 {
            // Income
            Some(Transaction {
                datetime: timestamp,
                from: "".to_owned(),
                to: self.account_name.clone(),
                amount: (amount * 100.0) as i64,
                description: description.to_string(),
            })
        } else {
            // Expense
            Some(Transaction {
                datetime: timestamp,
                from: self.account_name.clone(),
                to: "".to_owned(),
                amount: (amount * 100.0) as i64,
                description: description.to_string(),
            })
        }
    }
}

impl CsvParser for AlipayParser {
    fn map_row(&self, record: StringRecord) -> Option<Transaction> {
        let datetime = NaiveDateTime::parse_from_str(record.get(2)?, "%Y-%m-%d %T").ok()?;
        let target_account = record.get(7)?;
        let description = record.get(8)?;
        let amount = record.get(9)?.parse::<f64>().ok()?;
        let operation_type = record.get(15)?;

        match operation_type {
            "已收入" => Some(Transaction {
                datetime: datetime.timestamp(),
                from: target_account.to_owned(),
                to: self.account_name.clone(),
                amount: (amount * 100.0) as i64,
                description: description.to_string(),
            }),
            "已支出" => Some(Transaction {
                datetime: datetime.timestamp(),
                from: self.account_name.clone(),
                to: target_account.to_owned(),
                amount: (amount * 100.0) as i64,
                description: description.to_string(),
            }),
            _ => None,
        }
    }
}

impl CsvParser for WeChatPayParser {
    fn map_row(&self, record: StringRecord) -> Option<Transaction> {
        let datetime = NaiveDateTime::parse_from_str(record.get(0)?, "%Y-%m-%d %T").ok()?;
        let description = record.get(1)?;
        let target_account = record.get(2)?;
        let operation_type = record.get(4)?;
        // Remove the first ¥
        let amount_str: String = record.get(5)?.chars().skip(1).collect();
        let amount: f64 = amount_str.parse().ok()?;

        match operation_type {
            "收入" => Some(Transaction {
                datetime: datetime.timestamp(),
                from: target_account.to_owned(),
                to: self.account_name.clone(),
                amount: (amount * 100.0) as i64,
                description: description.to_string(),
            }),
            "支出" => Some(Transaction {
                datetime: datetime.timestamp(),
                from: self.account_name.clone(),
                to: target_account.to_owned(),
                amount: (amount * 100.0) as i64,
                description: description.to_string(),
            }),
            _ => None,
        }
    }
}

impl Parser for CmbDebitParser {
    fn parse(&self, _: String) -> Result<Vec<Transaction>> {
        unimplemented!()
    }
}
