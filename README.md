# Baller cli
### ⚠️ BETA, may have some issues

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

`baller -t 1111` <-- where 1111 is ticket number in JIRA

Usage with -b (base branch):

`baller -t 1111 -b BBS-9999` 

Usage with -b (base branch) and prefix argument:

`baller -t 1111 -b BBS-9999 bugs`
