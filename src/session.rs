
use time;
use std::io::Read;
use flate2::read::ZlibDecoder;
use rustc_serialize::json;

use baseconv;
use signing;

#[derive(Debug, PartialEq)]
pub enum SessionError {
    SessionInvalid,
    SignatureInvalid,
    SessionExpired,
}

pub fn validate(salt: &str, secret: &str, session_id: &str, max_age: Option<usize>) -> Result<String, SessionError> {
    let mut result = String::new();
    let mut parts: Vec<_> = session_id.rsplitn(2, ":").collect();
    let signature = parts[0];
    let data = parts[1];

    let data_signature = signing::base64_hmac(salt, secret, data);
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
        let c = signing::b64_decode(&b[..]);
        let mut r = ZlibDecoder::new(&c[..]);
        if let Err(_) = r.read_to_string(&mut result) {
            return Err(SessionError::SessionInvalid);
        }
    } else {
        result = match String::from_utf8(signing::b64_decode(&payload[..])) {
            Ok(v) => v,
            Err(_) => return Err(SessionError::SessionInvalid),
        };
    }

    return Ok(result);
}

pub fn get_user_id(payload: &str) -> Option<usize> {
    let session_json = match json::Json::from_str(payload) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let session_obj = match session_json.as_object() {
        Some(v) => v,
        None => return None,
    };

    match session_obj.get("_auth_user_id") {
        Some(v) => match v.as_u64() {
            Some(v) => Some(v as usize),
            None => None,
        },
        None => None,
    }
}


#[test]
fn test_session() {
    let salt = signing::DEFAULT_SALT;
    let secret = "6utnz%qfm$fs0legp)e@uxjlk-3hyo8sp4dc-y+x@z(!p=l@l9";
    let input = ".eJxVjrsSwiAQRf-F2kQgIWBKZ-y0s2cgbB4aQXk0Ov67ZEyT9u45e-8HmZuyg5NxesDbWUAtOiXvnrA_O2ucRTskVYqjTAG8HFUYM6F74MSIqtGE0w7zGvO-ogfWCE2ErjCvOAPQZCtr1d3Bmuz_O8vO2egnXS5IuV5DeXEG5uPKbh5M2SU5mbOd1LBsBVsMOlOzClF6eCUIMccUE1bgpsD8ikXL6pYy9P0Bm_FM9Q:1Z1WLZ:3jI2GHmVMInuElGvEsK7gVeR5io";
    let output = "{\"django_timezone\":\"Europe/London\",\"_auth_user_hash\":\"bfe71d836b172c07407f329568b18b307375eeb1\",\"_auth_user_backend\":\"django.contrib.auth.backends.ModelBackend\",\"_auth_user_id\":1,\"_language\":\"en-gb\",\"last_request\":\"2015-06-07T08:54:25\"}";

    assert_eq!(validate(salt, secret, input, None).unwrap(), output);

    assert_eq!(validate(salt, secret, input, Some(0)).err(), Some(SessionError::SessionExpired));

    assert_eq!(validate(salt, secret, input, Some(999999)).unwrap(), output);

    let session = validate(salt, secret, input, Some(999999)).unwrap();
    assert_eq!(get_user_id(&session), Some(1));
}
