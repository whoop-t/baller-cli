// JIRA logic
use reqwest::{Client, Error, StatusCode};
use serde::Deserialize;

use crate::config;

pub const JIRA_BASE_URL: &str = "https://baller-tv.atlassian.net";
pub const JIRA_API: &str = "rest/api/2";

#[derive(Deserialize, Debug)]
pub struct TicketFields {
    pub summary: String,
}

#[derive(Deserialize, Debug)]
pub struct Ticket {
    pub key: String,
    pub fields: TicketFields,
}

pub async fn fetch(branch_name: &str) -> Result<(String, String), Error> {
    // First try and get creds from local
    let creds = config::get_jira_creds().unwrap_or_else(|error| panic!("{}", error));

    let r = Client::new()
        .get(format!(
            "{}/{}/{}/{}",
            JIRA_BASE_URL, JIRA_API, "issue", branch_name
        ))
        .basic_auth(creds.email, Some(creds.token))
        .send()
        .await?
        .json::<Ticket>()
        .await?;

    let empty_commit_msg = format!("{}: {}", r.key, r.fields.summary);
    let pr_body = format!("Jira Link: [{}/{}/{}](url)", JIRA_BASE_URL, "browse", r.key);
    Ok((empty_commit_msg, pr_body))
}
