use crate::ast::*;
use crate::val::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    PhpOpen,
    Function,
    Global,
    If,
    Else,
    Foreach,
    As,
    Return,
    Echo,
    ArrayKw,
    Var(String),
    Ident(String),
    Int(i64),
    Float(f64),
    StringLit(String),
    Arrow,          // =>
    DoubleEq,       // ==
    TripleEq,       // ===
    NotEq,          // !=
    Lte,            // <=
    Gte,            // >=
    And,            // &&
    Or,             // ||
    Eq,             // =
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Dot,            // .
    Exclamation,    // !
    Lt,             // <
    Gt,             // >
    Semicolon,      // ;
    Comma,          // ,
    OpenParen,      // (
    CloseParen,     // )
    OpenBracket,    // [
    CloseBracket,   // ]
    OpenBrace,      // {
    CloseBrace,     // }
    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_next() == Some('/') {
                // Line comment
                while let Some(c) = self.advance() {
                    if c == '\n' {
                        break;
                    }
                }
            } else if ch == '/' && self.peek_next() == Some('*') {
                // Block comment
                self.advance(); // /
                self.advance(); // *
                while let Some(c) = self.advance() {
                    if c == '*' && self.peek() == Some('/') {
                        self.advance(); // /
                        break;
                    }
                }
            } else if ch == '#' {
                while let Some(c) = self.advance() {
                    if c == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();
        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        // Check for <?php tag
        if ch == '<' && self.peek_next() == Some('?') {
            let remaining: String = self.chars[self.pos..].iter().collect();
            if remaining.starts_with("<?php") {
                self.pos += 5;
                return Ok(Token::PhpOpen);
            }
        }

        // Variable: $name
        if ch == '$' {
            self.advance(); // consume $
            let mut name = String::new();
            while let Some(c) = self.peek() {
                if c.is_alphanumeric() || c == '_' {
                    name.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            return Ok(Token::Var(name));
        }

        // String literals
        if ch == '\'' || ch == '"' {
            let quote = self.advance().unwrap();
            let mut s = String::new();
            while let Some(c) = self.advance() {
                if c == quote {
                    return Ok(Token::StringLit(s));
                } else if c == '\\' {
                    if let Some(next) = self.advance() {
                        match next {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            '\\' => s.push('\\'),
                            q if q == quote => s.push(q),
                            other => {
                                s.push('\\');
                                s.push(other);
                            }
                        }
                    }
                } else {
                    s.push(c);
                }
            }
            return Err("Unterminated string literal".to_string());
        }

        // Numbers
        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            let mut is_float = false;
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    self.advance();
                } else if c == '.' && !is_float && self.peek_next().map_or(false, |n| n.is_ascii_digit()) {
                    is_float = true;
                    num_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            if is_float {
                let f = num_str.parse::<f64>().map_err(|e| e.to_string())?;
                return Ok(Token::Float(f));
            } else {
                let i = num_str.parse::<i64>().map_err(|e| e.to_string())?;
                return Ok(Token::Int(i));
            }
        }

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            let mut ident = String::new();
            while let Some(c) = self.peek() {
                if c.is_alphanumeric() || c == '_' {
                    ident.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            return match ident.to_lowercase().as_str() {
                "function" => Ok(Token::Function),
                "global" => Ok(Token::Global),
                "if" => Ok(Token::If),
                "else" => Ok(Token::Else),
                "foreach" => Ok(Token::Foreach),
                "as" => Ok(Token::As),
                "return" => Ok(Token::Return),
                "echo" => Ok(Token::Echo),
                "array" => Ok(Token::ArrayKw),
                _ => Ok(Token::Ident(ident)),
            };
        }

        // Two-character operators
        let next = self.peek_next();
        match (ch, next) {
            ('=', Some('>')) => {
                self.pos += 2;
                return Ok(Token::Arrow);
            }
            ('=', Some('=')) => {
                if self.chars.get(self.pos + 2).copied() == Some('=') {
                    self.pos += 3;
                    return Ok(Token::TripleEq);
                }
                self.pos += 2;
                return Ok(Token::DoubleEq);
            }
            ('!', Some('=')) => {
                self.pos += 2;
                return Ok(Token::NotEq);
            }
            ('<', Some('=')) => {
                self.pos += 2;
                return Ok(Token::Lte);
            }
            ('>', Some('=')) => {
                self.pos += 2;
                return Ok(Token::Gte);
            }
            ('&', Some('&')) => {
                self.pos += 2;
                return Ok(Token::And);
            }
            ('|', Some('|')) => {
                self.pos += 2;
                return Ok(Token::Or);
            }
            _ => {}
        }

        // Single character operators
        self.advance();
        match ch {
            ';' => Ok(Token::Semicolon),
            ',' => Ok(Token::Comma),
            '=' => Ok(Token::Eq),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            '.' => Ok(Token::Dot),
            '!' => Ok(Token::Exclamation),
            '<' => Ok(Token::Lt),
            '>' => Ok(Token::Gt),
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '[' => Ok(Token::OpenBracket),
            ']' => Ok(Token::CloseBracket),
            '{' => Ok(Token::OpenBrace),
            '}' => Ok(Token::CloseBrace),
            c => Err(format!("Unexpected character: {}", c)),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok == Token::Eof {
                tokens.push(Token::Eof);
                break;
            }
            // Skip <?php
            if tok != Token::PhpOpen {
                tokens.push(tok);
            }
        }
        Ok(tokens)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if tok != Token::Eof {
            self.pos += 1;
        }
        tok
    }

    fn match_token(&mut self, expected: &Token) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let tok = self.advance();
        if tok == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, tok))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::Eof {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Function => self.parse_function_def(),
            Token::Global => self.parse_global(),
            Token::If => self.parse_if(),
            Token::Foreach => self.parse_foreach(),
            Token::Return => self.parse_return(),
            Token::Echo => self.parse_echo(),
            _ => self.parse_expr_or_assign_stmt(),
        }
    }

    fn parse_function_def(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Function)?;
        let name = match self.advance() {
            Token::Ident(n) => n,
            tok => return Err(format!("Expected function name, got {:?}", tok)),
        };

        self.expect(Token::OpenParen)?;
        let mut params = Vec::new();
        if self.peek() != &Token::CloseParen {
            loop {
                let var_name = match self.advance() {
                    Token::Var(v) => v,
                    tok => return Err(format!("Expected parameter variable name, got {:?}", tok)),
                };
                let mut default_val = None;
                if self.match_token(&Token::Eq) {
                    default_val = Some(self.parse_expr()?);
                }
                params.push((var_name, default_val));
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.expect(Token::CloseParen)?;

        self.expect(Token::OpenBrace)?;
        let mut body = Vec::new();
        while self.peek() != &Token::CloseBrace && self.peek() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;

        Ok(Stmt::FunctionDef { name, params, body })
    }

    fn parse_global(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Global)?;
        let mut vars = Vec::new();
        loop {
            match self.advance() {
                Token::Var(v) => vars.push(v),
                tok => return Err(format!("Expected variable in global statement, got {:?}", tok)),
            }
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Global(vars))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect(Token::If)?;
        self.expect(Token::OpenParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::CloseParen)?;

        self.expect(Token::OpenBrace)?;
        let mut then_branch = Vec::new();
        while self.peek() != &Token::CloseBrace && self.peek() != &Token::Eof {
            then_branch.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;

        let mut else_branch = None;
        if self.match_token(&Token::Else) {
            self.expect(Token::OpenBrace)?;
            let mut else_stmts = Vec::new();
            while self.peek() != &Token::CloseBrace && self.peek() != &Token::Eof {
                else_stmts.push(self.parse_statement()?);
            }
            self.expect(Token::CloseBrace)?;
            else_branch = Some(else_stmts);
        }

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_foreach(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Foreach)?;
        self.expect(Token::OpenParen)?;
        let expr = self.parse_expr()?;
        self.expect(Token::As)?;

        let first_var = match self.advance() {
            Token::Var(v) => v,
            tok => return Err(format!("Expected variable in foreach, got {:?}", tok)),
        };

        let (key_var, val_var) = if self.match_token(&Token::Arrow) {
            let second_var = match self.advance() {
                Token::Var(v) => v,
                tok => return Err(format!("Expected value variable in foreach, got {:?}", tok)),
            };
            (Some(first_var), second_var)
        } else {
            (None, first_var)
        };

        self.expect(Token::CloseParen)?;

        self.expect(Token::OpenBrace)?;
        let mut body = Vec::new();
        while self.peek() != &Token::CloseBrace && self.peek() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;

        Ok(Stmt::Foreach {
            expr,
            key_var,
            val_var,
            body,
        })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Return)?;
        let expr = if self.peek() == &Token::Semicolon {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_echo(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Echo)?;
        let mut exprs = Vec::new();
        loop {
            exprs.push(self.parse_expr()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Echo(exprs))
    }

    fn parse_expr_or_assign_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;
        if self.match_token(&Token::Eq) {
            let rhs = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            match expr {
                Expr::Var(v) => Ok(Stmt::Assign { var: v, expr: rhs }),
                Expr::ArrayAccess { base, dim } => Ok(Stmt::ArrayDimAssign {
                    base,
                    dim: Some(dim),
                    expr: rhs,
                }),
                _ => Err("Invalid assignment target".to_string()),
            }
        } else if let Expr::ArrayAccess { .. } = &expr {
            // Check for empty brackets $arr[] = rhs
            // If dim is dummy or handled
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Expr(expr))
        } else {
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Expr(expr))
        }
    }

    // Expressions
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;
        while self.match_token(&Token::Or) {
            let right = self.parse_logical_and()?;
            left = Expr::BinOp {
                op: BinOpKind::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.match_token(&Token::And) {
            let right = self.parse_equality()?;
            left = Expr::BinOp {
                op: BinOpKind::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_relational()?;
        loop {
            if self.match_token(&Token::DoubleEq) {
                let right = self.parse_relational()?;
                left = Expr::BinOp {
                    op: BinOpKind::Eq,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::TripleEq) {
                let right = self.parse_relational()?;
                left = Expr::BinOp {
                    op: BinOpKind::Identical,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::NotEq) {
                let right = self.parse_relational()?;
                left = Expr::BinOp {
                    op: BinOpKind::NotEq,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_addition()?;
        loop {
            if self.match_token(&Token::Lt) {
                let right = self.parse_addition()?;
                left = Expr::BinOp {
                    op: BinOpKind::Lt,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::Gt) {
                let right = self.parse_addition()?;
                left = Expr::BinOp {
                    op: BinOpKind::Gt,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::Lte) {
                let right = self.parse_addition()?;
                left = Expr::BinOp {
                    op: BinOpKind::Lte,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::Gte) {
                let right = self.parse_addition()?;
                left = Expr::BinOp {
                    op: BinOpKind::Gte,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;
        loop {
            if self.match_token(&Token::Plus) {
                let right = self.parse_multiplication()?;
                left = Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::Minus) {
                let right = self.parse_multiplication()?;
                left = Expr::BinOp {
                    op: BinOpKind::Sub,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::Dot) {
                let right = self.parse_multiplication()?;
                left = Expr::BinOp {
                    op: BinOpKind::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            if self.match_token(&Token::Star) {
                let right = self.parse_unary()?;
                left = Expr::BinOp {
                    op: BinOpKind::Mul,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else if self.match_token(&Token::Slash) {
                let right = self.parse_unary()?;
                left = Expr::BinOp {
                    op: BinOpKind::Div,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&Token::Exclamation) {
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOpKind::Not,
                expr: Box::new(expr),
            });
        }
        if self.match_token(&Token::Minus) {
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&Token::OpenBracket) {
                if self.match_token(&Token::CloseBracket) {
                    // Empty bracket: $arr[] (handled at assign or placeholder)
                    expr = Expr::ArrayAccess {
                        base: Box::new(expr),
                        dim: Box::new(Expr::Literal(Value::Null)), // represents empty []
                    };
                } else {
                    let dim = self.parse_expr()?;
                    self.expect(Token::CloseBracket)?;
                    expr = Expr::ArrayAccess {
                        base: Box::new(expr),
                        dim: Box::new(dim),
                    };
                }
            } else if self.match_token(&Token::OpenParen) {
                // Function call or Dynamic call
                let mut args = Vec::new();
                if self.peek() != &Token::CloseParen {
                    loop {
                        args.push(self.parse_expr()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.expect(Token::CloseParen)?;
                match expr {
                    Expr::Var(v) => {
                        expr = Expr::DynamicCall {
                            callee: Box::new(Expr::Var(v)),
                            args,
                        };
                    }
                    _ => {
                        expr = Expr::DynamicCall {
                            callee: Box::new(expr),
                            args,
                        };
                    }
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Int(i) => Ok(Expr::Literal(Value::Int(i))),
            Token::Float(f) => Ok(Expr::Literal(Value::Float(f))),
            Token::StringLit(s) => Ok(Expr::Literal(Value::String(s))),
            Token::Var(v) => Ok(Expr::Var(v)),
            Token::Ident(name) => {
                let lower = name.to_lowercase();
                if lower == "true" {
                    return Ok(Expr::Literal(Value::Bool(true)));
                } else if lower == "false" {
                    return Ok(Expr::Literal(Value::Bool(false)));
                } else if lower == "null" {
                    return Ok(Expr::Literal(Value::Null));
                }

                // Check for regular function call: foo(...)
                if self.match_token(&Token::OpenParen) {
                    let mut args = Vec::new();
                    if self.peek() != &Token::CloseParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect(Token::CloseParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    // Bare string constant in legacy PHP / identifier
                    Ok(Expr::Literal(Value::String(name)))
                }
            }
            Token::ArrayKw => {
                self.expect(Token::OpenParen)?;
                let items = self.parse_array_elements(Token::CloseParen)?;
                Ok(Expr::ArrayLiteral(items))
            }
            Token::OpenBracket => {
                let items = self.parse_array_elements(Token::CloseBracket)?;
                Ok(Expr::ArrayLiteral(items))
            }
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen)?;
                Ok(expr)
            }
            tok => Err(format!("Unexpected token in expression: {:?}", tok)),
        }
    }

    fn parse_array_elements(&mut self, closing_token: Token) -> Result<Vec<(Option<Expr>, Expr)>, String> {
        let mut items = Vec::new();
        if self.peek() != &closing_token {
            loop {
                let first_expr = self.parse_expr()?;
                if self.match_token(&Token::Arrow) {
                    let val_expr = self.parse_expr()?;
                    items.push((Some(first_expr), val_expr));
                } else {
                    items.push((None, first_expr));
                }
                if !self.match_token(&Token::Comma) {
                    break;
                }
                if self.peek() == &closing_token {
                    break; // allow trailing comma
                }
            }
        }
        self.expect(closing_token)?;
        Ok(items)
    }
}
