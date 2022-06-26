use clap::Parser;
use reqwest::Error;
use spinners::{Spinner, Spinners};

mod jira;
mod github;
mod config;

/// Baller CLI
/// Auto create branches and prs using only ticket number
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

    // Start spinner after args processed
    let mut sp = Spinner::new(Spinners::Dots9, "Creating branch and pr...".into());
    
    // JIRA ticket response
    let (empty_commit_msg, pr_body) = jira::fetch(&branch_name).await?;

    // Run commands to create git branch/pr
    github::process(&branch_name, &empty_commit_msg, &pr_body);

    // Stop spinner
    sp.stop();
    
    Ok(())
}
