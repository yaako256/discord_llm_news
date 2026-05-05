use rss::Channel;
//use reqwest;
use crate::models::news::{FeedItem, NewsRss};

use reqwest;

use crate::config::{RSS_GET_NUM, SLEEP_TIME_MILLIS_RSS};
use crate::models::llm_request_fmt::LLMRrequestFmtFirst;

// サーバ負荷対策で待つために使う
use std::{thread, time::Duration};

pub fn news_rss_fetch(
    news_vec: &mut Vec<NewsRss>,
    errors: &mut Vec<String>,
) -> (Vec<FeedItem>, Vec<LLMRrequestFmtFirst>) {
    let mut feed_items: Vec<FeedItem> = Vec::new();
    let mut first_llm_request_vec: Vec<LLMRrequestFmtFirst> = Vec::new();

    for news in news_vec {
        let (feed_item, first_llm_request) = fetch_feed_item(
            news.id_start,
            news.genre.as_str(),
            news.rss_url.as_str(),
            errors,
        );
        feed_items.extend(feed_item);
        first_llm_request_vec.extend(first_llm_request);

        // サーバ負荷対策で秒数を開ける
        thread::sleep(Duration::from_millis(SLEEP_TIME_MILLIS_RSS));
    }

    (feed_items, first_llm_request_vec)
}

// RSSのデータを取得する
fn fetch_feed_item(
    id_start: i16,
    genre: &str,
    url: &str,
    errors: &mut Vec<String>,
) -> (Vec<FeedItem>, Vec<LLMRrequestFmtFirst>) {
    let mut feed_items = Vec::new();
    let mut first_llm_request_vec = Vec::new();

    // HTTP GET
    // get() も bytes() も両方 Ok のときだけ body_bytes を取り出す
    let body_bytes = match reqwest::blocking::get(url).and_then(|resp| resp.bytes()) {
        Ok(bytes) => bytes,
        Err(e) => {
            // エラーログを取得
            let msg = format!("Failed to fetch from {}: {}", url, e);
            //eprintln!("{}", msg);
            errors.push(msg);

            // エラーで取得できなかったらこの先意味ないのでリターン
            return (Vec::new(), Vec::new());
        }
    };

    // RSSパース
    let channel = match Channel::read_from(&body_bytes[..]) {
        Ok(ch) => ch,
        Err(e) => {
            let msg = format!("Failed to parse RSS from {}: {}", url, e);
            //eprintln!("{}", msg);
            errors.push(msg);
            return (Vec::new(), Vec::new());
        }
    };

    // フィードアイテムを抽出
    for (i, item) in channel.items().iter().enumerate().take(RSS_GET_NUM) {
        match (item.title(), item.link(), item.description()) {
            (Some(title), Some(link), Some(description)) => {
                // LLMに聞くフォーマット(1回目)にあわせる
                first_llm_request_vec.push(LLMRrequestFmtFirst::new(
                    id_start + (i as i16),
                    genre.to_string(),
                    title.to_string(),
                    description.to_string(),
                ));

                // idとリンクを結びつけておくよう
                feed_items.push(FeedItem::new(
                    id_start + (i as i16),
                    genre.to_string(),
                    link.to_string(),
                ));
            }
            _ => {
                let msg = format!("Item {} skipped: missing title or link", i + 1);
                //eprintln!("{}", msg);
                errors.push(msg);
            }
        }
    }

    (feed_items, first_llm_request_vec)
}
