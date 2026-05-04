use rss::Channel;
//use reqwest;
use crate::models::news::FeedItem;


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
