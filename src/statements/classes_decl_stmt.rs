use crate::class::Class;
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::scanner::declarations::TokenType;
use crate::statements::function_stmt::func_decl;
use crate::statements::ClassDeclStatement;


pub fn class_decl_statement(interpreter: &mut Interpreter) -> ClassDeclStatement {
    interpreter.parser.next();
    let class_name = interpreter.parser.current_token().lexeme.to_string();
    interpreter.parser.next();        

    interpreter.parser.check_token(TokenType::LEFTBRACE, "{");

    let mut methods = Vec::new();
    let mut constructor: Option<Function> = None;
    while interpreter.parser.current_token().token_type != TokenType::RIGHTBRACE {
        let funcname = interpreter.parser.current_token().lexeme.to_string();
        if funcname == "init" {
            constructor = Some(func_decl(interpreter, true));
            continue;
        }
        methods.push(func_decl(interpreter, false));
    }
    
    interpreter.parser.check_token(TokenType::RIGHTBRACE, "}");

    let class_obj = Class {
        name: class_name,
        methods,
        constructor
    };
    ClassDeclStatement {
        class: class_obj
    }
}

