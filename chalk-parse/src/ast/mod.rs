use lalrpop_intern::InternedString;

mod free_variables;
mod impls;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        Span { lo: lo, hi: hi }
    }
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Fact(Application),
    Rule(Rule),
}

#[derive(Debug)]
pub struct Rule {
    pub span: Span,
    pub consequence: Application,
    pub condition: Fact,
}

#[derive(Debug)]
pub struct Fact {
    pub span: Span,
    pub data: Box<FactData>
}

// A Fact looks something like one of these things:
// - `a + b`
// - `a |- c : T`
// - `a |- c : T with: out`
#[derive(Debug)]
pub enum FactData {
    And(Fact, Fact),
    Or(Fact, Fact),

    Not(Fact),

    Implication(Fact, Fact), // A => B
    Exists(Variable, Fact), // exists(x: A)
    ForAll(Variable, Fact), // forall(x: A)

    Apply(Application),
}

#[derive(Debug)]
pub struct Application {
    pub span: Span,
    pub bits: Vec<Bit>,
}

#[derive(Debug)]
pub struct Bit {
    pub span: Span,
    pub kind: BitKind
}

#[derive(Debug)]
pub enum BitKind {
    Value(Value),
    Operator(Operator),
}

// Component of a fact
#[derive(Debug)]
pub struct Value {
    pub span: Span,
    pub kind: ValueKind,
}

#[derive(Debug)]
pub enum ValueKind {
    Atom(Atom),
    Variable(Variable),
    Application(Application),
    Wildcard,
}

// `+`, `|-`, or `foo:`
#[derive(Debug)]
pub enum Operator {
    Colon(InternedString), // Foo:...
    Parens(InternedString), // Foo(...)
    SquareBrackets(Option<InternedString>), // Foo[...] or [...]
    Symbols(InternedString), // arbitrary stuff, like +
}

// `foo` or `bar`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Atom {
    pub id: InternedString
}

// `Foo` or `Bar`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub id: InternedString
}

