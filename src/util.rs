pub fn set(word: u32, mask: u32) -> u32 {
    return word | mask;
}

pub fn clear(word: u32, mask: u32) -> u32 {
    return word & !mask;
}

/// Extract bits in range [low, high)
pub fn bits(x: u32, low: u32, high: u32) -> u32 {
    assert!(high > low);
    assert!(low <= 32);
    assert!(high <= 32);
    let mask: u64 = (1 << (high - low)) - 1;
    return (x >> low) & (mask as u32);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set() {
        let a = 0b11111010111111010111101101011101;
        let b = 0b00110000111101010111101101011101;
        let c = 0b11001111000010101000010010100010;
        assert!(set(0, a) == a);
        assert!(set(b, a) == a);
        assert!(set(b, c) == !0);
    }

    #[test]
    fn test_clear() {
        let a = 0b11111010111111010111101101011101;
        let b = 0b00110000111100010111101101000100;
        let c = 0b11001010000011000000000000011001;
        assert!(clear(a, a) == 0);
        assert!(clear(0, a) == 0);
        assert!(clear(a, b) == c);
    }

    #[test]
    fn test_bits() {
        let x = 0b1110_0101_0001_0110_0011_0100_0000_1101;
        assert!(bits(x, 0, 32)  == x);
        assert!(bits(x, 0, 1) == 1);
        assert!(bits(x, 2, 10) == 0b00000011);
        assert!(bits(x, 0, 20) == 0b01100011010000001101);
        assert!(bits(x, 11, 27) == 0b1010001011000110);
    }
}
