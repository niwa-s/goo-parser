use scraper::{ElementRef, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = std::fs::read_to_string("tmp.txt")?;
    let doc = scraper::Html::parse_document(&body);
    let ol_selector = Selector::parse("ol").unwrap();
    let ol_list = doc.select(&ol_selector);
    for ol_element in ol_list {
        // list-data-bクラスに必要そうなデータが記載されているのでそれ以外はスルー
        let has_list_data_class = ol_element.value().classes().any(|s| s.eq("list-data-b"));
        if !has_list_data_class {
            continue;
        }


        // list-data-bクラスが付与されたol要素には、list-meaningsクラスとlist-idiomクラスのどちらかも付与されていて、
        // list-meaningsクラスが付与されたnodeの手前(兄node?)に、品詞の情報が記載されたnodeがあり、hinsi-headerクラスが付与されている（要検証）
        let has_list_meanings_class = ol_element.value().classes().any(|s| s.eq("list-meanings"));
        if has_list_meanings_class {
            match ol_element.prev_siblings().find(|node| node.value().is_element()) {
                Some(prev_sibling) => {
                    let prev_sibling = ElementRef::wrap(prev_sibling).unwrap();
                    let text = prev_sibling.text().collect::<String>();
                    println!("prev_html: {}", text);
                    for text in prev_sibling.text() {
                        let ok = if text.contains('動') {
                            "verb!"
                        } else {
                            "noun!"
                        };
                        println!("{}", ok);
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
            println!("text: {}", text);
        }
    }
    Ok(())
}
