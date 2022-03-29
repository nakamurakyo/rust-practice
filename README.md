# rust-practice

## プログラムのコンパイルと実行
rustc を用いる方法と、Cargoを使ってプロジェクト単位で行う場合がある。

### rustc でコンパイルと実行
1. 新しいファイルの作成
  `$ vi test.rs`

2. ファイル名の設定
  `$ export SCRIPT_NAME=hogehuga.rs`

※ 厳密にはコンパイル時に毎回ファイル名を書いても良いが面倒なため

3. コンパイル  
  `$ rustc $SCRIPT_NAME -C opt-level=3 -o sample_file`
  便利なオプション
    - バイナリの実行速度を最適化する   
      `-C opt-level=3`
    - 出力される実行形式のファイル名の変更  
      `-o hogehoge`
    - エラーメッセージの詳細化  
      `--explain`  
    
    - その他詳細はこちら
  https://doc.rust-lang.org/rustc/command-line-arguments.html#-o-filename-of-the-output

4. 実行  
  `$ ./sample_file`

### 命名
- 変数: snake_case
- 定数: SCREAMING_SNAKE_CASE
- 関数: snake_case

### Cargo でプロジェクトのビルド・実行
1. 新しいプロジェクトの作成  
  `$ cargo new rust_tour_sample_cargo`

```
$ cd rust_tour_sample_cargo/ && tree
.
├── Cargo.toml
└── src
    └── main.rs
```

- `Cargo.toml` はRust用のマニフェストファイルです。プロジェクトのメタデータに加え依存関係も記録されます。
- `src/main.rs` がアプリケーションのコードを書く場所です。

2. プログラムの実行(コンパイル含む)
`$ cargo run`

```
   Compiling rust_tour_sample v0.1.0 (/Users/*******/repo/rust-practice/rust_tour_sample)
    Finished dev [unoptimized + debuginfo] target(s) in 21.58s
     Running `target/debug/rust_tour_sample`
Hello, world!
```

## 参考
- Rust ツアー  
  https://tourofrust.com/00_ja.html

- CodeZine Rustとはどんな言語か？3つの特色や使われているサービスを『詳解Rustプログラミング』から紹介  
  https://codezine.jp/article/detail/15149?utm_source=codezine_ranking_20211212&utm_medium=email

## なぜRUST?
- AWSがプログラミング言語「Rust」に期待する理由    
  https://japan.zdnet.com/article/35183866/

- 和訳: なぜDiscordはGoからRustへ移行するのか  
  https://misonln41.hateblo.jp/entry/2020/02/12/232853
  - 原文: Why Discord is switching from Go to Rust - Discord Blog  
    https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f
