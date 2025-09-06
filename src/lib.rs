#[cfg(test)]
mod tests {
    use snapbox::assert_data_eq;
    use snapbox::str;
    use test_utils::snapbox;

    #[test]
    fn test_with_snapbox() {
        let actual = format!("hello {}", "world");
        assert_data_eq!(actual, str!["hello world"]);
    }
}
