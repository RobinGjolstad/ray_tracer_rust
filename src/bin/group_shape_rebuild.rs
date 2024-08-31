//! Test some possible approaches for restructuring shapes and groups.
//! It should be easy to find children from a group, and parent from shape.
#![allow(dead_code)]

use std::sync::Arc;

fn main() {
    let mut s1 = Object::new_sphere();
    s1.set_data(1.0);
    let mut s2 = Object::new_sphere();
    s2.set_data(2.0);
    let mut s3 = Object::new_sphere();
    s3.set_data(3.0);
    let mut p1 = Object::new_plane();
    p1.set_data(4.0);
    let mut p2 = Object::new_plane();
    p2.set_data(5.0);
    let mut g = Object::new_group(vec![s1, s2, s3, p1, p2]);
    g.set_data(6.0);

    let mut s4 = Object::new_sphere();
    s4.set_data(7.0);

    let world = World {
        objects: vec![g, s4].into(),
    };
    dbg!(&world);
    let xs = world.intersect(1.0);
    println!("xs: {:?}", xs);
}

#[derive(Debug, Default, Clone)]
enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Group(Group),

    #[default]
    Dummy,
}

impl Object {
    fn new_sphere() -> Object {
        Object::Sphere(Sphere::default())
    }
    fn new_plane() -> Object {
        Object::Plane(Plane::default())
    }
    fn new_group(children: Vec<Object>) -> Object {
        let gb = GroupBuilder::new();
        children
            .into_iter()
            .fold(gb, |gb, child| gb.add(child))
            .build()
    }
    fn set_data(&mut self, data: f64) {
        match self {
            Object::Sphere(s) => s.data = data,
            Object::Plane(p) => p.data = data,
            Object::Group(g) => {
                g.data = data;
                if let Some(children) = &g.children {
                    // Group has children.
                    // Make a copy of the children, and set data on each.
                    // Then replace the children in the group.
                    let mut new_children = Vec::with_capacity(children.len());
                    for child in children.iter() {
                        let mut new_child = child.clone();
                        new_child.set_data(g.data + child.data());
                        new_children.push(new_child);
                    }

                    g.children = Some(new_children.into());
                }
            }
            _ => {}
        }
    }
    fn data(&self) -> f64 {
        match self {
            Object::Sphere(s) => s.data,
            Object::Plane(p) => p.data,
            Object::Group(g) => g.data,
            _ => 0.0,
        }
    }
    fn as_sphere(&self) -> Option<&Sphere> {
        match self {
            Object::Sphere(s) => Some(s),
            _ => None,
        }
    }
    fn as_sphere_mut(&mut self) -> Option<&mut Sphere> {
        match self {
            Object::Sphere(s) => Some(s),
            _ => None,
        }
    }
    fn as_plane(&self) -> Option<&Plane> {
        match self {
            Object::Plane(p) => Some(p),
            _ => None,
        }
    }
    fn as_plane_mut(&mut self) -> Option<&mut Plane> {
        match self {
            Object::Plane(p) => Some(p),
            _ => None,
        }
    }
    fn as_group(&self) -> Option<&Group> {
        match self {
            Object::Group(g) => Some(g),
            _ => None,
        }
    }
    fn as_group_mut(&mut self) -> Option<&mut Group> {
        match self {
            Object::Group(g) => Some(g),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Sphere {
    data: f64,
}
#[derive(Debug, Default, Clone)]
struct Plane {
    data: f64,
}
#[derive(Debug, Default, Clone)]
struct Group {
    data: f64,
    children: Option<Arc<[Object]>>,
}
#[derive(Debug, Default, Clone)]
struct GroupBuilder {
    children: Vec<Object>,
}
impl GroupBuilder {
    fn new() -> GroupBuilder {
        GroupBuilder {
            children: Vec::new(),
        }
    }
    fn add(mut self, child: Object) -> GroupBuilder {
        self.children.push(child);
        self
    }
    fn build(self) -> Object {
        Object::Group(Group {
            data: 0.0,
            children: Some(self.children.into()),
        })
    }
}

#[derive(Debug, Clone)]
struct World {
    objects: Arc<[Object]>,
}
impl World {
    fn intersect(&self, ray: f64) -> Vec<f64> {
        let mut xs = Vec::new();
        for object in self.objects.iter() {
            let object_xs = object.data() / ray;
            xs.push(object_xs);
        }
        xs.sort_by(|a, b| a.partial_cmp(b).expect("What the duck happened?"));
        xs
    }
}
