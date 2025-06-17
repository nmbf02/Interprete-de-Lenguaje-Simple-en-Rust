use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

//
// AST: Expresiones
//

/// Representa una expresión (número, variable, operación binaria).
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    /// Evalúa la expresión dentro del contexto de variables.
    pub fn eval(&self, ctx: &Context) -> Result<i64, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Ident(name) => {
                ctx.get_variable(name)
                    .ok_or_else(|| format!("Variable no definida: {}", name))
            }
            Expr::Binary { left, op, right } => {
                let l = left.eval(ctx)?;
                let r = right.eval(ctx)?;
                match op {
                    Token::Plus => Ok(l + r),
                    Token::Minus => Ok(l - r),
                    Token::Star => Ok(l * r),
                    Token::Slash => {
                        if r == 0 {
                            Err("Error: División por cero".to_string())
                        } else {
                            Ok(l / r)
                        }
                    }
                    Token::DoubleEquals => Ok((l == r) as i64),
                    Token::LessThan => Ok((l < r) as i64),
                    Token::GreaterThan => Ok((l > r) as i64),
                    _ => Err(format!("Operador no válido: {:?}", op)),
                }
            }
        }
    }
}

//
// AST: Instrucciones
//

/// Representa una instrucción en el lenguaje.
#[derive(Debug)]
pub enum Statement {
    Assign(String, Expr),
    Print(Expr),
    If(Expr, Vec<Statement>),
    While(Expr, Vec<Statement>),
}

impl Statement {
    /// Ejecuta una instrucción dentro de un contexto.
    pub fn execute(&self, ctx: &mut Context) -> Result<(), String> {
        match self {
            Statement::Assign(name, expr) => {
                let val = expr.eval(ctx)?;
                ctx.set_variable(name, val);
                Ok(())
            }
            Statement::Print(expr) => {
                let val = expr.eval(ctx)?;
                println!("{}", val);
                Ok(())
            }
            Statement::If(cond, body) => {
                if cond.eval(ctx)? != 0 {
                    for stmt in body {
                        stmt.execute(ctx)?;
                    }
                }
                Ok(())
            }
            Statement::While(cond, body) => {
                while cond.eval(ctx)? != 0 {
                    for stmt in body {
                        stmt.execute(ctx)?;
                    }
                }
                Ok(())
            }
        }
    }
}

//
// Parser
//

/// Parser que convierte tokens en instrucciones (AST).
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    /// Crea un nuevo parser con un lexer.
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    /// Avanza al siguiente token.
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// Punto de entrada: parsear todas las instrucciones.
    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    /// Parsea una sola instrucción: asignación, print, if o while.
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.current_token {
            Token::Ident(name) if name == "print" => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Statement::Print(expr))
            }
            Token::Ident(name) => {
                let var_name = name.clone();
                self.advance();
                if self.current_token != Token::Equals {
                    return Err("Se esperaba '=' luego de la variable".to_string());
                }
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Statement::Assign(var_name, expr))
            }
            Token::If => {
                self.advance();
                let condition = self.parse_expression()?;
                let body = self.parse_block()?;
                Ok(Statement::If(condition, body))
            }
            Token::While => {
                self.advance();
                let condition = self.parse_expression()?;
                let body = self.parse_block()?;
                Ok(Statement::While(condition, body))
            }
            _ => Err(format!("Instrucción no válida: {:?}", self.current_token)),
        }
    }

    /// Parsea un bloque de instrucciones hasta encontrar `end`.
    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut block = Vec::new();
        while self.current_token != Token::End && self.current_token != Token::EOF {
            block.push(self.parse_statement()?);
        }
        if self.current_token == Token::End {
            self.advance();
            Ok(block)
        } else {
            Err("Se esperaba 'end' para cerrar el bloque".to_string())
        }
    }

    /// Parsea una expresión, respetando precedencia.
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    /// Comparaciones: ==, <, >
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;

        while matches!(
            self.current_token,
            Token::DoubleEquals | Token::LessThan | Token::GreaterThan
        ) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Suma y resta
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Multiplicación y división
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Valores primarios: números, variables, paréntesis.
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }
            Token::Ident(name) => {
                let ident = name.clone();
                self.advance();
                Ok(Expr::Ident(ident))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                if self.current_token != Token::RParen {
                    return Err("Se esperaba ')'".to_string());
                }
                self.advance();
                Ok(expr)
            }
            _ => Err(format!("Token inesperado en expresión: {:?}", self.current_token)),
        }
    }
}

//
// Contexto
//

/// Contexto de ejecución: almacena las variables y sus valores.
pub struct Context {
    variables: HashMap<String, i64>,
}

impl Context {
    /// Crea un contexto vacío.
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    /// Asigna un valor a una variable.
    pub fn set_variable(&mut self, name: &str, value: i64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Obtiene el valor de una variable.
    pub fn get_variable(&self, name: &str) -> Option<i64> {
        self.variables.get(name).cloned()
    }
}
