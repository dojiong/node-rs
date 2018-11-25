#!/bin/sh

cargo build
cp ../target/debug/libaddon.so addon.node
