#!/bin/bash
if [[ -f "catboat.log" ]]; then rm catboat.log; fi
./target/catboat >> catboat.log 2>&1 &