use std::{
    hash::{DefaultHasher, Hasher},
    time::SystemTime,
};

use knave::{
    AmnesiacRollingHostnameHasher, MachineFactory, Machines, NaiveHostnameHasher,
    RollingHostnameHasher,
};

const HOSTNAMES: [&'static str; 16] = [
    "azarath",
    "azarath",
    "cassea",
    "cygnus",
    "damdom",
    "elowyn",
    "elucia",
    "saira",
    "seamus",
    "cygnus",
    "ambroxan",
    "limonene",
    "galaxolide",
    "cashmeran",
    "sad",
    "das",
];

#[derive(Default)]
#[allow(nonstandard_style)]
struct eMudhraHasher;

impl Hasher for eMudhraHasher {
    fn finish(&self) -> u64 {
        4
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

#[derive(Default)]
struct LaplaceHasher;

impl Hasher for LaplaceHasher {
    fn finish(&self) -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap()
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

fn mk_machines<H: Hasher + Default>(mf: MachineFactory<H>) -> Machines {
    HOSTNAMES
        .iter()
        .map(|&m| mf.mk_machine(m))
        .collect::<Vec<_>>()
        .into()
}

#[test]
#[should_panic]
fn _default_collision() {
    let ms = mk_machines(MachineFactory::<DefaultHasher>::default());

    assert!(!ms.has_collisions());
}

#[test]
fn _default_consistency() {
    let ms = mk_machines(MachineFactory::<DefaultHasher>::default());

    assert!(ms.is_consistent());
}

#[test]
#[should_panic]
fn _naive_collision() {
    let ms = mk_machines(MachineFactory::<NaiveHostnameHasher>::default());

    assert!(!ms.has_collisions());
}

#[test]
fn _naive_consistency() {
    let ms = mk_machines(MachineFactory::<NaiveHostnameHasher>::default());

    assert!(ms.is_consistent())
}

#[test]
fn _amnesiac_collision() {
    let ms = mk_machines(MachineFactory::<AmnesiacRollingHostnameHasher>::default());

    assert!(!ms.has_collisions());
}

#[test]
fn _amnesiac_consistency() {
    let ms = mk_machines(MachineFactory::<AmnesiacRollingHostnameHasher>::default());

    assert!(ms.is_consistent())
}

#[test]
fn _rolling_collision() {
    let ms = mk_machines(MachineFactory::<RollingHostnameHasher>::default());

    assert!(!ms.has_collisions());
}

#[test]
fn _rolling_consistency() {
    let ms = mk_machines(MachineFactory::<RollingHostnameHasher>::default());

    assert!(ms.is_consistent())
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn _eMudhra_collision() {
    let ms = mk_machines(MachineFactory::<eMudhraHasher>::default());
    assert!(!ms.has_collisions());
}

#[test]
#[allow(non_snake_case)]
fn _eMudhra_consistency() {
    let ms = mk_machines(MachineFactory::<eMudhraHasher>::default());
    assert!(ms.is_consistent());
}

#[test]
#[should_panic]
/// Technically not a good test. Could fail under certain conditions. Test
/// was mostly made for fun.
fn _laplace_consistency() {
    let ms = mk_machines(MachineFactory::<LaplaceHasher>::default());
    assert!(ms.is_consistent());
}
