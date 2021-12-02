pub fn solve(input: &str) {
    println!("{}", count_triangles(&parse_horizontal(input)));
    println!("{}", count_triangles(&parse_vertical(input)));
}

fn parse_horizontal(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|nums| nums.len() == 3)
        .map(|nums| (nums[0], nums[1], nums[2]))
        .collect()
}

fn parse_vertical(input: &str) -> Vec<(u32, u32, u32)> {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    let mut vec_c = Vec::new();

    for (a, b, c) in parse_horizontal(input) {
        vec_a.push(a);
        vec_b.push(b);
        vec_c.push(c);
    }

    let mut vec = Vec::new();

    for i in (0..vec_a.len()).step_by(3) {
        vec.push((vec_a[i], vec_a[i + 1], vec_a[i + 2]));
        vec.push((vec_b[i], vec_b[i + 1], vec_b[i + 2]));
        vec.push((vec_c[i], vec_c[i + 1], vec_c[i + 2]));
    }

    vec
}

fn count_triangles(nums: &[(u32, u32, u32)]) -> usize {
    nums.iter().filter(|x| is_triangle(**x)).count()
}

fn is_triangle((a, b, c): (u32, u32, u32)) -> bool {
    if a < b {
        if b < c {
            a + b > c
        } else {
            a + c > b
        }
    } else if a < c {
        a + b > c
    } else {
        b + c > a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "5 10 25";
        assert_eq!(0, count_triangles(&parse_horizontal(input)));
    }

    #[test]
    fn example_parse_vert() {
        let input = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
        let triangles = parse_vertical(input);
        assert_eq!(6, triangles.len());
        assert!(triangles.contains(&(101, 102, 103)));
        assert!(triangles.contains(&(201, 202, 203)));
        assert!(triangles.contains(&(301, 302, 303)));
        assert!(triangles.contains(&(401, 402, 403)));
        assert!(triangles.contains(&(501, 502, 503)));
        assert!(triangles.contains(&(601, 602, 603)));
    }
}
