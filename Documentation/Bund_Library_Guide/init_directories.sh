#!/bin/bash

for f in `awk -F',' '{print $1}' < index.csv`; do
  if [ ! -d ./lib/$f ]; then
    echo "$f is missing"
    mkdir ./lib/$f; cp ./_proto/* ./lib/$f
  fi
done
