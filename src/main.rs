use structopt::StructOpt;

mod opt;
mod translator;

use self::opt::TranslatorOptions;
use self::translator::translate;

#[tokio::main]
async fn main() {
    let options = TranslatorOptions::from_args();
    let engine = options.engine.as_str();
    let text = options.text.join(" ");
    let sl = options.from.as_str();
    let tl = options.to.as_str();
    translate(engine, text.as_str(), sl, tl).await;
}
