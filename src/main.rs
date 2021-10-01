use goo_parser::client::get_html;
use goo_parser::parser::my_parse_docment;

fn main() {
    match get_html("milk") {
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

/********** result! **********/
/*
品詞: Noun
1 乳，母乳，（特に）牛乳

first milk初乳
cow's milk牛乳
a bottle of milk１びんのミルク
(as) white as milkまっ白な
a cow in milk乳の出ている牛
milk mustache牛乳でつけた上くちびるの白い筋
2 乳状の液，（植物の）樹乳，乳液

milk of lime石灰乳

cry over spilled [spilt] milk
過ぎたことを悔む

get [come] home with the milk
((英俗))（夜どおし騒いだあと）朝帰りする

milk and honey
乳とみつ；豊かな生活のかて

milk and water
1 水で割った牛乳
2 気の抜けた話
3 めそめそした感情

milk for babes [babies]
子ども向きの本［説教］

the milk in the coconut
((俗))物事の核心，要点

the milk of human kindness
((形式))心の優しさ（◆Shakespeare より）

========================================================
品詞: Verb
1 他自〈牛などの〉乳をしぼる

milk a cow [a goat]牛［ヤギ］の乳をしぼる
1a 他〈乳を〉（動物から）しぼる1b 自〈牛などが〉乳を出す1c 他〈動物・植物などの〉（毒・体液・樹液などを）しぼり出す≪of≫；他〈樹液・体液・毒などを〉（動物・植物などから）しぼり出す≪from≫2 他((略式))…を食いものにする，〈人から〉（金・情報などを）しぼり取る，引き出す≪of，for≫；〈金・情報などを〉（人などから）引き出す≪out of≫

milk an enterprise事業を食いものにする
milk a person for information人から情報を聞き出す
milk ... for all its worth…を最大限に利用する

========================================================
*/