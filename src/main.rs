use knave::{
    RollingHostnameHasher,
    machine::{MachineFactory, Machines},
};

fn main() {
    let mf: MachineFactory<RollingHostnameHasher> = MachineFactory::default();

    let mut machines: Vec<_> = [
        "azarath", "cassea", "cygnus", "damdom", "elowyn", "saira", "seamus", "elucia", "azarath",
    ]
    .iter()
    .map(|&n| mf.mk_machine(n))
    .collect();

    machines.sort_by(|a, b| a.addr().cmp(&b.addr()));
    for machine in &machines {
        println!("{:?}", machine);
    }

    println!();

    let ms: Machines = machines.into();
    println!("consistent?: {}", ms.is_consistent());
    println!("collisions?: {}", ms.has_collisions());
}
