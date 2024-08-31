#!/usr/bin/env pwsh

# Run the benchmark
cargo bench --bench camera >> bench_camera.txt
cargo bench --bench intersections >> bench_intersections.txt
cargo bench --bench matrix >> bench_matrix.txt
cargo bench --bench render >> bench_render.txt
cargo bench --bench tuple >> bench_tuple.txt

