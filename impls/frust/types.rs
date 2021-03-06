pub const KEYWORD_ESC: &str = "\u{29e}";
#[derive(Clone, Debug, PartialEq)]
pub enum MalType {
    Deref(Box<MalType>),
    HashMap(
        std::collections::HashMap<String, MalType>,
        Option<Box<MalType>>,
    ),
    // Keyword(String),
    List(Vec<Box<MalType>>, Option<Box<MalType>>),
    Nil,
    Number(f32),
    Quote(Box<MalType>),
    QuasiQuote(Box<MalType>),
    String(String),
    Symbol(String),
    SpliceUnquote(Box<MalType>),
    Unquote(Box<MalType>),
    Vector(Vec<Box<MalType>>, Option<Box<MalType>>),
    //
}

pub type MalFn = Box<dyn Fn(&[MalType]) -> super::MalResult>;
