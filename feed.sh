#!/bin/bash

feed_markov() {
  markov/target/release/markov train model.bc < $1
}

export -f feed_markov

find ./repos -name '*.rs' -exec bash -c 'feed_markov {}' \;
