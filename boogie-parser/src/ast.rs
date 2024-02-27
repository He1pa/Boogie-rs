// https://boogie-docs.readthedocs.io/en/latest/LangRef.html#grammar

#[derive(Debug, Clone)]
pub struct OneMore<T> {
    pub first: T,
    pub other: Vec<T>,
}

// boogie_program ::=  { axiom_decl | const_decl | func_decl | impl_decl | proc_decl |  type_decl | var_decl }
#[derive(Debug, Clone)]
pub struct BoogieProgram {
    pub declares: Vec<Declare>,
}

#[derive(Debug, Clone)]
pub enum Declare {
    AxiomDecl(AxiomDecl),
    ConstDecl(),
    FuncDecl(),
    ImplDecl(),
    ProcDecl(),
    TypeDecl(),
    VarDecl(VarDecl),
}

// "axiom" { AttrOrTrigger } proposition ";"
#[derive(Debug, Clone)]
pub struct AxiomDecl {
    // pub attrs: Vec<AttrOrTrigger>,
    // pub proposition: Proposition,
}

// "var" { attr } typed_idents_wheres ";"
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub attrs: Vec<Attr>,
    pub typed_idents_wheres: TypedIdentWheres,
}

// typed_idents_wheres ::=  typed_idents_where { "," typed_idents_where }
pub type TypedIdentWheres = OneMore<TypedIdentsWhere>;

// typed_idents_where ::=  typed_idents [ "where" expr ]
#[derive(Debug, Clone)]
pub struct TypedIdentsWhere {
    pub typed_idents: TypedIdents,
    pub expr: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct TypedIdents {
    pub idents: Idents,
    pub r#type: String,
}

// type ::=  ( type_atom | ident [ type_args ] | map_type )
#[derive(Debug, Clone)]
pub enum Type {
    TypeAtom(),
    IdentTypeArgs(Ident, Option<TypeArgs>),
    MapType(MapType),
}

// attr ::=  attr_or_trigger
pub type Attr = AttrOrTrigger;

// attr_or_trigger ::=  "{" ( ":" ident [ attr_param { "," attr_param } ] | exprs ) "}"
#[derive(Debug, Clone)]
pub enum AttrOrTrigger {
    Attr(Ident, Option<OneMore<AttrParam>>),
    Trigger(Exprs),
}

// exprs ::=  expr { "," expr }
pub type Exprs = OneMore<Expr>;

// idents ::=  ident { "," ident }
pub type Idents = OneMore<Ident>;

// ident ::= [ "\\" ] non_digit { non_digit | digit }
#[derive(Debug, Clone)]
pub struct Ident {
    pub escape: bool,
    pub non_digit: NonDigit,
    pub non_digit_or_digit: Vec<NonDigitOrDigit>,
}

#[derive(Debug, Clone)]
pub enum NonDigitOrDigit {
    NonDigit(NonDigit),
    Digit(Digit),
}
#[derive(Debug, Clone)]
pub enum AttrParam {
    Str(String),
    Expr(Expr),
}

// proposition ::=  expr
pub type Proposition = Expr;

// expr ::=  implies_expr { equiv_op implies_expr }
#[derive(Debug, Clone)]
pub struct Expr {}


// non_digit ::=  ( "A…Z" | "a…z" | "'" | "~" | "#" | "$" | "^" | "_" | "." | "?" | "`" )
pub type NonDigit = String;

// digits ::=  digit { digit }
pub type Digits = OneMore<Digit>;

// digit ::=  "0…9"
pub type Digit = String;

#[derive(Debug, Clone)]
pub struct MapType {}

#[derive(Debug, Clone)]
pub struct TypeArgs {}
