use clap::{App, Arg};
use goo_parser::client::get_html;
use goo_parser::parser::my_parse_docment;

fn main() {
    let app = App::new("goo-parser")
        .version("0.1.0")
        .author("niwa-s")
        .about("api parser for goo-dictionary")
        .arg(
            Arg::with_name("word")
                .help("Arguments for words to be translated into Japanese")
                .required(true),
        );

    let matches = app.get_matches();
    let word = matches.value_of("word").unwrap();
    match get_html(word) {
        Ok(document) => {
            let word = my_parse_docment(document).expect("failed parse document");
            for description in word.descriptions {
                println!("品詞: {:?}", description.part());
                println!("{}", description.text());
                println!("========================================================")
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
