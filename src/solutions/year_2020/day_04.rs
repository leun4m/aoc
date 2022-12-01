pub fn solve(input: &str) {
    let mut passports_valid_simple = 0;
    let mut passports_valid_advanced = 0;
    for passport_data in input.split("\n\n") {
        let mut passport = Passport::new();
        for pairs in passport_data.split_whitespace() {
            let key_value = pairs.split(':').collect::<Vec<&str>>();
            match *key_value.first().unwrap() {
                "byr" => passport.byr = key_value[1],
                "iyr" => passport.iyr = key_value[1],
                "eyr" => passport.eyr = key_value[1],
                "hgt" => passport.hgt = key_value[1],
                "hcl" => passport.hcl = key_value[1],
                "ecl" => passport.ecl = key_value[1],
                "pid" => passport.pid = key_value[1],
                "cid" => passport.cid = key_value[1],
                x => panic!("Unexpected key {}", x),
            }
        }

        if passport.is_valid_simple() {
            passports_valid_simple += 1;

            if passport.is_valid() {
                passports_valid_advanced += 1;
            }
        }
    }

    println!("Valid Passwords (1): {}", passports_valid_simple);
    println!("Valid Passwords (2): {}", passports_valid_advanced);
}

struct Passport<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: &'a str,
}

impl<'a> Passport<'a> {
    fn new() -> Self {
        Passport {
            byr: "",
            iyr: "",
            eyr: "",
            hgt: "",
            hcl: "",
            ecl: "",
            pid: "",
            cid: "",
        }
    }

    fn is_valid_simple(&self) -> bool {
        !self.byr.is_empty()
            && !self.iyr.is_empty()
            && !self.eyr.is_empty()
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }

    fn is_valid(&self) -> bool {
        Self::is_num_between(self.byr, 1920, 2002)
            && Self::is_num_between(self.iyr, 2010, 2020)
            && Self::is_num_between(self.eyr, 2020, 2030)
            && self.is_valid_height()
            && self.is_valid_hair_color()
            && self.is_valid_eye_color()
            && self.is_valid_pid()
    }

    fn is_num_between(num: &str, min: u32, max: u32) -> bool {
        match num.parse::<u32>() {
            Ok(x) => min <= x && x <= max,
            Err(_) => false,
        }
    }

    fn is_valid_height(&self) -> bool {
        if self.hgt.ends_with("cm") {
            Self::is_num_between(&self.hgt.replace("cm", ""), 150, 193)
        } else if self.hgt.ends_with("in") {
            Self::is_num_between(&self.hgt.replace("in", ""), 59, 76)
        } else {
            false
        }
    }

    fn is_valid_hair_color(&self) -> bool {
        self.hcl.starts_with('#') && u64::from_str_radix(&self.hcl.replace('#', ""), 16).is_ok()
    }

    fn is_valid_eye_color(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl)
    }

    fn is_valid_pid(&self) -> bool {
        self.pid.chars().count() == 9 && self.pid.parse::<u32>().is_ok()
    }
}
