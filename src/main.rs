fn main() {
    let n = std::env::args().nth(1).expect("no terms given");
    let output = rust_starter::fibonacci_sequence(n.parse::<usize>().unwrap_or(0))
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}
