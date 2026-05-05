
//use reqwest;
use crate::models::news::FeedItem;

use reqwest;
use html2md;
use scraper::{Html, Selector};
use std::{collections::HashSet};

use crate::config::{ SLEEP_TIME_MILLIS_BODY};
use crate::models::llm_request_fmt::{
    LLMRrequestFmtSecond, LLMRrequestFmtFinal
};

// サーバ負荷対策で待つために使う
use std::{thread, time::Duration};

// idでフィルターかけて、2回目のLMMリクエストのフォーマットにする
// ここでついでにHTTMPリクエストもしてる。
pub fn filter_feed_items(
    feed_items: &Vec<FeedItem>,
    id_list: &[i16],
    errors: &mut Vec<String>,
) -> Vec<LLMRrequestFmtSecond> {
    // IDをHashSetに変換（高速検索用）
    let id_set: HashSet<i16> = id_list.iter().cloned().collect();

    let mut result = Vec::new();

    for feed_item in feed_items {
        if id_set.contains(&feed_item.id) {
            println!("入ったよ ID:{}", feed_item.id);
            // 本文HTMLテキストを取得
            let response = match reqwest::blocking::get(&feed_item.link) {
                Ok(res) => res,
                Err(e) => {
                    let msg = format!(
                        "URL取得失敗 (ID: {}): {} (link:{})",
                        feed_item.id, e, feed_item.link
                    );
                    //eprintln!("{}", msg);
                    errors.push(msg);
                    continue; // この記事の処理を飛ばして次の記事へ
                }
            };
            // レスポンスをテキストに変換
            let body_text = match response.text() {
                Ok(t) => t,
                Err(e) => {
                    let msg = format!("テキスト変換失敗: {}", e);
                    //eprintln!("{}", msg);
                    errors.push(msg);
                    continue; // この記事の処理を飛ばして次の記事へ
                }
            };

            // これを忘れると .select() メソッドが使えない
            let document = Html::parse_document(&body_text);

            // タイトルのセレクタを作成（ID指定なら "article#uamods"）
            // メインはこれ"article#uamods"
            // タイトルはこれだった。<h1 class="sc-uzx6gd-1 gGHJFY">
            // クラスが意味なさそうな文章で怖いから、メインの中のh1をセレクト
            let title_selector = match Selector::parse("article h1") {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("タイトルセレクタのパース失敗: {}", e);
                    //eprintln!("{}", msg);
                    errors.push(msg);
                    continue; // この記事の処理を飛ばして次の記事へ
                }
            };

            // 該当する要素(タイトル)を探してテキストを抽出
            let title_text = if let Some(element) = document.select(&title_selector).next() {
                // 要素内のテキストを結合して取得
                element.text().collect::<Vec<_>>().join("")
            } else {
                let msg = format!("タイトル要素検出失敗 (ID:{})", feed_item.id);
                //eprintln!("{}", msg);
                errors.push(msg);
                continue; // この記事の処理を飛ばして次の記事へ
            };

            // 本文のセレクタを作成
            // メインはこれ"article#uamods"
            // 本文はこれだった。<div class="article_body highLightSearchTarget">
            // クラスが意味なさそうな文章で怖いから、メインの中のdiv article_bodyをセレクト
            let article_selector = match Selector::parse("article div.article_body") {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("本文セレクタのパース失敗: {}", e);
                    //eprintln!("{}", msg);
                    errors.push(msg);
                    continue; // この記事の処理を飛ばして次の記事へ
                }
            };

            // 該当する要素(本文)を探してテキストを抽出
            let article_text = if let Some(element) = document.select(&article_selector).next() {
                // 要素内のテキストを結合して取得
                element.text().collect::<Vec<_>>().join("")
            } else {
                // 指定したタグが見つからなかった場合
                let msg = format!("本文要素検出失敗 (ID:{})", feed_item.id);
                //eprintln!("{}", msg);
                errors.push(msg);
                continue; // この記事の処理を飛ばして次の記事へ
            };

            // 抽出したテキストをmdに変換
            let article_md = html2md::parse_html(&article_text);

            result.push(LLMRrequestFmtSecond::new(
                feed_item.id,
                feed_item.genre.clone(),
                title_text,
                article_md,
            ));

            // サーバ負荷対策で秒数を開ける
            thread::sleep(Duration::from_millis(SLEEP_TIME_MILLIS_BODY));
        }
    }
    result
}


/// 2回目の選別から最後のリクエストのフォーマットにする関数。
pub fn filter_second_items(
    feed_items: &Vec<LLMRrequestFmtSecond>,
    id_list: &[i16],
) -> Vec<LLMRrequestFmtFinal> {
    // id_list を HashSet に変換して検索を高速化
    let id_set: HashSet<i16> = id_list.iter().cloned().collect();

    feed_items
        .iter()
        .filter_map(|item| {
            if id_set.contains(&item.id) {
                // 一致する場合は LLMRrequestFmtFinal を作る
                Some(LLMRrequestFmtFinal {
                    genre: item.genre.clone(),
                    title: item.title.clone(),
                    contents: item.contents.clone(),
                })
            } else {
                None
            }
        })
        .collect()
}