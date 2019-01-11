use crate::github::Comments;
use hubcaps::issues::Issue;

#[derive(Debug)]
pub enum State {
    Open,
    Merged,
    Closed,
    Unknown,
}

#[derive(Debug)]
pub struct Rfc {
    pub state: State,
    pub issue: Issue,
    pub comments: Comments,
}

impl From<String> for State {
    fn from(s: String) -> Self {
        match s.as_str() {
            "open" => State::Open,
            "closed" => State::Closed,
            "merged" => State::Merged,
            _ => State::Unknown,
        }
    }
}

impl Rfc {
    pub fn new(issue: Issue, comments: Comments) -> Self {
        let state = State::from(issue.state.clone());
        Rfc {
            state: state,
            issue: issue,
            comments: comments,
        }
    }
}
