# Plan

1. trigger collects all ingredients from all meals.
2. get existing ingredients from grocery list (todoist api)
3. merge existing ingredients with ingredients for meals
4. update ingredients on todoist

1. run on rpi. email directs to rpi ip address. requires running a service on the rpi though.
2. run on lambda. email directs to lambda url. need to validate with a pw. api calls abound

## Upgrades

- cron
- local web ui
- allow allie to add/edit meals
