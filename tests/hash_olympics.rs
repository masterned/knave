use std::hash::{DefaultHasher, Hasher};

use knave::{
    AmnesiacRollingHostnameHasher, MachineFactory, Machines, NaiveHostnameHasher,
    RollingHostnameHasher,
};

fn mk_machines<H: Hasher + Default>(mf: MachineFactory<H>) -> Machines {
    [
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
    ]
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
