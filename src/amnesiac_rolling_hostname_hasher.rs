use std::hash::Hasher;

const POLYNOMIAL_HASH_MULTIPLIER: u64 = 31;

#[derive(Default)]
pub struct AmnesiacRollingHostnameHasher(u64);

impl Hasher for AmnesiacRollingHostnameHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        if bytes == [255] {
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
