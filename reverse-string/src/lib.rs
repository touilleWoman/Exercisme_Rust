//do "hexdump -C tests/reverse-string.rs" to look at binary of "uüu" in the test
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}


