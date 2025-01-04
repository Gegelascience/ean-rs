
/// Check if the given string is a correct EAN-8 or EAN-13
/// 
///  # Examples
/// ```
/// use ean_rs::is_correct_ean;
/// 
/// assert_eq!(is_correct_ean("3666154117284"),true);
/// 
/// ```
/// 
pub fn is_correct_ean(ean_test: &str) -> bool {
    
    // check length

    let test_length = ean_test.len();
    if test_length != 13 && test_length != 8 {
      return false;  
    }

    for c in ean_test.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    // calculate digit

    let len_withoutdigit = test_length-1;

    let digit = calculate_digit_check(&ean_test[..len_withoutdigit]);
    
    
    if ean_test[..test_length].chars().last().unwrap().to_digit(10).unwrap() != digit {
        return false;
    }

    return true;
}

/// Calculate the last digit of an ean
fn calculate_digit_check(ean_digit_check_less:&str) -> u32 {
    

    let test_length = ean_digit_check_less.len();
    let mut factor = 3;
    let mut somme = 0;

    for c in ean_digit_check_less.chars() {
        if !c.is_numeric() {
            panic!("Incorrect characters");
        }
    }

    let mut i: i32 = test_length as i32 -1;
    let step: i32 = -1;
    let end: i32 = -1;

    while i > end {
        somme += ean_digit_check_less.chars().nth(i.try_into().unwrap()).unwrap().to_digit(10).unwrap() * factor;
        factor = 4 - factor;
        i += step;
    }

    let digit_check = (10 - (somme % 10))%10;

    return digit_check;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_good_ean_13() {
        assert_eq!(is_correct_ean("3666154117284"),true);
    }

    #[test]
    fn test_bad_ean_13() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(is_correct_ean("366615411728"), false);
    }

    #[test]
    fn test_good_ean_8() {
        assert_eq!(is_correct_ean("12345670"),true);
    }

    #[test]
    fn test_bad_ean_8() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(is_correct_ean("12345678"), false);
    }
}