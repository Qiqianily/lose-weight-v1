fn main() {
    println!("this is the first test!");
}
#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(4, 2 * 2)
    }
}
