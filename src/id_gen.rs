use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const ID_LEN: usize = 4;

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    (0..ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn is_valid_id(s: &str) -> bool {
    s.len() == ID_LEN && s.chars().all(|c| c.is_alphanumeric())
}
