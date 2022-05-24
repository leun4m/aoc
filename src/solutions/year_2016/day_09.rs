pub fn solve(input: &str) {
    let decompressed = decompress(input);
    println!("Part 1: {}", decompressed.len())
}

fn sanatize(input: &str) -> String {
    let mut result = input.to_string();
    result = result.replace('\n', "");
    result = result.replace('\r', "");
    result = result.replace('\t', "");
    result = result.replace(' ', "");
    result
}

fn decompress(input: &str) -> String {
    let mut result = String::new();

    let s = sanatize(input);
    let mut chars = s.chars();
    while let Some(x) = chars.next() {
        match x {
            '(' => {
                let mut num = String::new();
                let mut run = true;
                while run {
                    if let Some(x) = chars.next() {
                        if x.is_digit(10) {
                            num.push(x);
                        } else {
                            run = false;
                        }
                    }
                }

                let mut num2 = String::new();
                run = true;
                while run {
                    if let Some(x) = chars.next() {
                        if x.is_digit(10) {
                            num2.push(x);
                        } else {
                            run = false;
                        }
                    }
                }

                let count = num.parse().unwrap();
                let times = num2.parse().unwrap();

                let mut substr = String::new();
                for _ in 0..count {
                    substr.push(chars.next().expect("Unexpected ending of input"));
                }

                for _ in 0..times {
                    result.push_str(&substr);
                }
            }
            c => result.push(c),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_works() {
        assert_eq!(&decompress("ADVENT"), "ADVENT");
        assert_eq!(&decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(&decompress("(3x3)XYZ"), "XYZXYZXYZ");
    }
}
