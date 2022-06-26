use clap::{Parser, ArgEnum};
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
    /// BBS ticket number
    ticket_num: String,
    
    #[clap(short, long, value_parser, default_value="testflight")]
    /// Base branch for PR e.g(BBS-1234)
    base_branch: String,
    
    #[clap(value_enum, default_value_t=BallerIssueType::Bbs)]
    /// Ticket type (BUGS, BBS)
    issue_type: BallerIssueType
}

// Issue prefix in JIRA, currently only BBS and BUGS
#[derive(ArgEnum, Debug, Clone)]
enum BallerIssueType {
    Bbs,
    Bugs
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    // Determine ticket prefix
    let branch_name = match args.issue_type {
        BallerIssueType::Bbs => format!("BBS-{}", args.ticket_num), 
        BallerIssueType::Bugs => format!("BUGS-{}", args.ticket_num), 
    };

    // Start spinner after args processed
    let mut sp = Spinner::new(Spinners::Dots9, "Creating branch and pr...".into());

    // JIRA ticket response
    let (empty_commit_msg, pr_body) = jira::fetch(&branch_name).await?;

    // Run commands to create git branch/pr
    github::process(&branch_name, &args.base_branch, &empty_commit_msg, &pr_body);

    // Stop spinner
    sp.stop();
    println!(" ");
    println!("Created! Check https://github.com/ballertv/baller/pulls for PR");
    
    Ok(())
}
