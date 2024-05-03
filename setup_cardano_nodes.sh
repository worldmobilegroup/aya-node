#!/usr/bin/env bash

# Set environment variables for Docker setup
NETWORK_NAME="aya_network"
CARDANO_IMAGE="ghcr.io/intersectmbo/cardano-node:8.9.1"
DATA_DIR="${PWD}/data"

# Create a network if it does not exist
docker network create ${NETWORK_NAME} || true

# Function to launch a Cardano node in Docker
launch_node() {
  local node_id=$1
  local db_dir="${DATA_DIR}/db/node-${node_id}"
  local socket_dir="${DATA_DIR}/socket"
  local port=$((3000 + node_id))

  # Create directories if they don't exist
  mkdir -p "${db_dir}" "${socket_dir}"

  # Run the Docker container
  docker run -d --rm \
    --name "cardano-node-${node_id}" \
    --network ${NETWORK_NAME} \
    -p "${port}:${port}" \
    -v "${DATA_DIR}:/cardano/data" \
    ${CARDANO_IMAGE} \
    run \
      --database-path "/cardano/data/db/node-${node_id}" \
      --socket-path "/cardano/data/socket/node-${node_id}-socket" \
      --port ${port} \
      --config "/cardano/data/config.yaml" \
      --topology "/cardano/data/topology.json" \
      --shelley-vrf-key "/cardano/data/node-${node_id}/vrf.skey" \
      --shelley-kes-key "/cardano/data/node-${node_id}/kes.skey" \
      --shelley-operational-certificate "/cardano/data/node-${node_id}/opcert"
}

# Launch nodes
for i in {1..3}; do
  launch_node $i
done

# Function to clean up Docker containers on exit
function cleanup() {
  echo "Cleaning up containers..."
  for i in {1..3}; do
    docker stop "cardano-node-${i}"
  done
}

trap cleanup EXIT
