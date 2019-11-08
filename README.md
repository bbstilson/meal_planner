# Meal Planner

My partner and I are both picky about what we eat, and also *hate* choosing what to eat.

To solve the former problem, we've built up a few dozen recipes over the years that meet several criteria specific to our diets, desires and constraints. We keep these in Trello.

To solve the latter problem, I used ༼つ ◕_◕ ༽つ ***T E C H N O L O G Y***.

The "algorithm" is laid out in [`planner.py`](./planner.py).

## Technology

[Lambda](https://aws.amazon.com/lambda/) is used to run the code.

[Cloudwatch Events](https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/WhatIsCloudWatchEvents.html) are used to trigger the code once per week.

[S3](https://aws.amazon.com/s3/) is used to store a CSV that keeps track of how many times a meal has been suggested.

[SES](https://aws.amazon.com/ses/) is used to email my partner and I with the meals for the week.

## Deploying

Everything is manual for now. Run this command and load it in the lambda function.

```bash
zip -r lambda.zip s3_util.py ses_util.py trello.py planner.py
```

## Running locally

It's easier to run this locally than zip up everything and run it in Lambda. Everything is dockerized, so it should be a breeze.

First, you'll need a `docker.env` file with all the secrets:

```
API_KEY=what
TOKEN=ever

BUCKET=these
KEY=are

URL_BASE=https://api.trello.com/1
LIST_ID=you

MY_EMAIL=gotta
SO_EMAIL=add

AWS_ACCESS_KEY_ID=them
AWS_SECRET_ACCESS_KEY=your
AWS_DEFAULT_REGION=self
```

Then, you can build the image:

```bash
docker build . -t meal-planner:latest
```

Finally, run it:

```bash
docker run --rm \
  --env-file docker.env \
  -v /path/to/meal-planner:/usr/local/src/app \
  -it meal-planner:latest
```

I like to chain them together:

```bash
docker build . -t meal-planner:latest && docker run --rm --env-file docker.env -v /path/to/meal-planner:/usr/local/src/app -it meal-planner:latest
```

You'll find yourself in the running docker container. From there you can run the script:

```
root@b07c5894d6a5:/usr/local/src/app# python planner.py
```
