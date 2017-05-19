use std::str;

named!(pub lex_lparen<&str, &str>,
    tag!("(")
);

named!(pub lex_rparen<&str, &str>,
    tag!(")")
);

named!(pub lex_word<&str, &str>,
    re_find!(r"^(?:[[:word:]]|/|-)+")
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_word_alpha() {
        assert_eq!(lex_word("ls more").unwrap(), (" more", "ls"));
    }

    #[test]
    fn lex_word_with_slash() {
        assert_eq!(lex_word("/bin/echo more").unwrap(), (" more", "/bin/echo"));
    }

    #[test]
    fn lex_word_with_dash() {
        assert_eq!(lex_word("-lol more").unwrap(), (" more", "-lol"));
    }

    #[test]
    fn lex_left_parenthesis() {
        assert_eq!(lex_lparen("(a").unwrap(), ("a", "("));
    }

    #[test]
    fn lex_right_parenthesis() {
        assert_eq!(lex_rparen(")f").unwrap(), ("f", ")"));
    }
}
