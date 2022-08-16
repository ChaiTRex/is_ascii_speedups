#![no_std]
#![feature(const_slice_index, const_trait_impl)]

// This macro creates part of a function that handles up to eight strips of consecutive
// matching codepoints. The strips must all be in separate 32-codepoint chunks
// (codepoints 0 to 31, 32 to 63, 64 to 95, 96 to 127, 128 to 159, 160 to 191,
// 192 to 223, or 224 to 255).
macro_rules! handle_strip_of_each_chunk {
    ($x: ident, $x_type: ty, $starting_codepoints: expr, $strip_lengths: expr) => {{
        // 32-codepoint chunk number.
        let chunk_number = ($x as u8 >> 5) as usize;

        // `const` to type check and to ensure all element evaluations are done at
        // compile time
        const STARTING_CODEPOINTS: [u8; 8] = $starting_codepoints;
        // Subtract the starting codepoint of this chunk from the input codepoint. This
        // will make sure that the matching codepoints in this strip are in
        // `0..length_of_strip`.
        let x = $x.wrapping_sub(STARTING_CODEPOINTS[chunk_number] as $x_type);

        // `const` to type check and to ensure all element evaluations are done at
        // compile time
        const STRIP_LENGTHS: [u8; 8] = $strip_lengths;
        // Check whether the adjusted value of the input codepoint is in
        // `0..length_of_strip`.
        x < STRIP_LENGTHS[chunk_number] as $x_type
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
        // `| 0b0010_0000` loses one bit of information, giving exactly two possible
        // inputs for every output. The exact two possible inputs for outputs `b'a'`
        // through `b'z'` are the lowercase and uppercase versions of each letter, so we
        // only need to check whether it's a lowercase letter.
        let x = (*self | 0b0010_0000).wrapping_sub(b'a');
        x < 26
    }

    #[must_use]
    #[inline]
    fn is_ascii_alphanumeric_2(&self) -> bool {
        let x = *self;
        handle_strip_of_each_chunk!(
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
        let x = self.wrapping_sub(b'0');
        x < 10
    }

    #[must_use]
    #[inline]
    fn is_ascii_graphic_2(&self) -> bool {
        let x = self.wrapping_sub(b'!');
        x < 94
    }

    #[must_use]
    #[inline]
    fn is_ascii_hexdigit_2(&self) -> bool {
        let x = *self;
        handle_strip_of_each_chunk!(
            x,
            u8,
            [0, b'0', b'A', b'a', 0, 0, 0, 0],
            [0, 10, 6, 6, 0, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_lowercase_2(&self) -> bool {
        let x = self.wrapping_sub(b'a');
        x < 26
    }

    #[must_use]
    #[inline]
    fn is_ascii_punctuation_2(&self) -> bool {
        // Add 6 to the codepoint so that each strip of consecutive matching codepoints
        // is in a separate 32-codepoint chunk. For more details, see the comment above
        // the `handle_strip_of_each_chunk` macro definition.
        let x = self.wrapping_add(6);
        handle_strip_of_each_chunk!(
            x,
            u8,
            [0, b'!' + 6, b':' + 6, b'[' + 6, b'{' + 6, 0, 0, 0],
            [0, 15, 7, 6, 4, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_uppercase_2(&self) -> bool {
        let x = self.wrapping_sub(b'A');
        x < 26
    }

    #[must_use]
    #[inline]
    fn is_ascii_whitespace_2(&self) -> bool {
        // The long binary number has bit indexes starting at 0 on the right and going
        // leftward until it ends at bit index 32. The bit index corresponds to the
        // codepoint of the input `u8`. The value of the bit there is 1 iff the `u8` is
        // an ASCII whitespace codepoint.
        let x = *self;
        x <= b' ' && ((0b1_0000_0000_0000_0000_0011_0110_0000_0000_u64 >> x) & 1) != 0
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
        // `| 0b0010_0000` loses one bit of information, giving exactly two possible
        // inputs for every output. The exact two possible inputs for outputs `'a'`
        // through `'z'` are the lowercase and uppercase versions of each letter, so we
        // only need to check whether it's a lowercase letter.
        let x = ((*self as u32) | 0b0010_0000).wrapping_sub('a' as u32);
        x < 26
    }

    #[must_use]
    #[inline]
    fn is_ascii_alphanumeric_2(&self) -> bool {
        let x = *self as u32;
        handle_strip_of_each_chunk!(
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
        let x = (*self as u32).wrapping_sub('0' as u32);
        x < 10
    }

    #[must_use]
    #[inline]
    fn is_ascii_graphic_2(&self) -> bool {
        let x = (*self as u32).wrapping_sub('!' as u32);
        x < 94
    }

    #[must_use]
    #[inline]
    fn is_ascii_hexdigit_2(&self) -> bool {
        let x = *self as u32;
        handle_strip_of_each_chunk!(
            x,
            u32,
            [0, b'0', b'A', b'a', 0, 0, 0, 0],
            [0, 10, 6, 6, 0, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_lowercase_2(&self) -> bool {
        let x = (*self as u32).wrapping_sub('a' as u32);
        x < 26
    }

    #[must_use]
    #[inline]
    fn is_ascii_punctuation_2(&self) -> bool {
        // Add 6 to the codepoint so that each strip of consecutive matching codepoints
        // is in a separate 32-codepoint chunk. For more details, see the comment above
        // the `handle_strip_of_each_chunk` macro definition.
        let x = (*self as u32).wrapping_add(6);
        handle_strip_of_each_chunk!(
            x,
            u32,
            [0, b'!' + 6, b':' + 6, b'[' + 6, b'{' + 6, 0, 0, 0],
            [0, 15, 7, 6, 4, 0, 0, 0]
        )
    }

    #[must_use]
    #[inline]
    fn is_ascii_uppercase_2(&self) -> bool {
        let x = (*self as u32).wrapping_sub('A' as u32);
        x < 26
    }

    #[must_use]
    #[inline]
    fn is_ascii_whitespace_2(&self) -> bool {
        // The long binary number has bit indexes starting at 0 on the right and going
        // leftward until it ends at bit index 32. The bit index corresponds to the
        // codepoint of the input `char`. The value of the bit there is 1 iff the `char`
        // is an ASCII whitespace codepoint.
        let x = *self as u32;
        x <= ' ' as u32 && ((0b1_0000_0000_0000_0000_0011_0110_0000_0000_u64 >> x) & 1) != 0
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
