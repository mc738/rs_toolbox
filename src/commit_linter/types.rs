pub struct CommitMessage {
    is_valid: bool,
    prefix: Option<String>,
    type_value: Option<String>,
    scope: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    footers: Vec<CommitMessageFooter>,
}

pub struct CommitMessageFooter {
    key: String,
    value: String,
}


impl CommitMessage {
    pub fn new(key: &str, value: &str) -> CommitMessage {
        CommitMessage {
            is_valid: false,
            prefix: None,
            type_value: None,
            scope: None,
            subject: None,
            body: None,
            footers: vec![],
        }
    }
    
    pub fn with_prefix(&self, prefix: String) -> CommitMessage {
        self.prefix = Some(prefix);
        self
    } 
}