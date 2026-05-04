use rss::Channel;
//use reqwest;
use crate::models::news::FeedItem;

use reqwest;
//use html2md;
use html2md;
use scraper::{Html, Selector};
use std::collections::HashSet;


use crate::models::llm_request_fmt::{
    final_llm_request_fmt, first_llm_request_fmt, second_llm_request_fmt,
};
use crate::models::news::NewsRss;

pub fn fetch_feed_items(url: &str) -> (Vec<FeedItem>, Vec<String>) {
    let mut feed_items = Vec::new();
    let mut errors = Vec::new();

    // HTTP GET
    let body_bytes = match reqwest::blocking::get(url) {
        Ok(resp) => match resp.bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                let msg = format!("Failed to read bytes from {}: {}", url, e);
                println!("{}", msg);
                errors.push(msg);
                return (feed_items, errors);
            }
        },
        Err(e) => {
            let msg = format!("Failed to GET {}: {}", url, e);
            println!("{}", msg);
            errors.push(msg);
            return (feed_items, errors);
        }
    };

    // RSSパース
    let channel = match Channel::read_from(&body_bytes[..]) {
        Ok(ch) => ch,
        Err(e) => {
            let msg = format!("Failed to parse RSS from {}: {}", url, e);
            println!("{}", msg);
            errors.push(msg);
            return (feed_items, errors);
        }
    };

    // フィードアイテムを抽出
    for (i, item) in channel.items().iter().enumerate() {
        match (item.title(), item.link(),item.description()) {
            (Some(title), Some(link),Some(description)) => {
                feed_items.push(FeedItem {
                    id :(0 as i16),
                    title: title.to_string(),
                    link: link.to_string(),
                    description:description.to_string()
                });
            }
            _ => {
                let msg = format!("Item {} skipped: missing title or link", i + 1);
                println!("{}", msg);
                errors.push(msg);
            }
        }
    }

    (feed_items, errors)
}





pub fn filter_feed_items(news_list: &Vec<NewsRss>, id_list: &[i16]) -> Vec<second_llm_request_fmt> {
    // IDをHashSetに変換（高速検索用）
    let id_set: HashSet<i16> = id_list.iter().cloned().collect();

    let mut result = Vec::new();

    for news in news_list {
        for feed_item in &news.feed_items {
            if id_set.contains(&feed_item.id) {
                // 本文HTMLテキストを取得
                let response = match reqwest::blocking::get(&feed_item.link) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("URL取得失敗 (ID: {}): {}", feed_item.id, e);
                        continue; // この記事の処理を飛ばして次の記事へ
                    }
                };
                // レスポンスをテキストに変換
                let body_text = match response.text() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("テキスト変換失敗: {}", e);
                        continue;
                    }
                };

                // これを忘れると .select() メソッドが使えません
                let document = Html::parse_document(&body_text);

                // 4. セレクタを作成（ID指定なら "article#uamods"）
                let selector = match Selector::parse("article#uamods") {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("セレクタのパース失敗: {}", e);
                        continue;
                    }
                };

                // 該当する要素を探してテキストを抽出
                let article_text = if let Some(element) = document.select(&selector).next() {
                    // 要素内のテキストを結合して取得
                    element.text().collect::<Vec<_>>().join("")
                } else {
                    // 指定したタグが見つからなかった場合
                    "内容が見つかりませんでした".to_string()
                };

                // 抽出したテキストをmdに変換
                let article_md = html2md::parse_html(&article_text);

                result.push(second_llm_request_fmt::new(
                    feed_item.id,
                    news.genre.clone(),
                    feed_item.title.clone(),
                    article_md,
                ));
            }
        }
    }

    result
}
