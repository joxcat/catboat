#!/bin/bash
if [[ ! -f "catboat.log" ]]; then
  echo "Cleaning up old log"
  mv catboat.log catboat.log.old
fi
if [[ ! -f "./target/release/catboat" ]]; then
  echo "Building"
  cargo build --release
fi
./target/release/catboat >> catboat.log 2>&1 &