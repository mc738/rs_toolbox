use crate::commit_linter::types::CommitMessage;

enum ParserState {
    Prefix,
    Type,
    Scope,
    Subject,
    Footers,
}

pub enum ParserError {
    InvalidPrefix,
    InvalidType,
    InvalidScope,
    InvalidSubject,
    InvalidFooters,
}

pub fn run(str: String) -> Result<CommitMessage, ParserError> {
    
    Err(ParserError::InvalidPrefix)
}
