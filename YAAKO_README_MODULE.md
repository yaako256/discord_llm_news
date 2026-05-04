# モジュール概要
このマークダウンでは、本プロジェクト作成に当たって調べ、使ったモジュール(LLMやRSSの取得)を、備忘録としてここに記す


## 使用するニュースサイトRSS
yahoo!ニュース[https://news.yahoo.co.jp/rss]

## RSS feedについて
### 参考サイト一覧
(使える◎、まぁ使える〇、あんま△)<br>
(1) ◎ [https://shinshin86.hateblo.jp/entry/2022/03/24/060000]
(2) △ [https://qiita.com/k4nd4/items/a99c84f367ee09f1ba18]
(3) 〇 [https://thinkit.co.jp/article/22922?page=1]
(4) △ [https://qiita.com/sen-ltd/items/aaf60af61c43294e19ba]
---

### RSSとは何か
```

```

### RSS feedの種類と特徴
参考で使ってる人も多そうであることから、一旦rssに手を出す。<br>
rss1.0はもうほぼ使えなく、現在はrss2.0が主流らしい。
| クレート名  |    使用法      | 参考使用数 | 特徴　|
| -----------| ------------- | ----------------- | ----------------- |
| rss        | rss = "2.0"   | 3|古くからあり、サポートしてるサイトが多い。<br>自由度が高いため、formatがまちまち|
| feed-rs    | feed-rs = "2.3"|1 |比較的新しめ？formatががっちりしてる。サポートしてないサイトもある|
| atom    | 記載なし|1 |よくわからない|


### その他必要なクレート
HTTPクライアント (reqwest)というものが必要そう。URLにアクセスするためのもの？<br>
参考にしたサイトはみんなこれを使っていた。<br>
blockingとはは同期(blocking)のこと。非同期(async)と区別。<br>
単純にリクエストを送って、結果が返ってくるまで待つため、blokingでよい。
↓参考ごとの実装方法↓<br>
```
(1) reqwest = { version = "0.11", features = ["blocking"] }
(2) reqwest = { version = "0.12", features = ["rustls-tls", "json"] }
(3) 記載なし
```
ただ実行するのつもりである。同期で実行するため、次をtomlに設定した。<br>
version0.12は非同期がデフォであるため、設定を明示
```
reqwest = { version = "0.12", features = ["blocking"] }
```
---
Htmlをmdに変換してLLMに渡したい。これ↓<br>
html2md
