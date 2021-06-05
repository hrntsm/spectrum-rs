# spectrum-rs

応答スペクトルの計算をするリポ

以下のリポで CPP で書かれたものを Rust に書き換えたもの
- [kakemotokeita/spectrum](https://github.com/kakemotokeita/spectrum)

## 使い方

`cargo run` または、release にある spectrum.exe ファイルを実行することで入力ファイル内の加速度波形から、応答スペクトルの計算をします。  
デフォルトの設定は以下です
- 時間刻み : 0.02s
- 減衰定数 : 0.05
- 入力ファイル : ./input.txt
- 出力ファイル : ./result.txt

オプションで上記デフォルの値を変更することができます。
オプションは以下です。
- -d : 時間刻みの設定
- -h :  減衰定数の設定
- -i :  インプットファイルパス
- -o :  アウトプットファイルパス

`cargo run -- --help` とすることでコマンドラインでヘルプを確認できます。

デフォルトの設定は以下のように書くことができます。
```bash
cargo run -- -i ./input.txt -o result.txt -h 0.05 -d 0.02
```

GitHub のRelease にあげているビルドされたものを使うときは以下です。

Macの場合
```
./spectrum -i ./input.txt -o result.txt -h 0.05 -d 0.02
```

Windows の場合
```
spectrum.exe -i ./input.txt -o result.txt -h 0.05 -d 0.02
```