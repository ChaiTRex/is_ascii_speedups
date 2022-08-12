#![feature(const_trait_impl, const_slice_index)]

// This macro creates part of a function that has a list of strips of consecutive characters
// as long as there is at most one strip in each chunk of 32 characters (character codes
// 0 to 31, 32 to 63, etc.).
macro_rules! is_in_a_strip_fn {
    ($x: ident, $x_type: ty, $starting_chars: expr, $char_counts: expr) => {{
        // 32-character chunk number.
        let chunk_number = ($x as u8 & 0b1110_0000).wrapping_shr(5) as usize;

        // This `u64` is secretly a `[u8; 8]`.
        const STARTING_CHARS: u64 = u64::from_le_bytes($starting_chars);
        // Subtract the starting `char` of this chunk from the input `char`. This will
        // make sure the matching `char`s are in
        // `0..number_of_valid_chars_in_this_chunk`.
        let x =
            $x.wrapping_sub(
                *unsafe { u64::to_le_bytes(STARTING_CHARS).get_unchecked(chunk_number) } as u8
                    as $x_type,
            );

        // This `u64` is secretly a `[u8; 8]`.
        const CHAR_COUNTS: u64 = u64::from_le_bytes($char_counts);
        // Check whether the adjusted value of the input `char` is in
        // `0..number_of_valid_chars_in_this_chunk`.
        x < *unsafe { u64::to_le_bytes(CHAR_COUNTS).get_unchecked(chunk_number) } as u8 as $x_type
    }};
}

pub trait IsAscii2 {
    fn is_ascii_2(&self) -> bool;
    fn is_ascii_alphabetic_2(&self) -> bool;
    fn is_ascii_alphanumeric_2(&self) -> bool;
    fn is_ascii_control_2(&self) -> bool;
    fn is_ascii_digit_2(&self) -> bool;
    fn is_ascii_graphic_2(&self) -> bool;
    fn is_ascii_hexdigit_2(&self) -> bool;
    fn is_ascii_lowercase_2(&self) -> bool;
    fn is_ascii_punctuation_2(&self) -> bool;
    fn is_ascii_uppercase_2(&self) -> bool;
    fn is_ascii_whitespace_2(&self) -> bool;
}

impl const IsAscii2 for u8 {
    #[must_use]
    #[inline]
    fn is_ascii_2(&self) -> bool {
        *self < 128
    }

    #[must_use]
    #[inline]
    fn is_ascii_alphabetic_2(&self) -> bool {
        let x = *self & 0b1101_1111;
        x <= b'Z' && x >= b'A'
    }

    #[must_use]
    #[inline]
    fn is_ascii_alphanumeric_2(&self) -> bool {
        let x = *self;
        is_in_a_strip_fn!(
            x,
            u8,
            [0, b'0', b'A', b'a', 0, 0, 0, 0],
            [0, 10, 26, 26, 0, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_control_2(&self) -> bool {
        let x = *self;
        x < b' ' || x == 127
    }

    #[must_use]
    #[inline]
    fn is_ascii_digit_2(&self) -> bool {
        let x = *self;
        x <= b'9' && x >= b'0'
    }

    #[must_use]
    #[inline]
    fn is_ascii_graphic_2(&self) -> bool {
        let x = *self;
        x <= b'~' && x >= b'!'
    }

    #[must_use]
    #[inline]
    fn is_ascii_hexdigit_2(&self) -> bool {
        let x = *self;
        is_in_a_strip_fn!(
            x,
            u8,
            [0, b'0', b'A', b'a', 0, 0, 0, 0],
            [0, 10, 6, 6, 0, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_lowercase_2(&self) -> bool {
        let x = *self;
        x <= b'z' && x >= b'a'
    }

    #[must_use]
    #[inline]
    fn is_ascii_punctuation_2(&self) -> bool {
        let x = (*self).wrapping_add(6);
        is_in_a_strip_fn!(
            x,
            u8,
            [0, b'!' + 6, b':' + 6, b'[' + 6, b'{' + 6, 0, 0, 0],
            [0, 15, 7, 6, 4, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_uppercase_2(&self) -> bool {
        let x = *self;
        x <= b'Z' && x >= b'A'
    }

    #[must_use]
    #[inline]
    fn is_ascii_whitespace_2(&self) -> bool {
        let x = *self;
        x <= b' ' && (0b100000000000000000011011000000000_u64.wrapping_shr(x as u32) & 1) != 0
    }
}

impl const IsAscii2 for char {
    #[must_use]
    #[inline]
    fn is_ascii_2(&self) -> bool {
        *self <= '\x7F'
    }

    #[must_use]
    #[inline]
    fn is_ascii_alphabetic_2(&self) -> bool {
        let x = (*self as u32) & 0b1111_1111_1111_1111_1111_1111_1101_1111;
        x <= 90 && x >= 65
    }

    #[must_use]
    #[inline]
    fn is_ascii_alphanumeric_2(&self) -> bool {
        let x = *self as u32;
        is_in_a_strip_fn!(
            x,
            u32,
            [0, b'0', b'A', b'a', 0, 0, 0, 0],
            [0, 10, 26, 26, 0, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_control_2(&self) -> bool {
        let x = *self;
        x < ' ' || x == '\x7F'
    }

    #[must_use]
    #[inline]
    fn is_ascii_digit_2(&self) -> bool {
        let x = *self;
        x <= '9' && x >= '0'
    }

    #[must_use]
    #[inline]
    fn is_ascii_graphic_2(&self) -> bool {
        let x = *self;
        x <= '~' && x >= '!'
    }

    #[must_use]
    #[inline]
    fn is_ascii_hexdigit_2(&self) -> bool {
        let x = *self as u32;
        is_in_a_strip_fn!(
            x,
            u32,
            [0, b'0', b'A', b'a', 0, 0, 0, 0],
            [0, 10, 6, 6, 0, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_lowercase_2(&self) -> bool {
        let x = *self;
        x <= 'z' && x >= 'a'
    }

    #[must_use]
    #[inline]
    fn is_ascii_punctuation_2(&self) -> bool {
        let x = (*self as u32).wrapping_add(6);
        is_in_a_strip_fn!(
            x,
            u32,
            [0, b'!' + 6, b':' + 6, b'[' + 6, b'{' + 6, 0, 0, 0],
            [0, 15, 7, 6, 4, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_uppercase_2(&self) -> bool {
        let x = *self;
        x <= 'Z' && x >= 'A'
    }

    #[must_use]
    #[inline]
    fn is_ascii_whitespace_2(&self) -> bool {
        let x = *self;
        x <= ' ' && (0b100000000000000000011011000000000_u64.wrapping_shr(x as u32) & 1) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::IsAscii2;

    #[test]
    fn ascii_u8() {
        for x in 0..=u8::MAX {
            assert!(x.is_ascii() == x.is_ascii_2(), "Failed on {}", x);
        }
    }

    #[test]
    fn alphabetic_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_alphabetic() == x.is_ascii_alphabetic_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn alphanumeric_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_alphanumeric() == x.is_ascii_alphanumeric_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn control_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_control() == x.is_ascii_control_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn digit_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_digit() == x.is_ascii_digit_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn graphic_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_graphic() == x.is_ascii_graphic_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn hexdigit_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_hexdigit() == x.is_ascii_hexdigit_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn lowercase_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_lowercase() == x.is_ascii_lowercase_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn punctuation_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_punctuation() == x.is_ascii_punctuation_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn uppercase_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_uppercase() == x.is_ascii_uppercase_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn whitespace_u8() {
        for x in 0..=u8::MAX {
            assert!(
                x.is_ascii_whitespace() == x.is_ascii_whitespace_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn ascii_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(x.is_ascii() == x.is_ascii_2(), "Failed on {}", x);
        }
    }

    #[test]
    fn alphabetic_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_alphabetic() == x.is_ascii_alphabetic_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn alphanumeric_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_alphanumeric() == x.is_ascii_alphanumeric_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn control_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_control() == x.is_ascii_control_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn digit_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_digit() == x.is_ascii_digit_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn graphic_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_graphic() == x.is_ascii_graphic_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn hexdigit_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_hexdigit() == x.is_ascii_hexdigit_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn lowercase_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_lowercase() == x.is_ascii_lowercase_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn punctuation_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_punctuation() == x.is_ascii_punctuation_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn uppercase_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_uppercase() == x.is_ascii_uppercase_2(),
                "Failed on {}",
                x
            );
        }
    }

    #[test]
    fn whitespace_char() {
        for x in ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}') {
            assert!(
                x.is_ascii_whitespace() == x.is_ascii_whitespace_2(),
                "Failed on {}",
                x
            );
        }
    }
}
