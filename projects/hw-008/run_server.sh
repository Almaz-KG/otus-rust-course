#!/usr/bin/env bash

set +e

# Kill the process, which holds 55082 port
kill -9 $(lsof -ti :55082 -sTCP:LISTEN)

# Run servers via calling cargo command
cargo run -p hw-008 --bin server -- -p 55082
