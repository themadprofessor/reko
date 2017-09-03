use easybench;

use super::*;

macro_rules! bench {
    () => {
        match ::std::env::var("RUST_BENCH") {
            Ok(num) => if num != "1" {
                return
            },
            Err(_) => return
        }
    };
}

macro_rules! test {
    () => {
        match ::std::env::var("RUST_BENCH") {
            Ok(num) => if num == "1" {
                return
            },
            Err(_) => {}
        }
    };
}

#[test]
fn bench_build_string_new() {
    bench!();
    let config = Config {
        interp: true,
        newline: true,
    };

    let args = vec!["Doot", "Deet", "Very Long String In Comparision", "Foo", "Bar", "$HOME"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

    println!("bench_build_string_new {}", easybench::bench(|| build_string(&config, &args)));
}

#[test]
fn bench_build_string_non() {
    bench!();
    let config = Config {
        interp: true,
        newline: false
    };

    let args = vec!["Doot", "Deet", "Very Long String In Comparision", "Foo", "Bar"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

    println!("bench_build_string_non {}", easybench::bench(|| build_string(&config, &args)));
}

#[test]
fn bench_escape() {
    bench!();
    let data = "SuperLong Co\\nnnected string to test \\\\escaping, \\0173, \\u7B";
    println!("bench_escape {}", easybench::bench(|| escape(data)));
}

#[test]
fn bench_escape_non() {
    bench!();
    let data = "SuperLong connected string to test escaping without escape codes";
    println!("bench_escape_non {}", easybench::bench(|| escape(data)))
}

#[test]
fn bench_parse_hex() {
    bench!();
    println!("bench_parse_hex {}", easybench::bench_env(vec!['7', 'B'].into_iter().peekable(),
                                              |mut data| parse_hex(&mut data)));
}

#[test]
fn bench_parse_oct() {
    bench!();
    println!("bench_parse_oct {}", easybench::bench_env(vec!['1', '7', '3'].into_iter().peekable(),
                                              |mut data| parse_oct(&mut data)));
}

#[test]
fn test_parse_hex_empty() {
    test!();
    let mut data = std::iter::empty().peekable();
    let res = parse_hex(&mut data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "")
}

#[test]
fn test_parse_oct_empty() {
    test!();
    let mut data = std::iter::empty().peekable();
    let res = parse_oct(&mut data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "")
}

#[test]
fn test_parse_hex_emptying() {
    test!();
    let mut data = vec!['7', 'B'].into_iter().peekable();
    let res = parse_hex(&mut data);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), '{');
    assert_eq!(data.len(), 0)
}

#[test]
fn test_parse_oct_emptying() {
    test!();
    let mut data = vec!['1', '7', '3'].into_iter().peekable();
    let res = parse_oct(&mut data);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), '{');
    assert_eq!(data.len(), 0)
}

#[test]
fn test_parse_hex_non_emptying() {
    test!();
    let mut data = vec!['7', 'B', '$'].into_iter().peekable();
    let res = parse_hex(&mut data);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), '{');
    assert_eq!(data.len(), 1);
    assert_eq!(data.next().unwrap(), '$');
}

#[test]
fn test_parse_oct_non_emptying() {
    test!();
    let mut data = vec!['1', '7', '3', '$'].into_iter().peekable();
    let res = parse_oct(&mut data);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), '{');
    assert_eq!(data.len(), 1);
    assert_eq!(data.next().unwrap(), '$');
}

#[test]
fn test_parse_hex_fail_size() {
    test!();
    let mut data = vec!['A', 'B'].into_iter().peekable();
    let res = parse_hex(&mut data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "AB")
}

#[test]
fn test_parse_hex_fail_invalid() {
    test!();
    let mut data = vec!['G', 'B'].into_iter().peekable();
    let res = parse_hex(&mut data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "")
}

#[test]
fn test_parse_oct_fail_size() {
    test!();
    let mut data = vec!['7', '7', '7'].into_iter().peekable();
    let res = parse_oct(&mut data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "777");
}

#[test]
fn test_parse_oct_fail_invalid() {
    test!();
    let mut data = vec!['8', '7', '3'].into_iter().peekable();
    let res = parse_oct(&mut data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "");
}