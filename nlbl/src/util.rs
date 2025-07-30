use crate::{Error, Result};

//format_dollars(): takes an amount of cents and formats it to ${X}+.XX
pub fn format_dollars(cents: &i32) -> String {
    let cents = { cents.to_string() };
    let dollars = match cents.len() {
        3.. => cents.split_at(cents.len() - 2),
        2 => ("0", cents.as_str()),
        1 => ("0", &format!("0{cents}")[..]),
        _ => ("0", "00"),
    };
    let mut output = String::from("$");
    output.push_str(dollars.0);
    output.push('.');
    output.push_str(dollars.1);
    output.to_string()
}

//TODO: DOLLAR TRAIT FOR STRINGS AND f32???

//dollars_to_cents(): takes a decimal amount of dollars and returns it in integer cents
pub fn dollars_to_cents(dollars: f32) -> i32 {
    (dollars * 100.0) as i32
}

//parse_dollar_string(): takes a string literal and returns an integer cent amount if valid, or error message if not
pub fn parse_dollar_string(s: &str) -> Result<i32> {
    if s.is_empty() {
        return Err(Error::InvalidDollarValue(s.into()));
    }
    let mut s = s;
    if s.starts_with('$') {
        s = &s[1..];
    }
    match s.parse::<i32>() {
        Ok(n) => Ok(n * 100),
        Err(_) => match s.parse::<f32>() {
            Ok(m) => Ok(dollars_to_cents(m)),
            Err(_) => Err(Error::InvalidDollarValue(s.into())),
        },
    }
}

//to_title_case(): takes a String and returns a new String with the first letter uppercase, and the rest lowercase
pub fn to_title_case(s: String) -> String {
    let mut out = s;
    if let Some(r) = out.get_mut(0..1) {
        if r == "*" {
            if let Some(s) = out.get_mut(1..2) {
                s.make_ascii_uppercase();
            }
        } else {
            r.make_ascii_uppercase();
        }
    }
    out.clone()
}
