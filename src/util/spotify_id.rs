use std;
use extprim::u128::u128;
use byteorder::{BigEndian, ByteOrder};
use std::ascii::AsciiExt;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct SpotifyId(u128);

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct FileId(pub [u8; 20]);

const BASE62_DIGITS: &'static [u8] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    pub fn from_base16(id: &str) -> SpotifyId {
        assert!(id.is_ascii());
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = BASE16_DIGITS.iter().position(|e| e == c).unwrap() as u64;
            n = n * u128::new(16);
            n = n + u128::new(d);
        }

        SpotifyId(n)
    }

    pub fn from_base62(id: &str) -> SpotifyId {
        assert!(id.is_ascii());
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = BASE62_DIGITS.iter().position(|e| e == c).unwrap() as u64;
            n = n * u128::new(62);
            n = n + u128::new(d);
        }

        SpotifyId(n)
    }

    pub fn from_raw(data: &[u8]) -> SpotifyId {
        assert_eq!(data.len(), 16);

        let high = BigEndian::read_u64(&data[0..8]);
        let low = BigEndian::read_u64(&data[8..16]);

        SpotifyId(u128::from_parts(high, low))
    }

    pub fn to_base16(&self) -> String {
        let &SpotifyId(ref n) = self;

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[31 - i] = BASE16_DIGITS[(n.wrapping_shr(4 * i as u32).low64() & 0xF) as usize];
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_base62(&self) -> String {
        let &SpotifyId(mut n) = self;

        let mut data = [0u8; 22];
        let sixty_two = u128::new(62);
        for i in 0..22 {
            data[21-i] = BASE62_DIGITS[(n % sixty_two).low64() as usize];
            n /= sixty_two;
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        let &SpotifyId(ref n) = self;

        let mut data = [0u8; 16];

        BigEndian::write_u64(&mut data[0..8], n.high64());
        BigEndian::write_u64(&mut data[8..16], n.low64());

        data
    }
}

impl FileId {
    pub fn to_base16(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .concat()
    }
}

#[test]
fn test_base16() {
    let spotify_id_str = "a719283ffb17abcd0192ea49b20139ff";
    let zeros = "00000000000000000000000000000000";
    assert_eq!(spotify_id_str, SpotifyId::from_base16(spotify_id_str).to_base16());
    assert_eq!(zeros, SpotifyId::from_base16(zeros).to_base16());
}

#[test]
fn test_base62() {
    let spotify_id_str = "6rqhFgbbKwnb9MLmUQDhG6";
    let zeros = "0000000000000000000000";
    assert_eq!(spotify_id_str, SpotifyId::from_base62(spotify_id_str).to_base62());
    assert_eq!(zeros, SpotifyId::from_base62(zeros).to_base62());
}
