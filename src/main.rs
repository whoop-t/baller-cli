use clap::Parser;
use reqwest::Error;
use spinners::{Spinner, Spinners};

mod jira;
mod github;
mod config;

/// Baller CLI
/// Auto create branches and prs using only ticket number and base branch
#[derive(Parser, Debug)]
#[clap(author, version, about)]

struct Args {
    #[clap(short, long, value_parser)]
    /// BBS ticket key (e.g BBS-1234)
    ticket_key: String,
    
    #[clap(short, long, value_parser, default_value="testflight")]
    /// Base branch for PR e.g(BBS-1234)
    base_branch: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    // Determine ticket prefix
    let branch_name = args.ticket_key;
    // Start spinner after args processed
    let mut sp = Spinner::new(Spinners::Dots9, "Creating branch and pr...".into());

    // JIRA ticket response
    let empty_commit_msg = jira::fetch(&branch_name).await?;

    // Run commands to create git branch/pr
    github::process(&branch_name, &args.base_branch, &empty_commit_msg);

    // Stop spinner
    sp.stop();
    println!(" ");
    println!("Created! Check https://github.com/ballertv/baller/pulls for PR");
    
    Ok(())
}
