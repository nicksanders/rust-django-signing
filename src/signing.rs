
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use crypto::mac::{Mac, MacResult};
use crypto::hmac::Hmac;

pub const DEFAULT_SALT: &'static str = "django.contrib.sessions.backends.signed_cookiessigner";

pub fn b64_encode(s: &[u8]) -> String {
    s.to_base64(base64::URL_SAFE).to_string().trim_matches('=').to_string()
}

pub fn b64_decode(s: &str) -> Vec<u8> {
    let num = (0 as isize - s.len() as isize) % 4;
    let pad = (0..num).map(|_| "=").collect::<Vec<_>>().join("");
    let padded_str = s.to_string() + &pad;
    padded_str.into_bytes().from_base64().ok().expect("b64_decode failed")
}

pub fn salted_hmac(salt: &str, secret: &str, value: &str) -> MacResult {
    let mut hasher = Sha1::new();
    let input_str = salt.to_string() + secret;
    hasher.input_str(&input_str);
    let mut sha1_hash = [0u8; 20];
    hasher.result(&mut sha1_hash);

    let mut hmac = Hmac::new(Sha1::new(), &sha1_hash);
    hmac.input(&value.to_string().as_bytes());
    hmac.result()
}

pub fn base64_hmac(salt: &str, secret: &str, value: &str) -> String {
    b64_encode(salted_hmac(salt, secret, value).code())
}

#[test]
fn test_b64_decode() {
    let input = "dpzEj8pbu98e1vde2zSv-0DbQ7o";
    let output = b"v\x9c\xc4\x8f\xca[\xbb\xdf\x1e\xd6\xf7^\xdb4\xaf\xfb@\xdbC\xba";

    assert_eq!(b64_decode(input), output);
}

#[test]
fn test_base64_hmac() {
    let salt = DEFAULT_SALT;
    let secret = "secret";
    let input = "hey there dude";
    let output = "cRW28cBU8dYu_qWqEzvIQw3dwMI";

    assert_eq!(base64_hmac(salt, secret, input), output);
}
