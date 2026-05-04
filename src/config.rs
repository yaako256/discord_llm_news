use crate::models::news::News;

pub fn get_news_config() -> Vec<News> {
    vec![
        News::new(
            100,
            "主要",
            "https://news.yahoo.co.jp/rss/topics/top-picks.xml",
        ),
        News::new(
            200,
            "国内",
            "https://news.yahoo.co.jp/rss/topics/top-picks.xml",
        ),
        News::new(
            300,
            "経済",
            "https://news.yahoo.co.jp/rss/topics/top-picks.xml",
        ),
        News::new(
            400,
            "エンタメ",
            "https://news.yahoo.co.jp/rss/topics/top-picks.xml",
        ),
        News::new(
            500,
            "IT",
            "https://news.yahoo.co.jp/rss/topics/top-picks.xml",
        ),
        News::new(
            600,
            "科学",
            "https://news.yahoo.co.jp/rss/topics/top-picks.xml",
        ),
    ]
}
