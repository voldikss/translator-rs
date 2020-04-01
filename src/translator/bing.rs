use super::TransResult;
use super::Translate;
use super::Translation;

use async_trait::async_trait;
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, USER_AGENT};
use reqwest::{Client, Url};

pub struct BingTranslator {
    client: Client,
    url: Url,
}

impl BingTranslator {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 6.2; rv:51.0) Gecko/20100101 Firefox/51.0"
                .parse()
                .unwrap(),
        );
        headers.insert(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());
        let client = Client::builder().default_headers(headers).build().unwrap();
        let url = Url::parse("http://cn.bing.com/dict/SerpHoverTrans").unwrap();
        BingTranslator { client, url }
    }

    fn engine(&self) -> String {
        String::from("bing")
    }

    fn paraphrase(&self, _resp: &str) -> Option<String> {
        None
    }

    fn phonetic(&self, html: &str) -> Option<String> {
        let re = Regex::new(r#"<span class="ht_attr" lang=".*?">\[(.*?)] </span>"#).unwrap();
        re.captures(html)
            .unwrap()
            .get(1)
            .map_or(None, |m| Some(m.as_str().to_string()))
    }

    fn explains(&self, html: &str) -> Vec<String> {
        let mut explains: Vec<String> = vec![];
        let re =
            Regex::new(r#"<span class="ht_pos">(.*?)</span><span class="ht_trs">(.*?)</span>"#)
                .unwrap();
        for cap in re.captures_iter(html) {
            explains.push(format!("{}{}", &cap[1], &cap[2]));
        }
        explains
    }
}

#[async_trait]
impl Translate for BingTranslator {
    async fn translate(&self, text: &str, _sl: &str, _tl: &str) -> TransResult<Translation> {
        let url = format!("{}?q={}", self.url, text);
        let html = self.client.get(&url).send().await?.text().await?;
        let translation = Translation {
            text: text.to_string(),
            engine: self.engine(),
            paraphrase: self.paraphrase(&html),
            phonetic: self.phonetic(&html),
            explains: self.explains(&html),
        };
        Ok(translation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate_bing() {
        let translator = BingTranslator::new();
        assert_eq!(
            r#"Translation { text: "good", engine: "bing", paraphrase: None, phonetic: Some("ɡʊd"), explains: ["adv.好", "n.好处；好人；益处；善行", "adj.有好处；好的；优质的；符合标准的"] }"#,
            format!(
                "{:?}",
                translator.translate("good", "en", "zh").await.unwrap()
            )
        )
    }
}
