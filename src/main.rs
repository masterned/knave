use knave::{MachineFactory, Machines, RollingHostnameHasher};

fn main() {
    let mf = MachineFactory::<RollingHostnameHasher>::default();
    let ms: Vec<_> = [
        "azarath", "cassea", "cygnus", "damdom", "elowyn", "elucia", "saira", "seamus",
    ]
    .iter()
    .map(|&m| mf.mk_machine(m))
    .collect();

    for m in &ms {
        println!("{m:?}");
    }

    println!();

    let ms: Machines = ms.into();
    println!("collisions?: {}", ms.has_collisions());
    println!("consistent?: {}", ms.is_consistent());
}
