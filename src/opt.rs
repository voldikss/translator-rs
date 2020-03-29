use structopt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "translator", about = "Rust version of skywind3000/translator")]
pub struct TranslatorOptions {
    #[structopt(short, long, default_value = "youdao", help = "Translate Engine")]
    pub engine: String,

    #[structopt(short, long, default_value = "en", help = "Souce language")]
    pub from: String,

    #[structopt(short, long, default_value = "zh", help = "Target language")]
    pub to: String,

    /// Text
    pub text: Vec<String>,
}

// TODO: read configuration from dosini file
