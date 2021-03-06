use reqwest::blocking::get;

pub fn get_html(word: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://dictionary.goo.ne.jp/word/en/{}/", word);
    let res = get(url)?;
    let doc = res.text()?;
    Ok(doc)
}

pub fn get_html_local(word: &str) -> std::io::Result<String> {
    let file_path = format!("{}.txt", word);
    let doc = std::fs::read_to_string(file_path)?;
    Ok(doc)
}
