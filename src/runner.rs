use std::fs;
use std::io::Write;
use std::process::Command;
use std::time::*;
use std::{error::Error, process::Stdio};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Encoding {
    BIG5,
    EUC_CN,
    EUC_JIS_2004,
    EUC_KR,
    EUC_TW,
    GB18030,
    GBK,
    ISO_8859_5,
    ISO_8859_6,
    ISO_8859_7,
    ISO_8859_8,
    JOHAB,
    KOI8R,
    KOI8U,
    LATIN1,
    LATIN2,
    LATIN3,
    LATIN4,
    LATIN5,
    LATIN6,
    LATIN7,
    LATIN8,
    LATIN9,
    LATIN10,
    MULE_INTERNAL,
    SJIS,
    SHIFT_JIS_2004,
    SQL_ASCII,
    UHC,
    UTF8,
    WIN866,
    WIN874,
    WIN1250,
    WIN1251,
    WIN1252,
    WIN1253,
    WIN1254,
    WIN1255,
    WIN1256,
    WIN1257,
    WIN1258,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    host: String,
    pgUser: String,
    pgPass: String,
    pgPort: i32,
    pgDBName: String,
    encoding: Encoding,
}

pub fn read_settings() -> Result<Settings, Box<dyn Error>> {
    let json = fs::read_to_string(r"D:\collage\rust\backup\settings.json")?;
    // type could be any type that implement deserialize
    let settings: Settings = serde_json::from_str(&json)?;
    return Ok(settings);
}

pub fn run(settings: Settings) -> Result<(), Box<dyn Error>> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let cmdSTDOUT = Command::new("echo")
        .args([format!("{:?}\n", now)])
        .output()?
        .stdout;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(r"D:\collage\rust\backup\logger.txt")?;

    file.write(&cmdSTDOUT)?;
    return Ok(());
}
