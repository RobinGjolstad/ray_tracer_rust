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
- [ ] Limit inverse calculations?
- [ ] Separate matrix/tuple types for different sizes.
- [ ] Unify matrix/tuple?
- [ ] `Intersections::hit()`, avoid clone.
- [ ] `Ray::intersect_world()`, accept `Intersections` as parameter.
- [ ] `World::shade_hit()`, calls `Ray::intersect_world()`. Pass `Intersections` in here as well?
- [x] `World::is_shadowed()` calls `Ray::intersect_world()`, but should only care about the first hit. Add `Ray::intersect_world_once()` to stop on first intersection? Is there a way to find the closest object and only calculate for that one?
- [ ] `World::refracted_color()` calls `World::color_at()` which again calls `Ray::intersect_world()`. Share intersection vector?
- [ ] `Shapes` use `get_position()` internally. Replace with direct access to the object.
- [ ] `Object` "owns" a `Shape` which requires a clone. Change to hold a reference? Eliminates `.clone()`.
- [ ] Store `transpose` of all matrices on objects.

## Changelog

### Marble Madness 25

- Original: 155s
- Rays::intersect() - Share intersection list: 109s
- World::is_shadowed() -> Rays::intersect_world_first() - Stop on first intersection: 81s

