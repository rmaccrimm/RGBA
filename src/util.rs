pub fn set(word: u32, mask: u32) -> u32 {
    return word | mask;
}

pub fn clear(word: u32, mask: u32) -> u32 {
    return word & !mask;
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
}
