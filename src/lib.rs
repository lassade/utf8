use core::marker::PhantomData;

// https://tools.ietf.org/html/rfc3629
const UTF8_CHAR_WIDTH: &[u8; 256] = &[
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];

#[inline]
pub const fn utf8_char_len(x: u8) -> usize {
    // we could reduce the code size by using `x.leading_ones() | 1` but is ofcourse slower than the LUT
    UTF8_CHAR_WIDTH[x as usize] as usize
}

// #[inline]
// pub const fn utf8_char_len_branch(x: u8) -> usize {
//     // this is even faster than the LUT? in some cases is what is looking like, but not in all
//     if x < 128 {
//         return 1;
//     }
//
//     if x >= 0xE0 {
//         if x >= 0xF0 {
//             return 4;
//         }
//         return 3;
//     }
//
//     return 2;
// }

#[rustfmt::skip]
const UTF8_CHAR_BYTE1_WHITESPACE: [u8; 256] = [
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, // 0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 4
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 5
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 6
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // C
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // D
    0, 4, 8 + 16, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // E
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];

#[rustfmt::skip]
const UTF8_CHAR_BYTE2_WHITESPACE: [u8; 64] = [
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    40, 16, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, // 1
     2,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
];

#[rustfmt::skip]
const UTF8_CHAR_BYTE3_WHITESPACE: [u8; 64] = [
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    46, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 2, 2, 2, 2,  2, // 0
     2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2, 2, 2, 2, 2, 18, // 1
     2,  2,  2,  2,  2,  2,  2,  2, 10, 10,  2, 2, 2, 2, 2, 10, // 2
     2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2, 2, 2, 2, 2,  2, // 3
];

#[inline]
pub const fn utf8_char_is_whitespace(x: u8, y: u8, z: u8) -> bool {
    let mask = UTF8_CHAR_BYTE1_WHITESPACE[x as usize];
    if mask == 0 {
        // most common case is not to be a whitespace
        return false;
    } else if x < 127 {
        // mask isn't zero and char is ascii
        return true;
    }

    // each one of LUTs below encodes one branch of the code bellow as a bit filed
    //
    // if x == 194 {
    //     return y == 133 || y == 160; // 2
    // } else if x == 225 {
    //     return y == 154 && z == 128; // 4
    // } else if x == 226 {
    //     if y == 128 {
    //         return (128 <= z && z <= 138) || z == 168 || z == 169 || z == 175; // 8
    //     } else if y == 129 {
    //         return z == 159; // 16
    //     }
    // } else if x == 227 {
    //     return y == 128 && z == 128; // 32
    // } else {
    //    false
    // }

    let mask = UTF8_CHAR_BYTE2_WHITESPACE[(y & !0xC0) as usize] & mask;
    if mask == 0 {
        return false;
    }

    let mask = UTF8_CHAR_BYTE3_WHITESPACE[(z & !0xC0) as usize] & mask;
    return mask != 0;
}

// #[inline]
// unsafe fn utf8_ltrim_str(v: &[u8]) -> &[u8] {
//     if v.len() == 0 {
//         return v;
//     }

//     let mut ptr = v.as_ptr();
//     let ptr_end = ptr.add(v.len());

//     loop {
//         if ptr >= ptr_end {
//             break;
//         }
//         let offset = utf8_char_len(*ptr) as usize;

//         // todo: maybe theres is a better way?
//         let mut bytes = [0u8; 4];
//         core::ptr::copy_nonoverlapping(ptr, bytes.as_mut_ptr(), offset);
//         if !utf8_char_is_whitespace(bytes[0], bytes[1], bytes[2]) {
//             break;
//         }

//         ptr = ptr.add(offset);
//     }

//     core::slice::from_raw_parts(
//         ptr,
//         ptr_end.offset_from(ptr) as usize, // todo: use sub_ptr once it get stable
//     )
// }

// pub fn ltrim_str(v: &str) -> &str {
//     // safety: `v` is a valid utf8 `str`
//     unsafe { core::str::from_utf8_unchecked(utf8_ltrim_str(v.as_bytes())) }
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(4))]
pub struct Utf8Char {
    pub bytes: [u8; 4],
}

impl Utf8Char {
    #[inline]
    pub const fn from_str(v: &str) -> Self {
        if v.len() == 0 {
            return Self {
                bytes: [0, 0, 0, 0],
            };
        }

        // safety: `v` isn't empty and is a valid `str` so `len` is within bounds
        unsafe {
            let ptr = v.as_ptr();

            let x = *ptr;
            // most common case
            if x < 0x80 {
                return Self {
                    bytes: [x, 0, 0, 0],
                };
            }

            let y = *ptr.add(1);
            if x >= 0xE0 {
                let z = *ptr.add(2);
                if x >= 0xF0 {
                    let w = *ptr.add(3);
                    return Self {
                        bytes: [x, y, z, w],
                    };
                }

                return Self {
                    bytes: [x, y, z, 0],
                };
            }

            return Self {
                bytes: [x, y, 0, 0],
            };
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        utf8_char_len(self.bytes[0])
    }

    #[inline]
    pub const fn is_whitespace(&self) -> bool {
        utf8_char_is_whitespace(self.bytes[0], self.bytes[1], self.bytes[2])
    }
}

// impl PartialEq for Utf8Char {
//     fn eq(&self, other: &Self) -> bool {
//         self.bytes == other.bytes
//     }
// }

// impl PartialEq<str> for Utf8Char {
//     fn eq(&self, other: &str) -> bool {

//     }
// }

pub struct Utf8Chars<'a> {
    ptr: *const u8,
    ptr_end: *const u8,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Utf8Chars<'a> {
    pub const fn from_str(v: &'a str) -> Self {
        Self {
            ptr: v.as_ptr(),
            ptr_end: unsafe { v.as_ptr().add(v.len()) },
            _marker: PhantomData,
        }
    }

    pub fn ltrim(&mut self) {
        todo!()
        // // safety: `bytes` are created from a valid `str`
        // self.bytes = unsafe { utf8_ltrim_str(self.bytes) };
    }
}

impl<'a> Iterator for Utf8Chars<'a> {
    type Item = Utf8Char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.ptr_end {
            return None;
        }

        // safety: `bytes` are created from a valid `str` so the `offset` is always within bounds
        unsafe {
            let x = *self.ptr;
            self.ptr = self.ptr.offset(1);

            // most common case
            if x < 0x80 {
                return Some(Utf8Char {
                    bytes: [x, 0, 0, 0],
                });
            }

            let y = *self.ptr;
            self.ptr = self.ptr.offset(1);

            if x >= 0xE0 {
                let z = *self.ptr;
                self.ptr = self.ptr.offset(1);

                if x >= 0xF0 {
                    let w = *self.ptr;
                    self.ptr = self.ptr.offset(1);
                    return Some(Utf8Char {
                        bytes: [x, y, z, w],
                    });
                }

                return Some(Utf8Char {
                    bytes: [x, y, z, 0],
                });
            }

            return Some(Utf8Char {
                bytes: [x, y, 0, 0],
            });
        }
    }
}

// impl<'a> Chars<'a> {
//     #[inline(always)]
//     pub fn cursor(&self) -> Cursor {
//         Cursor {
//             ptr: self.bytes.as_ptr(),
//         }
//     }

//     /// SAFETY: The cursor must be created from the same [`Utf8Iter`]
//     #[inline(always)]
//     pub unsafe fn sub_str_from_cursor(&self, cursor: Cursor) -> &'a str {
//         core::str::from_utf8_unchecked(core::slice::from_raw_parts(
//             cursor.ptr,
//             self.bytes.as_ptr().offset_from(cursor.ptr) as _,
//         ))
//     }
// }

#[cfg(test)]
mod tests {

    #[test]
    fn utf8_char_is_len() {
        let mut bytes = [0u8; 4];
        for c in 0..=std::char::MAX as u32 {
            if let Some(c) = std::char::from_u32(c) {
                c.encode_utf8(&mut bytes[..]);
                assert_eq!(
                    c.len_utf8(),
                    super::utf8_char_len(bytes[0]),
                    "{:?} {:?}",
                    c,
                    &bytes
                );
            }
        }
    }

    #[test]
    fn utf8_char_is_whitespace() {
        let mut bytes = [0u8; 4];
        for c in 0..=std::char::MAX as u32 {
            if let Some(c) = std::char::from_u32(c) {
                c.encode_utf8(&mut bytes[..]);
                assert_eq!(
                    c.is_whitespace(),
                    super::utf8_char_is_whitespace(bytes[0], bytes[1], bytes[2]),
                    "{:?} {:?}",
                    c,
                    &bytes
                );
            }
        }
    }
}
