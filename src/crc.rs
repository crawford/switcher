// Copyright 2015 Alex Crawford
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// CRC polynomial taken from [Wikipedia][poly].
///
/// [poly]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check.
pub static CRC_POLYNOMIAL: u32 = 0x5D6DCB;

/// # Examples
///
/// ```
/// # use switcher::crc::calculate;
/// assert_eq!(calculate(&[0x41, 0x42, 0x43, 0x44]), 0xA7629E);
/// ```
///
/// # Proof of operation
///
/// | Parameter                          |    Value |
/// |:-----------------------------------|---------:|
/// | Polynomial (normal representation) | 0x5D6DCB |
/// | Message                            |     ABCD |
/// | Remainder                          | 0xA7629E |
///
/// ```text
/// 01000001010000100100001101000100 000000000000000000000000
///  1010111010110110111001011
/// 00010110000110010011000110000100 000000000000000000000000
///    1010111010110110111001011
/// 00000011110011111110110100110100 000000000000000000000000
///       1010111010110110111001011
/// 00000001011101010011011010100010 000000000000000000000000
///        1010111010110110111001011
/// 00000000001010000101101101101001 000000000000000000000000
///           1010111010110110111001 011
/// 00000000000000111111011011010000 011000000000000000000000
///               101011101011011011 1001011
/// 00000000000000010100110000001011 111101100000000000000000
///                10101110101101101 11001011
/// 00000000000000000001000101100110 001111010000000000000000
///                    1010111010110 110111001011
/// 00000000000000000000010010110000 111000011011000000000000
///                      10101110101 10110111001011
/// 00000000000000000000000111000101 010101101001110000000000
///                        101011101 0110110111001011
/// 00000000000000000000000010011000 001110110101011100000000
///                         10101110 10110110111001011
/// 00000000000000000000000000110110 100011011011001010000000
///                           101011 1010110110111001011
/// 00000000000000000000000000011101 001000000000101111100000
///                            10101 11010110110111001011
/// 00000000000000000000000000001000 111101101101011101010000
///                             1010 111010110110111001011
/// 00000000000000000000000000000010 000111011011100100001000
///                               10 10111010110110111001011
/// 00000000000000000000000000000000 101001110110001010011110
///                                  A   7   6   2   9   E
/// ```

pub fn calculate(data: &[u8]) -> u32 {
    crc(data.iter().chain(&[0, 0, 0]))
}

/// # Examples
///
/// ```
/// # use switcher::crc::is_valid;
/// assert_eq!(is_valid(&[0x41, 0x42, 0x43, 0x44, 0xA7, 0x62, 0x9E]), true);
/// ```
///
/// # Proof of operation
///
/// | Parameter                          |    Value |
/// |:-----------------------------------|---------:|
/// | Polynomial (normal representation) | 0x5D6DCB |
/// | Message                            |     ABCD |
/// | Remainder                          | 0xA7629E |
///
/// ```text
/// 01000001010000100100001101000100 101001110110001010011110
///  1010111010110110111001011
/// 00010110000110010011000110000100 101001110110001010011110
///    1010111010110110111001011
/// 00000011110011111110110100110100 101001110110001010011110
///       1010111010110110111001011
/// 00000001011101010011011010100010 101001110110001010011110
///        1010111010110110111001011
/// 00000000001010000101101101101001 101001110110001010011110
///           1010111010110110111001 011
/// 00000000000000111111011011010000 110001110110001010011110
///               101011101011011011 1001011
/// 00000000000000010100110000001011 010100010110001010011110
///                10101110101101101 11001011
/// 00000000000000000001000101100110 100110100110001010011110
///                    1010111010110 110111001011
/// 00000000000000000000010010110000 010001101101001010011110
///                      10101110101 10110111001011
/// 00000000000000000000000111000101 111100011111111010011110
///                        101011101 0110110111001011
/// 00000000000000000000000010011000 100111000011010110011110
///                         10101110 10110110111001011
/// 00000000000000000000000000110110 001010101101000000011110
///                           101011 1010110110111001011
/// 00000000000000000000000000011101 100001110110100101111110
///                            10101 11010110110111001011
/// 00000000000000000000000000001000 010100011011010111001110
///                             1010 111010110110111001011
/// 00000000000000000000000000000010 101110101101101110010110
///                               10 10111010110110111001011
/// 00000000000000000000000000000000 000000000000000000000000
/// ```

pub fn is_valid(data: &[u8]) -> bool {
    crc(data.iter()) == 0
}

// Calculate the sum of data passed through the the 24-bit CRC.
fn crc<'a, T: Iterator<Item = &'a u8>>(data: T) -> u32 {
    fn shift_left(val: u32) -> (u32, bool) {
        let carry = (val & (1 << 31)) != 0;
        (val << 1, carry)
    }

    // The actual CRC remainder is stored in the three most significant bytes
    // of crc. The least significant byte holds the next byte of the message to
    // be shifted through the CRC.
    let mut crc: u32 = 0;

    for byte in data {
        // Set up the next byte in the holding area...
        crc |= *byte as u32;

        // ...and shift it through the CRC (assuming an 8-bit byte).
        for _ in 0..8 {
            crc = match shift_left(crc) {
                (crc, false) => crc,
                (crc, true) => crc ^ (CRC_POLYNOMIAL << 8),
            };
        }
    }

    // Extract the remainder (assuming an 8-bit byte).
    crc >> 8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(&[]), 0);
        assert_eq!(calculate(&[1]), 0x5D6DCB);
        assert_eq!(calculate(&[1, 2, 3, 4]), 0x629D29);
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(&[]), true);
        assert_eq!(is_valid(&[1, 0x5D, 0x6D, 0xCB]), true);
        assert_eq!(is_valid(&[1, 2, 3, 4, 0x62, 0x9D, 0x29]), true);
        assert_eq!(is_valid(&[1, 2, 3, 5, 0x62, 0x9D, 0x29]), false);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_calculate(b: &mut Bencher) {
        b.iter(|| calculate(&[0xA5u8; 10_000]));
    }

    #[bench]
    fn bench_is_valid(b: &mut Bencher) {
        // TODO
        let mut data = vec![0xA5u8; 10_000];
        data.extend_from_slice(&[0x25u8, 0xC5u8, 0xFEu8]);
        b.iter(|| is_valid(data.as_slice()));
    }
}
