#[allow(dead_code)]
use crate::lexer::lex::{SyntaxMode, Token};
use crate::parser::ast::{Assignment, ASTNode, Attribute, BinaryOperation, Block, BlockSyntax, ClassDeclaration, ConstDeclaration, Constructor, Declaration, EnumDeclaration, EnumVariant, Expression, Field, Function, FunctionCall, FunctionDeclaration, FunctionSignature, Identifier, Literal, MemberAccess, Mutability, Operator, Parameter, Parameters, ReturnStatement, Statement, StructDeclaration, TraitDeclaration, Type, TypeCast, UnaryOperation, UnaryOperator, VariableDeclaration, Visibility};
use crate::parser::parser_error::ParserErrorType::{ExpectColon, ExpectFunctionName, ExpectIdentifier, ExpectOperatorEqual, ExpectParameterName, ExpectValue, ExpectVariableName, ExpectedCloseParenthesis, ExpectedOpenParenthesis, ExpectedTypeAnnotation, InvalidFunctionDeclaration, InvalidTypeAnnotation, InvalidVariableDeclaration, UnexpectedEOF, UnexpectedEndOfInput, UnexpectedIndentation, UnexpectedToken, ExpectedParameterName, InvalidAssignmentTarget, ExpectedDeclaration};
use crate::parser::parser_error::{ParserError, ParserErrorType, Position};
use crate::tok::{Delimiters, Keywords, Operators, TokenType};

use num_bigint::BigInt;
use crate::parser::ast::Declaration::Variable;
//use crate::tok::TokenType::EOF;
//////////////////////Debut///////////////////////////


#[allow(dead_code)]
enum Associativity {
    Left,
    Right,
}

pub struct Parser {
    tokens: Vec<Token>, // liste des tokens genere par le lexer
    current: usize,     // index du token actuel
    syntax_mode: SyntaxMode,
    indent_level: Vec<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, syntax_mode: SyntaxMode) -> Self {
        Parser {
            tokens,
            current: 0,
            syntax_mode,
            indent_level: vec![0],
        }
    }

    fn parse_program(&mut self) -> Result<ASTNode, ParserError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(ASTNode::Program(statements))
    }


    pub fn current_position(&self) -> Position {
        Position {
            index: self.current,
        }
    }

    /// fonction pour aider le parsing des blocs

    fn get_syntax_mode(&self) ->SyntaxMode{
        self.syntax_mode
    }

    fn parse_block(&mut self) -> Result<ASTNode, ParserError> {
        match self.syntax_mode{
            SyntaxMode::Indentation => self.parse_indented_block(),
            SyntaxMode::Braces => self.parse_braced_block(),

        }
    }
    fn current_indent_level(&self) -> usize {
        if let Some(TokenType::INDENT) = self.current_token().map(|t| &t.token_type) {
            self.get_current_indent_level()
        } else {
            0
        }
    }
    pub fn get_current_indent_level(&self) -> usize {
        *self.indent_level.last().unwrap_or(&0)
    }

    fn parse_indented_block(&mut self) -> Result<ASTNode, ParserError> {
        println!("Parsing indented block");
        self.consume(TokenType::NEWLINE)?;
        self.consume(TokenType::INDENT)?;

        let mut statements = Vec::new();
        let initial_indent = self.current_indent_level();

        while !self.is_at_end() && self.current_indent_level() >= initial_indent {
            if self.match_token(&[TokenType::DEDENT]) {
                break;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // Consommer les newlines après chaque instruction
            while self.match_token(&[TokenType::NEWLINE]) {
                //self.advance();
            }
        }

        self.consume(TokenType::DEDENT)?;
        Ok(ASTNode::Block(Block{
            statements,
            syntax_mode:BlockSyntax::Indentation,
        }))
    }

    fn parse_braced_block(&mut self) -> Result<ASTNode, ParserError> {
        println!("Parsing braced block");
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;

        let mut statements = Vec::new();

        while !self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
            if self.is_at_end() {
                return Err(ParserError::new(UnexpectedEndOfInput, self.current_position()));
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // Consommer le point-virgule si présent
            if self.match_token(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]) {
                //self.advance();
            }

            // Consommer les newlines
            while self.match_token(&[TokenType::NEWLINE]) {
                //self.advance();
            }
        }

        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;

        Ok(ASTNode::Block(Block{
            statements,
            syntax_mode:BlockSyntax::Braces,
        }))

    }

    fn begin_block(&mut self) {
        // self.indent_level.push(self.current_token().unwrap().indent);
        todo!()
    }

    fn end_block(&mut self) {
        // self.indent_level.pop();
        todo!()
    }



    fn parse_statement(&mut self) -> Result<ASTNode, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::RETURN)]) {
            self.parse_return_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::IF)]){
            self.parse_if_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::WHILE)]) {
            self.parse_while_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::FOR)]) {
            self.parse_for_statement()
        } else {
            self.parse_expression_statement()
        }

    }

    /// fonction pour parser les expressions



    pub fn parse_expression(&mut self,precedence:u8) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression");
        //self.parse_binary_expression(0)
        //self.parse_assignment();
        let mut left = self.parse_primary_expression()?;
        //let mut left = self.parse_unary_expression()?;

        while let Some (operator) = self.peek_operator(){
            let operator_precedence = self.get_operator_precedence(&operator);
            if operator_precedence < precedence {
                break;
            }

            // let mut associativty = self.get_operator_associtivity(&operator);
            // let next_precedence = if let Associativity::Left = associativty {
            //     operator_precedence + 1
            // } else {
            //     operator_precedence
            // };

            self.advance();
            let right = self.parse_expression(precedence)?;
            left = Expression::BinaryOperation(BinaryOperation{
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });

        }

        println!("Fin du parsing de l'expression ");

        Ok(left)

    }

    pub fn parse_expression_statement(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de l'expression statement");
        let expr = self.parse_expression(0);
        println!("Expression parsée : {:?}", expr);
        self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON))?;
        println!("Separateur consommé");
        Ok(ASTNode::Expression(expr?))

    }

    fn get_operator_associtivity(&self,operator: &Operator) -> Associativity {
        match operator {
            Operator::Multiplication | Operator::Division | Operator::Modulo => Associativity::Left,
            Operator::Addition | Operator::Substraction => Associativity::Left,
            Operator::LessThan | Operator::GreaterThan | Operator::LesshanOrEqual | Operator::GreaterThanOrEqual => Associativity::Left,
            Operator::Equal | Operator::NotEqual => Associativity::Right,
            Operator::And | Operator::Or => Associativity::Left,

            _ => Associativity::Left,
        }
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression unaire");
        println!("Début du parsing de l'expression unaire, current_token = {:?}", self.current_token());


        if self.match_token(&[
            TokenType::OPERATOR(Operators::MINUS),
            TokenType::OPERATOR(Operators::EXCLAMATION),
            TokenType::OPERATOR(Operators::AMPER),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::MINUS) => UnaryOperator::Negative,
                TokenType::OPERATOR(Operators::EXCLAMATION) => UnaryOperator::Not,
                TokenType::OPERATOR(Operators::AMPER) => UnaryOperator::Reference,
                _ => unreachable!(),
            };

            println!("Opérateur unaire parsé : {:?}", operator);

            let right = self.parse_unary_expression()?;
            return Ok(Expression::UnaryOperation(UnaryOperation {
                operator,
                operand: Box::new(right),
            }));
        }
        println!("Aucun opérateur unaire trouvé, passage à l'expression pos");
        self.parse_postfix()

    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression primaire, current_token = {:?}", self.current_token());
        if let Some(token) = self.current_token() {
            let expr = match &token.token_type {
                TokenType::INTEGER { value } => {
                    let value = value.clone();
                    println!("Valeur entière parsée : {}", value);
                    self.advance();
                    Expression::Literal(Literal::Integer { value })
                }
                TokenType::FLOAT { value } => {
                    let value = *value;
                    println!("Valeur flottante parsée : {}", value);
                    self.advance();
                    Expression::Literal(Literal::Float { value })
                }
                TokenType::STRING { value, .. } => {
                    let value = value.clone();
                    println!("Valeur de chaîne parsée : {}", value);
                    self.advance();
                    Expression::Literal(Literal::String(value))
                }
                TokenType::KEYWORD(Keywords::TRUE) => {
                    self.advance(); // Consomme le token
                    Expression::Literal(Literal::Boolean(true))
                }
                TokenType::KEYWORD(Keywords::FALSE) => {
                    self.advance(); // Consomme le token
                    Expression::Literal(Literal::Boolean(false))
                }
                // TokenType::KEYWORD(Keywords::TRUE) => Expression::Literal(Literal::Boolean(true)),
                // TokenType::KEYWORD(Keywords::FALSE) => Expression::Literal(Literal::Boolean(false)),

                TokenType::KEYWORD(Keywords::SELF) =>{
                    self.advance();
                    let name = "self".to_string();
                    Expression::Identifier(name)

                }


                TokenType::IDENTIFIER { name } => {
                    let name = name.clone();
                    self.advance();
                    Expression::Identifier(name)
                }
                // TokenType::DELIMITER(Delimiters::LPAR) => {
                //     self.advance(); // Consomme '('
                //     let expr = self.parse_expression()?;
                //     self.consume(TokenType::DELIMITER(Delimiters::RPAR))?; // Consomme ')'
                //     expr
                // }
                TokenType::DELIMITER(Delimiters::LPAR) => {
                    self.advance();
                    let expr = self.parse_expression(0)?;
                    if let Some(token) = self.current_token() {
                        if matches!(token.token_type, TokenType::DELIMITER(Delimiters::RPAR)) {
                            self.advance();
                            expr
                        } else {
                            return Err(ParserError::new(
                                ExpectedCloseParenthesis,
                                self.current_position(),
                            ));
                        }
                    } else {
                        return Err(ParserError::new(
                            UnexpectedEndOfInput,
                            self.current_position(),
                        ));
                    }
                }
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            };
            Ok(expr)
        } else {
            Err(ParserError::new(
                UnexpectedEndOfInput,
                self.current_position(),
            ))
        }

    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression binaire");
        let mut left = self.parse_unary_expression()?;
        loop {
            println!("Current position: {}", self.current);
            let op = match self.peek_operator() {
                Some(op) => op,
                None => {
                    println!("No operator found, breaking loop");
                    break;
                }
            };
            let precedence = self.get_operator_precedence(&op);
            println!("Operator found: {:?} with precedence {}", op, precedence);
            if precedence < min_precedence {
                break;
            }
            self.advance(); // Consomme l'opérateur
            let right = self.parse_binary_expression(precedence )?;
            left = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });
        }
        println!("Fin du parsing de l'expression binaire");
        Ok(left)
    }
    fn parse_lambda_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'assignation");
        let expression = self.parse_equality()?;

        if self.match_token(&[TokenType::OPERATOR(Operators::EQUAL)]) {
            let value = self.parse_assignment()?;
            match expression {
                Expression::Identifier(name) => Ok(Expression::Assignment(Assignment {
                    left: Box::new(Expression::Identifier(name)),
                    right: Box::new(value),
                })),
                Expression::MemberAccess(member_access) => Ok(Expression::Assignment(Assignment {
                    left: Box::new(Expression::MemberAccess(member_access)),
                    right: Box::new(value),
                })),
                _ => Err(ParserError::new(
                    InvalidAssignmentTarget,
                    self.current_position(),
                )),
            }
        } else {
            Ok(expression)
        }
    }

    fn parse_equality(&mut self) -> Result<Expression,ParserError>{
        println!("Début du parsing de l'égalité");
        let mut expression = self.parse_comparison()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::EQEQUAL),
            TokenType::OPERATOR(Operators::NOTEQUAL)]){
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::EQEQUAL) => Operator::Equal,
                TokenType::OPERATOR(Operators::NOTEQUAL) => Operator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expression)
    }
    fn parse_comparison(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_term()?;

        while self.match_token(&[
            TokenType::OPERATOR(Operators::LESS),
            TokenType::OPERATOR(Operators::GREATER),
            TokenType::OPERATOR(Operators::LESSEQUAL),
            TokenType::OPERATOR(Operators::GREATEREQUAL),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::LESS) => Operator::LessThan,
                TokenType::OPERATOR(Operators::GREATER) => Operator::GreaterThan,
                TokenType::OPERATOR(Operators::LESSEQUAL) => Operator::LesshanOrEqual,
                TokenType::OPERATOR(Operators::GREATEREQUAL) => Operator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expression)

    }
    fn parse_term(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_factor()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::PLUS),
            TokenType::OPERATOR(Operators::MINUS),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::PLUS) => Operator::Addition,
                TokenType::OPERATOR(Operators::MINUS) => Operator::Substraction,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expression = Expression::BinaryOperation(BinaryOperation{
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })

        }
        Ok(expression)
    }
    fn parse_factor(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_unary_expression()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::STAR),
            TokenType::OPERATOR(Operators::SLASH),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::STAR) => Operator::Multiplication,
                TokenType::OPERATOR(Operators::SLASH) => Operator::Division,
                _ => unreachable!(),
            };
            let right = self.parse_unary_expression()?;
            expression = Expression::BinaryOperation(BinaryOperation{
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })

        }
        Ok(expression)
    }

    fn parse_postfix(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_primary_expression()?;
        loop {
            if self.match_token(&[TokenType::DELIMITER(Delimiters::DOT)]){
                let member_name = self.consume_identifier()?;
                expression = Expression::MemberAccess(MemberAccess{
                    object: Box::new(expression),
                    member: member_name,
                });
            // } else if self.match_token(&[TokenType::DELIMITER(Delimiters::LPAR)]) {
            //     // Consomme '('
            //     let arguments = self.parse_arguments_list()?;
            //     expression = Expression::FunctionCall(FunctionCall {
            //         name: Box::new(expression),
            //         arguments: arguments,
            //     });
            //     self.consume(TokenType::DELIMITER(Delimiters::RPAR))?; // Consomme ')'
            // } else {
                break;
            }
        }
        Ok(expression)
    }

    /// fonction pour parser les parametres

    fn parse_arguments_list(&mut self) -> Result<Vec<Expression>, ParserError> {
        todo!()
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, ParserError> {
        println!("Début du parsing des paramètres de fonction");
        let mut parameters = Vec::new();

        if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]){
            // pas de paramètres
            return Ok(parameters);
        }

        if !self.match_token(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
            loop {
                //let name = self.consume_parameter_name()?;
                let name = self.consume_identifier()?;
                println!("Nom du paramètre parsé : {}", name);
                self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
                let param_type = self.parse_type()?;
                println!("Type du paramètre parsé : {:?}", param_type);

                parameters.push(Parameter { name, parameter_type: param_type });

                if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                    continue;
                } else if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
                    break;
                }else {
                    println!("Erreur lors du parsing des paramètres, token actuel : {:?}", self.current_token());
                    return Err(ParserError::new(ExpectedParameterName, self.current_position()));
                }
            }
        }
        println!("Paramètres parsés : {:?}", parameters);
        Ok(parameters)
    }

    fn parse_function_body(&mut self) -> Result<Vec<ASTNode>, ParserError> {
        let mut body = Vec::new();

        match self.syntax_mode {
            SyntaxMode::Braces => {
                self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;
                while !self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
                    let statement = self.parse_statement()?;
                    body.push(statement);
                }
                self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;
            }
            SyntaxMode::Indentation => {
                // Consommer le NEWLINE initial
                self.consume(TokenType::NEWLINE)?;
                self.consume(TokenType::INDENT)?;
                while !self.check(&[TokenType::EOF]) {
                    if self.check(&[TokenType::DEDENT]) {
                        break;
                    }
                    let statement = self.parse_statement()?;
                    body.push(statement);
                }
            }
        }

        Ok(body)
    }




    /// fonction pour parser les declarations

    pub fn parse_declaration(&mut self) -> Result<ASTNode, ParserError> {
        let visibility = self.parse_visibility()?;

        if self.check(&[TokenType::KEYWORD(Keywords::LET)]) {
            self.parse_variable_declaration()
        } else if self.check(&[TokenType::KEYWORD(Keywords::CONST)]) {
            self.parse_const_declaration(visibility)
        } else if self.check(&[TokenType::KEYWORD(Keywords::FN)]) {
            self.parse_function_declaration(visibility)
        } else if self.check(&[TokenType::KEYWORD(Keywords::STRUCT)]) {
            self.parse_struct_declaration(visibility)
        } else if self.check(&[TokenType::KEYWORD(Keywords::ENUM)]) {
            self.parse_enum_declaration(visibility)
        } else if self.check(&[TokenType::KEYWORD(Keywords::TRAIT)]) {
            self.parse_trait_declaration(visibility)
        } else if self.check(&[TokenType::KEYWORD(Keywords::CLASS)]) {
            self.parse_class_declaration(visibility)
        } else if self.check(&[TokenType::KEYWORD(Keywords::IMPL)]) {
            self.parse_impl_declaration()
        } else {
            Err(ParserError::new(ExpectedDeclaration, self.current_position()))
        }
    }


    // fn oo(&mut self) -> Result<ASTNode, ParserError> {
    //     let a: bool;
    //
    //
    //     todo!()
    // }



    /// fonction pour parser les déclarations de variables
    /// Exemple: Brace Mode
    /// // let mut x: int = 5;
    /// // let y: float = 3.14;
    /// // let z = 42;
    /// // let a:bool = true;
    /// Exemple: Indentation Mode
    /// // let mut x: int = 5
    /// // let y: float = 3.14
    /// // let z = 42
    /// // let a:bool = true


    pub fn parse_variable_declaration(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de variable");
        self.consume(TokenType::KEYWORD(Keywords::LET))?;

        let mutability = self.parse_mutability()?;

        let  name = self.consume_identifier()?;
        println!("Nom de la variable parsé : {}", name);
        let variable_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.parse_type()?

        } else {
            Type::Infer
        };
        println!("Type de la variable parsé : {:?}", variable_type);

        println!("Debut de la valeur de la variable");
        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;

        let value = self.parse_expression(0)?;


        self.consume_seperator();
        println!("Valeur de la variable parsée : {:?}", value);

        Ok(ASTNode::Declaration(Declaration::Variable(VariableDeclaration{
            name,
            variable_type: Some(variable_type),
            value: Some(value),
            mutability,
        })))

    }


    pub fn parse_const_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de constante");

        //let visibility = self.parse_visibility()?;

        self.consume(TokenType::KEYWORD(Keywords::CONST))?;

        let name = self.consume_identifier()?;
        let variable_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.parse_type()?
        } else {
            Type::Infer
        };
        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;
        let value = self.parse_expression(0)?;
        self.consume_seperator();

        println!("la valeur de la constante parse : {:?}", value);

        Ok(ASTNode::Declaration(Declaration::Constante(ConstDeclaration{
            name,
            constant_type: Some(variable_type),
            value,
            visibility,
        })))

    }

    pub fn parse_function_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de fonction");
        self.consume(TokenType::KEYWORD(Keywords::FN))?;
        let name = self.consume_identifier()?;
        println!("Nom de la fonction parsé : {}", name);

        self.consume(TokenType::DELIMITER(Delimiters::LPAR))?;

        let parameters = self.parse_function_parameters()?;

        self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;

        let return_type = if self.match_token(&[TokenType::OPERATOR(Operators::RARROW)]) {
            self.parse_type()?
        } else {
            Type::Infer // Ou un type par défaut
        };

        if self.syntax_mode == SyntaxMode::Indentation{
            self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
        }

        let body = self.parse_function_body()?;

        self.consume_seperator();

        Ok(ASTNode::Declaration(Declaration::Function(FunctionDeclaration {
            name,
            parameters,
            return_type: Some(return_type),
            body,
            visibility,
        })))
    }

    pub fn parse_struct_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de structure");


        self.consume(TokenType::KEYWORD(Keywords::STRUCT))?;
        let name = self.consume_identifier()?;
        println!("Nom de la structure parsé : {}", name);
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;

        let fields = self.parse_struct_fields()?;
        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;

        self.consume_seperator();

        Ok(ASTNode::Declaration(Declaration::Structure(StructDeclaration{
            name,
            fields,
            visibility,
        })))

    }

    fn parse_enum_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Debut du parsing de la déclaration d'énumération");
        self.consume(TokenType::KEYWORD(Keywords::ENUM))?;
        let name = self.consume_identifier()?;
        println!("Nom de l'énumération parsé : {}", name);
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;
        let variantes = self.parse_enum_variantes()?;
        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;

        self.consume_seperator();

        println!("Variantes d'énumération parsées");
        Ok(ASTNode::Declaration(Declaration::Enum(EnumDeclaration{
            name,
            variantes,
            visibility,
        })))

    }

    fn parse_trait_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de trait");
        self.consume(TokenType::KEYWORD(Keywords::TRAIT))?;
        let name = self.consume_identifier()?;
        println!("Nom du trait parsé : {}", name);

        todo!()
    }

    fn parse_impl_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_class_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_methode_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    /// fonction pour parser les types
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let token = self
            .current_token()
            .ok_or_else(|| ParserError::new(ExpectedTypeAnnotation, self.current_position()))?;

        println!("Parsing type: {:?}", token);

        match &token.token_type {
            TokenType::KEYWORD(Keywords::INT) => {
                self.advance(); // Consomme le token `int`
                Ok(Type::Int)
            }
            TokenType::KEYWORD(Keywords::FLOAT) => {
                self.advance(); // Consomme le token `float`
                Ok(Type::Float)
            }
            TokenType::KEYWORD(Keywords::BOOL) => {
                self.advance(); // Consomme le token `bool`
                Ok(Type::Bool)
            }
            TokenType::KEYWORD(Keywords::STR) => {
                self.advance(); // Consomme le token `string`
                Ok(Type::String)
            }
            TokenType::KEYWORD(Keywords::CHAR) => {
                self.advance(); // Consomme le token `char`
                Ok(Type::Char)
            }
            _ => {
                println!("Unexpected token: {:?}", token);
                // Si le token actuel n'est pas un type valide, renvoyer une erreur
                Err(ParserError::new(
                    InvalidTypeAnnotation,
                    self.current_position(),
                ))
            }
        }
    }

    fn parse_type_cast(&mut self,expr: Expression) -> Result<Expression, ParserError> {
        todo!()
    }


    fn parse_generic_type(&mut self) -> Result<Type, ParserError> {
        todo!()
    }

    /// fonction  pour parser la mutabilité et la visibilité
    fn parse_mutability(&mut self) -> Result<Mutability, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]){
            Ok(Mutability::Mutable)
        } else {
            Ok(Mutability::Immutable)
        }
    }
    fn parse_visibility(&mut self) -> Result<Visibility, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]){
            Ok(Visibility::Public)
        } else {
            Ok(Visibility::Private)
        }
    }

    ///fonction pour parser les champs de structure STRUCT

    fn parse_struct_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        println!("Début du parsing des champs de structure");
        let mut fields = Vec::new();

        if self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]){
            return Ok(fields)
        }
        // ici  on  gere au cas ou on as  une structure vide
        if self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]){
            return Ok(fields)
        }
        loop {
            let field = self.parse_struct_field()?;
            fields.push(field);
            if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]){
                // ne pas exiger de NEWLINE après la virgule en mode indentation
                let _ = self.match_token(&[TokenType::NEWLINE]);
                // if self.syntax_mode == SyntaxMode::Indentation{
                //     self.consume(TokenType::NEWLINE)?;
                // }
               //continue;
            } else if self.match_token(&[TokenType::NEWLINE])  && self.syntax_mode==SyntaxMode::Indentation{

            } else if self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]){
                break;
            } else {
                return Err(ParserError::new(ExpectColon,self.current_position()))
            }
        }
        println!("Champs de structure parsés : {:?}", fields);
        Ok(fields)

    }
    fn parse_struct_field(&mut self) -> Result<Field, ParserError> {
        let visibility = self.parse_visibility()?;
        println!("Visibilité du champ parsée : {:?}", visibility);
        let name = self.consume_identifier()?;
        println!("Nom du champ parsé : {}", name);
        self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
        let field_type = self.parse_type()?;
        println!("Type du champ parsé : {:?}", field_type);
        Ok(Field{
            name,
            field_type,
            visibility

        })

    }

    fn parse_enum_variantes(&mut self) -> Result<Vec<EnumVariant>,ParserError>{
        println!("Début du parsing des variantes d'énumération");
        let mut variantes = Vec::new();
        if self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]){
            return Ok(variantes)
        }
        if self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]){
            return Ok(variantes)
        }
        loop{
            let variante = self.parse_enum_variant_fields()?;
            variantes.push(variante);
            if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]){

                let _ = self.match_token(&[TokenType::NEWLINE]);

            }else if self.match_token(&[TokenType::NEWLINE]) && self.syntax_mode == SyntaxMode::Indentation{

            }else if self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]){
                break;
            } else {
                return Err(ParserError::new(ExpectColon,self.current_position()))
            }
        }
        println!("Variantes d'énumération parsées : {:?}", variantes);
        Ok(variantes)
    }

    fn parse_enum_variant_fields(&mut self) ->  Result<EnumVariant,ParserError>{
        let visibility = self.parse_visibility()?;
        println!("Visibilité de la variante parsée : {:?}", visibility);
        let name = self.consume_identifier()?;
        println!("Nom de la variante parsée : {}", name);
        self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
        let variante_type = self.parse_type()?;
        println!("Type de la variante parsée : {:?}", variante_type);
        Ok(EnumVariant{
            name,
            variante_type,
            visibility
        })

    }


    /// fonction pour le gestion de structure de controle
    fn parse_if_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }
    fn parse_while_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }
    fn parse_for_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }




    fn parse_return_statement(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de l'instruction de retour");
        //self.consume(TokenType::KEYWORD(Keywords::RETURN))?;
        let value = if !self.match_token(&[TokenType::NEWLINE, TokenType::DEDENT, TokenType::EOF]) {
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        println!("Valeur de retour parsée : {:?}", value);

        Ok(ASTNode::Statement(Statement::Return(ReturnStatement{
            value,
        })))

    }


    /// fonction pour la gestion des emprunts
    fn parse_borrow(&mut self) -> Result<Expression, ParserError> {
        if self.match_token(&[TokenType::OPERATOR(Operators::AMPER)]){
            let mutable = self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]);
            let expression = self.parse_expression(0)?;
            Ok(Expression::UnaryOperation(UnaryOperation{
                operator: if mutable { UnaryOperator::ReferenceMutable} else {UnaryOperator::Reference},
                operand: Box::new(expression),
            }))
        } else {
            self.parse_primary_expression()
        }

    }

    // fn parse_annotation(&mut self) -> Result<Attribute, ParserError> {
    //     todo!()
    // }






    // fonction utilitaire pour aide au parsing

    fn is_operator(&self,token_type: &TokenType) -> bool {

        todo!()
    }

    fn get_operator_precedence(&self, operator: &Operator) -> u8 {
        match operator {
            Operator::Multiplication | Operator::Division | Operator::Modulo => 5,
            Operator::Addition | Operator::Substraction => 4,
            Operator::LessThan | Operator::GreaterThan | Operator::LesshanOrEqual | Operator::GreaterThanOrEqual => 3,
            Operator::Equal | Operator::NotEqual => 2,
            Operator::And | Operator::Or => 1,
            _ => 0,
        }
    }

    fn peek_operator(&self) -> Option<Operator> {
        let token = self.current_token()?;
        println!("Token: {:?}", token);
        match &token.token_type {
            TokenType::OPERATOR(op) => {
                match op {
                    Operators::PLUS => Some(Operator::Addition),
                    Operators::MINUS => Some(Operator::Substraction),
                    Operators::STAR => Some(Operator::Multiplication),
                    Operators::SLASH => Some(Operator::Division),
                    Operators::PERCENT => Some(Operator::Modulo),
                    Operators::LESS => Some(Operator::LessThan),
                    Operators::GREATER => Some(Operator::GreaterThan),
                    Operators::LESSEQUAL => Some(Operator::LesshanOrEqual),
                    Operators::GREATEREQUAL => Some(Operator::GreaterThanOrEqual),
                    Operators::EQEQUAL => Some(Operator::Equal),
                    Operators::NOTEQUAL => Some(Operator::NotEqual),
                    Operators::AND => Some(Operator::And),
                    Operators::OR => Some(Operator::Or),
                    _ => None,
                }
            }
            _ => None,
        }

    }

    /// fonction pour la gestion des


    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn peek_token(&self) -> Option<&Token>{
        self.tokens.get(self.current)
    }
    fn peek_next_token(&self) -> Option<&Token>{
        todo!()
    }

    fn previous_token(&self) -> Option<&Token> {
        if self.current > 0 {
            // &self.tokens(self.current - 1)
            Some(&self.tokens[self.current - 1])
        } else { None }
    }

    pub fn is_at_end(&self) -> bool{
        self.current >= self.tokens.len() || self.current_token().map_or(true, |t| t.token_type == TokenType::EOF)

    }

    ///  Fonctions de Vérification et de Correspondance des Tokens

    fn match_token(&mut self,expected:&[TokenType]) -> bool {
        if self.check(expected){
            self.advance();
            return true
        } else {
            false
        }
    }

    fn check(&self,expected:&[TokenType]) -> bool {
        if let Some(token) = self.current_token(){
            expected.contains(&token.token_type)
        } else {
            false
        }
    }

    fn consume(&mut self, expected: TokenType) -> Result<(), ParserError> {
        if let Some(token) = self.current_token() {
            if token.token_type == expected {
                println!("Consommation du token {:?}", token);
                //self.print_surrounding_tokens();
                self.advance();
                Ok(())
            } else {
                println!("PyRust:!!!!!!!!!!!!!!!!!!!! Erreur: token attendu {:?}, token actuel {:?}", expected, token);
                Err(ParserError::new(UnexpectedToken, self.current_position()))
            }
        } else {
            //self.print_surrounding_tokens();
            println!("PyRust:!!!!!!!!!!!!!!!!: Erreur: fin de l'entrée inattendue");
            Err(ParserError::new(UnexpectedEndOfInput, self.current_position()))
        }
    }

    // pub fn consume(&mut self, expected: TokenType) -> Result<Token, ParserError> {
    //     // on clone le token actuel pour ne pas avoir de problem avec le borrow checker
    //     let current_token = self.current_token().cloned().ok_or_else(|| {
    //         self.print_surrounding_tokens(); // Affiche les tokens autour de l'erreur
    //         ParserError::new(UnexpectedEOF, self.current_position())
    //     })?;
    //
    //     if current_token.token_type == expected {
    //         self.advance(); // Avance au prochain token
    //         Ok(current_token.clone()) // Renvoie le token consommé
    //     } else {
    //         self.print_surrounding_tokens(); // Affiche les tokens autour de l'erreur
    //         Err(ParserError::new(UnexpectedToken, self.current_position()))
    //     }
    // }

    /// fonctontion  pour aider a comsume les tokens

    fn consume_identifier(&mut self) -> Result<String, ParserError> {
        let current_token = self.current_token().ok_or_else(|| ParserError::new(UnexpectedEOF,self.current_position()))?;
        if let TokenType::IDENTIFIER {name:_} = &current_token.token_type{
            let name = current_token.text.clone();
            self.advance();
            Ok(name)
        } else { Err(ParserError::new(ExpectIdentifier,self.current_position())) }

    }

    /// Fonction pour afficher les tokens autour de l'erreur
    pub fn create_error_with_context(&self, error_type: ParserErrorType) -> ParserError {
        self.print_surrounding_tokens();
        ParserError::new(
            error_type,
            Position {
                index: self.current,
            },
        )
    }

    fn print_surrounding_tokens(&self) {
        let prev_token = if self.current > 0 {
            Some(&self.tokens[self.current - 1])
        } else {
            None
        };
        let current_token = self.current_token();
        let next_token = if self.current + 1 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
        } else {
            None
        };
        println!("");
        println!("---------------- Token Error Context--by-YmC ----------");
        if let Some(prev) = prev_token {
            println!("Previous Token: {:?}", prev);
        }
        if let Some(current) = current_token {
            println!("Current Token: {:?}", current);
        }
        if let Some(next) = next_token {
            println!("Next Token: {:?}", next);
        }
        println!("----------------------------------------------------------");
        println!("");
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Some(previous) = self.previous_token() {
                if previous.token_type == TokenType::DELIMITER(Delimiters::SEMICOLON) {
                    return;
                }
            }

            match self.current_token().map(|t| &t.token_type) {
                Some(TokenType::KEYWORD(Keywords::STRUCT)) |
                Some(TokenType::KEYWORD(Keywords::FN)) |
                Some(TokenType::KEYWORD(Keywords::LET)) |
                Some(TokenType::KEYWORD(Keywords::CONST)) |
                Some(TokenType::KEYWORD(Keywords::PUB)) => return,
                _ => { self.advance(); }
            }
        }
    }

    fn consume_seperator(&mut self)  {
        match self.syntax_mode{
            SyntaxMode::Indentation =>{
                println!("Indentation Mode");
                let _ = self.consume(TokenType::NEWLINE) ;
                if self.check(&[TokenType::EOF]){
                    let _ = self.consume(TokenType::EOF);
                }
                if self.check(&[TokenType::DEDENT]){
                    let _ = self.consume(TokenType::DEDENT);
                }
            }
            SyntaxMode::Braces =>{
                println!("Braces Mode");
                let _ = self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON));
            }
        }
    }

    /// fonction pour verifier la sequence de tokens a utiliser plus tard
    pub fn check_sequence(&self, tokens: &[TokenType]) -> bool {
        for (i, token_type) in tokens.iter().enumerate() {
            if self.current + i >= self.tokens.len() || self.tokens[self.current + i].token_type != *token_type {
                return false;
            }
        }
        true
    }

    // pub fn parse_declarations(&mut self) -> Result<Vec<ASTNode>, ParserError> {
    //     let mut declarations = Vec::new();
    //
    //     while !self.is_at_end() {
    //         match self.parse_declaration() {
    //             Ok(decl) => {
    //                 declarations.push(decl);
    //             },
    //             Err(e) => {
    //                 println!("Erreur lors du parsing : {:?}", e);
    //                 self.synchronize();
    //                 continue;
    //             }
    //         }
    //     }
    //
    //     Ok(declarations)
    // }











}

//by YmC






///////////////////////fin essai//////////////////////////////

// ////////////////////fin de mon  parse/////////////////////// */
