#!/usr/bin/env pwsh

# Prepend current time to each benchmark file.
$current_time = Get-Date -Format "yyyy-MM-ddTHH:mm:ss"

# Run the benchmark
$current_time >> bench_camera.txt
cargo bench --bench camera >> bench_camera.txt

$current_time >> bench_intersections.txt
cargo bench --bench intersections >> bench_intersections.txt

$current_time >> bench_matrix.txt
cargo bench --bench matrix >> bench_matrix.txt

$current_time >> bench_render.txt
cargo bench --bench render >> bench_render.txt

$current_time >> bench_tuple.txt
cargo bench --bench tuple >> bench_tuple.txt
