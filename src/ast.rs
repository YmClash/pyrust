



enum ASTNode {
    Program(Vec<Box<ASTNode>>),
    Statement(Box<ASTNode>),
    FunctionDef{
        name: String,
        params: Vec<String,String>,
        return_type: Option<String>,
        body:Box<ASTNode>
    },
    StructDef{
        name: String,
        fields: Vec<String,String>      // name, type
    },

    // Statements
    LetStatement{
        name: String,
        mutable: bool,
        type_annotation: Option<String>,
        value: Box<ASTNode>
    },
    ifStatement{
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    WhileStatement{
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
}