/*
models/news.rs

初期設定としてRSSを設定するときに使う構造体と
リンクを保持しとく用の構造体を定義するところ
*/

#[derive(Debug)]
// 取得するRSSを設定するときの構造体
pub struct NewsRss {
    // 初期設定するもの
    pub genre: String,
    pub rss_url: String,
    pub id_start: i16,
}
impl NewsRss {
    pub fn new(id_start: i16, genre: &str, rss_url: &str) -> Self {
        Self {
            genre: genre.to_string(),
            rss_url: rss_url.to_string(),
            id_start: id_start,
        }
    }
}

// idとリンクとタイトルを関連付けて保持しておく用
#[derive(Debug)]
pub struct FeedItem {
    pub id: i16,
    pub genre: String,
    pub link: String,
}
impl FeedItem {
    pub fn new(id: i16, genre: String, link: String) -> Self {
        Self {
            id: id,
            genre: genre,
            link: link,
        }
    }
}
