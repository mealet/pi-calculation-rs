mod pi;

fn main() {
    let pkg_name = env!("CARGO_BIN_NAME");
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        eprintln!("Not enough arguments!\nExample: {} [accuracy number] [numbers after dot]", pkg_name);
        std::process::exit(1);
    }

    let accuracy = args[1]
        .clone()
        .trim()
        .parse::<u64>()
        .unwrap();

    let numbers_after_dot = args[2]
        .clone()
        .trim()
        .parse::<usize>()
        .unwrap();

    let pi = pi::calculate_pi(accuracy);
    println!("Calculated PI Number:\n{:.1$}", pi, numbers_after_dot);
}
