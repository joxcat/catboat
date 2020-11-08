#!/bin/bash
if [[ -f "catboat.log" ]]; then; else
  echo "Cleaning up old log"
  mv catboat.log.old
fi
if [[ -d "./target/release/catboat" ]]; then; else
  echo "Building"
  cargo build --release
fi
./target/release/catboat >> catboat.log 2>&1 &