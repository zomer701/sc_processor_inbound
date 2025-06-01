use serde::{Serialize, Deserialize};
use clap::{ValueEnum};
use strum_macros::Display;

#[derive(Display, Debug, Clone, PartialEq, ValueEnum, Serialize, Deserialize)]
pub enum FileFormat {
    #[strum(serialize = "csv")]
    CSV,
    #[strum(serialize = "json")]
    JSON,
    #[strum(serialize = "gzip")]
    GZIP,
    #[strum(serialize = "txt")]
    TXT,
    #[strum(serialize = "xml")]
    XML,
}

#[derive(Display, Clone, Debug, PartialEq, ValueEnum, Serialize, Deserialize)]
pub enum Channel {
    TikTok,
    Facebook
}

#[derive(Display, Clone, Debug, PartialEq, ValueEnum, Serialize, Deserialize)]
pub enum DataType {
    Image,
    Price,
    Product,
    Stock
}