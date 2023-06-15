# Changelog Tracker

## A project by Datacord

> Fetches the latest changelogs from Discord and looks for new ones


## Setup and run with Docker

```docker
docker build -t datacord-changelog-tracker:latest .
```

```docker
docker volume create datacord-changelog-tracker
```

```docker
docker run \
-e CHANGELOG_WEBHOOK_URL='REPLACE_ME' \
-e ROLE_ID='REPLACE_ME' \
-e DB_URL='./db/changelogs.db' \
--mount type=volume,src=datacord-changelog-tracker,target=/changelog-tracker/db \
--name changelog-tracker \
datacord-changelog-tracker:latest
```
