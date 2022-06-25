use clap::Parser;
use regex::Regex;
use reqwest::{Client, Error};
use serde::Deserialize;
use std::process::Command;

const JIRA_BASE_URL: &str = "https://baller-tv.atlassian.net/rest/api/2";
const TOKEN: &str = "BTeaGFF7Cee0TPqhBKZ81C73"; // Change this to be added by the person using
const USER: &str = "tommy@baller.tv"; // Change this to be added by the person using
                                      //

#[derive(Deserialize, Debug)]
struct TicketFields {
    summary: String,
    description: String,
}

impl TicketFields {
    // Remove everything after the weird icon for sync stuff on jira ticket
    fn remove_sync_text(&mut self) {
        let re = Regex::new(r"┆.*$").unwrap();
        self.description = re.replace_all(&self.description, "").to_string();
    }
}

#[derive(Deserialize, Debug)]
struct Ticket {
    key: String,
    fields: TicketFields,
}

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
    let response = Client::new()
        .get(format!("{}/{}/{}", JIRA_BASE_URL, "issue", branch_name))
        .basic_auth(USER, Some(TOKEN))
        .send()
        .await?;
    let status = response.status();

    println!("{}", status);

    let mut parsed_response = response.json::<Ticket>().await?;

    parsed_response.fields.remove_sync_text();

    println!("{:?}", parsed_response.fields.description);

    if status == 200 {
        let checkout = Command::new("git")
            .args(["checkout", "-b", &branch_name, "main"])
            .output()
            .expect("failed");

        let u_push = Command::new("git")
            .args(["push", "--set-upstream", "origin", &branch_name])
            .output()
            .expect("failed");

        let commit = Command::new("git")
            .args(["commit", "--allow-empty", "-m", "New branch test"])
            .output()
            .expect("failed");

        let push = Command::new("git").args(["push"]).output().expect("failed");
        let pr = Command::new("gh")
            .args([
                "pr",
                "create",
                "-t",
                &parsed_response.fields.summary,
                "-b",
                &parsed_response.fields.description,
            ])
            .output()
            .expect("failed");
    }
    Ok(())
}
