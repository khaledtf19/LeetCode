fn main() {
    let res = number_of_beams(vec![
        "011001".to_string(),
        "000000".to_string(),
        "010100".to_string(),
        "001000".to_string(),
    ]);
    println!("{res}");
}

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut prev_num = 0;
    let mut result: i32 = 0;
    for row in bank.iter() {
        let current = row.chars().filter(|ch| *ch == '1').count() as i32;
        match current {
            0 => {}
            current => {
                result += current * prev_num;
                prev_num = current;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_lasers1() {
        assert_eq!(
            number_of_beams(vec![
                "011001".to_string(),
                "000000".to_string(),
                "010100".to_string(),
                "001000".to_string(),
            ]),
            8
        )
    }

    #[test]
    fn test_number_of_lasers2() {
        assert_eq!(
            number_of_beams(vec![
                "000".to_string(),
                "111".to_string(),
                "000".to_string(),
            ]),
            0
        )
    }
}
