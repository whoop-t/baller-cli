# Baller cli
### ⚠️ BETA, works but error handling isnt great

Quickly create standardized branch and PR with JIRA metadata.

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

Basic Usage with defaults:

`baller -t BBS-1111` <-- where BBS-1111 is ticket key in JIRA

Usage with -b (base branch):

`baller -t BBS-1111 -b BBS-9999` <-- overwrite testflight default and use your own base branch
