# Atcoder

## 新しい問題の始め方
```bash
cargo generate --git https://github.com/rust-lang-ja/atcoder-rust-base --branch ja --name abc086c
```

その後、`.vscode/settings.json`にフォルダを書き足す

## 環境構築
```bash
cargo install cargo-generate
```
https://github.com/cargo-generate/cargo-generate


また、拡張機能をrustのバージョンに合わせて入れる必要がある。
atcoderでは大分昔のバージョンを使う必要があり、rust-analyzer@v0.2.1056　で動作することを確認した。
vscode 拡張機能インストール画面の歯車から、バージョンを指定してインストールすることができる。

