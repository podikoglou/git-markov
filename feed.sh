#!/bin/bash

query=$1
model=$2

# feed_markov() {
#   markov/target/release/markov train $1 < $2
# }
#
# export -f feed_markov
#
find ./repos -name $query -exec cat {} \; | markov/target/release/markov train $model

# find ./repos -name $query -exec bash -c "feed_markov $model {}" \;
