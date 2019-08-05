ソースコードから VSCode のスニペットを生成してくれます。

# 使い方

入力と出力は `config.json` という名前の設定ファイルに書きます。このファイルの場所は、このレポジトリのメインファイルからの相対パスをプログラムオプションで指定します。例えばこのレポジトリのすぐ外においた場合は、`cargo run ..` とすると読みに行ってくれます。

## 設定ファイルの書き方

お手本です。

```json:{prefix}/config.json
[
  "output_file": "out.json",
  "input_dirs": [
    "input_dir"
  ]
]
```

- パスはすべて `config.json` の属するフォルダーからの相対パスです。
- `input_dirs` は複数のディレクトリを指定できます。
- 現状、拡張子等で入力ファイルを絞ることはできません。正規表現に対応したほうが良いでしょうか？

## 動かし方の例

次のようにすると導入できます。

```sh
git clone git@github.com:ngtkana/autolibrary.git
touch config.json
```

先述の通り `config.json` を編集したら、`autolibrary` に `cd` して `cargo run ..` すると動いてくれるはずです。

ちなみに私のアルゴリズムライブラリである ![cppalgo](https://github.com/ngtkana/cppalgo) は autolibrary を使っているので、ビルドスクリプトを参考にされても良いかもしれません。
