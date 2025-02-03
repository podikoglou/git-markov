#!/bin/bash

model="$1"
query="$2"

printf "$query" | markov/target/release/markov complete $model
