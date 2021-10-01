//! part of speech.

use scraper::ElementRef;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    Noun,         // 名詞
    Pronoun,      // 代名詞
    Verb,         // 動詞
    Adjective,    // 形容詞
    Adverb,       // 副詞
    Preposition,  // 前置詞
    Conjunction,  // 接続詞
    Interjection, // 間投詞
}

impl Part {
    pub fn from_element(element: &ElementRef) -> Option<Part> {
        for text in element.text() {
            for c in text.chars() {
                match c {
                    '名' => return Some(Part::Noun),
                    '代' => return Some(Part::Pronoun),
                    '動' => return Some(Part::Verb),
                    '形' => return Some(Part::Adjective),
                    '副' => return Some(Part::Adverb),
                    '前' => return Some(Part::Preposition),
                    '接' => return Some(Part::Conjunction),
                    '間' => return Some(Part::Interjection),
                    _ => {}
                }
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct Word {
    pub descriptions: Vec<Description>,
}

impl Word {
    pub fn new() -> Self {
        Self {
            descriptions: Vec::<Description>::new(),
        }
    }
    pub fn push(&mut self, part: Part, text: String) {
        self.descriptions.push(Description::new(part, text));
    }
}


#[derive(Debug)]
pub struct Description {
    part: Part,
    text: String,
}

impl Description {
    pub fn new(part: Part, text: String) -> Self {
        Self { part, text }
    }
    pub fn part(&self) -> Part {
        self.part
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}
