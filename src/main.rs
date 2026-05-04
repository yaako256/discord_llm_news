//use rss::Channel;
//use std::error::Error;

mod config;
mod models;

mod fetch_feed;
mod llm_request;

use crate::models::llm_request_fmt::{
    final_llm_request_fmt, first_llm_request_fmt, second_llm_request_fmt,
};
use crate::models::news::NewsRss;
use config::get_news_config;
use fetch_feed::fetch_feed_items;
use llm_request::{final_llm_request, first_llm_request, second_llm_request};
use reqwest;

// ログ出力用(デバッグ用)
use std::fs::File;
use std::io::Write;

use std::collections::HashSet;
use html2md;

fn main() {
    // 扱うニュース一覧(コンフィグ)を取得
    let mut news_vec: Vec<NewsRss> = get_news_config();

    // エラーを保持しておくための変数
    let mut errors = Vec::new();

    // 各ジャンルのタイトルとリンクを取得
    for news in &mut news_vec {
        let (feed_items, error) = fetch_feed_items(news.rss_url.as_str());
        news.feed_items = feed_items;
        errors.push(error);
    }

    // LLMに聞くフォーマット(1回目)にあわせる
    let mut first_llm_request_vec = Vec::new();
    for news in &mut news_vec {
        for (i, f) in news.feed_items.iter_mut().enumerate() {
            f.id = news.id_start + (i as i16);
            first_llm_request_vec.push(first_llm_request_fmt::new(
                f.id,
                news.genre.clone(),
                f.title.clone(),
            ));
        }
    }
    let mut file = File::create("news_md.md").expect("ファイル作成に失敗しました");
        writeln!(file, "{:#?}", news_vec[0]).expect("ファイル書き込みに失敗しました");

    /*
    // LLMに聞く(1回目:各ジャンル第3候補を選んでもらう)
    // 仮
    let id_list = [
        100, 101, 102, 200, 201, 202, 300, 301, 302, 400, 401, 402, 500, 501, 502, 600, 601, 602,
    ];
    //let list = first_llm_request(first_llm_request_vec);

    // LLMに聞くフォーマット(2回目)にあわせる
    // ついでにこの中でニュースの本文を取得してる
    let first_llm_request_vec = filter_feed_items(&news_vec, &id_list);

    // デバッグ出力
    let mut file = File::create("news_log.txt").expect("ファイル作成に失敗しました");
    for news in &first_llm_request_vec {
        writeln!(file, "{:#?}", news).expect("ファイル書き込みに失敗しました");
    }

    let mut file = File::create("news_md.md").expect("ファイル作成に失敗しました");
        writeln!(file, "{:#?}", first_llm_request_vec[0].contents).expect("ファイル書き込みに失敗しました");
     */
}

fn filter_feed_items(news_list: &Vec<NewsRss>, id_list: &[i16]) -> Vec<second_llm_request_fmt> {
    // IDをHashSetに変換（高速検索用）
    let id_set: HashSet<i16> = id_list.iter().cloned().collect();

    let mut result = Vec::new();

    for news in news_list {
        for feed_item in &news.feed_items {
            if id_set.contains(&feed_item.id) {
                // 本文htmlテキストを取得する。
                let response = match reqwest::blocking::get(&feed_item.link) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("URL取得失敗: {}", e);
                        return Vec::new(); // あるいは continue (ループ内なら)
                    }
                };

                let body = match response.text() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("テキスト変換失敗: {}", e);
                        return Vec::new();
                    }
                };
                let artcle_md = html2md::parse_html(&body);
                result.push(second_llm_request_fmt::new(
                    feed_item.id,
                    news.genre.clone(),
                    feed_item.title.clone(),
                    artcle_md
                ));
            }
        }
    }

    result
}
