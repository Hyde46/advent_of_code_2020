use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(
    regex = r#"^(?=.*ecl:(?Pecl<ecl>.*))(?=.*pid:*)(?=.*eyr:*)(?=.*hcl:*)(?=.*iyr:*)(?=.*byr:*)(?=.*hgt:*).*$"#
)]
struct Passport {
    ecl: String,
    pid: usize,
    eyr: usize,
    hcl: String,
    iyr: usize,
    byr: usize,
    hgt: String,
}
