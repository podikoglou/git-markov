#!/bin/bash

query=$1
model=$2

feed_markov() {
  markov/target/release/markov train $model < $1
}

export -f feed_markov

find ./repos -name $query -exec bash -c 'feed_markov {}' \;
