use clap::Parser;
use reqwest::Error;

mod jira;
mod github;
mod config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short, long, value_parser)]
    /// BBS ticket number
    ticket_num: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let branch_name = format!("BBS-{}", args.ticket_num);

    // JIRA ticket response
    let (empty_commit_msg, pr_body) = jira::fetch(&branch_name).await?;

    // Run commands to create git branch/pr
    github::process(&branch_name, &empty_commit_msg, &pr_body);

    Ok(())
}
