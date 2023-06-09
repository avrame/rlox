mod token {
  pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
  }

  pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
  }

  #[derive(Debug)]
  pub impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Token {
      Token {
        token_type,
        lexeme,
        literal,
        line,
      }
    }

    fn to_string(&self) -> String {
      format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
  }
}