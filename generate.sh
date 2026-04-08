#!/usr/bin/env bash
set -euo pipefail

cargo build --release
./target/release/es-midterm
