fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use snapbox::assert_data_eq;
    use snapbox::str;
    use test_utils::snapbox;

    macro_rules! test_wrapper {
        ($name:ident, || $e:expr) => {
            #[test]
            fn $name() {
                $e;
            }
        };
    }

    #[test]
    fn test_with_snapbox() {
        let actual = format!("hello {}", "world");
        assert_data_eq!(actual, str!["hello world"]);
    }

    test_wrapper!(test_with_snapbox_wrapped, || {
        let actual = format!("hello {}", "world");
        assert_data_eq!(actual, str!["hello world"]);
    });
}
