// パッケージ
pub mod config;
pub mod controller;
pub mod discord;
pub mod models;

// サーバ負荷対策やApi対策で待つために使う
use std::{thread, time::Duration};

// LLMにリクエストするときの型
use crate::models::llm_request_fmt::{
    LLMRrequestFmtFinal, LLMRrequestFmtFirst, LLMRrequestFmtSecond,
};

// RSS初期設定を受け取るための型
use crate::models::news::{FeedItem, NewsRss};

// RSSの情報をURLから持ってくる関数
use crate::controller::{fetch_feed, filter_item};
// LLMに聞く関数
use crate::controller::llm_request;

/// ニュースをRSSから取得、LLMで要約、discordに送信できる形にしてreturn
pub fn generate_news_summary(
    news_vec: &[NewsRss],
    config: &config::Config,
    errors: &mut Vec<String>,
) -> String {
    // RSSのリンクからFeeditemsを作り、idも振る
    // LLMに聞くフォーマット(1回目)も作り出す。
    let (feed_items, llm_request_first_vec): (Vec<FeedItem>, Vec<LLMRrequestFmtFirst>) =
        fetch_feed::news_rss_fetch(&news_vec, errors);

    // LLMに聞く(1回目:各ジャンル第3候補を選んでもらう)(候補idをリストを取得)
    // もし正しい返答が返ってこなかったらMAX_RETRIESまで繰り返す
    // レスポンス用配列の初期化
    let mut id_list: Vec<i16> = Vec::new();
    for i in 0..config::MAX_RETRIES {
        // APIの対策で秒数を開ける
        thread::sleep(Duration::from_millis(config::SLEEP_LLM_THINK_FIRST));
        match llm_request::llm_request_first(&llm_request_first_vec, &config, errors) {
            Ok(ids) => {
                id_list = ids;
                break; // 成功したのでループを抜ける
            }
            Err(_) => {
                if error_process(i, "first_request_loop", errors) {
                    return "".to_string();
                }
            }
        }
    }

    // LLMに聞くフォーマット(2回目)にあわせる
    // ついでにこの中でニュースの本文を取得してる
    let llm_request_second_vec: Vec<LLMRrequestFmtSecond> =
        filter_item::filter_feed_items(&feed_items, &id_list, errors);

    // LLMに聞く(2回目:各ジャンル第1候補を選んでもらう)(候補idをリストを取得)
    let mut id_list: Vec<i16> = Vec::new();
    for i in 0..config::MAX_RETRIES {
        // APIの対策で秒数を開ける
        thread::sleep(Duration::from_millis(config::SLEEP_LLM_THINK_SECOND));
        match llm_request::llm_request_second(&llm_request_second_vec, &config, errors) {
            Ok(ids) => {
                id_list = ids;
                break; // 成功したのでループを抜ける
            }
            Err(_) => {
                if error_process(i, "second_request_loop", errors) {
                    return "".to_string();
                }
            }
        }
    }

    // LLMに聞くフォーマット(3回目)にあわせる。
    let llm_request_final_vec: Vec<LLMRrequestFmtFinal> =
        filter_item::filter_second_items(&llm_request_second_vec, &id_list);

    // LLMに実際に要約してもらい、本文を作成する。
    let mut res_text_md = String::new();
    for i in 0..config::MAX_RETRIES {
        // APIの対策で秒数を開ける(正直いらない)
        thread::sleep(Duration::from_millis(config::SLEEP_LLM_THINK_FINAL));
        match llm_request::llm_request_final(&llm_request_final_vec, &config, errors) {
            Ok(contents) => {
                res_text_md = contents;
                break; // 成功したのでループを抜けるa
            }
            Err(_) => {
                if error_process(i, "final_request_loop", errors) {
                    return "".to_string();
                }
            }
        }
    }

    // 送信するテキストをreturn
    res_text_md
}

// リクエストがエラーだった時ののプロセス
fn error_process(count: usize, time: &str, errors: &mut Vec<String>) -> bool {
    let msg = format!(
        "試行 {}/{}: エラーが発生しました({})",
        count + 1,
        config::MAX_RETRIES,
        time
    );
    errors.push(msg);
    if count + 1 >= config::MAX_RETRIES{
        // 最後まで失敗した場合の処理
        let msg = format!("最大試行回数に達しました。({})", time);
        errors.push(msg);
        // 空のストリングを返す。
        return true;
    }
    return false;
}
