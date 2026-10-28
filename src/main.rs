use knave::machine::MachineFactory;

fn main() {
    let mut mf = MachineFactory::default();

    [
        "azarath", "cassea", "cygnus", "damdom", "elowyn", "saira", "seamus",
    ]
    .iter()
    .map(|&n| mf.mk_machine(n))
    .for_each(|m| println!("{:?}", m));
}
