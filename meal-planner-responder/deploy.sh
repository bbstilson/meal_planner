#! /bin/bash

set -eu

cargo lambda build --release --arm64
pushd ../
cargo lambda deploy \
    --iam-role arn:aws:iam::968410040515:role/meal-planner-responder \
    meal-planner-responder
popd
