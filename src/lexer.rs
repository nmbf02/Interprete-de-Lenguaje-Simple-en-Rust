/// Representa todos los posibles tokens del lenguaje.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Equals,         // =
    DoubleEquals,   // ==
    LessThan,       // <
    GreaterThan,    // >
    LParen,
    RParen,
    If,
    While,
    End,
    EOF,
}

/// Analizador léxico: convierte una cadena de texto en una secuencia de tokens.
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    /// Crea una nueva instancia del lexer con una cadena de entrada.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// Retorna el siguiente token del input.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.pos];

        match ch {
            '+' => {
                self.pos += 1;
                Token::Plus
            }
            '-' => {
                self.pos += 1;
                Token::Minus
            }
            '*' => {
                self.pos += 1;
                Token::Star
            }
            '/' => {
                self.pos += 1;
                Token::Slash
            }
            '=' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::DoubleEquals
                } else {
                    self.pos += 1;
                    Token::Equals
                }
            }
            '<' => {
                self.pos += 1;
                Token::LessThan
            }
            '>' => {
                self.pos += 1;
                Token::GreaterThan
            }
            '(' => {
                self.pos += 1;
                Token::LParen
            }
            ')' => {
                self.pos += 1;
                Token::RParen
            }
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "if" => Token::If,
                    "while" => Token::While,
                    "end" => Token::End,
                    _ => Token::Ident(ident),
                }
            }
            _ => {
                self.pos += 1;
                Token::EOF // ignoramos símbolos inválidos silenciosamente
            }
        }
    }

    /// Omite espacios en blanco y comentarios.
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_whitespace() {
                self.pos += 1;
            } else if ch == '#' {
                // Ignorar comentarios hasta el final de la línea
                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    /// Lee un número completo (solo enteros por ahora).
    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let num_str: String = self.input[start..self.pos].iter().collect();
        let value = num_str.parse::<i64>().unwrap_or(0);
        Token::Number(value)
    }

    /// Lee un identificador (letras y números), y lo clasifica si es palabra clave.
    fn read_ident(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric() {
            self.pos += 1;
        }
        self.input[start..self.pos].iter().collect()
    }

    /// Mira el siguiente carácter sin avanzar la posición.
    fn peek(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }
}
