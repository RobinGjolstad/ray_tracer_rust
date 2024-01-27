#![allow(unused)]

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, Weak},
};

use group::Group;
use sphere::Sphere;

trait SomeTrait {
    fn printme(&self);
    fn get_shared_data(&self) -> BaseShape;
}

#[derive(Clone, Debug, PartialEq)]
struct BaseShape {
    data_a: usize,
    data_b: usize,
}
impl Display for BaseShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.data_a, self.data_b)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ObjectEnum {
    Sphere(Sphere),
    Group(Group),
}
impl SomeTrait for ObjectEnum {
    fn printme(&self) {
        match self {
            ObjectEnum::Sphere(thingy) => thingy.printme(),
            ObjectEnum::Group(group) => group.printme(),
        }
    }
    fn get_shared_data(&self) -> BaseShape {
        match self {
            ObjectEnum::Sphere(thingy) => thingy.get_shared_data(),
            ObjectEnum::Group(group) => group.get_shared_data(),
        }
    }
}
impl std::fmt::Display for ObjectEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectEnum::Sphere(thingy) => write!(f, "{}", thingy),
            ObjectEnum::Group(group) => write!(f, "{}", group),
        }
    }
}

// ====== Thingy ======

mod sphere {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Sphere {
        pub shared_data: BaseShape,
        pub data: usize,
    }
    impl Sphere {
        pub fn new() -> Self {
            Sphere {
                shared_data: BaseShape {
                    data_a: 0,
                    data_b: 0,
                },
                data: 0,
            }
        }
    }
    impl SomeTrait for Sphere {
        fn printme(&self) {
            println!("Sphere: {:?}", self);
        }
        fn get_shared_data(&self) -> BaseShape {
            self.shared_data.clone()
        }
    }
    impl std::fmt::Display for Sphere {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "shared_data: {}\ndata: {}", self.shared_data, self.data)
        }
    }
}

// ====== Group ======

mod group {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Group {
        pub shared_data: BaseShape,
        pub data: usize,
    }
    impl Group {
        pub fn new() -> Self {
            Group {
                shared_data: BaseShape {
                    data_a: 0,
                    data_b: 0,
                },
                data: 0,
            }
        }
    }
    impl SomeTrait for Group {
        fn printme(&self) {
            println!("Group: {:?}", self);
        }
        fn get_shared_data(&self) -> BaseShape {
            self.shared_data.clone()
        }
    }
    impl std::fmt::Display for Group {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "shared_data: {}\ndata: {}", self.shared_data, self.data)
        }
    }
}

// ====== ObjectWrapper ======

#[derive(Clone, Debug, PartialEq)]
struct ObjectWrapper {
    data: ObjectEnum,
}
impl SomeTrait for ObjectWrapper {
    fn printme(&self) {
        self.data.printme();
    }
    fn get_shared_data(&self) -> BaseShape {
        self.data.get_shared_data()
    }
}
impl std::fmt::Display for ObjectWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

// ====== Tree Data Types ======

type ObjectDataRef = Arc<ObjectData>;
type WeakObjectDataRef = Weak<ObjectData>;
/// Parent relationship is one of non-ownership.
type Parent = RwLock<WeakObjectDataRef>; // not `RwLock<ObjectDataRef>` which would cause memory leak.
/// Children relationship is one of ownership.
type Children = RwLock<Vec<Child>>;
type Child = ObjectDataRef;

// ====== ObjectNode ======

#[derive(Debug)]
struct ObjectData {
    value: ObjectEnum,
    parent: Parent,
    children: Children,
}
impl ObjectData {
    fn new(value: ObjectEnum) -> Self {
        ObjectData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        }
    }
}
impl Deref for ObjectData {
    type Target = ObjectEnum;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl DerefMut for ObjectData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl PartialEq for ObjectData {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// ====== Object ======

#[derive(Clone, Debug, PartialEq)]
struct Object {
    arc_ref: ObjectDataRef,
}
impl Object {
    pub fn new(value: ObjectEnum) -> Object {
        let new_node = ObjectData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        };
        let arc_ref = Arc::new(new_node);
        Object { arc_ref }
    }

    pub fn get_copy_of_internal_arc(&self) -> ObjectDataRef {
        self.arc_ref.clone()
    }

    pub fn create_and_add_child(&self, value: ObjectEnum) -> ObjectDataRef {
        let new_child = Object::new(value);
        self.add_child_and_update_its_parent(&new_child);
        new_child.get_copy_of_internal_arc()
    }

    pub fn add_child_and_update_its_parent(&self, child: &Object) {
        {
            let mut my_children = self.arc_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        } // `my_children` guard dropped.

        {
            let mut childs_parent = child.arc_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        } // `my_parent` guard dropped.
    }

    pub fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }

    pub fn get_parent(&self) -> Option<ObjectDataRef> {
        let my_parent = self.arc_ref.parent.read().unwrap();
        my_parent.upgrade()
    }
}

impl SomeTrait for Object {
    fn printme(&self) {
        self.arc_ref.value.printme();

        if let Some(parent) = self.get_parent() {
            println!("Parent:");
            parent.printme();
        } else {
            println!("Parent: None");
        }

        let my_children = self.arc_ref.children.read().unwrap();
        if my_children.len() > 0 {
            println!("Children:");
            for child in my_children.iter() {
                child.printme();
            }
        } else {
            println!("Children: None");
        }
    }
    fn get_shared_data(&self) -> BaseShape {
        self.arc_ref.value.get_shared_data()
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arc_ref.value)
    }
}

fn main() {
    println!("helloes");

    let mut group = Object::new(ObjectEnum::Group(Group {
        shared_data: BaseShape {
            data_a: 1,
            data_b: 2,
        },
        data: 3,
    }));

    let mut sphere = Object::new(ObjectEnum::Sphere(Sphere {
        shared_data: BaseShape {
            data_a: 4,
            data_b: 5,
        },
        data: 6,
    }));

    let mut sphere2 = Object::new(ObjectEnum::Sphere(Sphere {
        shared_data: BaseShape {
            data_a: 7,
            data_b: 8,
        },
        data: 9,
    }));

    let mut sphere3 = Object::new(ObjectEnum::Sphere(Sphere {
        shared_data: BaseShape {
            data_a: 10,
            data_b: 11,
        },
        data: 12,
    }));

    let mut sphere4 = Object::new(ObjectEnum::Sphere(Sphere {
        shared_data: BaseShape {
            data_a: 13,
            data_b: 14,
        },
        data: 15,
    }));

    group.add_child_and_update_its_parent(&sphere);
    group.add_child_and_update_its_parent(&sphere2);
    group.add_child_and_update_its_parent(&sphere3);
    group.add_child_and_update_its_parent(&sphere4);

    dbg!(&group);

    group.printme();
}
