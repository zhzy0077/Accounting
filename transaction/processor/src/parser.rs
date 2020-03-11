use crate::error::Error;
use crate::Parser;
use crate::Result;
use chrono::{NaiveDate, NaiveTime};
use csv::{Reader, StringRecord, Trim};
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

pub(crate) struct CmbDebitParser;

impl Parser for CmbDebitParser {
    fn parse(&self, content: String) -> Result<Vec<Transaction>> {
        unimplemented!()
    }
}

pub(crate) struct AlipayParser;

impl Parser for AlipayParser {
    fn parse(&self, content: String) -> Result<Vec<Transaction>> {
        unimplemented!()
    }
}

pub(crate) struct WeChatPayParser;

impl Parser for WeChatPayParser {
    fn parse(&self, content: String) -> Result<Vec<Transaction>> {
        unimplemented!()
    }
}
