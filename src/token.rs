#[derive(Debug, Clone)]
pub enum Token {
    TexCommand(String, Location),
    LiaVariable(String, Location),
    LiaKeyword(String, Location),
    LiaMarkDown(String, Location),
    Newline,
    Whitespace(String),
    Nothing(String, Location)
}

impl Token {
    pub fn get_location(&self) -> Location {
        match self {
            Token::TexCommand(_, loc) => *loc,
            Token::LiaVariable(_, loc) => *loc,
            Token::LiaKeyword(_, loc) => *loc,
            Token::LiaMarkDown(_, loc) => *loc,
            Token::Newline => Location::default(),
            Token::Whitespace(_) => Location::default(),
            Token::Nothing(_, loc) => *loc,
        }
    }

    pub fn is_newline(&self) -> bool {
        match self {
            Token::Newline => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn stringify(&self) -> String {
        format!("{}:{}", self.line, self.column)
    }
}