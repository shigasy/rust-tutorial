# rust-tutorial
- macOSでコンパイルすると、Mach-Oの実行ファイルができる（macOS用）
- 公式インストーラー（rustup）を使えば、自動でcargoなどが入って環境変数やインストール先のsetupまでしてくれる

## コマンド
Cargoは、Rustのビルドシステム兼、パッケージマネージャ
```
$ cargo build コンパイル
$ cargo run コンパイルして実行
$ cargo check コンパイル可能か
$ cargo build --release 最適化コンパイル
```
Cargoを使用する追加の利点は、使用しているOSに関わらず、同じコマンドが使用できること