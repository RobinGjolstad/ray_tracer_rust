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

#[derive(Debug, Clone)]
enum ObjectType<'a> {
    Square(Square),
    Circle(Circle),
    Group(Group<'a>),
}

#[derive(Debug, Clone)]
struct Object<'a> {
    transform: f64,
    object: ObjectType<'a>,
    parent: Option<&'a Object<'a>>,
}

impl<'a> Object<'a> {
    fn new(object: ObjectType<'a>) -> Object<'a> {
        Object {
            transform: 0.0,
            object,
            parent: None,
        }
    }
}

impl<'a> ParentTrait for Object<'a> {
    fn do_something(&self) {
        match &self.object {
            ObjectType::Square(s) => s.do_something(),
            ObjectType::Circle(c) => c.do_something(),
            ObjectType::Group(g) => g.do_something(),
        }
    }
}

#[derive(Debug, Clone)]
struct Group<'a> {
    children: Vec<Object<'a>>,
}

impl<'a> Group<'a> {
    fn new() -> Group<'a> {
        Group {
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Object<'a>) {
        self.children.push(child);
    }

    /// Only set parent on the children if the group is "complete".
    fn set_parent(&mut self) {
        for child in &mut self.children {
            //child.parent = Some(self);
            todo!("Set parent. Can't find a good way to do this since we'll need access to parent data.");
        }
    }
}

impl<'a> ParentTrait for Group<'a> {
    fn do_something(&self) {
        for child in &self.children {
            child.do_something();
        }
    }
}

fn main() {
    let square = Square {};
    let circle = Circle {};

    let obj1 = Object::new(ObjectType::Square(square));
    let obj2 = Object::new(ObjectType::Circle(circle));
    dbg!(&obj1);
    dbg!(&obj2);

    let mut group = Group::new();
    group.add_child(obj1);
    group.add_child(obj2);
    dbg!(&group);
    group.set_parent();
    group.do_something();

    let obj3 = Object::new(ObjectType::Group(group));
    dbg!(&obj3);
    obj3.do_something();

    let obj4 = obj3.clone();
    dbg!(&obj4);
    obj4.do_something();

    let mut group2 = Group::new();
    group2.add_child(obj3);
    group2.add_child(obj4);
    group2.set_parent();
    dbg!(&group2);
    group2.do_something();
}
