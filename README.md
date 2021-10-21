# goo-parser

[goo和英辞書](https://dictionary.goo.ne.jp/en/)の簡単なパーサーです(作成中)

`https://dictionary.goo.ne.jp/word/*/*/`へのスクレイピングが禁止されていないことはrobots.txtで確認していますが、問題がありそうなら消します。

# Usage
## install
```
git clone git@github.com:niwa-s/goo-parser.git
```
## run

WORDの部分に検索したい英単語を入れてください。
```
cargo run <WORD>
```
## example

command
```
cargo run milk
```
result
```
品詞: Noun
1 乳，母乳，（特に）牛乳
2 乳状の液，（植物の）樹乳，乳液

品詞: Verb
1 他自〈牛などの〉乳をしぼる
1a 他〈乳を〉（動物から）しぼる
1b 自〈牛などが〉乳を出す
1c 他〈動物・植物などの〉（毒・体液・樹液などを）しぼり出す≪of≫；他〈樹液・体液・毒などを〉（動物・植物などから）しぼり出す≪from≫
2 他((略式))…を食いものにする，〈人から〉（金・情報などを）しぼり取る，引き出す≪of，for≫；〈金・情報などを〉（人などから）引き出す≪out of≫
```
