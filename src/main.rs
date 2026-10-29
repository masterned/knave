use clap::{Arg, command};
use knave::{MachineFactory, RollingHostnameHasher};

fn main() {
    let cmd = command!()
        .arg(
            Arg::new("hostname")
                .required(true)
                .help("The hostname of the machine"),
        )
        .arg_required_else_help(true);
    let matches = cmd.get_matches();

    let mf = MachineFactory::<RollingHostnameHasher>::default();

    let hostname = matches.get_one::<String>("hostname").unwrap();
    let machine = mf.mk_machine(hostname);
    println!("{machine:?}");
}
