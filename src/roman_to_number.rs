// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two ones added together.
// 12 is written as XII, which is simply X + II.
// The number 27 is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right.
// However, the numeral for four is not IIII. Instead, the number four is written as IV.
// Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX.

// There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.

// Given a roman numeral, convert it to an integer.

/// Calculates the value to add depending on the position of the Roman numeral in the input string.
///
/// # Arguments
///
/// * `roman_numbers` - A reference to a vector of characters representing the Roman numeral.
/// * `position` - The position of the current Roman numeral in the vector.
/// * `is_before` - A tuple representing the Roman numerals that can appear after the current numeral.
/// * `decimal_to_add` - The decimal value to add based on the position and the Roman numerals after it.
///
/// # Returns
///
/// The decimal value to add based on the position and the Roman numerals before it.
fn calculate_value_depending_on_position(
    roman_numerals: &Vec<char>,
    position: usize,
    after_numeral: (char, char),
    decimal_to_add: i32,
) -> i32 {
    let (first, second) = after_numeral;

    if position == roman_numerals.len() - 1 {
        return decimal_to_add;
    }

    if roman_numerals[position + 1] == first || roman_numerals[position + 1] == second {
        return -decimal_to_add;
    };

    decimal_to_add
}

/// Converts a Roman numeral to an integer.
///
/// # Arguments
///
/// * `roman_number` - A string representing the Roman numeral.
///
/// # Returns
///
/// The integer value of the Roman numeral.
pub fn roman_to_int(roman_numeral: String) -> i32 {
    let vector: Vec<char> = roman_numeral.chars().collect();

    let mut sum = 0;

    for (i, digit) in vector.iter().enumerate() {
        let to_sum = match digit {
            'I' => calculate_value_depending_on_position(&vector, i, ('V', 'X'), 1),
            'V' => 5,
            'X' => calculate_value_depending_on_position(&vector, i, ('L', 'C'), 10),
            'L' => 50,
            'C' => calculate_value_depending_on_position(&vector, i, ('D', 'M'), 100),
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        sum += to_sum;
    }

    sum
}

pub fn roman_to_int_alternative(roman_numeral: String) -> i32 {
    let vector: Vec<char> = roman_numeral
        .replace("IV", "IIII")
        .replace("IX", "VIIII")
        .replace("XL", "XXXX")
        .replace("XC", "LXXXX")
        .replace("CD", "CCCC")
        .replace("CM", "DCCCC")
        .chars()
        .collect();

    let mut sum = 0;

    for digit in vector.iter() {
        let to_sum = match digit {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        sum += to_sum;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{roman_to_int, roman_to_int_alternative};

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("IX")), 9);
        assert_eq!(roman_to_int(String::from("XL")), 40);
        assert_eq!(roman_to_int(String::from("XC")), 90);
        assert_eq!(roman_to_int(String::from("CD")), 400);
        assert_eq!(roman_to_int(String::from("CM")), 900);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn test_roman_to_int_alternative() {
        assert_eq!(roman_to_int_alternative(String::from("III")), 3);
        assert_eq!(roman_to_int_alternative(String::from("IV")), 4);
        assert_eq!(roman_to_int_alternative(String::from("IX")), 9);
        assert_eq!(roman_to_int_alternative(String::from("XL")), 40);
        assert_eq!(roman_to_int_alternative(String::from("XC")), 90);
        assert_eq!(roman_to_int_alternative(String::from("CD")), 400);
        assert_eq!(roman_to_int_alternative(String::from("CM")), 900);
        assert_eq!(roman_to_int_alternative(String::from("MCMXCIV")), 1994);
    }
}
