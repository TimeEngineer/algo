const P0: u16 = 0b000_0000_0000_0001;
const P1: u16 = 0b000_0000_0000_0010;
const P2: u16 = 0b000_0000_0000_1000;
const P3: u16 = 0b000_0000_1000_0000;
const P0MASK: u16 = 0b101_0101_0101_0100;
const P1MASK: u16 = 0b110_0110_0110_0100;
const P2MASK: u16 = 0b111_1000_0111_0000;
const P3MASK: u16 = 0b111_1111_0000_0000;
const P0PLUS: u16 = P0MASK | P0;
const P1PLUS: u16 = P1MASK | P1;
const P2PLUS: u16 = P2MASK | P2;
const P3PLUS: u16 = P3MASK | P3;

// --------------------
// Hamming Code 15 11 4
// --------------------
pub struct HammingCode15;

// --------------------
// Encode
// --------------------
impl HammingCode15 {
    fn encode_set_p0(byte: &mut u16) {
        let mask = *byte & P0MASK;
        *byte |= match mask.count_ones() {
            1 | 3 | 5 | 7 => P0,
            _ => 0,
        };
    }
    fn encode_set_p1(byte: &mut u16) {
        let mask = *byte & P1MASK;
        *byte |= match mask.count_ones() {
            1 | 3 | 5 | 7 => P1,
            _ => 0,
        };
    }
    fn encode_set_p2(byte: &mut u16) {
        let mask = *byte & P2MASK;
        *byte |= match mask.count_ones() {
            1 | 3 | 5 | 7 => P2,
            _ => 0,
        };
    }
    fn encode_set_p3(byte: &mut u16) {
        let mask = *byte & P3MASK;
        *byte |= match mask.count_ones() {
            1 | 3 | 5 | 7 => P3,
            _ => 0,
        };
    }
    fn encode_byte(byte: &mut u16) {
        *byte &= 0x7ff;

        // byte = da d9 d8 d7 d6 d5 d4
        //     p3 d3 d2 d1 p2 d0 p1 p0
        let byte0 = (*byte & 0b0001) << 2;
        let byte1 = (*byte & 0b1110) << 3;
        let byte2 = (*byte & 0x7f0) << 4;
        *byte = byte0 | byte1 | byte2;

        Self::encode_set_p0(byte);
        Self::encode_set_p1(byte);
        Self::encode_set_p2(byte);
        Self::encode_set_p3(byte);
    }
}

// --------------------
// Decode
// --------------------
impl HammingCode15 {
    fn decode_build_p0(byte: u16) -> u16 {
        let mask = byte & P0PLUS;
        match mask.count_ones() {
            1 | 3 | 5 | 7 => 0x1,
            _ => 0,
        }
    }
    fn decode_build_p1(byte: u16) -> u16 {
        let mask = byte & P1PLUS;
        match mask.count_ones() {
            1 | 3 | 5 | 7 => 0x2,
            _ => 0,
        }
    }
    fn decode_build_p2(byte: u16) -> u16 {
        let mask = byte & P2PLUS;
        match mask.count_ones() {
            1 | 3 | 5 | 7 => 0x4,
            _ => 0,
        }
    }
    fn decode_build_p3(byte: u16) -> u16 {
        let mask = byte & P3PLUS;
        match mask.count_ones() {
            1 | 3 | 5 | 7 => 0x8,
            _ => 0,
        }
    }
    fn decode_byte(byte: &mut u16) {
        let p0 = Self::decode_build_p0(*byte);
        let p1 = Self::decode_build_p1(*byte);
        let p2 = Self::decode_build_p2(*byte);
        let p3 = Self::decode_build_p3(*byte);
        let p = p0 | p1 | p2 | p3;

        let correction = 1 << (p as usize) >> 1;
        *byte ^= correction;

        let byte0 = (*byte & 0b000_0100) >> 2;
        let byte1 = (*byte & 0b111_0000) >> 3;
        let byte2 = (*byte & 0x7f00) >> 4;
        *byte = byte0 | byte1 | byte2;
    }
}

impl HammingCode15 {
    pub fn encode(bytes: &mut [u16]) {
        for byte in bytes.iter_mut() {
            Self::encode_byte(byte);
        }
    }
    pub fn decode(bytes: &mut [u16]) {
        for byte in bytes.iter_mut() {
            Self::decode_byte(byte);
        }
    }
}

#[test]
fn hc15() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let byte = rng.gen::<u16>() & 0x7ff;
    let mut bytes = [byte];

    HammingCode15::encode(&mut bytes);
    bytes[0] ^= 1 << (rng.gen::<usize>() % 15);
    HammingCode15::decode(&mut bytes);
    assert_eq!(bytes[0], byte);
}
