# List of optimization targets

## List of benchmarks

### Camera

1. camera.ray_for_pixel
    Using camera (160, 120, PI/3), ray_for_pixel(100, 50)

### Intersections

1. [...]

### Matrix

Perform various operations on matrices.

TODO: Use other matrices than identity.

### Render

Performs complete rendering of a scene, at various resolutions and reflections.

TODO: Bench multithreaded rendering.

### Tuple

Performs tests on all tuple methods.

## List of improvements

- [ ] Reduce temporary lists in intersections.
- [ ] Unroll matrices.
- [ ] `.par_iter()`
- [ ] Trait objects?
- [ ] Fixed matrix-size?

