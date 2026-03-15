#[cfg(test)]
mod tests {
    #[test]
    fn add_two() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(3 + 2, 5);
    }
}

