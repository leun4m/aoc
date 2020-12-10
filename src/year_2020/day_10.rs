pub fn main(input: &str) {
    let adapters = parse_input(input);
    let (difference_1, difference_3) = calculate_differences(&adapters);
    let mut product: i64 = 1;
    calculate_difference_vec(&adapters)
        .split(|&x| x == 3)
        .for_each(|x| {
            let a = x.len();
            if a > 1 {
                product *= match a {
                    2 => 2,
                    3 => 4,
                    4 => 7,
                    5 => 11,
                    _ => panic!("unexpected a: {}", a),
                }
            }
        });

    println!(
        "Difference: {} {} {}",
        difference_1,
        difference_3,
        difference_1 * difference_3
    );
    println!("Part TWO: {}", product);
}

fn calculate_difference_vec(adapters: &Vec<u64>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut previous = 0;
    for adapter in adapters {
        let diff = adapter - previous;
        result.push(diff as u8);
        previous = *adapter;
    }
    result
}

fn calculate_differences(adapters: &Vec<u64>) -> (i32, i32) {
    let mut difference_1 = 0;
    let mut difference_3 = 1;
    let mut previous = 0;
    for adapter in adapters {
        let diff = adapter - previous;
        if diff == 3 {
            difference_3 += 1
        } else if diff == 1 {
            difference_1 += 1
        }
        previous = *adapter;
    }
    (difference_1, difference_3)
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut adapters = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    adapters.sort();
    adapters
}

// #[cfg(test)]
// mod test {
//     use super::*;

// #[test]
// fn permutations_test() {
//     let a = arrangements(2);
//     println!("{:?}", a);
//     assert_eq!(4, a.len());
//     assert!(a.contains(&vec!(false, false)));
//     assert!(a.contains(&vec!(false, true)));
//     assert!(a.contains(&vec!(true, false)));
//     assert!(a.contains(&vec!(true, true)));
// }
// }
