#!/bin/bash

build() {
  cargo test -- --nocapture
}

watch() {
  if [[ $(uname) == "Darwin" ]]
  then
    fswatch -nor $1
  else
    inotifywait -q -r -e create,modify $1
  fi
}

build_on_change() {
  export -f build
  watch src Cargo.toml | xargs -IA bash -c build
  sleep 3
}

build
while true; do build_on_change; done
