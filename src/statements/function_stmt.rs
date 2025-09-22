use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::compiler::Compiler;
use crate::error_handler::{handle_error, ErrorType};
use crate::function::Function;
use crate::parser::declarations::{NIL};
use crate::parser::expressions::{Expression, LiteralExpr};
use crate::scanner::declarations::TokenType;
use crate::statements::controlflow_stmts::block_statements;
use crate::statements::simple_statement::var_statement;
use crate::statements::{FunctionDeclStatement, ReturnStatement, Statement}; 


pub fn return_statement(compiler: &mut Compiler) -> ReturnStatement {
    compiler.advance();
    let token = compiler.parser.current_token();
    compiler.environment.check_return_validity(&token.line);
    if token.token_type == TokenType::SEMICOLON {
        let nil_expr: Box<dyn Expression> = Box::new(LiteralExpr::new(Box::new(NIL), token.line) );
        compiler.advance();
        return ReturnStatement::new(nil_expr);
    }
    let expr: Box<dyn Expression> = compiler.parser.expression();
    compiler.environment.check_constructor_return_validity(&expr.get_line());
    compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), expr.get_line());
    compiler.parser.check_token(TokenType::SEMICOLON, ";");
    ReturnStatement::new(expr)
}

pub fn block_func_statement(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    while compiler.not_reach_end() {
        let token = compiler.parser.current_token();
        match token.token_type {
            TokenType::VAR => {
                stmts.push(Box::new(var_statement(compiler)));
            },
            TokenType::RIGHTBRACE => {
                compiler.advance();
                return stmts;
            },
            TokenType::FUN => {
                stmts.push(Box::new(func_decl_statement(compiler)));
            },
            _ => stmts.append(&mut block_statements(compiler, token.token_type))
        } 
    }

    compiler.parser.current_index -= 1;
    let last_token = compiler.parser.current_token();
    handle_error(&last_token.line, ErrorType::SyntacticError, 
        format!("Error at {}: Expect '}}'", last_token.lexeme).as_str());
}

pub fn func_decl(compiler: &mut Compiler, funcname: String) -> Function {
    compiler.advance();        
    let mut params: Vec<String> = Vec::new();
    compiler.parser.check_token(TokenType::LEFTPAREN, "(");
    let mut current_token = compiler.parser.current_token();
    let line = current_token.line;
    if current_token.token_type != TokenType::RIGHTPAREN {
        loop {
            current_token = compiler.parser.current_token();
            params.push(current_token.lexeme.to_string());
            compiler.parser.check_token(TokenType::IDENTIFIER, "Identifier");
            if compiler.parser.current_token().token_type != TokenType::COMMA {
                break;
            } 
            compiler.advance();
        }
    }
    compiler.parser.check_token(TokenType::RIGHTPAREN, ")");

    has_duplicates_elmts(&params, line);

    compiler.environment.set_func_params(&params);

    compiler.parser.check_token(TokenType::LEFTBRACE, "{");

    let statements = block_func_statement(compiler);
    Function {
        name: funcname.into(),
        params_names: params.into(),
        statements: Rc::new(statements),
        extra_map: HashMap::new()
    }
}

pub fn func_decl_statement(compiler: &mut Compiler) -> FunctionDeclStatement {
    compiler.advance();
    let ident_str = compiler.parser.current_token().lexeme.to_string();
    compiler.environment.start_function(&ident_str);
    let func = func_decl(compiler, ident_str);
    let extern_declarations = compiler.environment.end_function();

    let func_decl = FunctionDeclStatement {
        function_decl: func,
        extern_variables: extern_declarations
    };
    func_decl
}

fn has_duplicates_elmts(vec: &Vec<String>, line: u32) -> bool {
    let mut seen = HashSet::new();
    for item in vec {
        if !seen.insert(item.clone()) {
            handle_error(&line, ErrorType::SyntacticError, 
                format!("Error at {}: Already a variable with this name in this scope.", item.clone()).as_str());
        }
    }
    false
}