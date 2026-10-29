use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

const MAX_ADDRESS: usize = 255;
const OFFSET: usize = 2;

fn address_clamp(number: usize) -> usize {
    (number % (MAX_ADDRESS - OFFSET)) + OFFSET
}

#[derive(Default)]
pub struct MachineFactory<H: Hasher>(PhantomData<H>);

impl<H: Hasher + Default> MachineFactory<H> {
    pub fn new() -> MachineFactory<H> {
        Self::default()
    }

    fn hash_hostname(&self, word: &str) -> usize {
        let mut hasher = H::default();
        word.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn hostname_to_addr(&self, name: &str) -> usize {
        address_clamp(self.hash_hostname(name))
    }

    pub fn mk_machine(&self, hostname: impl Into<String>) -> Machine {
        let hostname = hostname.into();
        let addr = self.hostname_to_addr(&hostname);
        Machine { hostname, addr }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Machine {
    hostname: String,
    addr: usize,
}

impl Machine {
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn addr(&self) -> usize {
        self.addr
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.hostname != other.hostname && self.addr == other.addr
    }

    pub fn is_inconsistent_with(&self, other: &Self) -> bool {
        self.hostname == other.hostname && self.addr != other.addr
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
        machines.sort_by_key(Machine::addr);

        let mut mi = machines.into_iter();
        if let Some(cur) = mi.next() {
            let mut cur = cur;
            for next in mi {
                if cur.collides_with(&next) {
                    return true;
                }
                cur = next;
            }
        }

        false
    }

    pub fn is_consistent(&self) -> bool {
        let mut machines: Vec<Machine> = self.clone().0;
        machines.sort_by(|a, b| a.hostname().cmp(b.hostname()));

        let mut mi = machines.into_iter();
        if let Some(cur) = mi.next() {
            let mut cur = cur;
            for next in mi {
                if cur.is_inconsistent_with(&next) {
                    return false;
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
