#!/bin/bash

# Function to connect to container
connect_to_container() {
  echo "Connecting to container..."
  docker exec --tty --interactive paltalabs-reflector-challenge bash
}

if [[ $# -eq 0 ]]; then
  # No arguments, connect to paltalabs-reflector-challenge
  connect_to_container
elif [[ $1 == "--no-blockchain" || $1 == "--nb" ]]; then
  # With --no-blockchain, start only paltalabs-reflector-challenge container and connect
  echo "Starting only paltalabs-reflector-challenge container..."
  docker-compose up -d paltalabs-reflector-challenge
  connect_to_container
else
  # Any other argument, just connect to paltalabs-reflector-challenge
  connect_to_container
fi
