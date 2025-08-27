

// use crate::error_handler::{ErrorType, handle_error};


// #[derive(Clone, Hash, PartialEq, Eq)]
// pub enum Token {
//     LEFTPAREN,
//     RIGHTPAREN,    
//     LEFTBRACE,
//     RIGHTBRACE,
//     COMMA,
//     SEMICOLON,
//     DOT,
//     PLUS,
//     MINUS,
//     STAR,
//     SLASH,
//     EQUAL,
//     BANG,
//     BANGEQUAL,
//     EQUALEQUAL,
//     LESS,
//     LESSEQUAL,
//     GREATER,
//     GREATEREQUAL,
//     IDENTIFIER(String),
//     STRING(String),
//     NUMBER(String),
//     AND,
//     CLASS,
//     ELSE,
//     FALSE,
//     FOR,
//     FUN,
//     IF,
//     NIL,
//     OR,
//     PRINT,
//     RETURN,
//     SUPER,
//     THIS,
//     TRUE,
//     VAR,
//     WHILE
// }

// pub fn keywords_map() -> HashMap<&'static str, Token> {
//     HashMap::from([
//         ("and", Token::AND),
//         ("class", Token::CLASS),
//         ("else", Token::ELSE),
//         ("false", Token::FALSE),
//         ("for", Token::FOR),
//         ("fun", Token::FUN),
//         ("if", Token::IF),
//         ("nil", Token::NIL),
//         ("or", Token::OR),
//         ("print", Token::PRINT),
//         ("return", Token::RETURN),
//         ("super", Token::SUPER),
//         ("this", Token::THIS),
//         ("true", Token::TRUE),
//         ("var", Token::VAR),
//         ("while", Token::WHILE)
//     ])
// }


// fn number(symbols: &Vec<char>,  start_char: char, index: &mut usize, n: &usize, line: &u32) -> String {
//     let mut num = String::from(start_char);
//     let mut last_char: char = start_char;
//     while *index < *n {
//         last_char = symbols[*index];
//         if last_char.is_ascii_digit() {
//             num.push(last_char);
//             *index += 1;
//         }
//         else {
//             break;
//         }
//     }
//     if last_char == '.' {
//         while *index < *n {
//             last_char = symbols[*index];
//             if last_char.is_ascii_digit() {
//                 num.push(last_char);
//                 *index += 1;
//             }
//             else {
//                 break;
//             }
//         }
//     }
//     if last_char.is_ascii_alphabetic() || last_char == '_' {
//         handle_error(line, ErrorType::LexicalError, format!("Unexpected character: {last_char}").as_str());
//     }  
//     num  
// }

// fn string(symbols: &Vec<char>, index: &mut usize, n: &usize, line: &u32) -> String {
//     let mut val= String::from("");
//     let mut end_string_found = false;
//     while *index < *n {
//         let c = symbols[*index];
//         if c != '\"' {
//             val.push(c);
//             *index += 1;
//         } 
//         else {
//             end_string_found = true;
//             break;
//         }
//     }
//     if !end_string_found {
//         handle_error(line, ErrorType::LexicalError, "Expected character: \"");
//     }
//     val
// }

// fn identifier(symbols: &Vec<char>, start_char: char, index: &mut usize, n: &usize, line: &u32) -> String {
//     let mut ident = String::from(start_char);
//     let mut last_char: char = start_char;
//     while *index < *n {
//         last_char = symbols[*index];
//         if is_identifier_symbol(last_char) {
//             ident.push(last_char);
//             *index += 1;
//         }
//     }
//     if !(last_char.is_whitespace() || is_identifier_symbol(last_char)) {
//         handle_error(line, ErrorType::LexicalError, format!("Unexpected character: {last_char}").as_str());
//     }
//     ident
// }

// fn is_identifier_symbol(c: char) -> bool {
//     return c.is_ascii_alphanumeric() || c == '_';
// }


// pub fn tokenize(file_text: String) -> Vec<Token> {
//     let mut token_list: Vec<Token> = Vec::new();
//     let code_symbols: Vec<char> = file_text.chars().collect();
//     let mut line = 1;
//     let mut index: usize = 0;
//     let n = code_symbols.len();
//     while index < n {
//         let c = code_symbols[index];
//         if c == '\n' {
//             line += 1;
//             continue;
//         }
//         else if c.is_whitespace() {
//             continue;   
//         }
//         match c {
//             '(' => {
//                 token_list.push(Token::LEFTPAREN);
//             },
//             ')' => {
//                 token_list.push(Token::RIGHTPAREN);
//             }, 
//             '{' => {
//                 token_list.push(Token::LEFTBRACE);
//             },
//             '}' => {
//                 token_list.push(Token::RIGHTBRACE);
//             },
//             ',' => {
//                 token_list.push(Token::COMMA);
//             },
//             ';' => {
//                 token_list.push(Token::SEMICOLON);
//             },
//             '.' => {
//                 token_list.push(Token::DOT);
//             },
//             '+' => {
//                 token_list.push(Token::PLUS);
//             },
//             '-' => {
//                 token_list.push(Token::MINUS);
//             },
//             '/' => {
//                 token_list.push(Token::SLASH);
//             },
//             '*' => {
//                 token_list.push(Token::STAR);
//             },
//             '=' => {
//                 let next_ind = index + 1;
//                 if next_ind < n && code_symbols[next_ind] == '=' {
//                     token_list.push(Token::EQUALEQUAL);
//                     index += 2;
//                     continue;
//                 }
//                 token_list.push(Token::EQUAL);
//             },
//             '!' => {
//                 let next_ind = index + 1;
//                 if next_ind < n && code_symbols[next_ind] == '=' {
//                     token_list.push(Token::BANGEQUAL);
//                     index += 2;
//                     continue;
//                 }
//                 token_list.push(Token::BANG);
//             },
//             '<' => {
//                 let next_ind = index + 1;
//                 if next_ind < n && code_symbols[next_ind] == '=' {
//                     token_list.push(Token::LESSEQUAL);
//                     index += 2;
//                     continue;
//                 }
//                 token_list.push(Token::LESS);
//             },
//             '>' => {
//                 let next_ind = index + 1;
//                 if next_ind < n && code_symbols[next_ind] == '=' {
//                     token_list.push(Token::GREATEREQUAL);
//                     index += 2;
//                     continue;
//                 }
//                 token_list.push(Token::GREATER);
//             },
//             '\"' => {
//                 token_list.push(Token::STRING(string(&code_symbols, &mut index, &n, &line)));
//             },
//             _ => {
//                 if c.is_ascii_digit() {
//                     token_list.push(
//                         Token::NUMBER(number(&code_symbols, c, &mut index, &n,  &line))
//                     );
//                 }
//                 else if c.is_ascii_lowercase() || c == '_' {
//                     let ident = identifier(&code_symbols, c, &mut index, &n, &line);
//                     let keywordsmap = keywords_map();

//                     match keywordsmap.get(ident.as_str()) {
//                         Some(token) => token_list.push(token.clone()),
//                         None => token_list.push(Token::IDENTIFIER(ident)),
//                     } 
//                 }
//                 else {
//                     handle_error(&line, ErrorType::LexicalError, format!("Unexpected character: {c}").as_str());
//                 }
//                 break;
//             }
//         }
//         index += 1;
 
//     }
//     token_list
// }

// pub struct BlockStatement {
//     pub statements: Vec<Box<dyn Statement>>
// }

// impl Statement for BlockStatement  {
//     fn run(&self, state: &mut BlockScopes) {
//         state.start_child_block();
//         for stmt in self.statements.iter() {
//             stmt.run(state);
//         }
//         state.end_child_block();
//     }
// }


// pub struct PartIfStatement {
//     pub condition: Box<dyn Expression>,
//     pub body: Box<dyn Statement>,
// }

// impl Statement for PartIfStatement {
//     fn run(&self, state: &mut BlockScopes) {
//         self.body.run(state);
//     }
// }

// fn get_condition(cond_option: Box<dyn Object>) -> bool {
//     match cond_option.as_bool() {
//         Some(cond) => cond.0,
//         None => false
//     }
// }

// pub struct IfStatement {
//     pub condition: Box<dyn Expression>,
//     pub body: Box<dyn Statement>,
//     pub else_if_options: Vec<PartIfStatement>,
//     pub else_statement: Option<Box<dyn Statement>>
// }

// impl Statement for IfStatement  {
//     fn run(&self, state: &mut BlockScopes) {
    
//         let condition = get_condition(self.condition.evaluate(state));
//         if condition {
//             self.body.run(state);
//             if let Some(_return_val) = state.get_variable(&"return".to_string()) {
//                 return;
//             }
//         }
//         else {
            
//             for stmt in self.else_if_options.iter() {
//                 let condition = get_condition(stmt.condition.evaluate(state));
//                 if condition {
//                     stmt.run(state);
//                     if let Some(_return_val) = state.get_variable(&"return".to_string()) {
//                         return;
//                     }
//                     return;
//                 }               
//             }

//             if let Some(else_stmt) = &self.else_statement {
//                 else_stmt.run(state);
//                 if let Some(_return_val) = state.get_variable(&"return".to_string()) {
//                     return;
//                 }
//             }
//         }
//     }
// }

// pub struct WhileStatement {
//     pub condition: Box<dyn Expression>,
//     pub body: Box<dyn Statement>, 
// }

// impl Statement for WhileStatement  {
//     fn run(&self, state: &mut BlockScopes) {
//         let mut condition = get_condition(self.condition.evaluate(state));
//         while condition {
//             self.body.run(state);
//             if let Some(_return_val) = state.get_variable(&"return".to_string()) {
//                 return;
//             }
//             condition = get_condition(self.condition.evaluate(state));
//         }        
//     }
// }


// pub struct ForStatement {
//     pub init_declaration: Option<VarStatement>,
//     pub init_assignation: Option<ExprStatement>,
//     pub condition: Box<dyn Expression>,
//     pub body: Box<dyn Statement>, 
//     pub last_instruction: Option<Box<dyn Expression>>
// }

// impl Statement for ForStatement  {
//     fn run(&self, state: &mut BlockScopes) {
//         if let Some(init_decl) = &self.init_declaration {
//             init_decl.run(state);
//         }
//         else if let Some(init_assign) = &self.init_assignation {
//             init_assign.run(state);
//         }
//         let mut for_condition = get_condition(self.condition.evaluate(state));
//         while for_condition {
//             self.body.run(state);
//             if let Some(_return_val) = state.get_variable(&"return".to_string()) {
//                 return;
//             }
//             if let Some(last_instruction) = &self.last_instruction {
//                 last_instruction.evaluate(state);
//             }
//             for_condition = get_condition(self.condition.evaluate(state));
//         }        
//     }
// }




// pub struct BlockFuncStatement {
//     pub statements: Vec<Box<dyn Statement>>
// }

// impl Statement for BlockFuncStatement  {

//     fn run(&self, state: &mut BlockScopes) {
//         for stmt in self.statements.iter() {
//             stmt.run(state);
//             if let Some(_return_val) = state.get_variable(&"return".to_string()) {
//                 break;
//             }
//         }
//     }
// }

// use crate::parser::expressions::{CallExpr, GetSetExpr, IdentifierExpr};

//     pub struct GetSetExpr {
//     pub instance: Box<dyn Expression>,
//     pub prop_identifier: String,
//     pub prop_method_params: Option<Vec<Box<dyn Expression>>>,
//     pub value_to_assign: Option<Box<dyn Expression>>,
//     pub line: u32
// }

// impl GetSetExpr {
//     pub fn new(instance: Box<dyn Expression>, prop_identifier: String, prop_method_params: Option<Vec<Box<dyn Expression>>>,
//         value_to_assign: Option<Box<dyn Expression>>, line: u32) -> GetSetExpr {
//             GetSetExpr { 
//                 instance, 
//                 prop_identifier, 
//                 prop_method_params, 
//                 value_to_assign, 
//                 line 
//             }
//         }
// }

    
// fn identifier_expr(&mut self, prec_get_set_expr: Option<Box<dyn Expression>>, token: &Token) -> Box<dyn Expression> {
//         let ident_str = token.lexeme.to_string();
//         if self.current_index + 1 >= self.size {
//             handle_error(&token.line, ErrorType::SyntacticError, "Unexpected end of file");
//             process::exit(SYNTAXIC_ERROR_CODE)
//         }
//         self.next();
//         let next_token = &self.tokens_list[self.current_index];
//         if next_token.token_type == TokenType::EQUAL {
//             self.next();
//             let expr = self.expression();
//             let result_expr: Box<dyn Expression> =  match prec_get_set_expr {
//                 Some(prec_expr) => Box::new(
//                     GetSetExpr::new(prec_expr, ident_str, None, 
//                         Some(expr), next_token.line)
//                 ),
//                 None => Box::new(
//                     IdentifierExpr::new(ident_str, Some(expr), next_token.line)
//                 )
//             };
//             return result_expr;
//         }
//         else if next_token.token_type == TokenType::DOT {
//             let result_expr: Box<dyn Expression> = match prec_get_set_expr {
//                 Some(prec_expr) => Box::new(
//                     GetSetExpr::new(prec_expr, ident_str, None, 
//                         None, next_token.line)
//                 ),
//                 None => Box::new(
//                     IdentifierExpr::new(ident_str, None, next_token.line)
//                 )
//             };
//             self.next();
//             let next_token = &self.tokens_list[self.current_index];
//             return self.identifier_expr(Some(result_expr), next_token);
//         }
//         self.callable_expr(prec_get_set_expr, Some(ident_str), next_token)
//     }

//     fn callable_expr(&mut self, prec_get_set_expr: Option<Box<dyn Expression>>, 
//         prev_callable: Box<dyn Expression>, token: &Token) -> Box<dyn Expression> {
//         if self.current_index > self.size - 2 || self.size == 1 {
//             return prev_callable;
//         }
//         let line = token.line;
//         if token.token_type == TokenType::LEFTPAREN {
//             let mut params: Vec<Box<dyn Expression>> = Vec::new();
//             self.next();
//             if self.current_token().token_type != TokenType::RIGHTPAREN {
//                 loop {
//                     params.push(self.expression());
//                     if self.current_token().token_type != TokenType::COMMA {
//                         break;
//                     } 
//                     self.next();
//                 }
//             }
//             self.check_token(TokenType::RIGHTPAREN, ")");
               
//             let callable = CallExpr::new(prev_callable, params, line);
//             let next_token = self.current_token();
//             return self.callable_expr(prec_get_set_expr, Box::new(callable), next_token);
//         }
//         else if token.token_type == TokenType::DOT {
//             self.next();
//             let token = self.current_token();
//             let ident_str = token.lexeme.to_string();
//             let ident_expr = Box::new(
//                 IdentifierExpr {
//                     ident_name: ident_str,
//                     value_to_assign: None,
//                     line: token.line
//                 }
//             );
//             let prec_expr = match prec_get_set_expr {
//                 Some(expr) => {
//                     Box::new(
//                         GetSetExpr::new(prec_expr, ident_str, None, 
//                         None, token.line)
//                     )
//                 },
//                 None => func_obj_expr
//             };
            
//             return self.callable_expr(Some(prec_expr), ident_expr);
//         }
//         match prec_get_set_expr {
//             Some(expr) => Box::new(
//                 GetSetExpr::new(prec_expr, ident_str, None, 
//                 None, token.line)
//             ),
//             None => func_obj_expr
//         }
//     }

// impl Expression for GetSetExpr  {

//     fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {
//         let mut instance_class = self.instance.evaluate(state_scope);
//         if instance_class.get_type() != Type::CLASSINSTANCE {
//             process::exit(RUNTIME_ERROR_CODE);
//         }
//         let instance = instance_class.as_class_instance().unwrap();

//         match &self.value_to_assign {
//             Some(value) => {
//                 let obj_value = value.evaluate(state_scope);
//                 instance.set(&self.prop_identifier, obj_value.dyn_clone());
//                 obj_value
//             },
//             None => {
//                 match &self.prop_method_params {
//                     Some(params) => {
//                         let callable_val = instance.get(&self.prop_identifier);
//                         if callable_val.get_type() == Type::FUNCTION {
//                             let func = callable_val.as_function().unwrap();
//                             func.call(&params, state_scope, &self.line)
//                         }
//                         else {
//                             handle_error(&self.line, ErrorType::RuntimeError, 
//                                 "Can only call functions and classes.");
//                             process::exit(RUNTIME_ERROR_CODE); 
//                         }
//                     },
//                     None => {
//                         instance.get(&self.prop_identifier)
//                     }
//                 }
//             }
//         }
//     }   

//     fn contains_identifier(&self, ident: &String) -> bool {
//         self.instance.contains_identifier(ident)
//     }

//     fn to_string(&self) -> String {
//         self.instance.to_string()
//     }
// }
