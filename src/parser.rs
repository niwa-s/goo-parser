use crate::models::Word;

use super::models::Part;
use scraper::{ElementRef, Selector};

pub fn my_parse_docment(docment: String) -> Result<Word, Box<dyn std::error::Error>> {
    let html = scraper::Html::parse_document(&docment);
    let ol_selector = Selector::parse("ol").unwrap();
    let ol_list = html.select(&ol_selector);
    let mut part = None;
    let mut description = String::new();
    let mut word = Word::new();
    for ol_element in ol_list {
        // list-data-bクラスに必要そうなデータが記載されているのでそれ以外はスルー
        let has_list_data_class = ol_element.value().classes().any(|s| s.eq("list-data-b"));
        if !has_list_data_class {
            continue;
        }

        // ol要素にはlist-meaningsクラスとlist-idiomクラスのどちらかが付与されているので、
        // もしもlist-meaningsクラスが付与されていた場合は、兄要素に記載されている品詞の情報も取得しておく
        let has_list_meanings_class = ol_element.value().classes().any(|s| s.eq("list-meanings"));
        if has_list_meanings_class {
            if let Some(p) = part {
                word.push(p, description.to_string());
            }
            match ol_element
                .prev_siblings()
                .find(|node| node.value().is_element())
            {
                Some(prev_sibling) => {
                    let prev_sibling = ElementRef::wrap(prev_sibling).unwrap();
                    if let Some(p) = Part::from_element(&prev_sibling) {
                        part = Some(p);
                        description = String::new();
                    }
                }
                None => {
                    unreachable!();
                }
            }
        }

        for li_element in ol_element.children() {
            if !li_element.value().is_element() {
                continue;
            }
            let li_element = ElementRef::wrap(li_element).unwrap();
            let text = li_element.text().collect::<String>();
            description += &text;
        }
    }
    if let Some(p) = part {
        word.push(p, description);
    }
    Ok(word)
}