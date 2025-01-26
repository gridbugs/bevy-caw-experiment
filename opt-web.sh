#!/bin/sh
set -xeu

wasm-opt -Oz -o /tmp/bevy-caw-experiment_bg.wasm dist/bevy-caw-experiment_bg.wasm
mv /tmp/bevy-caw-experiment_bg.wasm dist/bevy-caw-experiment_bg.wasm
