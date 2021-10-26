use super::models::Part;
use crate::{consts, models::Word};
use scraper::{ElementRef, Selector};
use std::convert::TryFrom;

pub fn parse(docment: String) -> Result<Word, Box<dyn std::error::Error>> {
    let html = scraper::Html::parse_document(&docment);
    let ol_selector = Selector::parse("ol.list-meanings").unwrap();
    let ol_list = html.select(&ol_selector);
    let mut word = Word::new();

    for ol_elem in ol_list {
        // ol_elemの兄要素に単語の品詞情報が含まれているので抽出する
        let part = get_part(&ol_elem).expect("Failed to get part");
        let mut description = String::new();

        for li_node in ol_elem.children() {
            if !li_node.value().is_element() {
                continue;
            }
            let li_elem = ElementRef::wrap(li_node).unwrap();
            let mut text: String = li_elem
                .text()
                .skip(1)
                .map(wrap_hinshi)
                .take_while(|&s| !s.eq("\n"))
                .map(|s| s.trim())
                .collect();
            text.push('\n');
            description.push_str(&text);
        }

        word.push(part, description);
    }

    Ok(word)
}

fn get_part(elem: &ElementRef) -> Result<Part, String> {
    match elem.prev_siblings().find(|node| node.value().is_element()) {
        Some(prev_sibling) => {
            let prev_sibling = ElementRef::wrap(prev_sibling).unwrap();
            match Part::try_from(prev_sibling) {
                Ok(part) => Ok(part),
                Err(_) => Err("Failed to parse part of speech".to_owned()),
            }
        }
        None => Err("Failed to get prev_siblings".to_owned()),
    }
}

fn wrap_hinshi(s: &str) -> &str {
    match s {
        "自" => consts::INTRANSITIVE_VERB,
        "他" =>  consts::TRANSITIVE_VERB,
        _ => s
    }
}
