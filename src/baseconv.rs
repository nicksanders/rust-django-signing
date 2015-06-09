
pub const BASE10_CHARS: &'static str = "0123456789";

pub enum Base {
    Base2,
    Base10,
    Base16,
    Base36,
    Base56,
    Base62,
    Base64
}

pub struct BaseConv {
    chars: &'static str,
    sign: &'static str
}

impl BaseConv {

    pub fn new(base: Base) -> BaseConv {
        BaseConv{chars: BaseConv::get_chars(&base), sign: BaseConv::get_sign(&base)}
    }

    pub fn new_custom(chars: &'static str, sign: &'static str) -> BaseConv {
        BaseConv{chars: chars, sign: sign}
    }

    pub fn get_chars(base: &Base) -> &'static str {
        match *base {
            Base::Base2 => "012",
            Base::Base10 => BASE10_CHARS,
            Base::Base16 => "0123456789ABCDEF",
            Base::Base36 => "0123456789abcdefghijklmnopqrstuvwxyz",
            Base::Base56 => "23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz",
            Base::Base62 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            Base::Base64 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"
        }
    }

    pub fn get_sign(base: &Base) -> &'static str {
        match *base {
            Base::Base64 => "$",
            _            => "-"
        }
    }

    pub fn convert(&self, number: &str, from_digits: String, to_digits: String, sign: &str) -> (bool, String) {
        let mut neg = false;
        let mut number = number;

        if number.starts_with(sign) {
            number = number.trim_left_matches(sign);
            neg = true;
        }

        let mut res = String::new();
        let mut x: u32 = 0;
        for digit in number.chars() {
            x = x * from_digits.len() as u32 + from_digits.find(digit).unwrap() as u32;
        }

        if x == 0 {
            res = to_digits.chars().nth(0).unwrap().to_string();
        } else {
            while x > 0 {
                let digit = x % to_digits.len() as u32;
                res = to_digits.chars().nth(digit as usize).unwrap().to_string() + &res;
                x = x / to_digits.len() as u32;
            }
        }

        (neg, res)
    }

    pub fn encode(&self, s: &str) -> String {
        let (neg, mut res) = self.convert(s, BASE10_CHARS.to_string(), self.chars.to_string(), "-");
        if neg {
            res = self.sign.to_string() + &res;
        }
        res
    }

    pub fn decode(&self, s: &str) -> i64 {
        let (neg, mut res) = self.convert(s, self.chars.to_string(), BASE10_CHARS.to_string(), self.sign);
        if neg {
            res = "-".to_string() + &res;
        }
        res.to_string().parse().unwrap()
    }
}

#[test]
fn test_base62() {
    assert_eq!(BaseConv::new(Base::Base62).decode("1Z1WLZ"), 1433667265);
    assert_eq!(BaseConv::new(Base::Base62).encode("1433667265"), "1Z1WLZ");
}

#[test]
fn test_base64() {
    assert_eq!(BaseConv::new(Base::Base64).decode("$1Z1WLZ"), -1661338979);
    assert_eq!(BaseConv::new(Base::Base64).encode("-1661338979"), "$1Z1WLZ");
}

#[test]
fn test_custom() {
    let baseconv = BaseConv::new_custom("0123456789-", "$");
    assert_eq!(baseconv.encode("-1234"), "$-22");
    assert_eq!(baseconv.decode("$-22"), -1234);
}
