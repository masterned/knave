use std::hash::Hasher;

const SENTINEL_OCTET: u8 = 0xff;

#[derive(Default)]
pub struct NaiveHostnameHasher(u64);

impl Hasher for NaiveHostnameHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        if bytes == [SENTINEL_OCTET] {
            return;
        }
        self.0 = bytes.iter().map(|&b| b as u64).sum();
    }
}
