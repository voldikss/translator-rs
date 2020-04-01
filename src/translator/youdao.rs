use super::TransError; //TODO
use super::Translate;
use super::Translation;

use async_trait::async_trait;
use md5;
use reqwest::header::HeaderMap;
use reqwest::header::{COOKIE, REFERER, USER_AGENT};
use reqwest::{Client, Url};
use serde::Deserialize;
use serde_json;
use std::time::SystemTime;

pub struct YoudaoTranslator {
    client: Client,
    url: Url,
}

#[derive(Deserialize, Clone, Debug)]
struct YoudaoBasicResponse {
    tgt: String,
    src: String,
}

#[derive(Deserialize, Clone, Debug)]
struct YoudaoSmartResponse {
    entries: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct YoudaoReponse {
    translateResult: Vec<Vec<YoudaoBasicResponse>>,
    errorCode: i32,
    r#type: String,
    smartResult: YoudaoSmartResponse,
}

impl YoudaoTranslator {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 6.2; rv:51.0) Gecko/20100101 Firefox/51.0"
                .parse()
                .unwrap(),
        );
        headers.insert(REFERER, "http://fanyi.youdao.com/".parse().unwrap());
        headers.insert(
            COOKIE,
            "OUTFOX_SEARCH_USER_ID=-2022895048@10.168.8.76;"
                .parse()
                .unwrap(),
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        let url = Url::parse("https://fanyi.youdao.com/translate_o").unwrap();
        YoudaoTranslator { client, url }
    }

    fn sign(&self, text: &str, salt: &str) -> String {
        let token = "97_3(jkMYg@T[KZQmqjTK";
        let sign_raw = md5::compute("fanyideskweb".to_string() + text + salt + token);
        format!("{:x}", sign_raw)
    }

    fn salt(&self) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
            .floor();
        now.to_string()
    }

    fn engine(&self) -> String {
        String::from("youdao")
    }

    fn paraphrase(&self, resp: &YoudaoReponse) -> Option<String> {
        // TODO: None?
        Some(resp.translateResult[0][0].tgt.clone())
    }

    fn phonetic(&self, _resp: &YoudaoReponse) -> Option<String> {
        None
    }

    fn explains(&self, resp: &YoudaoReponse) -> Vec<String> {
        resp.smartResult.entries.clone()
    }
}

#[async_trait]
impl Translate for YoudaoTranslator {
    async fn translate(&self, text: &str, sl: &str, tl: &str) -> Result<Translation, TransError> {
        let salt = self.salt();
        let sign = self.sign(text, salt.as_str());
        let params: Vec<(&str, &str)> = vec![
            ("i", text),
            ("from", sl),
            ("to", tl),
            ("smartresult", "dict"),
            ("client", "fanyideskweb"),
            ("doctype", "json"),
            ("version", "2.1"),
            ("keyfrom", "fanyi.web"),
            ("action", "FY_BY_CL1CKBUTTON"),
            ("typoResult", "true"),
            ("salt", salt.as_str()),
            ("sign", sign.as_str()),
        ];
        let res = self
            .client
            .post(self.url.as_str())
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        let resp: YoudaoReponse = serde_json::from_str(&res)?;
        let translation = Translation {
            text: text.to_string(),
            engine: self.engine(),
            paraphrase: self.paraphrase(&resp),
            phonetic: self.phonetic(&resp),
            explains: self.explains(&resp),
        };
        Ok(translation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate_youdao() {
        let translator = YoudaoTranslator::new();
        assert_eq!(
            r#"Translation { text: "good morning", engine: "youdao", paraphrase: Some("早上好"), phonetic: None, explains: ["", "int. 早安，早上好\r\n"] }"#,
            format!(
                "{:?}",
                translator
                    .translate("good morning", "en", "zh")
                    .await
                    .unwrap()
            )
        )
    }
}
