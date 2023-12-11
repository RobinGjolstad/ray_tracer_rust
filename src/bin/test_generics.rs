#![allow(dead_code)]

trait ParentTrait {
    fn do_something(&self) {
        println!("ParentTrait::do_something()");
    }
}

#[derive(Debug, Clone, Copy)]
struct Square {}
impl ParentTrait for Square {
    fn do_something(&self) {
        println!("Square::do_something()");
    }
}

#[derive(Debug, Clone, Copy)]
struct Circle {}
impl ParentTrait for Circle {
    fn do_something(&self) {
        println!("Circle::do_something()");
    }
}

struct Object<T: ParentTrait> {
    object: T,
}
impl<T: ParentTrait> Object<T> {
    fn new(object: T) -> Object<T> {
        Object { object }
    }
}
impl<T: ParentTrait> ParentTrait for Object<T> {
    fn do_something(&self) {
        self.object.do_something();
    }
}

struct Group<T: ParentTrait> {
    children: Vec<Object<T>>,
}
impl<T: ParentTrait> Group<T> {
    fn new() -> Group<T> {
        Group {
            children: Vec::new(),
        }
    }
}
impl<T: ParentTrait> ParentTrait for Group<T> {
    fn do_something(&self) {
        for child in &self.children {
            child.do_something();
        }
    }
}

fn main() {
    let square = Object::new(Square {});
    let mut group = Group::new();
    group.children.push(square);
}
