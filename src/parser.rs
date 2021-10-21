use super::models::Part;
use crate::models::Word;
use scraper::{ElementRef, Selector};
use std::convert::TryFrom;

pub fn my_parse_docment(docment: String) -> Result<Word, Box<dyn std::error::Error>> {
    let html = scraper::Html::parse_document(&docment);
    let ol_selector = Selector::parse("ol").unwrap();
    let ol_list = html.select(&ol_selector);
    let mut part = None;
    let mut description = String::new();
    let mut word = Word::new();

    for ol_element in ol_list {
        // list-data-bクラスに必要なデータが記載されているのでそれ以外はスルー
        let has_list_meanings_class = ol_element.value().classes().any(|s| s.eq("list-meanings"));
        if !has_list_meanings_class {
            continue;
        }

        // ol要素にはlist-meaningsクラスとlist-idiomクラスのどちらかが付与されているので、
        // もしもlist-meaningsクラスが付与されていた場合は、兄要素に記載されている品詞の情報も取得しておく
        if let Some(p) = part {
            word.push(p, description.to_string());
        }
        match ol_element
            .prev_siblings()
            .find(|node| node.value().is_element())
        {
            Some(prev_sibling) => {
                let prev_sibling = ElementRef::wrap(prev_sibling).unwrap();
                if let Ok(p) = Part::try_from(prev_sibling) {
                    part = Some(p);
                    description = String::new();
                }
            }
            None => {
                unreachable!();
            }
        }

        for li_node in ol_element.children() {
            if !li_node.value().is_element() {
                continue;
            }
            let li_element = ElementRef::wrap(li_node).unwrap();
            let text: String = li_element.text().take_while(|&s| !s.eq("\n")).collect();
            let text = text.trim();
            description += text;
            description += "\n";
        }
    }

    if let Some(p) = part {
        word.push(p, description);
    }

    Ok(word)
}
