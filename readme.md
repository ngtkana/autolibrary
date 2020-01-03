ソースコードから VSCode のスニペットを生成してくれます。

# 使い方

```sh
cargo run ..
```
プログラムオプション（ここでは `..`）は、設定ファイルのあるディレクトリです。上記の場合は `../confib.json` を見に行ってくれます。

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

パスはすべて `config.json` の属するディレクトリからの相対パスです。
これをすると、`input_dirs` に列挙されたディレクトリの部分木にあるファイルを探索して、整形して `out.json` に出力してくれます。これがスニペットです。
