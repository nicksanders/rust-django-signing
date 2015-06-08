
extern crate flate2;
extern crate crypto;
extern crate rustc_serialize;
extern crate time;

use std::io::prelude::*;
use flate2::read::ZlibDecoder;
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use crypto::mac::{Mac, MacResult};
use crypto::hmac::Hmac;

pub mod baseconv;

pub const DEFAULT_SALT: &'static str = "django.contrib.sessions.backends.signed_cookiessigner";


#[derive(Debug, PartialEq)]
pub enum SessionError {
    SessionInvalid,
    SignatureInvalid,
    SessionExpired,
}

pub fn b64_encode(s: &[u8]) -> String {
    s.to_base64(base64::URL_SAFE).to_string().trim_matches('=').to_string()
}

pub fn b64_decode(s: &str) -> Vec<u8> {
    let num = (0 as isize - s.len() as isize) % 4;
    let pad = (0..num).map(|_| "=").collect::<Vec<_>>().connect("");
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

pub fn session_decode(salt: &str, secret: &str, session_id: &str, max_age: Option<usize>) -> Result<String, SessionError> {
    let mut result = String::new();
    let mut parts: Vec<_> = session_id.rsplitn(2, ":").collect();
    let signature = parts[0];
    let data = parts[1];

    let data_signature = base64_hmac(salt, secret, data);
    if data_signature != signature {
        return Err(SessionError::SignatureInvalid);
    }

    parts = data.rsplitn(2, ":").collect();
    let timestamp = parts[0];
    let payload = parts[1];

    if max_age != None {
        let min_secs = time::get_time().sec - max_age.unwrap_or(0) as i64;
        if baseconv::b62_decode(timestamp) < min_secs {
            return Err(SessionError::SessionExpired);
        }
    }

    if payload.starts_with(".") {
        let b = match String::from_utf8(payload.as_bytes()[1..].to_vec()) {
            Ok(v) => v,
            Err(_) => return Err(SessionError::SessionInvalid),
        };
        let c = b64_decode(&b[..]);
        let mut r = ZlibDecoder::new(&c[..]);
        if let Err(_) = r.read_to_string(&mut result) {
            return Err(SessionError::SessionInvalid);
        }
    } else {
        result = match String::from_utf8(b64_decode(&payload[..])) {
            Ok(v) => v,
            Err(_) => return Err(SessionError::SessionInvalid),
        };
    }

    return Ok(result);
}


#[test]
fn test_session_decode() {
    let salt = DEFAULT_SALT;
    let secret = "6utnz%qfm$fs0legp)e@uxjlk-3hyo8sp4dc-y+x@z(!p=l@l9";
    let input = ".eJxVjrsSwiAQRf-F2kQgIWBKZ-y0s2cgbB4aQXk0Ov67ZEyT9u45e-8HmZuyg5NxesDbWUAtOiXvnrA_O2ucRTskVYqjTAG8HFUYM6F74MSIqtGE0w7zGvO-ogfWCE2ErjCvOAPQZCtr1d3Bmuz_O8vO2egnXS5IuV5DeXEG5uPKbh5M2SU5mbOd1LBsBVsMOlOzClF6eCUIMccUE1bgpsD8ikXL6pYy9P0Bm_FM9Q:1Z1WLZ:3jI2GHmVMInuElGvEsK7gVeR5io";
    let output = "{\"django_timezone\":\"Europe/London\",\"_auth_user_hash\":\"bfe71d836b172c07407f329568b18b307375eeb1\",\"_auth_user_backend\":\"django.contrib.auth.backends.ModelBackend\",\"_auth_user_id\":1,\"_language\":\"en-gb\",\"last_request\":\"2015-06-07T08:54:25\"}";

    assert_eq!(session_decode(salt, secret, input, None).unwrap(), output);

    assert_eq!(session_decode(salt, secret, input, Some(0)).err(), Some(SessionError::SessionExpired));

    assert_eq!(session_decode(salt, secret, input, Some(999999)).unwrap(), output);
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
