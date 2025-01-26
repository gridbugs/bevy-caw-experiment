#!/bin/sh
set -xeu

wasm-opt -O -ol 100 -s 100 -o /tmp/bevy-caw-experiment_bg.wasm dist/bevy-caw-experiment_bg.wasm
mv /tmp/bevy-caw-experiment_bg.wasm dist/bevy-caw-experiment_bg.wasm
