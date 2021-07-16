use std::io::Error;

const B64_URL_ENCODE: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2d, 0x5f,
];

const B64_URL_PAD: u8 = 0x3d;

pub struct B64Url<T>(T);

pub trait B64UrlEncode<I, O, E> {
    fn encode(input: I) -> Result<O, E>;
}

impl B64UrlEncode<&[u8], Vec<u8>, Error> for B64Url<Vec<u8>> {
    fn encode(input: &[u8]) -> Result<Vec<u8>, Error> {
        let bytes = input;
        let length = input.len();
        let mut vector = Vec::<u8>::with_capacity(length * 4 / 3);
        let mut index = 0;

        while index + 3 <= length {
            let value = (bytes[index] as u32) << 16
                | (bytes[index + 1] as u32) << 8
                | (bytes[index + 2] as u32);
            vector.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
            vector.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
            vector.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
            vector.push(B64_URL_ENCODE[(value & 0b11_1111) as usize]);
            index += 3;
        }

        match length - index {
            2 => {
                let value = (bytes[index] as u32) << 16 | (bytes[index + 1] as u32) << 8;
                vector.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vector.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vector.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vector.push(B64_URL_PAD);
            }
            1 => {
                let value = (bytes[index] as u32) << 16;
                vector.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vector.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vector.push(B64_URL_PAD);
                vector.push(B64_URL_PAD);
            }
            _ => {}
        };

        Ok(vector)
    }
}

#[cfg(test)]
mod b64_url_encode {
    use super::*;

    #[test]
    fn t1() {
        let result = B64Url::<Vec<u8>>::encode(b"").unwrap();
        assert_eq!(result, b"");
    }

    #[test]
    fn t2() {
        let result = B64Url::<Vec<u8>>::encode(b"f").unwrap();
        assert_eq!(result, b"Zg==");
    }

    #[test]
    fn t3() {
        let result = B64Url::<Vec<u8>>::encode(b"fo").unwrap();
        assert_eq!(result, b"Zm8=");
    }

    #[test]
    fn t4() {
        let result = B64Url::<Vec<u8>>::encode(b"foo").unwrap();
        assert_eq!(result, b"Zm9v");
    }

    #[test]
    fn t5() {
        let result = B64Url::<Vec<u8>>::encode(b"foob").unwrap();
        assert_eq!(result, b"Zm9vYg==");
    }

    #[test]
    fn t6() {
        let result = B64Url::<Vec<u8>>::encode(b"fooba").unwrap();
        assert_eq!(result, b"Zm9vYmE=");
    }

    #[test]
    fn t7() {
        let result = B64Url::<Vec<u8>>::encode(b"foobar").unwrap();
        assert_eq!(result, b"Zm9vYmFy");
    }
}
