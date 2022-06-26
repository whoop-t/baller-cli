// Git and GH(cli) commands
use std::process::Command;

pub fn process(branch_name: &str, title: &str, body: &str) {
    checkout(branch_name);
    set_upstream(branch_name);
    empty_commit();
    push();
    create_pr(title, body);
}

fn checkout(branch_name: &str) {
    Command::new("git")
        .args(["checkout", "-b", branch_name, "main"])
        .output()
        .expect("failed");
}

fn set_upstream(branch_name: &str) {
    Command::new("git")
        .args(["push", "--set-upstream", "origin", branch_name])
        .output()
        .expect("failed");
}

fn empty_commit() {
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "Empty commit for PR"])
        .output()
        .expect("failed");
}

fn push() {
    Command::new("git").arg("push").output().expect("failed");
}

fn create_pr(title: &str, body: &str) {
    Command::new("gh")
        .args(["pr", "create", "-t", title, "-b", body])
        .output()
        .expect("PR creation failed, please check gh cli is installed and auth is set up");
}
