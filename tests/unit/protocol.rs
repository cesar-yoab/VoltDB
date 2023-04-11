#[cfg(test)]
mod tests {
    use crate::protocol::encoder::{Command, Encoder};
    use crate::protocol::parser::{CommandParser, Parser};

    #[test]
    fn test_parser() {
        let input = "INSERT key1 1.0 2.0 3.0\r\n".to_string();
        let parser = CommandParser::new(input.as_bytes());
        let command = parser.parse();
        assert_eq!(
            command,
            Ok(Command::Insert("key1".to_string(), vec![1.0, 2.0, 3.0]))
        );
    }

    #[test]
    fn test_encoder() {
        let command = Command::Insert("key1".to_string(), vec![1.0, 2.0, 3.0]);
        let encoder = Encoder::new();
        let encoded = encoder.encode(&command);
        assert_eq!(encoded, "INSERT key1 1.0 2.0 3.0\r\n".to_string());
    }

    #[test]
    fn test_parser_encoder_integration() {
        let input = "INSERT key1 1.0 2.0 3.0\r\n".to_string();
        let parser = CommandParser::new(input.as_bytes());
        let command = parser.parse().unwrap();

        let encoder = Encoder::new();
        let encoded = encoder.encode(&command);
        assert_eq!(encoded, "INSERT key1 1.0 2.0 3.0\r\n".to_string());
    }
}
