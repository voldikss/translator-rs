use async_trait::async_trait;
use reqwest;
use serde_json;
use url;

pub mod ciba;
pub mod youdao;

#[derive(Debug)]
struct Translation {
    text: String,
    engine: String,
    paraphrase: Option<String>,
    phonetic: Option<String>,
    explains: Vec<String>,
}

#[derive(Debug)]
pub enum TransError {
    HttpRequest(reqwest::Error),
    JsonParse(serde_json::Error),
    UrlParse(url::ParseError),
}

impl From<reqwest::Error> for TransError {
    fn from(err: reqwest::Error) -> TransError {
        TransError::HttpRequest(err)
    }
}

impl From<serde_json::Error> for TransError {
    fn from(err: serde_json::Error) -> TransError {
        TransError::JsonParse(err)
    }
}

impl From<url::ParseError> for TransError {
    fn from(err: url::ParseError) -> TransError {
        TransError::UrlParse(err)
    }
}

type TransResult<Translation> = Result<Translation, TransError>;

#[async_trait]
trait Translate {
    async fn translate(&self, text: &str, sl: &str, tl: &str) -> TransResult<Translation>;
    // fn engine(&self) -> String;
    // fn paraphrase(&self, res: &T) -> Option<String>;
    // fn phonetic(&self, res: &T) -> Option<String>;
    // fn explains(&self, res: &T) -> Vec<String>;
}

use ciba::CibaTranslator;
use youdao::YoudaoTranslator;

struct Translator;
impl Translator {
    fn from(text: &str) -> Option<Box<dyn Translate>> {
        match text {
            "youdao" => Some(Box::new(YoudaoTranslator::new())),
            "ciba" => Some(Box::new(CibaTranslator::new())),
            _ => None,
        }
    }
}

pub async fn translate(engine: &str, text: &str, sl: &str, tl: &str) {
    let translator = Translator::from(engine).unwrap();
    let translation = translator.translate(text, sl, tl).await.unwrap();
    echohl(&translation);
}

fn echohl(translation: &Translation) {
    // TODO: echo with highlight
    println!("{:#?}", translation);
}

#[cfg(test)]
mod tests {
    // TODO
}
