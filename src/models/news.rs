
#[derive(Debug)]
pub struct FeedItem {
    pub id: i16,
    pub title: String,
    pub description:String,
    pub link: String,
}


#[derive(Debug)]
// RSS取得するときの構造体
pub struct NewsRss {
    // 初期設定するもの
    pub genre: String,
    pub rss_url: String,
    pub id_start: i16,

    // 後から持つもの
    pub feed_items: Vec<FeedItem>,
}
impl NewsRss {
    pub fn new(id_start: i16, genre: &str, rss_url: &str) -> Self {
        Self {
            genre: genre.to_string(),
            rss_url: rss_url.to_string(),
            id_start: id_start,

            // デフォルト値で初期化
            feed_items: Vec::new(),
        }
    }
}
