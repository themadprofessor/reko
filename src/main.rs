#![feature(ascii_ctype)]
#[macro_use] extern crate clap;
extern crate itertools;
#[cfg(test)] extern crate easybench;

use std::env;
use std::borrow::Cow;
use std::io;
use std::io::Write;
use std::ascii::AsciiExt;
use std::iter::Peekable;

use clap::Arg;
use itertools::Itertools;

#[cfg(test)] mod tests;

struct Config {
    newline: bool,
    interp: bool,
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("escape")
             .short("e")
             .help("enable interpretation of backslash escapes"))
        .arg(Arg::with_name("no_end")
             .short("n")
             .help("do not output the trailing newline"))
        .arg(Arg::with_name("STRING")
             .help("string(s) to be outputed")
             .multiple(true)).get_matches();
    let config = Config {
        interp: matches.is_present("escape"),
        newline: !matches.is_present("no_end")
    };

    if let Some(data) = matches.values_of_lossy("STRING") {
        let result = build_string(&config, &data);
        let result = if config.interp {
            escape(result.as_ref())
        } else {
            Cow::Borrowed(result)
        };
        let mut out = io::stdout();
        out.write_all(result.as_bytes()).expect("Failed to output data");
    }
}

fn build_string(config: &Config, args: &[String]) -> String {
    let mut tmp = String::with_capacity(args.iter().map(String::len).sum::<usize>() + args.len());
    for arg in args {
        if tmp.starts_with('$') {
            tmp += &env::var_os(&arg[1..]).unwrap_or_default().to_string_lossy()
        } else {
            tmp += &arg;
            tmp.push(' ');
        }
    }
    if config.newline {
        tmp.push('\n')
    }
    tmp
}

fn escape<'a, S: Into<Cow<'a, str>>>(s: S) -> Cow<'a, str> {
    let s = s.into();
    if s.contains('\\') {
        let mut buff = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(_) = chars.peek() {
            buff.extend(chars.take_while_ref(|c| *c != '\\'));
            match chars.next() {
                Some('\\') => buff.push('\\'),
                Some('a') => buff.push('\u{0007}'),
                Some('b') => buff.push('\u{0008}'),
                Some('c') => return Cow::Owned(buff),
                Some('e') => buff.push('\u{001B}'),
                Some('f') => buff.push('\u{000C}'),
                Some('n') => buff.push('\n'),
                Some('r') => buff.push('\r'),
                Some('t') => buff.push('\t'),
                Some('v') => buff.push('\u{000B}'),
                Some('0') => match parse_oct(&mut chars) {
                    Ok(c) => buff.push(c),
                    Err(s) => {buff.push_str("\\0"); buff.push_str(&s);}
                },
                Some('x') => match parse_hex(&mut chars) {
                    Ok(c) => buff.push(c),
                    Err(s) => {buff.push_str("\\x"); buff.push_str(&s);}
                },
                Some(c) => {buff.push('\\'); buff.push(c)},
                None => continue,
            }
        }
        Cow::Owned(buff)
    } else {
        s
    }
}

fn parse_hex<I>(chars: &mut Peekable<I>) -> std::result::Result<char, String> where I: Iterator<Item=char> {
    let mut buff = String::with_capacity(2);
    for _ in 0..2 {
        let res = if let Some(c) = chars.peek() {
            if c.is_ascii_hexdigit() {
                true
            } else {
                false
            }
        } else {
            false
        };
        if res {
            buff.push(chars.next().unwrap())
        } else {
            return Err(buff);
        }
    }

    if let Ok(num) = u8::from_str_radix(&buff, 16) {
        if num < 128 {
            return Ok(char::from(num));
        }
    }
    Err(buff)
}

fn parse_oct<I>(chars: &mut Peekable<I>) -> std::result::Result<char, String> where I: Iterator<Item=char> {
    let mut buff = String::with_capacity(2);
    for _ in 0..3 {
        let res = if let Some(c) = chars.peek() {
            if c >= &'0' || c <= &'7' {
                true
            } else {
                false
            }
        } else {
            false
        };
        if res {
            buff.push(chars.next().unwrap())
        } else {
            return Err(buff);
        }
    }

    if let Ok(num) = u8::from_str_radix(&buff, 8) {
        if num < 128 {
            return Ok(char::from(num));
        }
    }
    Err(buff)
}
