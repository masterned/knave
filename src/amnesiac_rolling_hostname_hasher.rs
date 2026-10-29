use std::hash::Hasher;

const SENTINEL_OCTET: u8 = 0xff;
const POLYNOMIAL_HASH_MULTIPLIER: u64 = 31;

#[derive(Default)]
pub struct AmnesiacRollingHostnameHasher(u64);

impl Hasher for AmnesiacRollingHostnameHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        if bytes == [SENTINEL_OCTET] {
            return;
        }

        let mut hash = 0_u64;

        for &byte in bytes {
            hash = hash
                .wrapping_mul(POLYNOMIAL_HASH_MULTIPLIER)
                .wrapping_add(u64::from(byte));
        }

        self.0 = hash;
    }
}
