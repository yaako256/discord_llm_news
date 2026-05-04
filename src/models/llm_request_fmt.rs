/*
models/llm_request.rs

LLMにリクエスト時に使うJsonファイル形式を定義する。
*/
#[derive(Debug)]
pub struct first_llm_request_fmt {
    id: i16,
    genre: String,
    title: String,
    description :String,
}
impl first_llm_request_fmt {
    pub fn new(id: i16, genre: String, title: String,description :String) -> Self {
        Self {
            id: id,
            genre: genre,
            title: title,
            description:description
        }
    }
}

#[derive(Debug)]
pub struct second_llm_request_fmt {
    pub id: i16,
    pub genre: String,
    title: String,
    pub contents: String,
}
impl second_llm_request_fmt {
    pub fn new(id: i16, genre: String,title: String, contents: String) -> Self {
        Self {
            id: id,
            genre: genre,
            title:title,
            contents: contents,
        }
    }
}

#[derive(Debug)]
pub struct final_llm_request_fmt {
    genre: String,
    title: String,
    contents: String,
}
impl final_llm_request_fmt {
    pub fn new( genre: String, title: String,contents: String) -> Self {
        Self {
            genre: genre,
            title:title,
            contents: contents,
        }
    }
}

