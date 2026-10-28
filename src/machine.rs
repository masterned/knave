use std::hash::{DefaultHasher, Hash, Hasher};

const MAX_ADDRESS: usize = 255;
const OFFSET: usize = 2;

fn address_clamp(number: usize) -> usize {
    (number % (MAX_ADDRESS - OFFSET)) + OFFSET
}

pub struct MachineFactory<H: Hasher>(H);

impl<H: Hasher> MachineFactory<H> {
    pub fn new(hasher: H) -> MachineFactory<H> {
        MachineFactory(hasher)
    }

    fn hash_name(&mut self, word: &str) -> usize {
        word.hash(&mut self.0);
        self.0.finish() as usize
    }

    fn name_to_addr(&mut self, name: &str) -> usize {
        address_clamp(self.hash_name(name))
    }

    pub fn mk_machine(&mut self, name: impl Into<String>) -> Machine {
        let name = name.into();
        let addr = self.name_to_addr(&name);
        Machine { name, addr }
    }
}

impl Default for MachineFactory<DefaultHasher> {
    fn default() -> Self {
        Self::new(DefaultHasher::new())
    }
}

#[derive(Clone, Debug)]
pub struct Machine {
    name: String,
    addr: usize,
}

impl Machine {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn addr(&self) -> usize {
        self.addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _cannot_land_on_port_0_1_or_255_or_higher() {
        assert_ne!(address_clamp(0), 0);
        assert_ne!(address_clamp(1), 1);
        assert_ne!(address_clamp(255), 255);
        assert!(address_clamp(256) < 255);
    }
}
