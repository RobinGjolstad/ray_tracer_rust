//! Test some possible approaches for restructuring shapes and groups.
//! It should be easy to find children from a group, and parent from shape.

use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct World {
    shapes: Arc<[Box<dyn ShapeActions>]>,
}

struct Ray;

trait ShapeActions: Debug {
    fn children_indeces(&self) -> &[usize];
    fn parent_index(&self) -> usize;
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Sphere {
    prop: u32,
    parent_index: usize,
}

impl ShapeActions for Sphere {
    fn children_indeces(&self) -> &[usize] {
        &[]
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Group {
    prop: u32,
    children_indeces: Arc<[usize]>,
    parent_index: usize,
}

impl ShapeActions for Group {
    fn children_indeces(&self) -> &[usize] {
        self.children_indeces.as_ref()
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }
}

fn main() {
    let mut world = World {
        shapes: Arc::new([]),
    };

    let mut group = Group {
        prop: 0,
        children_indeces: Arc::new([]),
        parent_index: 0,
    };

    let mut sphere1 = Sphere {
        prop: 0,
        parent_index: 0,
    };

    let mut sphere2 = Sphere {
        prop: 0,
        parent_index: 0,
    };

    let mut world_shapes: Vec<Box<dyn ShapeActions>> = vec![
        Box::new(group.clone()),
        Box::new(sphere1.clone()),
        Box::new(sphere2.clone()),
    ];

    world_shapes[0] = Box::new(Group {
        prop: 11,
        children_indeces: Arc::new([1, 2]),
        parent_index: 0,
    });
    world_shapes[1] = Box::new(Sphere {
        prop: 42,
        parent_index: 0,
    });
    world_shapes[2] = Box::new(Sphere {
        prop: 12,
        parent_index: 0,
    });

    world.shapes = world_shapes.into();
    dbg!(&world);

    println!(
        "Group contains: {:?} children.",
        world.shapes[0].children_indeces().len()
    );
}
