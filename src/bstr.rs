use std::collections::HashSet;

/// Byte string.
pub type BString = Vec<u8>;

/// Byte string slice.
#[allow(non_camel_case_types)]
pub type bstr = [u8];

/// Converts a string slice to a byte string slice.
pub fn str2bstr(text: &str) -> &bstr {
    text.as_bytes()
}

/// Converts a string slice to a byte string.
pub fn str2bstring(text: &str) -> BString {
    text.as_bytes().to_vec()
}

/// Converts a byte string slice to a string.
pub fn bstr2string(bstr: &bstr) -> String {
    String::from_utf8(bstr.to_vec()).expect("must be parsed.")
}

/// Converts byte strings to a string.
pub fn bstr2str_sli(bstrs: &[&bstr]) -> String {
    let strings: Vec<_> = bstrs.iter().map(|x| bstr2string(x)).collect();
    format!("{:?}", strings)
}

/// Converts byte strings to a string.
pub fn bstrs2string(bstrs: &[BString]) -> String {
    let strings: Vec<_> = bstrs.iter().map(|x| bstr2string(x)).collect();
    format!("{:?}", strings)
}

/// Converts byte strings to a string.
pub fn bstrs2string_set(bstrs: &HashSet<BString>) -> String {
    let mut strings: Vec<_> = bstrs.iter().map(|x| bstr2string(x)).collect();
    strings.sort();
    format!("{:?}", strings)
}

pub fn bstr2bstring(bstr: &bstr) -> BString {
    bstr.to_vec()
}
