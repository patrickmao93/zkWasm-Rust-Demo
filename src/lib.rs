use std::ops::{Add, Mul, Rem};

use num_bigint::BigUint;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let data = vec![
        BigUint::from_bytes_be(b"A"),
        BigUint::from_bytes_be(b"B"),
        BigUint::from_bytes_be(b"C"),
    ];
    let mimc = Mimc17::new();
    let h = mimc.hash(&data);
    0
}

pub struct Mimc17 {
    fr: BigUint,
}

impl Mimc17 {
    fn new() -> Mimc17 {
        Mimc17 { fr: get_fr() }
    }

    fn hash(&self, m: &Vec<BigUint>) -> BigUint {
        let mut s = BigUint::default();

        for x in m {
            let mut res = self.encrypt(&s, &x);
            res = self.add(&res, &s);
            s = self.add(&res, &x);
        }

        return s;
    }

    fn encrypt(&self, s: &BigUint, m: &BigUint) -> BigUint {
        let mut x = m.clone();
        let constants = get_constants();

        for c in constants {
            x = self.add(&x, &s);
            x = self.add(&x, &c);
            x = self.pow17(&x);
        }
        return self.add(&x, s);
    }

    fn add(&self, a: &BigUint, b: &BigUint) -> BigUint {
        return a.add(b).rem(&self.fr);
    }

    fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        return a.mul(b).rem(&self.fr);
    }

    fn pow17(&self, x: &BigUint) -> BigUint {
        let mut r = self.mul(x, x);
        r = self.mul(&r, &r);
        r = self.mul(&r, &r);
        r = self.mul(&r, &r);
        return self.mul(&r, &x);
    }
}

fn get_fr() -> BigUint {
    let f = [
        0x12, 0xab, 0x65, 0x5e, 0x9a, 0x2c, 0xa5, 0x56, 0x60, 0xb4, 0x4d, 0x1e, 0x5c, 0x37, 0xb0,
        0x01, 0x59, 0xaa, 0x76, 0xfe, 0xd0, 0x00, 0x00, 0x01, 0x0a, 0x11, 0x80, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];
    return BigUint::from_bytes_be(&f);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_str() {
//         let fr = get_fr();
//         let frbits = fr.bits();
//         println!("{} {}", &fr, &frbits);
//     }
// }

fn get_constants() -> Vec<BigUint> {
    return vec![
        BigUint::from_bytes_be(&[
            0x2e, 0x2e, 0xbb, 0xb1, 0x78, 0x29, 0x6b, 0x63, 0xd8, 0x8e, 0xc1, 0x98, 0xf0, 0x97,
            0x6a, 0xd9, 0x8b, 0xc1, 0xd4, 0xeb, 0x0d, 0x92, 0x1d, 0xdd, 0x2e, 0xb8, 0x6c, 0xb7,
            0xe7, 0x0a, 0x98, 0xe5,
        ]),
        BigUint::from_bytes_be(&[
            0x21, 0xbf, 0xc1, 0x54, 0xb5, 0xb0, 0x71, 0xd2, 0x2d, 0x06, 0x10, 0x56, 0x63, 0x55,
            0x38, 0x01, 0xf8, 0x58, 0xc1, 0xf2, 0x31, 0x02, 0x0b, 0x4c, 0x29, 0x1a, 0x72, 0x9d,
            0x62, 0x81, 0xd3, 0x49,
        ]),
        BigUint::from_bytes_be(&[
            0x12, 0x6c, 0xfa, 0x35, 0x2b, 0x0e, 0x27, 0x01, 0x44, 0x2b, 0x36, 0xe0, 0xc2, 0xfc,
            0x88, 0x28, 0x7c, 0xfd, 0x3b, 0xfe, 0xcc, 0xe8, 0x42, 0xaf, 0xc0, 0xe3, 0xe7, 0x8d,
            0x8e, 0xdb, 0x4a, 0xd8,
        ]),
        BigUint::from_bytes_be(&[
            0x03, 0x09, 0xd7, 0x06, 0x7a, 0xb6, 0x5d, 0xe1, 0xa9, 0x9f, 0xe2, 0x3f, 0x45, 0x8d,
            0x0b, 0xc3, 0xf1, 0x8c, 0x59, 0xb6, 0x64, 0x2e, 0xf4, 0x8a, 0xfc, 0x67, 0x9e, 0xf1,
            0x7c, 0xb6, 0x92, 0x8c,
        ]),
        BigUint::from_bytes_be(&[
            0x19, 0x4c, 0x46, 0x93, 0x40, 0x99, 0x66, 0x96, 0x0b, 0xe8, 0x85, 0x13, 0xcf, 0xe3,
            0x29, 0x87, 0xc1, 0x25, 0xf7, 0x13, 0x98, 0xa7, 0x82, 0xe4, 0x49, 0x73, 0xfb, 0x8a,
            0xf4, 0x79, 0x8b, 0xd8,
        ]),
        BigUint::from_bytes_be(&[
            0x05, 0xa8, 0x49, 0x68, 0x4b, 0xc5, 0x8c, 0xc0, 0xd6, 0xe9, 0xf3, 0x19, 0xb4, 0xda,
            0xe2, 0x6d, 0xb1, 0x71, 0x73, 0x3b, 0xf6, 0x0f, 0x31, 0xd9, 0x78, 0xe4, 0x1d, 0x09,
            0xa7, 0x5a, 0x63, 0x19,
        ]),
        BigUint::from_bytes_be(&[
            0x18, 0xbd, 0x4d, 0xae, 0x51, 0x34, 0x53, 0x8b, 0xd2, 0xf9, 0x0d, 0x41, 0xbb, 0xb1,
            0xe3, 0x30, 0xb2, 0xa8, 0x28, 0x6b, 0xa4, 0xa0, 0x9a, 0xca, 0x3f, 0xbb, 0xdc, 0xf9,
            0x32, 0x53, 0x4b, 0xe5,
        ]),
        BigUint::from_bytes_be(&[
            0x07, 0x36, 0xc6, 0x0c, 0xd3, 0x9f, 0xd1, 0x64, 0x9d, 0x48, 0x45, 0xb4, 0xf9, 0xa6,
            0xec, 0x9b, 0xac, 0xa8, 0x9f, 0xb2, 0xde, 0x0a, 0x3d, 0x7e, 0xea, 0xbe, 0x43, 0x50,
            0x4b, 0x56, 0x07, 0xfa,
        ]),
        BigUint::from_bytes_be(&[
            0x25, 0xa6, 0x97, 0x1a, 0x9d, 0x2c, 0x1d, 0xe9, 0xf3, 0x74, 0x37, 0x8d, 0x8f, 0x61,
            0x49, 0x2b, 0x1b, 0xd3, 0xc4, 0x65, 0x84, 0xc0, 0x76, 0xa7, 0x6c, 0x43, 0xc3, 0xcd,
            0x1a, 0x74, 0x75, 0x12,
        ]),
        BigUint::from_bytes_be(&[
            0x0a, 0x33, 0x73, 0xd1, 0x5f, 0xa6, 0xdc, 0xe2, 0x21, 0xf8, 0x32, 0x26, 0xc0, 0x2d,
            0x41, 0xf8, 0xae, 0xa5, 0xcf, 0xc6, 0xda, 0x4c, 0x9f, 0x49, 0x81, 0xad, 0xa1, 0xbd,
            0x4b, 0x50, 0xf5, 0x6e,
        ]),
        BigUint::from_bytes_be(&[
            0x2b, 0x70, 0x02, 0x8e, 0x2b, 0xf4, 0xe0, 0x08, 0xe2, 0x2e, 0xdd, 0xb7, 0x8d, 0x41,
            0x90, 0xd7, 0x3c, 0x28, 0x9d, 0xc6, 0x44, 0x5b, 0x3f, 0x64, 0xe1, 0x5f, 0x8b, 0xd0,
            0xec, 0x02, 0xc6, 0x72,
        ]),
        BigUint::from_bytes_be(&[
            0x0b, 0x24, 0xef, 0x46, 0x1a, 0x71, 0xee, 0xd9, 0x3d, 0xd3, 0x66, 0x34, 0x2f, 0x9c,
            0xa4, 0xee, 0xbb, 0x74, 0x9c, 0x8a, 0x5a, 0x60, 0x57, 0xc8, 0x01, 0xd5, 0x38, 0xc7,
            0xc0, 0x66, 0x6b, 0xa4,
        ]),
        BigUint::from_bytes_be(&[
            0x05, 0xd1, 0xe0, 0xac, 0x57, 0x6d, 0x1e, 0xc8, 0x14, 0xb6, 0x21, 0x51, 0x63, 0x39,
            0xae, 0x1a, 0x29, 0x1c, 0x7d, 0xf3, 0x6b, 0x5f, 0xd6, 0xcf, 0x0b, 0x4e, 0x3c, 0x9c,
            0xd2, 0x5e, 0x30, 0x72,
        ]),
        BigUint::from_bytes_be(&[
            0x27, 0x1c, 0xfb, 0xf8, 0x8e, 0x97, 0x44, 0xb8, 0x59, 0x6e, 0x7e, 0x2d, 0x68, 0x75,
            0xc8, 0x00, 0x5d, 0x0e, 0x62, 0x01, 0x40, 0x10, 0xac, 0x35, 0xe9, 0x5a, 0x7c, 0xe2,
            0x39, 0x0b, 0xc5, 0x0f,
        ]),
        BigUint::from_bytes_be(&[
            0x19, 0x63, 0x09, 0xf1, 0xd1, 0x70, 0xd7, 0x41, 0xab, 0x1c, 0xe9, 0x0c, 0x39, 0x77,
            0x20, 0x17, 0xfb, 0x7c, 0xde, 0xc7, 0x8c, 0x37, 0x88, 0x2b, 0x98, 0xa6, 0xb5, 0x69,
            0x56, 0xc1, 0x3d, 0xef,
        ]),
        BigUint::from_bytes_be(&[
            0x12, 0x7c, 0x11, 0x16, 0xc5, 0x75, 0xc0, 0x3c, 0x7f, 0x6d, 0x83, 0x41, 0x7d, 0x8c,
            0x1b, 0x38, 0x08, 0xf9, 0x2e, 0xe1, 0x69, 0x24, 0xa5, 0x40, 0x94, 0xbf, 0x09, 0x47,
            0x21, 0xe9, 0xe4, 0xf5,
        ]),
        BigUint::from_bytes_be(&[
            0x1b, 0xff, 0x78, 0x04, 0x7e, 0xe6, 0x7d, 0x38, 0xa5, 0x4f, 0xdc, 0x54, 0x0f, 0x9a,
            0x2b, 0xa0, 0x7f, 0x63, 0x48, 0x9a, 0xcd, 0x36, 0x42, 0x5f, 0x1a, 0xe2, 0x10, 0xac,
            0x32, 0x98, 0x26, 0xf5,
        ]),
        BigUint::from_bytes_be(&[
            0x06, 0xc7, 0xdc, 0x7b, 0xba, 0xe6, 0x15, 0xfc, 0xf1, 0x89, 0x6f, 0x2b, 0x8d, 0xb7,
            0xd9, 0x2c, 0x05, 0xdc, 0x1e, 0xa1, 0xc8, 0x13, 0x4e, 0x9d, 0xb6, 0xfd, 0x58, 0x86,
            0x72, 0xc5, 0x3e, 0x9a,
        ]),
        BigUint::from_bytes_be(&[
            0x12, 0xdf, 0x78, 0xcb, 0xa1, 0x75, 0xef, 0x76, 0xdb, 0xfc, 0xc9, 0xc7, 0x85, 0x92,
            0x6b, 0xb3, 0x94, 0x9a, 0x87, 0xec, 0x75, 0x33, 0xe2, 0x55, 0x9a, 0x27, 0xa6, 0x4b,
            0x91, 0xce, 0xbb, 0xa5,
        ]),
        BigUint::from_bytes_be(&[
            0x2b, 0xd4, 0xcd, 0xc9, 0x62, 0xe3, 0xda, 0x62, 0xcb, 0x3c, 0x96, 0xf7, 0xc4, 0x28,
            0xa9, 0xb0, 0xd5, 0x18, 0xbf, 0xa7, 0xce, 0x26, 0xf8, 0xfc, 0xe7, 0xa6, 0xaf, 0x76,
            0x9a, 0xfb, 0x65, 0x40,
        ]),
        BigUint::from_bytes_be(&[
            0x24, 0xed, 0xd3, 0x84, 0x7f, 0xeb, 0xbe, 0x44, 0xc4, 0xcc, 0x39, 0x02, 0x46, 0xe3,
            0x37, 0x9b, 0x47, 0xfd, 0x01, 0xa0, 0x30, 0xd0, 0xcd, 0x0b, 0x4f, 0xcf, 0x7f, 0xbd,
            0x1c, 0xab, 0xfe, 0x58,
        ]),
        BigUint::from_bytes_be(&[
            0x1c, 0xe0, 0x65, 0xd2, 0xc2, 0x56, 0x1b, 0xb5, 0x73, 0xe4, 0xcf, 0x42, 0x59, 0xd3,
            0xb0, 0xb0, 0xe9, 0xea, 0xcb, 0x44, 0x77, 0x51, 0xc6, 0x2b, 0x77, 0xd0, 0xbc, 0x5e,
            0x4e, 0x3c, 0x7d, 0x15,
        ]),
        BigUint::from_bytes_be(&[
            0x18, 0x05, 0x3e, 0x9f, 0x0d, 0x45, 0xf9, 0xee, 0xfb, 0xda, 0x13, 0x5b, 0xfd, 0x39,
            0x32, 0x9e, 0x34, 0x83, 0x7e, 0x63, 0x35, 0x65, 0xc3, 0x14, 0xfb, 0x90, 0x30, 0xb9,
            0xdb, 0x73, 0x81, 0xbb,
        ]),
        BigUint::from_bytes_be(&[
            0x16, 0x2f, 0xfa, 0x87, 0x42, 0x13, 0x8b, 0xbe, 0x51, 0x61, 0x68, 0xbf, 0x86, 0xec,
            0x78, 0xb1, 0xad, 0x1e, 0x8b, 0x53, 0x5a, 0xc4, 0x55, 0xa7, 0xcf, 0xbb, 0x22, 0xc1,
            0x3f, 0x9c, 0x5a, 0x9e,
        ]),
        BigUint::from_bytes_be(&[
            0x07, 0x9e, 0xea, 0x42, 0xe1, 0x6a, 0xc6, 0x44, 0x2c, 0xa8, 0x26, 0x23, 0xfc, 0x0e,
            0x8d, 0x9a, 0xd3, 0x99, 0x6a, 0x47, 0xa8, 0x01, 0x3e, 0xa9, 0xcb, 0x73, 0x85, 0x8c,
            0xa4, 0x2b, 0x71, 0x59,
        ]),
        BigUint::from_bytes_be(&[
            0x0a, 0x49, 0xaf, 0x2b, 0xbe, 0x11, 0xb0, 0x5b, 0xd0, 0x2a, 0x69, 0xa4, 0x7b, 0x1b,
            0xad, 0x5b, 0x21, 0x70, 0x40, 0x7a, 0xda, 0x21, 0x14, 0x2f, 0x06, 0xe4, 0xe1, 0x09,
            0xde, 0x88, 0xa1, 0xb6,
        ]),
        BigUint::from_bytes_be(&[
            0x12, 0xc3, 0x4e, 0xeb, 0xba, 0xa6, 0x9c, 0xcc, 0xc3, 0x69, 0x29, 0xe8, 0xf4, 0xa6,
            0xe4, 0x07, 0x71, 0xe1, 0x53, 0xff, 0x77, 0x94, 0x3d, 0xa5, 0x5c, 0x4f, 0xc8, 0x60,
            0x53, 0x7b, 0x73, 0x3a,
        ]),
        BigUint::from_bytes_be(&[
            0x00, 0x8d, 0xe5, 0xac, 0x6b, 0x4e, 0x35, 0x93, 0x35, 0xb6, 0xfc, 0xe5, 0x8d, 0xc0,
            0xe5, 0xe4, 0x3f, 0xd2, 0xae, 0xfd, 0x86, 0xba, 0xc3, 0x5a, 0xbe, 0x57, 0x9b, 0x8c,
            0xac, 0xe5, 0xdb, 0xc8,
        ]),
        BigUint::from_bytes_be(&[
            0x04, 0xa6, 0xe9, 0x88, 0xb5, 0x0d, 0x91, 0x57, 0x34, 0xbf, 0x32, 0x96, 0xd8, 0x30,
            0x57, 0xff, 0xe6, 0xa5, 0x50, 0xf8, 0x98, 0x7e, 0x45, 0x97, 0xbe, 0xe7, 0xd3, 0x33,
            0xcd, 0x24, 0xa8, 0x65,
        ]),
        BigUint::from_bytes_be(&[
            0x24, 0x11, 0x26, 0x33, 0x92, 0x6c, 0xfc, 0x60, 0x28, 0xfa, 0x2f, 0xfd, 0x9f, 0x09,
            0x0b, 0x1e, 0x54, 0x28, 0xa0, 0xa8, 0x7d, 0x71, 0x18, 0x35, 0x6e, 0x48, 0xb5, 0xd4,
            0x70, 0x44, 0x92, 0x17,
        ]),
        BigUint::from_bytes_be(&[
            0x0d, 0x56, 0x32, 0x99, 0x82, 0xf3, 0xdf, 0x38, 0xa3, 0xf1, 0x9f, 0xb8, 0x14, 0xc3,
            0x01, 0x3f, 0x41, 0x9b, 0xa0, 0xeb, 0x84, 0x03, 0xb2, 0x7c, 0x0c, 0x0e, 0x75, 0xc6,
            0xfe, 0x1c, 0xf4, 0x68,
        ]),
        BigUint::from_bytes_be(&[
            0x1f, 0x01, 0xef, 0x80, 0x76, 0x3c, 0x95, 0xf5, 0x3c, 0x43, 0x41, 0x64, 0x49, 0x3d,
            0x96, 0x73, 0xae, 0xef, 0x29, 0x0b, 0xf1, 0xaa, 0x19, 0x97, 0xd6, 0x77, 0xb5, 0x57,
            0xb9, 0x69, 0x2e, 0x8a,
        ]),
        BigUint::from_bytes_be(&[
            0x10, 0x5c, 0x52, 0x57, 0xf8, 0x01, 0x52, 0x7e, 0x60, 0xb0, 0x36, 0x1c, 0x00, 0x07,
            0x5b, 0x5a, 0x79, 0xd2, 0xdc, 0x68, 0x21, 0xd8, 0xa1, 0x25, 0x8d, 0x90, 0x6e, 0xd4,
            0x53, 0xc7, 0xe7, 0xbe,
        ]),
        BigUint::from_bytes_be(&[
            0x03, 0xdb, 0x50, 0x5a, 0x0c, 0x32, 0xcb, 0x61, 0xca, 0x09, 0x93, 0x89, 0xc2, 0x18,
            0x0e, 0x1c, 0x83, 0x82, 0x7f, 0xb4, 0x1d, 0x9f, 0xed, 0x84, 0xd8, 0x87, 0x66, 0xdf,
            0x44, 0xc6, 0x30, 0x79,
        ]),
        BigUint::from_bytes_be(&[
            0x12, 0x62, 0xe7, 0x38, 0xf3, 0x8d, 0xb6, 0xc7, 0x9d, 0x24, 0xd9, 0x72, 0x72, 0x94,
            0x42, 0x1c, 0xd9, 0x5a, 0xfa, 0x24, 0xf4, 0x70, 0x0c, 0x13, 0x23, 0xab, 0x83, 0xc3,
            0xa0, 0x6a, 0xce, 0x32,
        ]),
        BigUint::from_bytes_be(&[
            0x0e, 0xe6, 0x8c, 0x3e, 0x38, 0xc1, 0x94, 0x03, 0x39, 0x94, 0xc0, 0xd4, 0xd7, 0xbd,
            0xe3, 0x5b, 0xfa, 0xfa, 0x35, 0xb2, 0x2a, 0x95, 0xf9, 0x15, 0xf8, 0x2c, 0x5a, 0x3b,
            0x04, 0x22, 0xbd, 0x9a,
        ]),
        BigUint::from_bytes_be(&[
            0x2e, 0xe5, 0x42, 0x7b, 0xd2, 0x0c, 0x47, 0xf8, 0xd2, 0xf0, 0xaa, 0x9e, 0x64, 0x19,
            0xf7, 0x92, 0x6a, 0xbc, 0xd5, 0x96, 0x50, 0x84, 0x29, 0x2a, 0xe5, 0x4d, 0xd7, 0x80,
            0x07, 0x7e, 0x69, 0x02,
        ]),
        BigUint::from_bytes_be(&[
            0x1e, 0x54, 0x2d, 0x31, 0xd2, 0xa3, 0x81, 0x79, 0x2e, 0x0a, 0x92, 0x41, 0xc4, 0x62,
            0x29, 0xa2, 0x2f, 0xd9, 0x38, 0x24, 0x43, 0xe4, 0x23, 0xa0, 0xe4, 0x19, 0xd0, 0xfe,
            0xb5, 0x86, 0x56, 0xaf,
        ]),
        BigUint::from_bytes_be(&[
            0x0b, 0xa3, 0x9f, 0x01, 0x46, 0x2a, 0xb6, 0xa7, 0xcf, 0x62, 0x19, 0x52, 0x75, 0x2f,
            0xcd, 0xe4, 0x86, 0x77, 0xd7, 0xf3, 0x2d, 0xf4, 0x7e, 0x94, 0x0e, 0xac, 0xf4, 0x95,
            0x4c, 0x5e, 0xf6, 0x32,
        ]),
        BigUint::from_bytes_be(&[
            0x29, 0xc0, 0x0b, 0x05, 0x8c, 0x17, 0x80, 0x01, 0x46, 0xbd, 0xc0, 0x6b, 0x1e, 0x73,
            0xff, 0x5d, 0x0f, 0xf5, 0x3d, 0xf9, 0x6f, 0x84, 0x63, 0x81, 0x8c, 0x05, 0x72, 0xd1,
            0x1f, 0xca, 0xf8, 0x8b,
        ]),
        BigUint::from_bytes_be(&[
            0x0b, 0x62, 0x00, 0x89, 0x5b, 0x60, 0xa6, 0xc6, 0x79, 0x4f, 0xcf, 0x1c, 0x2b, 0x1b,
            0x15, 0xd0, 0x3a, 0x71, 0x3c, 0x90, 0x5a, 0x8b, 0xa1, 0xf1, 0x31, 0x5f, 0x75, 0x01,
            0xfe, 0x1a, 0x50, 0xb8,
        ]),
        BigUint::from_bytes_be(&[
            0x2b, 0xc6, 0x39, 0xb1, 0xb8, 0x5d, 0x73, 0x1f, 0x62, 0xd2, 0xc6, 0xf3, 0x91, 0xd4,
            0x49, 0x8e, 0x39, 0x2c, 0xb7, 0x5e, 0xdc, 0xbd, 0x5c, 0x4c, 0x0f, 0xa8, 0xb2, 0x6d,
            0x32, 0xd6, 0x8a, 0x12,
        ]),
        BigUint::from_bytes_be(&[
            0x2a, 0x89, 0xf3, 0x8e, 0x64, 0x40, 0xce, 0x64, 0x11, 0x27, 0x04, 0x6b, 0x67, 0xd8,
            0xe6, 0x15, 0xf1, 0x45, 0x03, 0xd7, 0x2d, 0x76, 0xbf, 0x3c, 0x70, 0x3a, 0x01, 0xd1,
            0x46, 0x3a, 0x84, 0x45,
        ]),
        BigUint::from_bytes_be(&[
            0x17, 0x50, 0xed, 0xe7, 0xee, 0xeb, 0x4e, 0xdd, 0x78, 0x38, 0xb6, 0x7f, 0xac, 0x6d,
            0x25, 0x0a, 0x54, 0x05, 0x5e, 0xee, 0xad, 0x10, 0xe6, 0x9b, 0x3a, 0x6e, 0x1f, 0x07,
            0x6c, 0xa8, 0x78, 0x68,
        ]),
        BigUint::from_bytes_be(&[
            0x0c, 0x2d, 0x65, 0x08, 0x4b, 0xea, 0xd2, 0xa7, 0x43, 0x11, 0x5b, 0xe5, 0x32, 0x9d,
            0x54, 0x58, 0xd2, 0x98, 0x02, 0x08, 0x1f, 0x6f, 0x9d, 0xac, 0x41, 0x65, 0xc4, 0x26,
            0x51, 0xf9, 0xbe, 0x2b,
        ]),
        BigUint::from_bytes_be(&[
            0x28, 0x30, 0x3e, 0x2d, 0x83, 0x4e, 0x16, 0xe1, 0xfe, 0x33, 0xc9, 0xab, 0x72, 0x6a,
            0x3e, 0x75, 0xdd, 0x0d, 0xad, 0x9b, 0xfe, 0xa1, 0xa4, 0x32, 0x67, 0x19, 0x9e, 0x1f,
            0x24, 0x39, 0x93, 0xfb,
        ]),
        BigUint::from_bytes_be(&[
            0x2b, 0x57, 0x28, 0x11, 0xca, 0x34, 0xea, 0x51, 0x10, 0xd1, 0x07, 0x72, 0xe4, 0xce,
            0xd3, 0x62, 0xeb, 0xef, 0xd7, 0xcd, 0x1e, 0x18, 0x84, 0xb7, 0x69, 0xe9, 0x43, 0x59,
            0x14, 0xef, 0xc5, 0xe5,
        ]),
        BigUint::from_bytes_be(&[
            0x17, 0x52, 0x1c, 0xa5, 0x79, 0x9f, 0xe2, 0xea, 0x82, 0xc6, 0x7c, 0x0a, 0x8d, 0x08,
            0x63, 0xb5, 0xee, 0xc0, 0xef, 0x9b, 0x70, 0x3e, 0x19, 0x5d, 0xd4, 0x02, 0xb7, 0x00,
            0x8b, 0x53, 0xf6, 0xb4,
        ]),
        BigUint::from_bytes_be(&[
            0x04, 0x07, 0xe5, 0x4b, 0x96, 0xa5, 0xb6, 0x3c, 0x60, 0x9f, 0xa3, 0x79, 0x7b, 0x22,
            0x3c, 0x73, 0xd2, 0x60, 0xa3, 0x65, 0xad, 0x58, 0xb2, 0x58, 0x91, 0xa5, 0x66, 0x02,
            0x72, 0x09, 0x6b, 0xd5,
        ]),
        BigUint::from_bytes_be(&[
            0x1a, 0x3c, 0xd1, 0x55, 0xb0, 0x3c, 0x7d, 0x33, 0xcc, 0x82, 0x22, 0xc9, 0x97, 0x42,
            0x4b, 0xc1, 0x40, 0x69, 0xe2, 0xed, 0xbf, 0x4b, 0x8a, 0xa5, 0x64, 0xc9, 0xe5, 0x83,
            0x2b, 0xda, 0xce, 0x91,
        ]),
        BigUint::from_bytes_be(&[
            0x29, 0x62, 0x55, 0xb5, 0xe6, 0x97, 0xe5, 0x17, 0xc5, 0x02, 0xba, 0x49, 0xb1, 0x8a,
            0xaa, 0xd8, 0x95, 0x14, 0xa4, 0x90, 0xa0, 0x2e, 0x7a, 0x87, 0x8b, 0x5d, 0x55, 0x98,
            0x41, 0xb9, 0x3f, 0xbd,
        ]),
        BigUint::from_bytes_be(&[
            0x17, 0x48, 0x35, 0x80, 0x1a, 0x1f, 0x15, 0x25, 0xb4, 0xc2, 0x18, 0x53, 0xb9, 0x65,
            0xc5, 0x04, 0x8a, 0xf4, 0x65, 0xe9, 0xf7, 0x9d, 0xe9, 0xd1, 0x67, 0x48, 0xc6, 0x79,
            0x53, 0xda, 0x79, 0xa7,
        ]),
        BigUint::from_bytes_be(&[
            0x2d, 0x4a, 0xfe, 0xd7, 0xa7, 0x08, 0xe5, 0x97, 0x2e, 0x84, 0xd7, 0x66, 0x29, 0x2f,
            0x2c, 0x84, 0x1c, 0x5d, 0x85, 0x70, 0x96, 0x10, 0x74, 0xd5, 0x9a, 0xd3, 0xf5, 0x1e,
            0x93, 0x69, 0xa5, 0x97,
        ]),
        BigUint::from_bytes_be(&[
            0x1c, 0x0e, 0xb0, 0x67, 0x44, 0xc9, 0x86, 0x6e, 0x27, 0x1c, 0xd2, 0x9a, 0x7f, 0x17,
            0xf7, 0x29, 0x64, 0xfa, 0xba, 0x3c, 0xd0, 0x88, 0xb9, 0x5e, 0x73, 0xdc, 0xce, 0x9d,
            0x92, 0xc7, 0x9b, 0xa6,
        ]),
        BigUint::from_bytes_be(&[
            0x26, 0x70, 0x5e, 0x7e, 0x4f, 0x23, 0xa7, 0xd7, 0x86, 0xad, 0x17, 0x86, 0xb3, 0x53,
            0xa2, 0xf8, 0xb8, 0x22, 0x69, 0xc7, 0xb5, 0x8a, 0xb7, 0x0d, 0x7b, 0x93, 0xf4, 0x16,
            0x85, 0xd3, 0x4d, 0x45,
        ]),
        BigUint::from_bytes_be(&[
            0x04, 0xe6, 0x74, 0xd8, 0x8b, 0x90, 0xb1, 0x18, 0x83, 0x53, 0x10, 0x6a, 0xe2, 0x5c,
            0x04, 0x47, 0xac, 0xac, 0xe9, 0xdc, 0x6d, 0x62, 0xcf, 0xe7, 0xfe, 0xc2, 0xd7, 0x99,
            0x3d, 0xfd, 0x7a, 0x22,
        ]),
        BigUint::from_bytes_be(&[
            0x0d, 0xf3, 0x33, 0x5d, 0xa1, 0x3f, 0xf4, 0x6f, 0x65, 0x09, 0x5f, 0x97, 0x5d, 0x15,
            0x78, 0x86, 0x24, 0x1a, 0xec, 0xcf, 0xf3, 0x8f, 0xd9, 0xbb, 0xa9, 0x26, 0x44, 0xf8,
            0x96, 0x9d, 0x7e, 0x09,
        ]),
        BigUint::from_bytes_be(&[
            0x2d, 0xff, 0xf6, 0x2b, 0x92, 0x82, 0xec, 0x05, 0xb1, 0xfa, 0x44, 0x47, 0x9a, 0x6e,
            0x9d, 0xeb, 0xe9, 0xac, 0x63, 0x18, 0x13, 0xd2, 0xb1, 0x0e, 0x44, 0xb9, 0xe0, 0xfe,
            0x19, 0xe4, 0xd4, 0xee,
        ]),
        BigUint::from_bytes_be(&[
            0x08, 0xec, 0xe2, 0x48, 0xfe, 0x1c, 0xe1, 0xcd, 0x70, 0x56, 0x99, 0xb5, 0xcd, 0x07,
            0xc9, 0x90, 0xec, 0x27, 0x72, 0x1b, 0xab, 0x59, 0xb6, 0x57, 0xbb, 0x13, 0x8e, 0x48,
            0x7e, 0xe6, 0x69, 0x4d,
        ]),
        BigUint::from_bytes_be(&[
            0x2c, 0x1a, 0xb8, 0x1d, 0xb6, 0x07, 0xba, 0x76, 0xdb, 0xf7, 0x1f, 0x48, 0x75, 0x2c,
            0x85, 0x6b, 0xf1, 0x83, 0x04, 0x49, 0x81, 0xc3, 0xb6, 0xd1, 0xfd, 0x31, 0xb1, 0x79,
            0xa0, 0x78, 0xf5, 0x71,
        ]),
        BigUint::from_bytes_be(&[
            0x01, 0xde, 0x6f, 0x88, 0x86, 0x86, 0x8e, 0x35, 0x1b, 0xf4, 0xca, 0xad, 0x29, 0x3b,
            0xd8, 0x6e, 0xd2, 0x9e, 0xf6, 0x38, 0x10, 0xe1, 0x5c, 0xb8, 0x09, 0x54, 0x2e, 0x01,
            0xbf, 0xbb, 0xcb, 0x88,
        ]),
    ];
}
