#[allow(dead_code)]
use crate::lexer::lex::{SyntaxMode, Token};
use crate::parser::ast::{Assignment, ASTNode, Attribute, BinaryOperation, Block, BlockSyntax, ClassDeclaration, ConstDeclaration, Constructor, Declaration, EnumDeclaration, Expression, Field, Function, FunctionDeclaration, FunctionSignature, Identifier, Literal, MemberAccess, Operator, Parameters, ReturnStatement, Statement, StructDeclaration, TraitDeclaration, Type, TypeCast, UnaryOperation, UnaryOperator, VariableDeclaration};
use crate::parser::parser_error::ParserErrorType::{ExpectColon, ExpectFunctionName, ExpectIdentifier, ExpectOperatorEqual, ExpectParameterName, ExpectValue, ExpectVariableName, ExpectedCloseParenthesis, ExpectedOpenParenthesis, ExpectedTypeAnnotation, InvalidFunctionDeclaration, InvalidTypeAnnotation, InvalidVariableDeclaration, UnexpectedEOF, UnexpectedEndOfInput, UnexpectedIndentation, UnexpectedToken, ExpectedParameterName, InvalidAssignmentTarget};
use crate::parser::parser_error::{ParserError, ParserErrorType, Position};
use crate::tok::{Delimiters, Keywords, Operators, TokenType};

use num_bigint::BigInt;

//////////////////////Debut///////////////////////////

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
    fn parse_block(&mut self, syntax: BlockSyntax) -> Result<Block, ParserError> {
        todo!()
    }

    fn begin_block(&mut self) {
        // self.indent_level.push(self.current_token().unwrap().indent);
        todo!()
    }

    fn end_block(&mut self) {
        // self.indent_level.pop();
        todo!()
    }



    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    /// fonction pour parser les expressions

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn parse_binary_expression(&mut self, precedence: i32) -> Result<Expression, ParserError> {
        todo!()
    }
    fn parse_lambda_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }





    /// fonction pour parser les declarations

    fn parse_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_variable_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_const_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_function_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_struct_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_enum_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_trait_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_class_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    /// fonction pour parser les types
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        todo!()
    }


    fn parse_generic_type(&mut self) -> Result<Type, ParserError> {
        todo!()
    }

    /// fonction  pour parser la mutabilité et la visibilité
    fn parse_mutability(&mut self) -> Result<bool, ParserError> {
        todo!()
    }
    fn parse_visibility(&mut self) -> Result<bool, ParserError> {
        todo!()
    }

    /// fonction pour le gestion de structure de controle
    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
    fn parse_for_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }


    /// fonction pour la gestion des emprunts
    fn parse_borrow(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    // fn parse_annotation(&mut self) -> Result<Attribute, ParserError> {
    //     todo!()
    // }






    // fonction utilitaire pour aide au parsing

    fn is_operator(&self,token_type: &TokenType) -> bool {
        todo!()
    }

    fn get_operator_precedence(&self, operator: &Operator) -> i32 {
        todo!()
    }

    /// fonction pour la gestion des


    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    fn advance(&mut self){
        if !self.is_at_end(){
            self.current += 1;
        }
    }

    fn peek_token(&self) -> Option<&Token>{
        todo!()
    }
    fn peek_next_token(&self) -> Option<&Token>{
        todo!()
    }

    fn previous_token(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool{
        self.current <= self.tokens.len()
    }

    ///  Fonctions de Vérification et de Correspondance des Tokens

    fn match_token(&mut self,expected:&[TokenType]) -> bool {
        if self.check(expected){
            self.advance();
            return true
        } else {
            false
        }
        todo!()
    }

    fn check(&self,expected:&[TokenType]) -> bool {
        todo!()
    }

    pub fn consume(&mut self, expected: TokenType) -> Result<Token, ParserError> {
        // on clone le token actuel pour ne pas avoir de problem avec le borrow checker
        let current_token = self.current_token().cloned().ok_or_else(|| {
            self.print_surrounding_tokens(); // Affiche les tokens autour de l'erreur
            ParserError::new(UnexpectedEOF, self.current_position())
        })?;

        if current_token.token_type == expected {
            self.advance(); // Avance au prochain token
            Ok(current_token.clone()) // Renvoie le token consommé
        } else {
            self.print_surrounding_tokens(); // Affiche les tokens autour de l'erreur
            Err(ParserError::new(UnexpectedToken, self.current_position()))
        }
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

    fn print_surrounding_tokens(&self){
        todo!()
    }

    fn synchronize(&mut self){
        todo!()
    }










}

//by YmC






///////////////////////fin essai//////////////////////////////

// ////////////////////fin de mon  parse/////////////////////// */
