#[cfg(test)]
mod tests {
    use main;
    use std::fs;

    #[test]
    fn it_parses_organic_results() {
        let expected = vec![OrganicResult {}, OrganicResult {}];

        let contents = fs::read_to_string("data/rst-tesla.html")
            .expect("Something went wrong reading the file");

        assert_eq!(format!("{:?}", parse(&contents)), format!("{:?}", expected));
    }

    #[test]
    fn it_saves_html() {
        assert_eq!(2 + 2, 4);
    }
}
