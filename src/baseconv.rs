
pub const BASE2_CHARS: &'static str = "012";
pub const BASE10_CHARS: &'static str = "0123456789";
pub const BASE16_CHARS: &'static str = "0123456789ABCDEF";
pub const BASE36_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";
pub const BASE56_CHARS: &'static str = "23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz";
pub const BASE62_CHARS: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const BASE64_CHARS: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

pub fn convert(number: &str, from_digits: String, to_digits: String) -> (bool, String) {
    let mut neg = false;
    let mut number = number;

    if number.starts_with("-") {
        number = number.trim_left_matches('-');
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

pub fn encode(s: &str, digits: String) -> String {
    let (neg, mut res) = convert(s, BASE10_CHARS.to_string(), digits);
    if neg {
        res = "-".to_string() + &res;
    }
    res
}

pub fn decode(s: &str, digits: String) -> i64 {
    let (neg, mut res) = convert(s, digits, BASE10_CHARS.to_string());
    if neg {
        res = "-".to_string() + &res;
    }
    res.to_string().parse().unwrap()
}

pub fn b62_decode(s: &str) -> i64 {
    println!("1 {}", s);
    let x = decode(s, BASE62_CHARS.to_string());
    println!("2 {}", x);
    x
}

#[test]
fn test_b62_convert() {
    assert_eq!(b62_decode("1Z1WLZ"), 1433667265);
    assert_eq!(encode("1433667265", BASE62_CHARS.to_string()), "1Z1WLZ");
}
