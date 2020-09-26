pub mod parser;

#[cfg(test)]
mod test {
    use super::parser::*;

    #[test]
    fn test_numerals() {
        assert_eq!(NumeralParser::new().parse("12345"), Ok(12345));
        assert_eq!(NumeralParser::new().parse("0b101010"), Ok(42));
        assert_eq!(NumeralParser::new().parse("0B101010"), Ok(42));
        assert_eq!(NumeralParser::new().parse("0o15033"), Ok(6683));
        assert_eq!(NumeralParser::new().parse("0O15033"), Ok(6683));
        assert_eq!(NumeralParser::new().parse("0x233F"), Ok(09023));
        assert_eq!(NumeralParser::new().parse("0X233F"), Ok(09023));

        assert!(NumeralParser::new().parse("0b1").is_ok());
        assert!(NumeralParser::new().parse("0b12").is_err());
        assert!(NumeralParser::new().parse("0o1234567").is_ok());
        assert!(NumeralParser::new().parse("0o12345678").is_err());
        assert!(NumeralParser::new().parse("0x123456789ABCDEF").is_ok());
        assert!(NumeralParser::new().parse("0x123456789ABCDEFG").is_err());
    }
}
