struct ParserPayload {
    pub command: String,
    pub arguments: Vec<String>
}

impl ParserPayload {
    pub fn parse(prefix: String, message: String) -> Self {
        let prefix_length = prefix.chars().count();
        let separated = message.split_whitespace();
        let mut first_piece: bool = true;
        let mut first_string: String = String::new();
        let mut arguments: Vec<String> = Vec::<String>::new();
        for piece in separated {
            let piece_string = piece.to_string();
            if first_piece {
                first_string = piece_string;
                first_piece = false;
            } else {
                arguments.push(piece_string);
            }
        };
        let command_split: (&str, &str) = first_string.split_at(prefix_length);
        let command: String = command_split.1.to_string();
        Self { command,  arguments }
    }
}

#[cfg(test)]
mod tests {
    use super::ParserPayload;
    #[test]
    fn test_parser() {
        let prefix: String = ">>".to_owned();
        let content: String = ">>ban @someone Didn't like mah spaghooty.".to_owned();
        let payload: ParserPayload = ParserPayload::parse(prefix, content);
        assert_eq!("ban".to_owned(), payload.command);
    }
}