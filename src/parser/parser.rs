#[allow(dead_code)]
use crate::lexer::lex::{SyntaxMode, Token};
use crate::parser::ast::{ASTNode, Attribute, BinaryOperation, Block, ClassDeclaration, ConstanteDeclaration, Declaration, EnumDeclaration, Expression, Field, Function, FunctionDeclaration, FunctionSignature, Identifier, Literal, Operator, Parameters, ReturnStatement, Statement, StructDeclaration, TraitDeclaration, Type, TypeCast, UnaryOperation, UnaryOperator, VariableDeclaration};
use crate::parser::parser_error::ParserErrorType::{
    ExpectColon, ExpectFunctionName, ExpectIdentifier, ExpectOperatorEqual, ExpectParameterName,
    ExpectValue, ExpectVariableName, ExpectedCloseParenthesis, ExpectedOpenParenthesis,
    ExpectedTypeAnnotation, InvalidFunctionDeclaration, InvalidTypeAnnotation,
    InvalidVariableDeclaration, UnexpectedEOF, UnexpectedEndOfInput, UnexpectedIndentation,
    UnexpectedToken,
};
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
    pub fn current_position(&self) -> Position {
        Position {
            index: self.current,
        }
    }

    pub fn parse_block(&mut self) -> Result<Block, ParserError> {
        match self.syntax_mode {
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


    pub fn parse_indented_block(&mut self) -> Result<Block, ParserError> {
        println!("Début du parsing du bloc indenté");
        let mut statements = Vec::new();
        let initial_indent = self.current_indent_level();

        // Consommer le token INDENT
        if self.match_token(&[TokenType::INDENT]) {
            self.advance();
        }

        while !self.is_at_end() {
            let current_indent = self.current_indent_level();
            if current_indent < initial_indent || self.match_token(&[TokenType::DEDENT]) {
                println!("Fin du parsing du bloc indenté");
                break;
            }

            match self.parse_statement() {
                Ok(statement) => {
                    println!("Instruction parsée : {:?}", statement);
                    statements.push(statement);
                },
                Err(e) => {
                    println!("Erreur lors du parsing de l'instruction : {:?}", e);
                    return Err(e);
                }
            }

            // Consommer les newlines après chaque instruction
            while self.match_token(&[TokenType::NEWLINE]) {
                self.advance();
            }
        }

        // Consommer explicitement le DEDENT si présent
        if self.match_token(&[TokenType::DEDENT]) {
            println!("Consommation du token DEDENT");
            self.advance();
        }

        Ok(Block {
            statements,
            syntax_mode: self.syntax_mode,
            indent_level: Some(initial_indent),
            braces: None,
        })
    }
    pub fn parse_braced_block(&mut self) -> Result<Block, ParserError> {
        println!("Début du parsing du bloc");
        let opening_brace = self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;
        println!("Accolade ouvrante consommée");

        let mut statements = Vec::new();
        while !self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
            if self.is_at_end() {
                return Err(ParserError::new(UnexpectedEndOfInput, self.current_position()));
            }
            let stmt = self.parse_statement()?;
            println!("Instruction parsée : {:?}", stmt);
            statements.push(stmt);

            // Consommer le point-virgule si présent
            if self.match_token(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]) {
                self.advance();
            }

            // Consommer les newlines
            while self.match_token(&[TokenType::NEWLINE]) {
                self.advance();
            }
        }

        let closing_brace = self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;
        println!("Accolade fermante consommée");

        println!("Fin du parsing du bloc");
        Ok(Block {
            statements,
            syntax_mode: self.syntax_mode,
            indent_level: None,
            braces: Some((opening_brace, closing_brace)),
        })
    }

    pub fn get_syntax_mode(&self) -> SyntaxMode {
        self.syntax_mode
    }
    pub fn get_current_indent_level(&self) -> usize {
        *self.indent_level.last().unwrap_or(&0)
    }

    fn parse_type_cast(&mut self, expr: Expression) -> Result<Expression, ParserError> {
        self.consume(TokenType::KEYWORD(Keywords::AS))?;
        let cast_type = self.parse_type()?;
        Ok(Expression::TypeCast(TypeCast {
            expression: Box::new(expr),
            target_type: cast_type,
        }))
    }

    // Cette méthode est un exemple de parsing d'instructions dans un bloc
    fn parse_function_parameters(&mut self) -> Result<Vec<(String, Type)>, ParserError> {
        println!("Début du parsing des paramètres de fonction");
        let mut parameters = Vec::new();

        if !self.match_token(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
            loop {
                let name = self.consume_identifier()?;
                //println!("Nom du paramètre parsé : {}", name);

                self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
                //println!("Deux-points ':' consommés");

                let param_type = self.parse_type()?;
                //println!("Type du paramètre parsé : {:?}", param_type);

                parameters.push((name, param_type));

                if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                    self.consume(TokenType::DELIMITER(Delimiters::COMMA))?;
                    //println!("Virgule consommée, continuation du parsing des paramètres");
                } else {
                    //println!("Fin du parsing des paramètres");
                    break;
                }
            }
        } else {
            println!("Aucun paramètre détecté");
        }

        //println!("Paramètres parsés : {:?}", parameters);
        Ok(parameters)
    }

    #[allow(dead_code)]
    fn parse_declaration(&mut self) -> Result<Declaration, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::LET)]) {
            self.parse_variable_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::FN)]) {
            self.parse_function_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::CONST)]) {
            self.parse_constant_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::ENUM)]) {
            self.parse_enum_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::STRUCT)]) {
            self.parse_struct_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::CLASS)]) {
            self.parse_class_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::IMPL)]) {
            self.parse_impl_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::TRAIT)]) {
            self.parse_trait_declaration()
        } else {// Au lieu de retourner un Statement, nous allons lever une erreur
            Err(ParserError::new(
                ParserErrorType::ExpectedDeclaration,
                self.current_position(),
            ))
        }
    }


    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
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

    #[allow(dead_code)]
    pub fn parse_variable_declaration(&mut self) -> Result<Declaration, ParserError> {
        /* Vérifie et consomme le mot-clé "let" */

        self.consume(TokenType::KEYWORD(Keywords::LET))?;

        // Vérifie si la variable est mutable
        let mutable = if self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]) {
            self.advance(); // Utilisation manuelle ici, car nous ne voulons pas une erreur si "mut" est absent
            true
        } else {
            false
        };

        // Vérifie et consomme l'identifiant de la variable
        let name_token = self
            .current_token()
            .ok_or_else(|| ParserError::new(ExpectVariableName, self.current_position()))?;

        let name = if let TokenType::IDENTIFIER { name: _ } = &name_token.token_type {
            name_token.text.clone()
        } else {
            return Err(ParserError::new(
                ExpectVariableName,
                self.current_position(),
            ));
        };
        self.advance(); // Consomme l'identifiant

        // Vérifie et consomme l'annotation de type après ":"
        let variable_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.advance(); // Consomme-le ":"
            Some(self.parse_type()?) // Parse le type
        } else {
            None // Aucun type spécifié
        };

        // Vérifie et consomme l'opérateur "="
        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;

        // Parse l'expression pour la valeur de la variable
        let value = self
            .parse_expression()
            .or_else(|_| Err(ParserError::new(ExpectValue, self.current_position())))?;

        // Crée et retourne le nœud AST pour la déclaration de variable
        Ok(Declaration::Variable(VariableDeclaration {
            name,
            variable_type,
            value: Some(value),
            mutable,
        }))
    }

    pub fn parse_function_declaration(&mut self) -> Result<Declaration, ParserError> {
        let public_accees = if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]) {
            self.advance();
            true
        } else {
            false
        };

        self.consume(TokenType::KEYWORD(Keywords::FN))?;
        let name = self.consume_identifier()?;
        self.consume(TokenType::DELIMITER(Delimiters::LPAR))?;
        let parameters = self.parse_function_parameters()?;
        self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;

        let return_type = if self.match_token(&[TokenType::OPERATOR(Operators::RARROW)]) {
            self.consume(TokenType::OPERATOR(Operators::RARROW))?;
            let rt = self.parse_type()?;
            Some(rt)
        } else {
            None
        };

        match self.syntax_mode {
            SyntaxMode::Indentation => {
                println!("Parsing function block with indentation syntax");
                self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
                self.consume(TokenType::NEWLINE)?;
            },
            SyntaxMode::Braces => {
                println!("Parsing function block with braces syntax");
                // Optionnellement, vous pouvez consommer un point-virgule ici si votre langage le permet
                if self.match_token(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]) {
                    self.advance();
                }
            }
        }

        let body = self.parse_block()?;
        Ok(Declaration::Function(FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
            public_access: public_accees,
        }))
    }

    pub fn parse_constant_declaration(&mut self) -> Result<Declaration, ParserError> {
        let public_access = if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]){
            self.advance();
            true
        } else {
            false
        };


        self.consume(TokenType::KEYWORD(Keywords::CONST))?;
        let name = self.consume_identifier()?;
        let constant_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;
        let value = self.parse_expression()?;
        match self.syntax_mode {
            SyntaxMode::Indentation => {
                self.consume(TokenType::NEWLINE)?;
            }
            SyntaxMode::Braces => {
                self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON))?;
            }
        }
        Ok(Declaration::Constante(ConstanteDeclaration{
            name,
            constant_type,
            value,
            public_access,
        }))

    }


    pub fn parse_struct_declaration(&mut self) -> Result<Declaration, ParserError> {
        let public_access = if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]) {
            self.advance();
            true
        } else {
            false
        };
        // la declaration d'une structure commence par le mot-cle struct
        // la declaration est uniforme dans les deux modes de syntaxe
        self.consume(TokenType::KEYWORD(Keywords::STRUCT))?;
        let name = self.consume_identifier()?;
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;

        let fields = self.parse_struct_fields()?;
        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;
        // Consommer un newline si nous sommes en mode indentation
        // if self.syntax_mode == SyntaxMode::Indentation {
        //     self.consume(TokenType::NEWLINE)?;
        // }
        Ok(Declaration::Structure(StructDeclaration {
            name,
            fields,
            public_access,
        }))
    }

    // fn parse_struct_fields(&mut self) -> Result<Vec<Field>, ParserError> {
    //     let mut fields = Vec::new();
    //     if self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
    //         return Ok(fields); // Structure vide
    //     }
    //     while !self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
    //         let field = self.parse_struct_field()?;
    //         fields.push(field);
    //
    //         if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
    //             self.advance();
    //         } else if !self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
    //             return Err(ParserError::new(ParserErrorType::ExpectedCommaOrCloseBrace, self.current_position()));
    //         }
    //     }
    //     Ok(fields)
    // }

    fn parse_struct_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        let mut fields = Vec::new();
        if self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
            return Ok(fields); // Structure vide
        }

        loop {
            let field = self.parse_struct_field()?;
            fields.push(field);

            if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                self.advance();
                // En mode indentation, permettre un saut de ligne après la virgule
                if self.syntax_mode == SyntaxMode::Indentation {
                    self.consume_if(TokenType::NEWLINE);
                }
            } else if self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
                break;
            } else {
                return Err(ParserError::new(ParserErrorType::ExpectedCommaOrCloseBrace, self.current_position()));
            }
        }

        Ok(fields)
    }
    fn parse_struct_field(&mut self) -> Result<Field, ParserError> {
        let name = self.consume_identifier()?;
        self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
        let field_type = self.parse_type()?;

        Ok(Field {
            name,
            field_type,
            mutable: false, // Par défaut, les champs ne sont pas mutables
        })
    }


    fn parse_indented_struct_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        let mut fields = Vec::new();
        let indent_level = self.get_current_indent_level();

        while self.get_current_indent_level() > indent_level {
            let field = self.parse_struct_field()?;
            fields.push(field);
            self.consume(TokenType::NEWLINE)?;

            if self.get_current_indent_level() <= indent_level {
                break;
            }
        }

        Ok(fields)
    }

    pub fn parse_class_declaration(&mut self) -> Result<Declaration, ParserError> {
        println!("Debut du parsing de la declaration de classe");
        let public_access = if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]) {
            self.advance();
            true
        } else {
            false
        };

        self.consume(TokenType::KEYWORD(Keywords::CLASS))?;

        // Ajout de logs pour le débogage
        println!("Debut de parsing: current token: {:?}", self.current_token());
        let name = self.consume_identifier()?;
        println!("Nom de la classe: {}", name);

        let parent_classes = self.parse_class_inheritance()?;
        //println!("Parent classes: {:?}", parent_classes);

        println!("Debut du parsing des Attributes");

        let (attribute,methods,constructor) = match self.syntax_mode {
            SyntaxMode::Indentation => {self.parse_indented_class_body()?},
            SyntaxMode::Braces => {self.parse_braced_class_body()?},
        };

        // let (attributes,methods,constructor) = match self.syntax_mode {
        //     SyntaxMode::Indentation => {self.parse_indented_class_body()?},
        //     SyntaxMode::Braces => {self.parse_braced_class_body()?},
        // };


        //maintenant on  vas parse le corps de la classe selon le mode de syntaxe


        let attribute = self.parse_class_fields()?;

        println!("Fin du parsing de la declaration de classe");

        Ok(Declaration::Class(ClassDeclaration{
            name,
            parent_classes,
            attributes,
            constructor,
            methods,
            public_access,
        }))
    }

    fn parse_class_attributes_declaration(&mut self) -> Result<Attribute, ParserError> {
        // let mut attributes = Vec::new();
        self.consume(TokenType::KEYWORD(Keywords::LET))?;

        //let name = self.consume_identifier()?;
        let name = if let TokenType::IDENTIFIER { name } = &self.current_token().unwrap().token_type {
            name.clone()
        } else {
            return Err(ParserError::new(ExpectVariableName, self.current_position()));
        };
        self.advance();

        let variable_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.advance();
            Some(self.parse_type()?)
        }else {
            None
        };
        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;

        println!("Parsing de l'expression de la valeur de l'attribut");
        let value = self
            .parse_expression()
            .or_else(|_| Err(ParserError::new(ExpectValue, self.current_position())))?;

        println!("Fin du parsing de la declaration d'attribut");

        Ok(Attribute {
            name,
            attr_type: variable_type,
            default_value: Some(value),
            mutable,

        })
    }

    fn parse_class_inheritance(&mut self) -> Result<Vec<String>,ParserError>{
        let mut parent_classes = Vec::new();

        if self.match_token(&[TokenType::DELIMITER(Delimiters::LPAR)]){
            self.advance();
            loop {
                let parent = self.consume_identifier()?;
                parent_classes.push(parent.clone());
              //  println!("Nom de classe parent: {}", parent);

                if !self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]){
                    break;
                }
                self.advance();
            }
            self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;
        }
        println!("Classes parentes parsées: {:?}", parent_classes);
        Ok(parent_classes)

    }

    fn parse_indented_class_body(&mut self) -> Result<(Vec<Field>, Vec<FunctionDeclaration>, Option<FunctionDeclaration>), ParserError> {
        self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
        self.consume(TokenType::NEWLINE)?;
        self.consume(TokenType::INDENT)?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut constructor = None;

        while !self.match_token(&[TokenType::DEDENT]) {
            if self.is_at_end() {
                return Err(ParserError::new(ParserErrorType::UnexpectedEOF, self.current_position()));
            }

            if self.match_token(&[TokenType::KEYWORD(Keywords::FN)]) {
                let function = self.parse_function_declaration()?;
                if let Declaration::Function(func) = function {
                    if func.name == "init" {
                        constructor = Some(func);
                    } else {
                        methods.push(func);
                    }
                }
            } else {
                fields.push(self.parse_class_field()?);
            }

            self.consume_newline_or_semicolon()?;
        }

        self.consume(TokenType::DEDENT)?;

        Ok((fields, methods, constructor))
    }
    // Fonction pour parser le corps de la classe en mode accolades
    fn parse_braced_class_body(&mut self) -> Result<(Vec<Field>, Vec<FunctionDeclaration>, Option<FunctionDeclaration>), ParserError> {
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut constructor = None;

        while !self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
            if self.is_at_end() {
                return Err(ParserError::new(ParserErrorType::UnexpectedEOF, self.current_position()));
            }

            if self.match_token(&[TokenType::KEYWORD(Keywords::FN)]) {
                let function = self.parse_function_declaration()?;
                if let Declaration::Function(func) = function {
                    if func.name == "init" {
                        constructor = Some(func);
                    } else {
                        methods.push(func);
                    }
                }
            } else {
                fields.push(self.parse_class_field()?);
            }

            self.consume_newline_or_semicolon()?;
        }

        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;

        Ok((fields, methods, constructor))
    }

    fn parse_class_body(&mut self) -> Result<(Vec<Parameters>, Vec<FunctionDeclaration>), ParserError> {
       todo!()
    }



    fn parse_class_fields(&mut self) -> Result<Parameters,ParserError>{
        todo!()
    }
    fn parse_class_field(&mut self) -> Result<Field, ParserError> {
        println!("Parsing class field , current token: {:?}", self.current_token());
        let mutable = if self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]) {
            self.advance();
            true
        } else {
            false
        };
        let name = self.consume_identifier()?;
        println!("Nom du champ de classe: {}", name);
        self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
        let field_type = self.parse_type()?;
        println!("Type du champ de classe: {:?}", field_type);

        // let default_value = if self.match_token(&[TokenType::OPERATOR(Operators::EQUAL)]) {
        //     self.advance();
        //     Some(self.parse_expression()?)
        // } else {
        //     None
        // };

        Ok(Field {
            name,
            field_type,
            mutable,
            //default_value,
        })


    }

    fn parse_enum_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn parse_enum_variants(&mut self) -> Result<Vec<String>, ParserError> {
        todo!()
    }

    fn parse_indented_enum_variants(&mut self) -> Result<Vec<String>, ParserError> {
       todo!()
    }

    fn parse_impl_declaration(&mut self) -> Result<Declaration,ParserError>{
        self.consume(TokenType::KEYWORD(Keywords::IMPL))?;
        let trait_name = self.consume_identifier()?;
        self.consume(TokenType::KEYWORD(Keywords::FOR))?;
        let for_type = self.consume_identifier()?;
        let methods = self.parse_impl_body()?;
        todo!()
    }

    fn parse_trait_declaration(&mut self) -> Result<Declaration,ParserError>{
        let public_access = if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]) {
            self.advance();
            true
        } else {
            false
        };
        self.consume(TokenType::KEYWORD(Keywords::TRAIT))?;
        let name = self.consume_identifier()?;

        let method_signatures = self.parse_trait_body()?;

        Ok(Declaration::Trait(TraitDeclaration {
            name,
            method_signatures,
            public_access,
        }))
    }

    fn parse_trait_body(&mut self) -> Result<Vec<FunctionSignature>, ParserError> {
        todo!()
    }
    fn parse_impl_body(&mut self) -> Result<Vec<FunctionDeclaration>, ParserError> {
        todo!()
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression : current_token = {:?}", self.current_token());
        let mut left = self.parse_primary()?; // Parse l'expression primaire (comme un identifiant ou un littéral)
        println!("Expression primaire parsée : {:?}", left);

        // Gérer les opérations binaires
        while let Some(operator) = self.match_operator() {
            self.advance(); // Consomme l'opérateur
            println!("Opérateur binaire détecté : {:?}", operator);
            let right = self.parse_primary()?; // Parse l'expression à droite de l'opérateur
            println!("Expression à droite de l'opérateur parsée : {:?}", right);
            left = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        println!("Fin du parsing de l'expression : {:?}", left);
        Ok(left)
    }
    fn match_operator(&mut self) -> Option<Operator> {
        println!("Checking for operator, current token: {:?}", self.current_token());
        match self.current_token()?.token_type {
            TokenType::OPERATOR(Operators::PLUS) => Some(Operator::Addition),
            TokenType::OPERATOR(Operators::MINUS) => Some(Operator::Substraction),
            TokenType::OPERATOR(Operators::STAR) => Some(Operator::Multiplication),
            TokenType::OPERATOR(Operators::SLASH) => Some(Operator::Division),
            _ => None,
        }
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParserError> {
        let expression = self.parse_equality()?;

        if self.match_token(&[TokenType::OPERATOR(Operators::EQUAL)]) {
            let value = self.parse_assignment()?;
            if let Expression::Identifier(name) = expression {
                Ok(Expression::BinaryOperation(BinaryOperation {
                    left: Box::new(Expression::Identifier(name)),
                    operator: Operator::Equal,
                    right: Box::new(value),
                }))
            } else {
                Err(ParserError::new(
                    ParserErrorType::InvalidAssignmentTarget,
                    self.current_position(),
                ))
            }
        } else {
            Ok(expression)
        }
    }

    fn parse_equality(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_comparison()?;

        while self.match_token(&[
            TokenType::OPERATOR(Operators::EQEQUAL),
            TokenType::OPERATOR(Operators::NOTEQUAL),
        ]) {
            let operator = match self.previous().token_type {
                TokenType::OPERATOR(Operators::EQEQUAL) => Operator::Equal,
                TokenType::OPERATOR(Operators::NOTEQUAL) => Operator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_term()?;

        while self.match_token(&[
            TokenType::OPERATOR(Operators::LESS),
            TokenType::OPERATOR(Operators::GREATER),
            TokenType::OPERATOR(Operators::LESSEQUAL),
            TokenType::OPERATOR(Operators::GREATEREQUAL),
        ]) {
            let operator = match self.previous().token_type {
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
    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_factor()?;

        while self.match_token(&[
            TokenType::OPERATOR(Operators::PLUS),
            TokenType::OPERATOR(Operators::MINUS),
        ]) {
            let operator = match self.previous().token_type {
                TokenType::OPERATOR(Operators::PLUS) => Operator::Addition,
                TokenType::OPERATOR(Operators::MINUS) => Operator::Substraction,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expression)
    }
    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_unary()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::STAR),
            TokenType::OPERATOR(Operators::SLASH),
        ]) {
            let operator = match self.previous().token_type {
                TokenType::OPERATOR(Operators::STAR) => Operator::Multiplication,
                TokenType::OPERATOR(Operators::SLASH) => Operator::Division,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expression)
    }
    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token(&[
            TokenType::OPERATOR(Operators::MINUS),
            TokenType::OPERATOR(Operators::EXCLAMATION),
        ]) {
            let operator = match self.previous().token_type {
                TokenType::OPERATOR(Operators::MINUS) => UnaryOperator::Negative,
                TokenType::OPERATOR(Operators::EXCLAMATION) => UnaryOperator::Not,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            return Ok(Expression::UnaryOperation(UnaryOperation {
                operator,
                operand: Box::new(right),
            }));
        }

        self.parse_primary()
    }
    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression primaire, current_token = {:?}", self.current_token());
        if let Some(token) = self.current_token() {
            let expr = match &token.token_type {
                TokenType::INTEGER { value } => {
                    let value = value.clone();
                    Expression::Literal(Literal::Integer { value })
                }
                TokenType::FLOAT { value } => {
                    let value = *value;
                    Expression::Literal(Literal::Float { value })
                }
                TokenType::STRING { value, .. } => {
                    let value = value.clone();
                    Expression::Literal(Literal::String(value))
                }
                TokenType::KEYWORD(Keywords::TRUE) => Expression::Literal(Literal::Boolean(true)),
                TokenType::KEYWORD(Keywords::FALSE) => Expression::Literal(Literal::Boolean(false)),
                TokenType::IDENTIFIER { name } => {
                    let name = name.clone();
                    Expression::Identifier(name)
                }
                TokenType::DELIMITER(Delimiters::LPAR) => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    if let Some(token) = self.current_token() {
                        if matches!(token.token_type, TokenType::DELIMITER(Delimiters::RPAR)) {
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
            self.advance();
            Ok(expr)
        } else {
            Err(ParserError::new(
                UnexpectedEndOfInput,
                self.current_position(),
            ))
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        println!("Début du parsing de l'instruction, Current token: {:?}", self.current_token());

        // Ignorer les newlines au début d'une instruction
        while self.match_token(&[TokenType::NEWLINE]) {
            self.advance();
        }

        if self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
            return Err(ParserError::new(UnexpectedToken, self.current_position()));
        }

        if self.match_token(&[TokenType::KEYWORD(Keywords::RETURN)]) {
            self.parse_return_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::LET)]) {
            // Appeler une nouvelle méthode pour parser la déclaration de variable
            self.parse_variable_declaration_statement()
        }else if self.is_declaration_start() {
            let declaration = self.parse_declaration()?;
            Ok(Statement::Declaration(declaration))
        }else {
            let expr = self.parse_expression()?;
            Ok(Statement::Expression(expr))
        }
    }

    fn is_declaration_start(&self) -> bool {
        self.match_token(&[
            TokenType::KEYWORD(Keywords::LET),
            TokenType::KEYWORD(Keywords::FN),
            TokenType::KEYWORD(Keywords::STRUCT),
            TokenType::KEYWORD(Keywords::CLASS),
            TokenType::KEYWORD(Keywords::ENUM),
        ])
    }
    fn parse_variable_declaration_statement(&mut self) -> Result<Statement, ParserError> {
        let variable_decl = self.parse_variable_declaration()?; // Réutilise la méthode existante
        Ok(Statement::Declaration(variable_decl))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        println!("Parsing return statement");
        self.consume(TokenType::KEYWORD(Keywords::RETURN))?;
        let value = if !self.match_token(&[TokenType::NEWLINE, TokenType::DEDENT, TokenType::EOF]) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        println!("Return statement parsed: {:?}", value);
        Ok(Statement::Return(ReturnStatement { value }))
    }

    fn is_statement_end(&self) -> bool{
        match self.syntax_mode {
            SyntaxMode::Indentation => self.match_token(&[TokenType::NEWLINE,TokenType::DEDENT,TokenType::EOF]),
            SyntaxMode::Braces => self.match_token(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]),
        }
    }

    fn consume_statement_end(&mut self) -> Result<(),ParserError> {
        match self.syntax_mode {
            SyntaxMode::Braces => self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON)).map(|_| ()),
            SyntaxMode::Indentation => {
                if self.match_token(&[TokenType::NEWLINE,TokenType::DEDENT,TokenType::EOF]){
                    Ok(())
                } else {
                    Err(ParserError::new(UnexpectedToken, self.current_position()))
                }
            }

        }
    }




    /// Fonction Utilitaire pour le parser

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        //println!("Advanced to token: {:?}", self.current_token());
    }



    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
        // let _ = self.current >= self.tokens.len();
        // return true;
    }

    // fn peek(&self) ->&Token{
    //     &self.tokens[self.current]
    // }



    fn match_token(&self, types: &[TokenType]) -> bool {
        if let Some(token) = self.current_token() {
            types.iter().any(|t| t == &token.token_type)
        } else {
            false
        }
    }

    fn create_error(&self, error_type: ParserErrorType) -> ParserError {
        ParserError::new(
            error_type,
            Position {
                index: self.current,
            },
        )
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

    pub fn consume_identifier(&mut self) -> Result<String, ParserError> {
        let current_token = self
            .current_token()
            .ok_or_else(|| ParserError::new(UnexpectedEOF, self.current_position()))?;
        if let TokenType::IDENTIFIER { name: _ } = &current_token.token_type {
            let name = current_token.text.clone();
            self.advance();
            Ok(name)
        } else {
            Err(ParserError::new(ExpectIdentifier, self.current_position()))
        }
    }
    // methode utilitaire pour consommer un token s'il est present
    // sans generé d'erreur s'il n'est pas present
    fn consume_if (&mut self,expected:TokenType) -> bool {
        if self.match_token(&[expected.clone()]){
            self.advance();
            true
        } else {
            false
        }
    }
    fn consume_separators(&mut self) -> Result<(), ParserError> {
        match self.syntax_mode {
            SyntaxMode::Indentation => {
                while self.match_token(&[TokenType::NEWLINE]) {
                    self.advance();
                }
            },
            SyntaxMode::Braces => {
                if self.match_token(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]) {
                    self.advance();
                }
            },
        }
        Ok(())
    }

    // fn expect(&mut self, token_type: TokenType) -> Result<&Token, ParserError> {
    //     if let Some(token) = self.current_token(){
    //         if &token.token_type == &token_type {
    //             self.advance();
    //             return Ok(token);
    //         }
    //     }
    //     Err(self.create_error(UnexpectedToken {
    //         expected: token_type,
    //         found: self.current_token().unwrap().token_type.clone(),
    //     }))
    // }

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

    pub fn create_error_with_context(&self, error_type: ParserErrorType) -> ParserError {
        self.print_surrounding_tokens();
        ParserError::new(
            error_type,
            Position {
                index: self.current,
            },
        )
    }

    fn is_class_end(&self) -> bool {
        match self.syntax_mode {
            SyntaxMode::Indentation => self.match_token(&[TokenType::DEDENT]),
            SyntaxMode::Braces => self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]),
        }
    }

    fn consume_newline_or_semicolon(&mut self) -> Result<(), ParserError> {
        match self.syntax_mode {
            SyntaxMode::Indentation => self.consume(TokenType::NEWLINE),
            SyntaxMode::Braces => self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON)),
        }.map(|_| ())
    }



}
//by YmC






///////////////////////fin essai//////////////////////////////

// ////////////////////fin de mon  parse/////////////////////// */
