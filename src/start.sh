#!/bin/bash

# Start the Rust server in the background
echo "Starting Rust server..."
cargo run &
SERVER_PID=$!

# Define the port the Rust server will listen on
RUST_SERVER_PORT=8000
LOCALHOST=127.0.0.1

# Wait until the Rust server is responding on the specified port
until curl -s "http://$LOCALHOST:$RUST_SERVER_PORT" > /dev/null; do
    echo "Waiting for Rust server to start on port $RUST_SERVER_PORT..."
    sleep 1
done

echo "Rust server is up, starting SSH tunnel..."

# Start the SSH tunnel using request_port.sh
./src/request_port.sh &

# Wait for the Rust server to finish
wait $SERVER_PID
