
use crate::compiler::Compiler;
use crate::error_handler::{handle_error, ErrorType};
use crate::statements::classes_decl_stmt::class_decl_statement;
use crate::statements::function_stmt::{func_decl_statement, return_statement};
use crate::statements::simple_statement::{expr_statement, print_statement, var_statement};
use crate::statements::{ BackToStatement, EndBlockStatement, ExprStatement, GoToStatement, JumpStatement, StartBlockStatement, Statement};
use crate::scanner::declarations::TokenType;
use crate::parser::{declarations::Bool, expressions::{Expression, LiteralExpr}};

pub fn block_scope(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    compiler.environment.start_block();
    stmts.push(Box::new(StartBlockStatement{}));
    compiler.advance();
    while compiler.not_reach_end() {
        let token = compiler.parser.current_token();
        match token.token_type {
            TokenType::VAR => {
                stmts.push(Box::new(var_statement(compiler)));
            },
            TokenType::RIGHTBRACE => {
                compiler.advance();
                stmts.push(Box::new(EndBlockStatement{}));
                compiler.environment.end_block();
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

pub fn statement(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    let token = compiler.parser.current_token();
    match token.token_type {
        TokenType::FUN => {
            let fun_stmt: Box<dyn Statement> = Box::new(func_decl_statement(compiler));
            Vec::from([fun_stmt])
        },
        TokenType::VAR => {
            let var_stmt: Box<dyn Statement> = Box::new(var_statement(compiler));
            Vec::from([var_stmt])
        },
        _ => block_statements(compiler, token.token_type)
    } 
}

fn statement_condition(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    let token = compiler.parser.current_token();
    match token.token_type {
        TokenType::VAR | TokenType::FUN | TokenType::CLASS => {
            handle_error(&token.line, ErrorType::SyntacticError, "Error: Expect expression.");
        },
        _ => block_statements(compiler, token.token_type)
    } 
}

pub fn block_statements(compiler: &mut Compiler, tokentype: TokenType) -> Vec<Box<dyn Statement>> {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    match tokentype {
        TokenType::IDENTIFIER => {
            stmts.push(Box::new(expr_statement(compiler)));
        },
        TokenType::LEFTBRACE => {
            stmts.append(&mut block_scope(compiler));
        },
        TokenType::IF => {
            stmts.append(&mut if_statement(compiler));
        },
        TokenType::WHILE => {
            stmts.append(&mut while_statement(compiler));
        },
        TokenType::FOR => {
            stmts.append(&mut for_statement(compiler));
        },
        TokenType::PRINT => {
            stmts.push(Box::new(print_statement(compiler)));
        },
        TokenType::RETURN => {
            stmts.push(Box::new(return_statement(compiler)));
        },
        TokenType::CLASS => {
            stmts.push(Box::new(class_decl_statement(compiler)));
        },
        _ => {
            stmts.push(Box::new(expr_statement(compiler)));
        } 
    }
    return stmts;
}

pub fn if_statement(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    compiler.advance();
    let cond_expr = compiler.parser.expression();
    compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), cond_expr.get_line());
    let mut if_body = statement_condition(compiler);

    let size_ifblock = if_body.len() + 2;
    let jumpif = jump(cond_expr, size_ifblock);
    let mut stmt_count = size_ifblock;

    let mut result_stmts: Vec<Box<dyn Statement>> = Vec::new();
    result_stmts.push(jumpif);
    result_stmts.append(&mut if_body);
    if compiler.parser.current_index == compiler.parser.size {
        result_stmts.push(go_to( 1));
        return result_stmts;
    }

    let mut new_token = compiler.parser.current_token().clone();
    let mut elif_stmts: Vec<Vec<Box<dyn Statement>>> = Vec::new();
    let mut conditions: Vec<Box<dyn Expression>> = Vec::new();
    let mut sizes_block: Vec<usize> = Vec::new();
    let mut else_stmt = None;
    while new_token.token_type == TokenType::ELSE {
        compiler.advance();
    
        new_token = compiler.parser.current_token().clone();
        
        if new_token.token_type == TokenType::IF {
            compiler.advance();
            let sub_if_cond = compiler.parser.expression();
            compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), sub_if_cond.get_line());

            let sub_if_body = statement_condition(compiler);
            let size_block = sub_if_body.len() + 2;
            stmt_count += size_block;
            conditions.push(sub_if_cond);
            elif_stmts.push(sub_if_body);
            sizes_block.push(size_block);
            if compiler.parser.current_index < compiler.parser.size {
                new_token = compiler.parser.current_token().clone();
                continue;
            }
            break;
        }
        else {
            let else_statement = statement_condition(compiler);
            stmt_count += else_statement.len() + 1;

            else_stmt = Some(else_statement);
        }
    }

    let mut steps_to_end = stmt_count - size_ifblock;
    result_stmts.push(go_to(steps_to_end + 1));

    for ((size_block, stmt), cond) in sizes_block.iter().zip(&mut elif_stmts).zip(conditions) {
        result_stmts.push(jump(cond, *size_block));
        result_stmts.append(stmt);
        steps_to_end = steps_to_end - size_block;
        result_stmts.push(go_to(steps_to_end + 1));
    }
    if let Some(mut else_statement) = else_stmt {
        result_stmts.append(&mut else_statement);
        result_stmts.push(go_to(1));
    }
    result_stmts
}

pub fn while_statement(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    compiler.advance();
    let cond_expr = compiler.parser.expression();
    compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), cond_expr.get_line());

    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    let mut while_body = statement_condition(compiler);

    let size_whileblock = while_body.len() + 2;
    stmts.push(jump(cond_expr, size_whileblock));
    stmts.append(&mut while_body);
    stmts.push(back_to(size_whileblock - 1));
    stmts
}

pub fn for_statement(compiler: &mut Compiler) -> Vec<Box<dyn Statement>> {
    compiler.advance();
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    stmts.push(Box::new(StartBlockStatement{}));
    compiler.environment.start_block();
    compiler.parser.check_token(TokenType::LEFTPAREN, "(");
    let token = compiler.parser.current_token();
    let line = token.line;
    if token.token_type == TokenType::VAR {
        stmts.push(Box::new(var_statement(compiler)));
    }
    else if token.token_type == TokenType::IDENTIFIER {
        stmts.push(Box::new(expr_statement(compiler)));
    }
    else {
        compiler.parser.check_token(TokenType::SEMICOLON, ";");
    }
    let mut condition: Box<dyn Expression> = Box::new(LiteralExpr::new(Box::new(Bool(true)), line)); 
    if compiler.parser.current_token().token_type != TokenType::SEMICOLON {
        condition = compiler.parser.expression();
        compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), condition.get_line());

    }
    let mut body_stmts: Vec<Box<dyn Statement>> = Vec::new();

    compiler.parser.check_token(TokenType::SEMICOLON, ";");
    let mut last_instruction: Option<Box<dyn Expression>> = None;
    if compiler.parser.current_token().token_type != TokenType::RIGHTPAREN {
        let last_expr = compiler.parser.expression();
        compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), last_expr.get_line());
        last_instruction = Some(last_expr);
    }
    compiler.parser.check_token(TokenType::RIGHTPAREN, ")");
    let mut for_body = statement_condition(compiler);
    body_stmts.append(&mut for_body);
    if let Some(expr) = last_instruction {
        let last_stmt = Box::new(ExprStatement{expression: expr});
        body_stmts.push(last_stmt);
    }
    body_stmts.push(back_to(body_stmts.len() + 1));
    stmts.push(jump(condition, body_stmts.len() + 1));
    stmts.append(&mut body_stmts);
    compiler.environment.end_block();
    stmts.push(Box::new(EndBlockStatement{}));

    stmts
}


fn jump(cond: Box<dyn Expression>, steps: usize) -> Box<dyn Statement> {
    Box::new(JumpStatement { 
        condition: cond, 
        steps: steps
    })
}

fn go_to(steps: usize) -> Box<dyn Statement> {
    Box::new(GoToStatement { 
        steps: steps 
    })
}

fn back_to(steps: usize) -> Box<dyn Statement> {
    Box::new(BackToStatement { 
        steps: steps 
    })
}