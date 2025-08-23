use std::process;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::function_manage::fun_declaration;
use crate::interpreter::Interpreter; 
use crate::statements::function_stmt::return_statement;
use crate::statements::simple_statement::{expr_statement, print_statement, var_statement};
use crate::statements::{BlockStatement, ExprStatement, ForStatement, IfStatement, PartIfStatement, Statement, VarStatement, WhileStatement};
use crate::scanner::declarations::TokenType;
use crate::parser::{declarations::Bool, expressions::{Expression, LiteralExpr}};

pub fn block_scope(interpreter: &mut Interpreter) -> BlockStatement {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    interpreter.next();
    while interpreter.parser.current_index < interpreter.parser.size {
        let token = interpreter.parser.current_token();
        match token.token_type {
            TokenType::VAR => {
                stmts.push(Box::new(var_statement(interpreter)));
            },
            TokenType::RIGHTBRACE => {
                interpreter.next();
                return BlockStatement {
                    statements: stmts
                };
            },
            TokenType::FUN => {
                fun_declaration(interpreter);
            },
            _ => stmts.push(block_statements(interpreter, token.token_type))
        } 
    }

    interpreter.parser.current_index -= 1;
    let last_token = interpreter.parser.current_token();
    handle_error(&last_token.line, ErrorType::SyntacticError, 
        format!("Error at {}: Expect '}}'", last_token.lexeme).as_str());
    process::exit(SYNTAXIC_ERROR_CODE);  
}

pub fn statement(interpreter: &mut Interpreter) -> Box<dyn Statement> {
    if interpreter.parser.current_token().token_type == TokenType::FUN {
        fun_declaration(interpreter);
    }
    let token = interpreter.parser.current_token();
    match token.token_type {
        TokenType::VAR => {
            Box::new(var_statement(interpreter))
        },
        _ => block_statements(interpreter, token.token_type)
    } 
}

fn statement_condition(interpreter: &mut Interpreter) -> Box<dyn Statement> {
    let token = interpreter.parser.current_token();
    match token.token_type {
        TokenType::VAR | TokenType::FUN => {
            handle_error(&token.line, ErrorType::SyntacticError, "Error: Expect expression.");
            process::exit(SYNTAXIC_ERROR_CODE)
        },
        _ => block_statements(interpreter, token.token_type)
    } 
}

pub fn block_statements(interpreter: &mut Interpreter, tokentype: TokenType) -> Box<dyn Statement> {
    match tokentype {
        TokenType::IDENTIFIER => {
            Box::new(expr_statement(interpreter))
        },
        TokenType::LEFTBRACE => {
            Box::new(block_scope(interpreter))
        },
        TokenType::IF => {
            Box::new(if_statement(interpreter))
        },
        TokenType::WHILE => {
            Box::new(while_statement(interpreter))
        },
        TokenType::FOR => {
            Box::new(for_statement(interpreter))
        },
        TokenType::PRINT => {
            Box::new(print_statement(interpreter))
        },
        TokenType::RETURN => {
            Box::new(return_statement(interpreter))
        },
        _ => {
            Box::new(expr_statement(interpreter))
        } 
    }
}

pub fn if_statement(interpreter: &mut Interpreter) -> IfStatement {
    interpreter.next();
    let cond_expr = interpreter.parser.expression();
    let if_body = statement_condition(interpreter);
    if interpreter.parser.current_index == interpreter.parser.size {
        return IfStatement {
            condition: cond_expr,
            body: if_body,
            else_if_options: Vec::new(),
            else_statement: None
        };  
    }
    let mut new_token = interpreter.parser.current_token();
    let mut elif_stmts: Vec<PartIfStatement> = Vec::new();
    while new_token.token_type == TokenType::ELSE {
        interpreter.next();
        new_token = interpreter.parser.current_token();

        if new_token.token_type == TokenType::IF {
            interpreter.next();
            let sub_if_cond = interpreter.parser.expression();
            let sub_if_body = statement_condition(interpreter);
            elif_stmts.push(
                PartIfStatement {
                    condition: sub_if_cond,
                    body: sub_if_body
                }
            );
            if interpreter.parser.current_index < interpreter.parser.size {
                new_token = interpreter.parser.current_token();
                continue;
            }
            break;
        }
        else {
            let else_stmt = statement_condition(interpreter);
            return IfStatement {
                condition: cond_expr,
                body: if_body,
                else_if_options: elif_stmts,
                else_statement: Some(else_stmt)
            };
        }
    }
    IfStatement {
        condition: cond_expr,
        body: if_body,
        else_if_options: elif_stmts,
        else_statement: None
    }

}

pub fn while_statement(interpreter: &mut Interpreter) -> WhileStatement {
    interpreter.next();
    let cond_expr = interpreter.parser.expression();
    let while_body = statement_condition(interpreter);
    WhileStatement {
        condition: cond_expr,
        body: while_body,
    } 
}

pub fn for_statement(interpreter: &mut Interpreter) -> ForStatement {
    interpreter.next();
    interpreter.check_token(TokenType::LEFTPAREN, "(");
    let mut var_decl: Option<VarStatement> = None;
    let token = interpreter.parser.current_token();
    let mut assign_decl: Option<ExprStatement> = None;
    if token.token_type == TokenType::VAR {
        var_decl = Some(var_statement(interpreter));
    }
    else if token.token_type == TokenType::IDENTIFIER {
        assign_decl = Some(expr_statement(interpreter));
    }
    else {
        interpreter.check_token(TokenType::SEMICOLON, ";");
    }
    let mut condition: Box<dyn Expression> = Box::new(LiteralExpr{ value: Box::new(Bool(true)) }); 
    if interpreter.parser.current_token().token_type != TokenType::SEMICOLON {
        condition = interpreter.parser.expression();
    }
    interpreter.check_token(TokenType::SEMICOLON, ";");
    let mut last_instruction: Option<Box<dyn Expression>> = None;
    if interpreter.parser.current_token().token_type != TokenType::RIGHTPAREN {
        last_instruction = Some(interpreter.parser.expression());
    }
    interpreter.check_token(TokenType::RIGHTPAREN, ")");
    let for_body = statement_condition(interpreter);
    ForStatement {
        init_declaration: var_decl,
        init_assignation: assign_decl,
        condition: condition,
        body: for_body,
        last_instruction: last_instruction
    }
}
