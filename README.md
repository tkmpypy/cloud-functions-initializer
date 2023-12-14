# cloud-functions-initializer

Cloud Functionsのプロジェクト開始コマンド（自分用）<br />
以下のCloud Functionsのドキュメントをベースにしています

- [Cloud Functions の関数を作成する](https://cloud.google.com/functions/docs/writing)
- [HTTP関数を作成する](https://cloud.google.com/functions/docs/writing/write-http-functions)
- [イベントドリブン関数を作成する](https://cloud.google.com/functions/docs/writing/write-event-driven-functions)$

※ **JVM系ランタイム** と **.NET系ランタイム** は実装していません

## Installation

```sh
$ git clone https://github.com/tkmpypy/cloud-functions-initializer.git
$ cd cloud-functions-initializer
$ cargo install --path .
```

## Usage

```sh
cloud-functions-initializer --lang go --func http -o .
```

```sh
cloud-functions-initializer --lang node --func event -o /path/to/work
```

### Options

| Long        | short | Default | Comment           |
| ----------- | ----- | ------- | ----------------- |
| `--lang`    | `-l`  | `node`  | not runtime       |
| `--func`    | `-f`  | `http`  | `http` or `event` |
| `--output`  | `-o`  | `.`     | \-                |
| `--parents` | `-p`  | `false` | like `mkdir -p`   |

