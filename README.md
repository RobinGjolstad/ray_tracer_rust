# The Ray Tracer Challenge

This is a project based on the book `The Ray Tracer Challenge` by Jamis Buck.

In this project I intend to learn Rust, which I have no real knowledge of from earlier. In addition I will also try to exclusively use Vim motions for all my editing in a hope of getting more comfortable with that as well.

## Progress

- Chapter 1 completed
- Chapter 2 started

## Todo list

- Revisit Chapter 1 to restructure the tuples, points, and vectors.  
  These are currently uncomfortably intertwined and prevent usage of things like tuples in other modules, such as the colors introduced in chapter 2.
  A possible fix could be to convert the `tuples` to be generic, with a three- and four-element tuple, each with defined operators which could be used for all tuple-types. Points and vectors could then use the four-element variety, and colors could use the three-elements.  
  - This appears to be a bit more useful eventually. Should definitely be done. Will need to restructure tests and such for _all_ basic modules though.
- Idea for other matrix construction:
  - Always use 4x4 matrix as an array. This allows use of `Copy` trait. 
  - Only fill with the used data.
  - Use a `size` field to keep track of the matrix size.
  - ? How to initialize with several sizes? 
    - `Matrix::new_2x2()` etc?
    - Use `Matrix::new()` with vector and "translate" into a 4x4 matrix?
- All matrices should have their "fancy" data calculated on creation and stored privately in the struct.