#!/bin/bash

set -e

RUST_LOG="walrus=error"

# Idk if it's really the goal of this script
# # Make sure rustup has the target, only procduces a dbg message if already installed
# rustup target add wasm32-unknown-unknown

# Build using wanted profile
if [ "$1" = release ] || [ "$1" = r ]
then
  echo Building front using release mode
  cargo build -p front --release --target=wasm32-unknown-unknown
  mode=release
else
  echo Building front using debug mode
  cargo build -p front --target=wasm32-unknown-unknown
  mode=debug
fi

echo Bindgen
# Generate the js bindings for the wasm prog
wasm-bindgen --target=web --out-dir=./target/wasm-bindgen/$mode ./target/wasm32-unknown-unknown/$mode/front.wasm --no-typescript

if ! [ -d "./static/" ]; then
  echo Creating ouput directory
  mkdir ./static/
fi

# Copy the .wasm & .js to the static directory
cp ./target/wasm-bindgen/$mode/* ./static/

# Copy the translations to static
cp ./front/resources/i18n/* ./static/resources/i18n/

echo Done
