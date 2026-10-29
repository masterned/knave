use std::hash::Hasher;

const POLYNOMIAL_HASH_MULTIPLIER: u64 = 31;

#[derive(Default)]
pub struct RollingHostnameHasher(u64);

impl Hasher for RollingHostnameHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = self
                .0
                .wrapping_mul(POLYNOMIAL_HASH_MULTIPLIER)
                .wrapping_add(u64::from(byte));
        }
    }
}
