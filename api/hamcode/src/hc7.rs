// --------------------
// Hamming Code 7 4 3
// --------------------
pub struct HammingCode7;

// --------------------
// Encode
// --------------------
impl HammingCode7 {
    fn encode_set_p0(byte: &mut u8) {
        // p0 = 0b000_0001
        let d0 = (*byte & 0b000_0100) >> 2;
        let d1 = (*byte & 0b001_0000) >> 4;
        let d3 = (*byte & 0b100_0000) >> 6;
        *byte |= d0 ^ d1 ^ d3;
    }
    fn encode_set_p1(byte: &mut u8) {
        // p1 = 0b000_0010
        let d0 = (*byte & 0b000_0100) >> 1;
        let d2 = (*byte & 0b010_0000) >> 4;
        let d3 = (*byte & 0b100_0000) >> 5;
        *byte |= d0 ^ d2 ^ d3;
    }
    fn encode_set_p2(byte: &mut u8) {
        // p2 = 0b000_1000
        let d1 = (*byte & 0b001_0000) >> 1;
        let d2 = (*byte & 0b010_0000) >> 2;
        let d3 = (*byte & 0b100_0000) >> 3;
        *byte |= d1 ^ d2 ^ d3;
    }
    fn encode_byte(byte: &mut u8) {
        *byte &= 0xf;

        // byte = d3 d2 d1 p2 d0 p1 p0
        let byte0 = (*byte & 0b0001) << 2;
        let byte1 = (*byte & 0b1110) << 3;
        *byte = byte0 | byte1;

        Self::encode_set_p0(byte);
        Self::encode_set_p1(byte);
        Self::encode_set_p2(byte);
    }
}

// --------------------
// Decode
// --------------------
impl HammingCode7 {
    fn decode_build_p0(byte: u8) -> u8 {
        // p0 = 0b000_0001
        let d0 = (byte & 0b000_0100) >> 2;
        let d1 = (byte & 0b001_0000) >> 4;
        let d3 = (byte & 0b100_0000) >> 6;
        let p0 = (byte & 0b000_0001) >> 0;
        p0 ^ d0 ^ d1 ^ d3
    }
    fn decode_build_p1(byte: u8) -> u8 {
        // p1 = 0b000_0010
        let d0 = (byte & 0b000_0100) >> 1;
        let d2 = (byte & 0b010_0000) >> 4;
        let d3 = (byte & 0b100_0000) >> 5;
        let p1 = (byte & 0b000_0010) >> 0;
        p1 ^ d0 ^ d2 ^ d3
    }
    fn decode_build_p2(byte: u8) -> u8 {
        // p2 = 0b000_0100
        let d1 = (byte & 0b001_0000) >> 2;
        let d2 = (byte & 0b010_0000) >> 3;
        let d3 = (byte & 0b100_0000) >> 4;
        let p2 = (byte & 0b000_1000) >> 1;
        p2 ^ d1 ^ d2 ^ d3
    }
    fn decode_byte(byte: &mut u8) {
        let p0 = Self::decode_build_p0(*byte);
        let p1 = Self::decode_build_p1(*byte);
        let p2 = Self::decode_build_p2(*byte);
        let p = p0 | p1 | p2;

        let correction = 1 << (p as usize) >> 1;
        *byte ^= correction;

        let byte0 = (*byte & 0b000_0100) >> 2;
        let byte1 = (*byte & 0b111_0000) >> 3;
        *byte = byte0 | byte1;
    }
}

impl HammingCode7 {
    pub fn encode(bytes: &mut [u8]) {
        for byte in bytes.iter_mut() {
            Self::encode_byte(byte);
        }
    }
    pub fn decode(bytes: &mut [u8]) {
        for byte in bytes.iter_mut() {
            Self::decode_byte(byte);
        }
    }
}

#[test]
fn hc7() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let byte = rng.gen::<u8>() & 0xf;
    let mut bytes = [byte];

    HammingCode7::encode(&mut bytes);
    bytes[0] ^= 1 << (rng.gen::<usize>() % 7);
    HammingCode7::decode(&mut bytes);
    assert_eq!(bytes[0], byte);
}
