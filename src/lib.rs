fn example() -> i64 {
    1 + 1
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(super::example(), 2);
    }
}
