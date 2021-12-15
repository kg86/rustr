use crate::bstr::*;

/// Checks whether a given string is a palindrome.
pub fn is_palindrome(text: &bstr) -> bool {
    for i in 0..text.len() / 2 {
        if text[i] != text[text.len() - i - 1] {
            return false;
        }
    }
    return true;
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(str2bstr("a")));
    assert!(is_palindrome(str2bstr("aa")));
    assert!(is_palindrome(str2bstr("aaa")));
    assert!(is_palindrome(str2bstr("aba")));
    assert!(is_palindrome(str2bstr("abba")));
    assert!(!is_palindrome(str2bstr("ab")));
    assert!(!is_palindrome(str2bstr("abb")));
    assert!(!is_palindrome(str2bstr("aaba")));
}
