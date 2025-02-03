#!/bin/bash

ffeed() {
  query=$1
  model=$2

  for order in 2 3; do

    echo "[INFO] Feeding $model, order = $order"

    path=models/order-$order/$model 

    mkdir -p models/order-$order

    ./feed.sh $query $path $order
  done
}

ffeed '*.html' html.bc
ffeed '*.md'   markdown.bc
ffeed '*.js'   javascript.bc
ffeed '*.ts'   typescript.bc
ffeed '*.rs'   rust.bc
ffeed '*.go'   go.bc
ffeed '*.py'   python.bc
ffeed '*.sh'   sh.bc
