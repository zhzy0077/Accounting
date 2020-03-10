use crate::Parser;
use entities::Transaction;
use crate::error::Error;
use crate::Result;

pub(crate) struct CgbCreditParser;

impl Parser for CgbCreditParser {
    fn parse(&self, content: String) -> Result<Vec<Transaction>> {
        unimplemented!()
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
