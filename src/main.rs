//
// main.rs
// Copyright (C) 2020 voldikss <dyzplus@gmail.com>
// Distributed under terms of the MIT license.
//

#![warn(unused_imports)]
use env_logger;
use log::{info, warn};
use reqwest::Url;
use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;
use std::env;

struct Translation {
    text: String,
    paraphrase: Option<String>,
    phonetic: Option<String>,
    explains: Vec<String>,
}

trait Translate {
    fn translate(&self, text: String) -> Translation;
}

struct YoudaoTranslator {
    name: String,
}
impl YoudaoTranslator {
    fn new() -> YoudaoTranslator {
        YoudaoTranslator {
            name: String::from("youdao"),
        }
    }
}
impl Translate for YoudaoTranslator {
    fn translate(&self, text: String) -> Translation {
        let mut explains = Vec::new();
        explains.push(String::from("explain1"));
        explains.push(String::from("explain2"));
        Translation {
            text,
            paraphrase: Some("释义".to_string()),
            phonetic: Some("[phonetic]".to_string()),
            explains: explains,
        }
    }
}

struct CibaTranslator {
    name: String,
}
impl CibaTranslator {
    fn new() -> CibaTranslator {
        CibaTranslator {
            name: String::from("ciba"),
        }
    }
}
impl Translate for CibaTranslator {
    fn translate(&self, text: String) -> Translation {
        let url = format!(
            "https://fy.iciba.com/ajax.php?a={0}&f={1}&t={2}&w={3}",
            "fy", "en", "zh", "void"
        );
        let body = reqwest::get(&url);
        // println!("{:?}", body);
        let mut explains = Vec::new();
        explains.push(String::from("explain1"));
        explains.push(String::from("explain2"));
        Translation {
            text,
            paraphrase: Some("释义".to_string()),
            phonetic: Some("[phonetic]".to_string()),
            explains: explains,
        }
    }
}

fn getopt(args: &[String]) -> (HashMap<&str, &str>, String) {
    let mut options = HashMap::new();
    let mut text = String::new();
    for arg in args.iter() {
        if arg[..1] == "-".to_string() {
            let res: Vec<&str> = arg.trim_matches('-').split('=').collect();
            options.insert(res[0], res[1]);
        } else {
            text.push_str(arg);
        }
    }
    return (options, text);
}

fn print_help() {
    let msg = "usage: translator.py {--engine=xx} {--from=xx} {--to=xx}";
    println!("{}", msg);
}

fn main() {
    env_logger::init();
    info!("Start!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (options, text) = getopt(&args[1..]);
    if text == "" {
        print_help();
    }

    let youdao = YoudaoTranslator::new();
    let res = youdao.translate(text);
    println!("{:?}", res.text);
}
