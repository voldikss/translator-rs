use super::TransResult;
use super::Translate;
use super::Translation;

use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::Deserialize;
use serde_json;

pub struct CibaTranslator {
    client: Client,
    url: Url,
}

#[derive(Deserialize, Clone)]
struct CibaContent {
    ph_en: String,
    ph_am: String,
    ph_en_mp3: String,
    ph_am_mp3: String,
    ph_tts_mp3: String,
    word_mean: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct CibaResponse {
    status: i32,
    content: CibaContent,
}

impl CibaTranslator {
    pub fn new() -> Self {
        let client = Client::new();
        let url = Url::parse("https://fy.iciba.com/ajax.php").unwrap();
        CibaTranslator { client, url }
    }

    fn engine(&self) -> String {
        String::from("ciba")
    }

    fn paraphrase(&self, _resp: &CibaResponse) -> Option<String> {
        None
    }

    fn phonetic(&self, resp: &CibaResponse) -> Option<String> {
        Some(resp.content.ph_en.clone())
    }

    fn explains(&self, resp: &CibaResponse) -> Vec<String> {
        resp.content.word_mean.clone()
    }
}

#[async_trait]
impl Translate for CibaTranslator {
    async fn translate(&self, text: &str, sl: &str, tl: &str) -> TransResult<Translation> {
        let params: Vec<(&str, &str)> = vec![("a", "fy"), ("f", sl), ("t", tl), ("w", text)];
        let url = Url::parse_with_params(self.url.as_str(), &params)?;
        let res = self.client.get(url).send().await?.text().await?;
        let resp: CibaResponse = serde_json::from_str(&res)?;
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
    async fn test_translate_ciba() {
        let translator = CibaTranslator::new();
        assert_eq!(
            r#"Translation { text: "good morning", engine: "ciba", paraphrase: None, phonetic: Some("ɡud ˈmɔ:niŋ"), explains: ["int. （上午见面时用语）早安，你（们）好;（上午分别时用语）再会;"] }"#,
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
