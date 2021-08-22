# gemini-something

最近は新しいgeminiプロトコルを調べていたら、自分のサーバを建てたくなりました。しかし個人サーバで使っているOpenBSDというOSのパッケージレポジトリには、簡単に使えるgeminiサーバはありませんでした。そこで、[ソレーヌさんのブログ](https://dataswamp.org/~solene/2020-11-30-gemini-vger-server.html)を読んだら、スクリプトのようなものを[inetd(8)](https://man.openbsd.org/inetd)で操作することでサーバが作れることを理解し、自分もやってみたいと思いました。

## TODO

- [ ] MIME検出
- [ ] コマンド引数 (gemrootを指定するなど)
- [ ] inetdの設定手順

## 備忘録

ローカル環境でテスト
```
cargo build --release
ncat --ssl -kvlp 1965 -e target/release/gemini-server
```