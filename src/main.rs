fn main() {
    let matches = clap::Command::new("generate_sequence")
        .version("0.1.0")
        .about("Generate a Fibonacci sequence up to the given number of terms")
        .arg(
            clap::arg!([n] "The number of terms")
                .value_parser(clap::value_parser!(usize))
                .required(true),
        )
        .get_matches();

    let n = matches.get_one::<usize>("n").unwrap_or(&0);
    let output = rust_starter::fibonacci_sequence(*n)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", output);
}
