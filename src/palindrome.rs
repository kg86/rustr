/// Checks whether a given string is a palindrome.
pub fn is_palindrome<T>(text: &[T]) -> bool
where
    T: PartialEq,
{
    for i in 0..text.len() / 2 {
        if text[i] != text[text.len() - i - 1] {
            return false;
        }
    }
    true
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(br"a"));
    assert!(is_palindrome(br"aa"));
    assert!(is_palindrome(br"aaa"));
    assert!(is_palindrome(br"aba"));
    assert!(is_palindrome(br"abba"));
    assert!(!is_palindrome(br"ab"));
    assert!(!is_palindrome(br"abb"));
    assert!(!is_palindrome(br"aaba"));
}
