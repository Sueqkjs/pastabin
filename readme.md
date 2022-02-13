# pastabin

## これはなに
SvelteとRustで作ったゴミ

## 実行

### 必要なもの
- redis(server)

### やり方
- install redis
```bash
# Linux
apt install redis-server
```
- ファイルをいじる
- .env
- (カレントディレクトリにpastabinのバイナリがあることも確認してください)
```env
# 公開するアドレス。nginxとか通しても大丈夫なはず
bind=0.0.0.0:8080
# redisサーバーのアドレス。これはredisのデフォ
redis_addr=127.0.0.1:6379
# スレッド数
workers=2
```


## セルフビルド

### 必要なもの
- Cargo
- Rustup (wasm32-unknown-unknownターゲットの追加)
- wasm-pack
- Nodejs
- Npm
- スマートな脳
- エラー文を読む力

### やり方
cloneしてbuildっていうファイルを実行するだけ
```bash
git clone https://github.com/Sueqkjs/pastabin
cd pastabin
# Linux or Mac
./build --release
# Windows
node build --release
```


## LICENSE
This library is free and open-source software released under the MIT license.
