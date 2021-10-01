use goo_parser::client::get_html_local;
use goo_parser::parser::my_parse_docment;

fn main() {
    match get_html_local("milk") {
        Ok(document) => {
            let word = my_parse_docment(document).expect("failed parse document");
            for description in word.descriptions {
                println!("品詞: {:?}\n{}", description.part(), description.text());
                println!("========================================================")
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
