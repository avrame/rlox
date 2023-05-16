use token::{Token, TokenType};

struct Scanner {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  fn new(source: String) -> Scanner {
    Scanner {
      source,
      tokens: Vec::new(),
      start = 0,
      current = 0,
      line = 1,
    }
  }

  fn scan_tokens() -> Vec<Token> {
    while !self.is_at_end() {
      // We are at the beginning of the next lexeme.
      start = current;
      self.scan_token();
    }

    tokens.push(Token::new(TokenType::EOF, "", None, line));
    return tokens
  }

  fn is_at_end() -> bool {
    return current >= source.len();
  }

  fn scan_token() {
    let c: char = self.advance();
    match c {
      '(' => self.add_token(TokenType::LEFT_PAREN),
      ')' => self.add_token(TokenType::RIGHT_PAREN),
      '{' => self.add_token(TokenType::LEFT_BRACE),
      '}' => self.add_token(TokenType::RIGHT_BRACE),
      ',' => self.add_token(TokenType::COMMA),
      '.' => self.add_token(TokenType::DOT),
      '+' => self.add_token(TokenType::PLUS),
      ';' => self.add_token(TokenType::SEMICOLON),
      '*' => self.add_token(TokenType::STAR),
    }
  }

  fn advance() -> char {
    current += 1;
    return source[current];
  }

  fn add_token(token_type: TokenType) {
    self.add_token_literal(token_type, None);
  }

  fn add_token_literal(token_type: TokenType, literal: String) {
    let text: String = source[start..current].to_string();
    tokens.push(Token::new(token_type, text, literal, line));
  }
}