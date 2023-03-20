// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

fn recognize_digit(lines: &[&str]) -> Option<Digit> {
    match lines {
        [" _ ", "| |", "|_|", "   "] => Some(Digit::Zero),
        _ => None,
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.len() != 3 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
        lines.push(line.trim());
    }
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    let mut result = String::new();
    for i in (0..lines.len()).step_by(4) {
        let digit_lines = [lines[i], lines[i + 1], lines[i + 2], lines[i + 3]];
        match recognize_digit(&digit_lines) {
            Some(Digit::Zero) => result.push('0'),
            Some(Digit::One) => result.push('1'),
            Some(Digit::Two) => result.push('2'),
            Some(Digit::Three) => result.push('3'),
            Some(Digit::Four) => result.push('4'),
            Some(Digit::Five) => result.push('5'),
            Some(Digit::Six) => result.push('6'),
            Some(Digit::Seven) => result.push('7'),
            Some(Digit::Eight) => result.push('8'),
            Some(Digit::Nine) => result.push('9'),
            None => result.push('?'),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::ocr_numbers as ocr;

    #[test]
    fn input_with_lines_not_multiple_of_four_is_error() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "| |\n" +
                "   ";
        assert_eq!(Err(ocr::Error::InvalidRowCount(3)), ocr::convert(&input));
    }
    #[test]
    fn input_with_columns_not_multiple_of_three_is_error() {
        #[rustfmt::skip]
    let input = "    \n".to_string() +
                "   |\n" +
                "   |\n" +
                "    ";
        assert_eq!(Err(ocr::Error::InvalidColumnCount(4)), ocr::convert(&input));
    }
    #[test]
    fn unrecognized_characters_return_question_mark() {
        #[rustfmt::skip]
    let input = "   \n".to_string() +
                "  _\n" +
                "  |\n" +
                "   ";
        assert_eq!(Ok("?".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_0() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "| |\n" +
                "|_|\n" +
                "   ";
        assert_eq!(Ok("0".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_1() {
        #[rustfmt::skip]
    let input = "   \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";
        assert_eq!(Ok("1".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_2() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                " _|\n" +
                "|_ \n" +
                "   ";
        assert_eq!(Ok("2".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_3() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                " _|\n" +
                " _|\n" +
                "   ";
        assert_eq!(Ok("3".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_4() {
        #[rustfmt::skip]
    let input = "   \n".to_string() +
                "|_|\n" +
                "  |\n" +
                "   ";
        assert_eq!(Ok("4".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_5() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_ \n" +
                " _|\n" +
                "   ";
        assert_eq!(Ok("5".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_6() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_ \n" +
                "|_|\n" +
                "   ";
        assert_eq!(Ok("6".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_7() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";
        assert_eq!(Ok("7".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_8() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_|\n" +
                "|_|\n" +
                "   ";
        assert_eq!(Ok("8".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_9() {
        #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_|\n" +
                " _|\n" +
                "   ";
        assert_eq!(Ok("9".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_110101100() {
        #[rustfmt::skip]
    let input = "       _     _        _  _ \n".to_string() +
                "  |  || |  || |  |  || || |\n" +
                "  |  ||_|  ||_|  |  ||_||_|\n" +
                "                           ";
        assert_eq!(Ok("110101100".to_string()), ocr::convert(&input));
    }
    #[test]
    fn replaces_only_garbled_numbers_with_question_mark() {
        #[rustfmt::skip]
    let input = "       _     _           _ \n".to_string() +
                "  |  || |  || |     || || |\n" +
                "  |  | _|  ||_|  |  ||_||_|\n" +
                "                           ";
        assert_eq!(Ok("11?10?1?0".to_string()), ocr::convert(&input));
    }
    #[test]
    fn recognizes_string_of_decimal_numbers() {
        #[rustfmt::skip]
    let input = "    _  _     _  _  _  _  _  _ \n".to_string() +
                "  | _| _||_||_ |_   ||_||_|| |\n" +
                "  ||_  _|  | _||_|  ||_| _||_|\n" +
                "                              ";
        assert_eq!(Ok("1234567890".to_string()), ocr::convert(&input));
    }
    #[test]
    fn numbers_across_multiple_lines_are_joined_by_commas() {
        #[rustfmt::skip]
    let input = "    _  _ \n".to_string() +
                "  | _| _|\n" +
                "  ||_  _|\n" +
                "         \n" +
                "    _  _ \n" +
                "|_||_ |_ \n" +
                "  | _||_|\n" +
                "         \n" +
                " _  _  _ \n" +
                "  ||_||_|\n" +
                "  ||_| _|\n" +
                "         ";
        assert_eq!(Ok("123,456,789".to_string()), ocr::convert(&input));
    }
}
