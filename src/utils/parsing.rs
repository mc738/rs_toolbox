
pub(crate) fn is_char(actual: char, excepted: char) -> bool {
     actual == excepted
}

pub(crate) fn is_chars(actual: Vec<char>, expected: Vec<char>) -> bool {
    actual == expected
}

//pub(crate) fn read