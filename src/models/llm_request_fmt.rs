/*
models/llm_request.rs

LLMにリクエスト時に使うJsonファイル形式を定義する。
*/
#[derive(Debug)]
pub struct first_llm_request_fmt {
    id: i16,
    genre: String,
    title: String,
}
impl first_llm_request_fmt {
    pub fn new(id: i16, genre: String, title: String) -> Self {
        Self {
            id: id,
            genre: genre,
            title: title,
        }
    }
}

#[derive(Debug)]
pub struct second_llm_request_fmt {
    id: i8,
    genre: String,
    title: String,
    body: String,
}
#[derive(Debug)]
pub struct final_llm_request_fmt {
    genre: String,
    title: String,
    body: String,
}
