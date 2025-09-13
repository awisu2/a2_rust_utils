#

## 久しぶりな私のために

- 新規ライブラリを作る方法: `cargo new mylib --lib`
- このライブラリはどう利用すればいい? => 後述
- 最新化するための手順
  - 指定バージョン内で良いなら: `cargo update`
  - `cargo upgrade`
    - **cargo-edit** の機能のため、事前にインストール必要: `cargo install cargo-edit`
- テスト: `cargo test`
  - バージョンアップ後などに
- モジュールの作り方:
  1. いくつかの形式がある
     - 内部実装形式: `mod sub {}`. テストなどはこれで `#[cfg(test)]` を付与して実装する
     - ファイル形式: `hello.rs`
     - ディレクトリ形式: `hello/mod.rs`: ディレクトリの中に `mod.rs`
  2. どちらも `src/lib.rs` に `pub mod hello;` の記述が必要
  3. 第一階層は `lib.rs` がエントリーポイント、以降ディレクトリモジュールは `mod.rs` がエントリーポイントになる`
     - 上記通り `pub mod xxx` と記載することで、外部公開の管理ができる
     - => ライブラリ利用者に エントリー関数を用意しそれだけを公開すればシンプルなライブラリの出来上がり

## このライブラリの利用方法

### ssh で利用する

private でも認証通せるので、おすすめだが、ssh の config 利用に課題あり

```bash
cargo install cargo-edit
CARGO_NET_GIT_FETCH_WITH_CLI=true cargo add mylib --git ssh://git@github.com:awisu2/a2_rust_utils.git --package a2_utils
```

結果

```toml
a2_utils = { git = "ssh://github.com/awisu2/a2_rust_utils.git", version = "0.1.8" }
```

- **CARGO_NET_GIT_FETCH_WITH_CLI** について:
  - true で os の ssh を利用、false で rust 組み込みの ssh を利用する (デフォルト false)
  - **環境変数に登録しておくといいかも**
