use base62::encode;
use md5::compute;

pub fn generate(value: &str) -> String {
    let hash = compute(value);
    let hash_bytes: [u8; 16] = hash.into();
    let hash_u128: u128 = u128::from_le_bytes(hash_bytes);
    encode(hash_u128)[..7].to_string()
}
