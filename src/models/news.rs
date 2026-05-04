#[derive(Debug)]
pub struct FeedItem {
    pub title: String,
    pub link: String,
}

#[derive(Debug)]
pub struct Contents {
    pub title: String,
    pub body: String,
}

#[derive(Debug)]
pub struct News {
    // 初期設定するもの
    pub genre: String,
    pub rss_url: String,
    pub id_start: i16,

    // 後から持つもの
    pub feed_items: Vec<FeedItem>,
    pub contents: Vec<Contents>,
}
impl News {
    pub fn new(id_start: i16, genre: &str, rss_url: &str) -> Self {
        Self {
            genre: genre.to_string(),
            rss_url: rss_url.to_string(),
            id_start: id_start,

            // デフォルト値で初期化
            feed_items: Vec::new(),
            contents: Vec::new(),
        }
    }
}
