use super::rukinia_result::RukiniaError;

#[derive(Clone)]
pub enum Syntax {
    Not,
    As(String),
    Id(String),
}

impl Syntax {

    pub const NOT: &'static str = "not";
    pub const AS: &'static str = "as";
    pub const ID: &'static str = "id";

    pub fn _get_number_args(&self) -> &'static i8{
        match self {
            Syntax::Not => &0,
            Syntax::As(_) => &1,
            Syntax::Id(_) => &1,
        }
    }

    pub fn get_argument(&self) -> Option<String> {
        match self {
            Syntax::As(argument) => Some(argument.clone()),
            Syntax::Id(argument) => Some(argument.clone()),
            _ => None,
        }
    }

    pub async fn find_syntax_as(expression: Vec<String>) -> Option<Self> {
        let mut iter = expression.iter();

        while let Some(word) = iter.next() {
            match word.as_str() {
                Syntax::AS => {
                    if let Some(argument) = iter.next() {
                        return Some(Syntax::As(argument.clone()));
                    } else {
                        eprintln!("Error: Missing argument for 'as'");
                        return None;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        None
    }

}

#[derive(Clone)]
pub enum SyntaxForTrait{
    Arguments(Vec<Syntax>),
}

impl SyntaxForTrait {
    pub fn contains_not(&self) -> bool {
        match self {
            SyntaxForTrait::Arguments(vector) => vector.iter().any(|s| matches!(s, Syntax::Not)),
        }
    }

    pub fn get_as(&self) -> Option<String> {
        match self {
            SyntaxForTrait::Arguments(vector) => vector.iter()
            .find(|s| matches!(s, Syntax::As(_)))
            .and_then(|s| s.get_argument()),
        }
    }

    pub async fn extract_syntax(expression: Vec<String>) -> Result<(Self, Vec<String>),RukiniaError> {
        let mut syntax = Vec::new();
        let mut iter = expression.iter();

        while let Some(word) = iter.next() {
            match word.as_str() {
                Syntax::NOT => {
                    syntax.push(Syntax::Not);
                }
                Syntax::AS => match iter.next() {
                    Some(argument) => {
                        syntax.push(Syntax::As(argument.clone()));
                    }
                    None => {
                        return Err(RukiniaError::new(
                            "AS label error".to_string(),
                            "Missing argument for AS".to_string(),
                            "No argument found".to_string(),
                        ));
                    }
                },
                Syntax::ID => match iter.next() {
                    Some(argument) => {
                        syntax.push(Syntax::Id(argument.clone()));
                    }
                    None => {
                        return Err(RukiniaError::new(
                            "ID label error".to_string(),
                            "Missing argument for ID".to_string(),
                            "No argument found".to_string(),
                        ));
                    }
                },
                _ => {
                    let mut remaining = Vec::new();
                    remaining.push(word.clone());
                    remaining.extend(iter.map(|s| s.clone()));
                    return Ok((SyntaxForTrait::Arguments(syntax), remaining));
                }
            }
        }
        Ok((SyntaxForTrait::Arguments(syntax), Vec::new()))
    }

}