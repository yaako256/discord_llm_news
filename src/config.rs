use crate::models::news::NewsRss;

// RSSから最新のニュースをいくつ扱うか
pub const RSS_GET_NUM:usize = 5;

// サーバ負荷対策で何ミリ秒待つか
pub const SLEEP_TIME_MILLIS_RSS:u64 = 1000;
pub const SLEEP_TIME_MILLIS_BODY:u64 = 10000;
/*
yahooのRSS[https://news.yahoo.co.jp/rss]の
カテゴリごとのニュースから取得。
トピックスは本文へのurlじゃなかった。
*/
pub fn get_news_config() -> Vec<NewsRss> {
    vec![
        NewsRss::new(
            100,
            "国内",
            "https://news.yahoo.co.jp/rss/categories/domestic.xml",
        ),
        NewsRss::new(
            200,
            "国際",
            "https://news.yahoo.co.jp/rss/categories/world.xml",
        ),
        NewsRss::new(
            300,
            "経済",
            "https://news.yahoo.co.jp/rss/categories/business.xml",
        ),
        NewsRss::new(
            400,
            "エンタメ",
            "https://news.yahoo.co.jp/rss/categories/entertainment.xml",
        ),
        NewsRss::new(
            500,
            "スポーツ",
            "https://news.yahoo.co.jp/rss/categories/sports.xml",
        ),
        NewsRss::new(
        600,
            "IT",
            "https://news.yahoo.co.jp/rss/categories/it.xml",
        ),
        NewsRss::new(
            700,
            "科学",
            "https://news.yahoo.co.jp/rss/categories/science.xml",
        ),
        NewsRss::new(
            800,
            "ライフ",
            "https://news.yahoo.co.jp/rss/categories/life.xml",
        ),
        NewsRss::new(
            900,
            "地域",
            "https://news.yahoo.co.jp/rss/categories/local.xml",
        ),
    ]
}
