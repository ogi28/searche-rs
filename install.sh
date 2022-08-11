#!/bin/bash

cargo build --release ; sudo cp target/release/searchers /usr/bin/ ; cargo clean

