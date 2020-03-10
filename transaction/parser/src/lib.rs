use std::io;
use std::fmt;
use std::path::Path;
use entities::Transaction;
use std::fs::{File, OpenOptions};
use crate::error::Error;
use std::io::Read;
use encoding::{label, DecoderTrap};
use crate::parser::{CgbCreditParser, CmbDebitParser, AlipayParser, WeChatPayParser};
use crate::TransactionSource::CmbDebit;

pub mod error;
mod parser;

pub type Result<T> = std::result::Result<T, Error>;

const ENCODING_DETECT_CONFIDENCE_THRESHOLD: f32 = 0.2;

trait Parser {
    fn parse(&self, content: String) -> Result<Vec<Transaction>>;
}

pub enum TransactionSource {
    CgbCredit,
    CmbDebit,
    Alipay,
    WeChatPay,
}

pub struct ParserConfig {
    source: TransactionSource,
    encoding: Option<String>,

}

pub fn parse<P: AsRef<Path>>(path: P, config: ParserConfig) -> Result<Vec<Transaction>> {
    let mut fd = OpenOptions::new().read(true).open(path)?;
    let mut byte_content = Vec::new();

    fd.read_to_end(&mut byte_content)?;

    let encoding_label = config.encoding.ok_or(Error::UnknownEncoding)
        .or_else((|_| { detect_encoding(&byte_content) }))?;

    let coder = label::encoding_from_whatwg_label(&encoding_label).ok_or(Error::UnknownEncoding)?;

    let content = coder.decode(&byte_content, DecoderTrap::Replace)?;

    get_parser(config.source).parse(content)
}

fn detect_encoding(content: &Vec<u8>) -> Result<String> {
    let (charset, confidence, _) = chardet::detect(content);

    if confidence < ENCODING_DETECT_CONFIDENCE_THRESHOLD {
        Err(Error::Encoding { best_match: charset })
    } else {
        let encoding = chardet::charset2encoding(&charset);
        Ok(encoding.to_owned())
    }
}

fn get_parser(source: TransactionSource) -> Box<dyn Parser> {
    match source {
        TransactionSource::CgbCredit => Box::new(CgbCreditParser),
        TransactionSource::CmbDebit => Box::new(CmbDebitParser),
        TransactionSource::Alipay => Box::new(AlipayParser),
        TransactionSource::WeChatPay => Box::new(WeChatPayParser),
    }
}