# Baller cli

Quickly create standardized branch and PR with JIRA metadata on using only ticket number.

Install:
```
brew tap whoop-t/baller-cli
brew install baller-cli
```



Setup:
 - Create `.ballercliconfig` in your HOME dir (~)
 - [Get JIRA access token ](https://id.atlassian.com/manage-profile/security/api-tokens)
 - Add JIRA email and token to `.ballercliconfig` as follows(just top 2 lines):

```
 email=fart@baller.tv
 token=xxxxxxxxx
 ```
 - Install gh cli(previously hub) with `brew install gh`
 - auth your gh cli with `gh auth login` ([for help](https://cli.github.com/manual/gh_auth_login))

How to use:

`baller -h` for all commands

Currently, main purpose is for one line branch and pr creation

`baller -t 1111` <-- where 1111 is ticket number in JIRA

This will run commands to auto create branch and create PR with metadata from JIRA to populate title and summary.

### ⚠️ Currently only creates branch based off testflight, will add options later
