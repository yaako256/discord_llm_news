mod controller;
mod models;
mod config;

// RSSの初期設定用
use crate::config::get_news_config;

// LLMにリクエストするときの型
use crate::models::llm_request_fmt::{
    LLMRrequestFmtFinal,
    LLMRrequestFmtFirst,
    LLMRrequestFmtSecond, //LLMRrequestFmtFinal
};
// RSS初期設定を受け取るための型
use crate::models::news::{FeedItem, NewsRss};

// RSSの情報をURLから持ってくる関数
use crate::controller::{fetch_feed, filter_item};

// LLMに聞く関数
use crate::controller::llm_request;

// ログ出力用(デバッグ用)
use std::fs::File;
use std::io::Write;

pub fn generate_news_summary() -> String {
    // 扱うニュース一覧(コンフィグ)を取得
    let mut news_vec: Vec<NewsRss> = get_news_config();
    // エラーを保持しておくための変数
    let mut errors: Vec<String> = Vec::new();

    // RSSのリンクからFeeditemsを作り、idも振る
    // LLMに聞くフォーマット(1回目)も作り出す。
    let (feed_items, llm_request_first_vec): (Vec<FeedItem>, Vec<LLMRrequestFmtFirst>) =
        fetch_feed::news_rss_fetch(&mut news_vec, &mut errors);

    // LLMに聞く(1回目:各ジャンル第3候補を選んでもらう)(候補idをリストを取得)
    // 仮
    let id_list = [
        100, 101, 102, 200, 201, 202, 300, 301, 302, 400, 401, 402, 500, 501, 502, 600, 601, 602,
        700, 701, 702, 800, 801, 802, 900, 901, 902,
    ];
    let id_lista = llm_request::llm_request_first(&llm_request_first_vec);

    // LLMに聞くフォーマット(2回目)にあわせる
    // ついでにこの中でニュースの本文を取得してる
    let llm_request_second_vec: Vec<LLMRrequestFmtSecond> =
        filter_item::filter_feed_items(&feed_items, &id_list, &mut errors);

    // LLMに聞く(2回目:各ジャンル第1候補を選んでもらう)(候補idをリストを取得)
    // 仮
    let id_list = [100, 200, 300, 400, 500, 600, 700, 800, 900];
    let id_lista = llm_request::llm_request_second(&llm_request_second_vec);

    // LLMに聞くフォーマット(3回目)にあわせる。
    let llm_request_final_vec: Vec<LLMRrequestFmtFinal> =
        filter_item::filter_second_items(&llm_request_second_vec, &id_list);

    // LLMに実際に要約してもらい、本文を作成する。
    let text = llm_request::llm_request_final(&llm_request_final_vec);




    // デバッグ出力
    let mut file = File::create("logs/news_log.txt").expect("ファイル作成に失敗しました");
    for news in &llm_request_final_vec {
        writeln!(file, "{:#?}", news).expect("ファイル書き込みに失敗しました");
    }

    // 2. Markdownファイル (純粋なテキストとして書き出す)
    let mut md_file = File::create("logs/news_md.md").expect("ファイル作成に失敗しました");
    // {:#?} をやめて、直接文字列を流し込む
    // 1つ目の記事の内容を「そのまま」出力します
    if let Some(first_news) = llm_request_final_vec.get(0) {
        // .write_all を使うか、writeln! なら "{}" を使う
        write!(md_file, "{}", first_news.contents.clone()).expect("ファイル書き込みに失敗しました");
        return first_news.contents.clone();
    }

    println!("{:#?}", errors);
    "".to_string()
}
