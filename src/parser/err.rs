#[derive(Debug)]
pub struct MCParseError {
    pub at: (u64, u64),
    pub literal_len: usize,
    pub full_msg: String,
    pub msg: String,
}
