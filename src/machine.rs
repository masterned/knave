use std::hash::{Hash, Hasher};

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

impl<H: Hasher + Default> Default for MachineFactory<H> {
    fn default() -> Self {
        Self::new(H::default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn collides_with(&self, other: &Self) -> bool {
        self.name != other.name && self.addr == other.addr
    }
}

#[derive(Clone, Debug, Default)]
pub struct Machines(Vec<Machine>);

impl From<Vec<Machine>> for Machines {
    fn from(value: Vec<Machine>) -> Self {
        Self(value)
    }
}

impl Machines {
    pub fn has_collisions(&self) -> bool {
        let mut machines: Vec<Machine> = self.clone().0;
        machines.sort_by(|a, b| a.name().cmp(b.name()));

        let mut mi = machines.into_iter();
        if let Some(cur) = mi.next() {
            let mut cur = cur;
            while let Some(next) = mi.next() {
                let collide = cur.collides_with(&next);
                if collide {
                    return true;
                }
                cur = next;
            }
        }

        false
    }

    pub fn is_consistent(&self) -> bool {
        let mut machines: Vec<Machine> = self.clone().0;
        machines.sort_by(|a, b| a.name().cmp(b.name()));

        let mut mi = machines.into_iter();
        if let Some(cur) = mi.next() {
            let mut cur = cur;
            while let Some(next) = mi.next() {
                if cur.name() == next.name() {
                    if cur.addr() != next.addr() {
                        return false;
                    }
                }
                cur = next;
            }
        }

        true
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
