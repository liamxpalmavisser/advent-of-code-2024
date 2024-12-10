#!/bin/bash

TEMPLATE_DIR="day-11/src/bin"
START_DAY=12
END_DAY=25

for DAY in $(seq $START_DAY $END_DAY); do
    DAY_DIR="day-$(printf "%02d" $DAY)"
    echo "Setting up $DAY_DIR..."

    # Create directory structure
    mkdir -p "$DAY_DIR/src/bin"

    # Copy template files
    cp -n "$TEMPLATE_DIR/part1.rs" "$DAY_DIR/src/bin/part1.rs"
    cp -n "$TEMPLATE_DIR/part2.rs" "$DAY_DIR/src/bin/part2.rs"
    cp -n "$TEMPLATE_DIR/input.txt" "$DAY_DIR/src/bin/input.txt"
    cp -n "$TEMPLATE_DIR/example.txt" "$DAY_DIR/src/bin/example.txt"

    # Create a basic Cargo.toml
    cat > "$DAY_DIR/Cargo.toml" <<EOF
[package]
name = "day-$(printf "%02d" $DAY)"
version = "0.1.0"
edition = "2021"

[dependencies]
utils = { path = "../utils" } # Adjust if using a utility crate
EOF
done

echo "All days set up from day-$START_DAY to day-$END_DAY!"
