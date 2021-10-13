# goo-parser

[goo和英辞書](https://dictionary.goo.ne.jp/en/)の簡単なパーサーです(作成中)

# Usage
## install
```
git clone git@github.com:niwa-s/goo-parser.git
```
---
## run

WORDの部分に検索したい英単語を入れてください。
```
cargo run <WORD>
```
---
## example

command
```
cargo run rust
```
result
```
品詞: Noun
1 鉄［赤］さび（iron rust）1a 〔しばしば複合語で〕赤褐色，さび色2 （一般に）金属のさび2a 〔通例修飾語を伴って〕《植物・病気》さび病（菌）3 （能力・活動などの）さびつき，鈍化
========================================================
品詞: Verb
1 自〈鉄などが〉さびる；他さびさせる（away，out）2 自〈葉など（の色）が〉さび色になる；他…をさび色にする3 自〈植物が〉さび病になる；他…をさび病にする4 自〈能力・活動などが〉さびつく；他…をさびつかせる（away，out）

Better to wear out than rust out.((諺))さびるよりすり減ったほうがまし（◆無為の戒め）

========================================================
```