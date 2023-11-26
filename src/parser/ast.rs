struct AST {
    statements: Vec<Statement>,
}

struct Statement {
    mutability: Mutability,
    identifier: Identifier,
    expression: Expression,
}

enum Mutability {
    Constant,
    Mutable,
}
