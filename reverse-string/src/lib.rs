pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for vi in input.split("").into_iter() {
        result = vi.to_owned() + &result;
    }
    result
}
