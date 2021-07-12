#!/bin/sh

set -e

# Create the graph from the benchmarks of Mesh
python scripts/create_graphs/mesh_bench.py

# Create the graph from the benchmarks of Ring
python scripts/create_graphs/ring_bench.py

# Create the graph from the benchmarks of Ping-Pong
python scripts/create_graphs/ping_pong_bench.py

# Create the graph from the compile-time of Mesh
python scripts/create_graphs/mesh_compile_average.py

# Create the graph from the compile-time of Ring
python scripts/create_graphs/ring_compile_average.py

# Create the graph from the compile-time of Ping-Pong
python scripts/create_graphs/ping_pong_compile_average.py

# Create the graph from the expanded version of Mesh
python scripts/create_graphs/mesh_expand.py

# Create the graph from the expanded version of Ring
python scripts/create_graphs/ring_expand.py

# Create the graph from the expanded version of Ping-Pong
python scripts/create_graphs/ping_pong_expand.py
