use futures::{Future as StdFuture, Stream as StdStream};
use hubcaps::comments::{Comment, CommentListOptions};
use hubcaps::issues::{IssueListOptions, State};
use hubcaps::Credentials;
use hyper;
use hyper_tls;

type Connection = hubcaps::Github<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;
pub type Comments = Vec<Comment>;

pub struct Github {
    owner: String,
    repo: String,
    conn: Connection,
}

use crate::rfc::Rfc;

/// A type alias for `Streams` that may result in `hubcaps::Errors`
pub type Stream<T> = Box<StdStream<Item = T, Error = hubcaps::errors::Error> + Send>;

impl Github {
    pub fn new<T: Into<String>>(owner: T, repo: T, token: String) -> Self {
        let conn = hubcaps::Github::new(
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
            Credentials::Token(token),
        );
        Github {
            owner: owner.into(),
            repo: repo.into(),
            conn: conn,
        }
    }

    pub fn rfcs(self) -> Stream<Rfc> {
        Box::new(
            self.conn
                .repo(self.owner.clone(), self.repo.clone())
                .issues()
                .iter(
                    &IssueListOptions::builder()
                        .per_page(100)
                        .state(State::All)
                        .build(),
                )
                .and_then(move |issue| {
                    // println!("Crawling {}", issue.number);
                    let comments: Result<Comments, _> = self
                        .conn
                        .repo(self.owner.clone(), self.repo.clone())
                        .issues()
                        .get(issue.number)
                        .comments()
                        .list(&CommentListOptions::builder().build())
                        .wait();
                    Ok((issue, comments?))
                })
                .and_then(|(issue, comments)| Ok(Rfc::new(issue, comments))),
        )
    }
}
