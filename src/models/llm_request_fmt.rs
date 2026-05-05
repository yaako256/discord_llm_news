/*
models/llm_request.rs

LLMにリクエスト時に使うJsonファイル形式を定義する。
*/
// 未使用だよっていう注意をなくしてくれるやつ
#![allow(dead_code)]

#[derive(Debug)]
pub struct LLMRrequestFmtFirst {
    id: i16,
    genre: String,
    title: String,
    description: String,
}
impl LLMRrequestFmtFirst {
    pub fn new(id: i16, genre: String, title: String, description: String) -> Self {
        Self {
            id: id,
            genre: genre,
            title: title,
            description: description,
        }
    }
}

#[derive(Debug)]
pub struct LLMRrequestFmtSecond {
    pub id: i16,
    pub genre: String,
    pub title: String,
    pub contents: String,
}
impl LLMRrequestFmtSecond {
    pub fn new(id: i16, genre: String, title: String, contents: String) -> Self {
        Self {
            id: id,
            genre: genre,
            title: title,
            contents: contents,
        }
    }
}

#[derive(Debug)]
pub struct LLMRrequestFmtFinal {
    pub genre: String,
    pub title: String,
    pub contents: String,
}
impl LLMRrequestFmtFinal {
    pub fn new(genre: String, title: String, contents: String) -> Self {
        Self {
            genre: genre,
            title: title,
            contents: contents,
        }
    }
}
