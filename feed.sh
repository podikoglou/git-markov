#!/bin/bash

feed_markov() {
  cat $1 | markov/target/release/markov train model.bc
}

export -f feed_markov

find ./repos -name '*.rs' -exec bash -c 'feed_markov {}' \;
