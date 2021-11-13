pub fn main(input: &str) {
    let addresses = parse(input);
    println!("Part 1: {}", part_one(&addresses));
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|x| !x.is_empty()).collect()
}

fn part_one(addresses: &[&str]) -> usize {
    addresses.iter().filter(|x| is_tls(x)).count()
}

fn is_tls(ip: &str) -> bool {
    let mut contains_in_supernet = false;

    for part in separate(ip).iter() {
        match part {
            IPPart::Supernet(x) => {
                if is_abba(x) {
                    contains_in_supernet = true;
                }
            }
            IPPart::Hypernet(x) => {
                if is_abba(x) {
                    return false;
                }
            }
        }
    }

    contains_in_supernet
}

fn is_abba(word: &str) -> bool {
    let chars = word.chars().collect::<Vec<char>>();

    for i in 0..word.len() {
        if i + 3 >= word.len() {
            break;
        }

        if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }

    false
}

fn separate(ip: &str) -> Vec<IPPart> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut is_supernet = true;

    for c in ip.chars() {
        match c {
            '[' => {
                result.push(IPPart::Supernet(current.clone()));
                current = String::new();
                is_supernet = false;
            }
            ']' => {
                result.push(IPPart::Hypernet(current.clone()));
                current = String::new();
                is_supernet = true;
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        let part = if is_supernet {
            IPPart::Supernet(current.clone())
        } else {
            IPPart::Hypernet(current.clone())
        };
        result.push(part);
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
enum IPPart {
    Supernet(String),
    Hypernet(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_abba_works() {
        assert!(is_abba("abba"));
        assert!(is_abba("xyyx"));
        assert!(is_abba("xxabba"));
        assert!(!is_abba(""));
        assert!(!is_abba("abc"));
        assert!(!is_abba("aaaa"));
        assert!(!is_abba("xyxx"));
        assert!(!is_abba("abcd"));
    }

    #[test]
    fn separate_works() {
        assert_eq!(
            separate("abba[mnop]qrst"),
            vec![
                IPPart::Supernet("abba".to_string()),
                IPPart::Hypernet("mnop".to_string()),
                IPPart::Supernet("qrst".to_string())
            ]
        );
    }

    #[test]
    fn is_tls_works() {
        assert!(is_tls("abba[mnop]qrst"));
        assert!(!is_tls("abcd[bddb]xyyx"));
        assert!(!is_tls("aaaa[qwer]tyui"));
        assert!(is_tls("ioxxoj[asdfgh]zxcvbn"));
    }
}
