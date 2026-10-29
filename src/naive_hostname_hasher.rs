use std::hash::Hasher;

#[derive(Default)]
pub struct NaiveHostnameHasher(u64);

impl Hasher for NaiveHostnameHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        if bytes == [255] {
            return;
        }
        self.0 = bytes.iter().map(|&b| b as u64).sum();
    }
}
