pub fn fibonacci_sequence(n: usize) -> Vec<usize> {
    let mut sequence = vec![1; n];
    for i in 2..n {
        sequence[i] = sequence[i - 2] + sequence[i - 1];
    }
    sequence
}
