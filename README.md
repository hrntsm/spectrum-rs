# SPECTRUM-rs

応答スペクトルの計算をするリポ

以下のリポで CPP で書かれたものを Rust に書き換えたもの
- [kakemotokeita/spectrum](https://github.com/kakemotokeita/spectrum)

## 使い方

`cargo run` することで input.txt に記載されている加速度波形から、応答スペクトルの計算をします。  

時間刻みは 0.02s、減衰定数は 0.05 が適用されます。