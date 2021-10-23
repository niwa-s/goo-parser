//! part of speech.

use std::convert::TryFrom;

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

impl TryFrom<ElementRef<'_>> for Part {
    type Error = ();
    fn try_from(element: ElementRef) -> Result<Part, Self::Error> {
        for text in element.text() {
            for c in text.chars() {
                match c {
                    '名' => return Ok(Part::Noun),
                    '代' => return Ok(Part::Pronoun),
                    '動' => return Ok(Part::Verb),
                    '形' => return Ok(Part::Adjective),
                    '副' => return Ok(Part::Adverb),
                    '前' => return Ok(Part::Preposition),
                    '接' => return Ok(Part::Conjunction),
                    '間' => return Ok(Part::Interjection),
                    _ => {}
                }
            }
        }
        Err(())
    }
}

#[derive(Debug, Default)]
pub struct Word {
    descriptions: Vec<Description>,
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
    pub fn descriptions(self) -> impl IntoIterator<Item = Description> {
        self.descriptions.into_iter()
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
