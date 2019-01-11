use failure::Error;
use futures::stream::Stream;
use std::env;

mod github;
mod rfc;

use crate::github::Github;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new("rust-lang", "rfcs", token);

    tokio::run(
        github
            .rfcs()
            .map_err(|e| println!("error = {:?}", e))
            .for_each(|rfc| {
                println!("{}", rfc.issue.number);
                Ok(())
            }),
    );

    Ok(())
}
