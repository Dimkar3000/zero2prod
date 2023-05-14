fn return_hello() -> String {
    "Hello, world!".to_string()
}

#[cfg(not(tarpaulin))]
fn main() {
    println!("{}", return_hello());
}

#[cfg(test)]
mod tests {
    use crate::return_hello;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
        return_hello();
    }
}
