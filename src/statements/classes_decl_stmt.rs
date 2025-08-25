use crate::class_manage::Class;
use crate::interpreter::Interpreter;
use crate::scanner::declarations::TokenType;
use crate::statements::ClassDeclStatement;


pub fn class_decl_statement(interpreter: &mut Interpreter) -> ClassDeclStatement {
    interpreter.parser.next();
    let class_name = interpreter.parser.current_token().lexeme.to_string();
    interpreter.parser.next();        


    interpreter.parser.check_token(TokenType::LEFTBRACE, "{");
    interpreter.parser.check_token(TokenType::RIGHTBRACE, "}");

    let class_obj = Class {
        name: class_name
    };
    ClassDeclStatement {
        class: class_obj
    }
}