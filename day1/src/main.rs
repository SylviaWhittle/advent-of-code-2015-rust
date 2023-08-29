use std::fs;

pub fn main() -> std::io::Result<()> {
    println!("~ Day 1 ~");

    let mut data = fs::read_to_string("./src/day1.txt")?;
    data = data.replace("\n", "");
    let floor = count_perentheses(&data);
    println!("floor: {floor}");

    Ok(())
}

fn count_perentheses(string: &str) -> i32 {
    let mut num: i32 = 0;

    for c in string.chars() {
        if c == '(' {
            num += 1;
        } else if c == ')' {
            num -= 1;
        }
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_count_perentheses() {
        assert_eq!(count_perentheses(&String::from("(())")), 0);
        assert_eq!(count_perentheses(&String::from("(((")), 3);
        assert_eq!(count_perentheses(&String::from(")))")), -3);
    }
}
