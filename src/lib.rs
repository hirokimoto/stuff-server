extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

mod tray;
mod event;
mod ocr;
mod screen;
mod zip;
mod p2p_chat;
mod screens;
mod network;

pub use screen::{capture_proposal, capture_screen};
pub use event::callback;
pub use tray::build_tray;
pub use zip::{read_zip, zip_proposal, zip_report, zip_screenshot, zip_text};
pub use ocr::read_screenshot;
pub use p2p_chat::p2p_chat;
pub use screens::report::build_report;
pub use screens::daily_report::build_daily;

use chrono::{Utc, DateTime};
use once_cell::sync::Lazy;
use preferences::{AppInfo, PreferencesMap, Preferences};
use regex::RegexBuilder;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static APP_INFO: AppInfo = AppInfo{name: "monitor", author: "Hiroki Moto"};
static PREFES_KEY: &str = "info/docs/monitor";

pub static DOCUMENTS: &[u8] = b"D:\\_documents/";
pub static PASS: &[u8] = b"firemouses!";

pub static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static LOGGED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static REPORT: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub type AppResult<T = ()> = std::result::Result<T, std::io::Error>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Events {
    ClickTrayIcon,
    DoubleClickTrayIcon,
    Exit,
    Item1,
    Item2,
    Item3,
}

pub fn init_folders() {
    let mut path = PathBuf::from("D:\\");
    path.push("_documents");
    if !path.exists() {
        match fs::create_dir("D:\\_documents") {
            Ok(..) => {
                match fs::create_dir("D:\\_documents/logs") {
                    Ok(..) => (),
                    Err(..) => {
                        print!("failed to create documents/logs folders.");
                    }
                };
                match fs::create_dir("D:\\_documents/screens") {
                    Ok(..) => (),
                    Err(..) => {
                        print!("failed to create documents/screens folders.");
                    }
                };
                match fs::create_dir("D:\\_documents/proposals") {
                    Ok(..) => (),
                    Err(..) => {
                        print!("failed to create documents/proposals folders.");
                    }
                };
                match fs::create_dir("D:\\_documents/reports") {
                    Ok(..) => (),
                    Err(..) => {
                        print!("failed to create documents/reports folders.");
                    }
                };
            },
            Err(..) => {
                print!("failed to create documents folders.");
                std::process::exit(0);
            }
        };
    }
}

pub fn init_status() -> String {
    let mut logs = String::new();

    let now: DateTime<Utc> = Utc::now();
    let fname = now.format("%Y-%m-%d").to_string();
    logs = read_zip(&fname, "log.txt");

    let load_result = PreferencesMap::<String>::load(&APP_INFO, PREFES_KEY);
    match load_result {
        Ok(prefs) => {
            println!("{:?}", prefs.get("boot".into()).unwrap());
            let info: String = format!("@app ended at: {} \n", prefs.get("boot".into()).unwrap());
            logs += &info;
        },
        Err(..) => {
            println!("failed to read preferences.");
        }
    };


    let now = Utc::now();
    let x: String = format!("{}", now);
    let now_parsed: DateTime<Utc> = x.parse().unwrap();
    let info: String = format!("@app started at: {} \n", now_parsed.to_string());
    logs += &info;

    match zip_text(logs.clone()) {
        Ok(_) => {
            println!("monitor has saved machine status.");
        },
        Err(e) => println!("failed to save machine status: {e:?}"),
    };

    logs
}

pub fn is_messengers(text: String) -> bool {
    let re =
        RegexBuilder::new(
            r"skype|discord|telegram|signal|slack|line|whatsapp|wechat|snapchat
            |zoom|hangouts|google meet|google chat
        ")
        .case_insensitive(true)
        .build().unwrap();

    let ok = re.is_match(&text);

    ok
}

pub fn is_money(text: String) -> bool {
    let re =
        RegexBuilder::new(r"payoneer|paypal|exodus|metamask|payment|money")
        .case_insensitive(true)
        .build().unwrap();

    let ok = re.is_match(&text);

    ok
}