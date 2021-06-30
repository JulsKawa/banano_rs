use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlockCount {

    pub count: String,
    pub unchecked: String,
    pub cemented: String
}