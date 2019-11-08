#!/bin/bash

cd package
zip -r9 ${OLDPWD}/lambda.zip .
cd $OLDPWD
zip -g lambda.zip s3_util.py ses_util.py trello.py planner.py
