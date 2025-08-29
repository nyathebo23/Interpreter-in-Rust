use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::compiler::Compiler;
use crate::error_handler::{check_class_keywords_usage, check_init_classfunc_return, check_var_redeclaration, check_var_selfinit, handle_error, ErrorType};
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
    if token.token_type == TokenType::SEMICOLON {
        let nil_expr: Box<dyn Expression> = Box::new(LiteralExpr::new(Box::new(NIL), token.line) );
        compiler.advance();
        return ReturnStatement::new(nil_expr);
    }
    check_init_classfunc_return(&compiler.parser.current_token().line, &compiler.environment);
    let expr: Box<dyn Expression> = compiler.parser.expression();
    check_class_keywords_usage(&expr, &compiler.environment);
    compiler.parser.check_token(TokenType::SEMICOLON, ";");
    ReturnStatement::new(expr)
}

pub fn block_func_statement(compiler: &mut Compiler, func_params: &Vec<String>) -> Vec<Box<dyn Statement>> {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    let mut var_stmts_ident: Vec<String> = Vec::from(func_params.to_vec()); 
    while compiler.not_reach_end() {
        let token = compiler.parser.current_token();
        let line = token.line;
        match token.token_type {
            TokenType::VAR => {
                let var_stmt = var_statement(compiler);
                check_var_redeclaration(&var_stmts_ident, &var_stmt.name, &line);
                check_var_selfinit(&var_stmt.expression, &var_stmt.name, &line);
                var_stmts_ident.push(var_stmt.name.clone());
                stmts.push(Box::new(var_stmt));
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

pub fn func_decl(compiler: &mut Compiler) -> Function {
    let ident_str = compiler.parser.current_token().lexeme.to_string();
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

    compiler.parser.check_token(TokenType::LEFTBRACE, "{");

    let statements = block_func_statement(compiler, &params);

    Function {
        name: ident_str.into(),
        params_names: params.into(),
        statements: Rc::new(statements),
        extra_map: HashMap::new()
    }
}

pub fn func_decl_statement(compiler: &mut Compiler) -> FunctionDeclStatement {
    compiler.advance();
    compiler.environment.start_function();
    let func_decl = FunctionDeclStatement {
        function_decl: func_decl(compiler),
    };
    compiler.environment.end_function();
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