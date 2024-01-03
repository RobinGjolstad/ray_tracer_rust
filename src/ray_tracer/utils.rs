/// A fixed value used for comparing f64
pub const EPSILON: f64 = 0.00005;

/// Compares two f64 and asserts whether they are within a difference defined by `utils::EPSILON`
pub fn is_float_equal(actual: &f64, comparison: f64) -> bool {
    (actual - comparison).abs() < EPSILON
}

/// This tree structure is _almost_ directly copied from
/// <https://developerlife.com/2022/02/24/rust-non-binary-tree/>.
///
/// This was found after quite a while of struggling with handling children and parents for shapes
/// and groups in this ray tracer.
/// I got close to a solution a few times, but there was always something missing. Instead of
/// struggling on with making my own tree, I'll steal and modify this one.
///
/// Usage of this tree module requires any used type to implement the Display trait.
pub(crate) mod tree {

    use core::fmt::Debug;
    use std::{
        fmt::{self, Display},
        ops::Deref,
        sync::{Arc, RwLock, Weak},
    };

    type NodeDataRef<T> = Arc<NodeData<T>>;
    type WeakNodeNodeRef<T> = Weak<NodeData<T>>;
    /// Parent relationship is one of non-ownership.
    type Parent<T> = RwLock<WeakNodeNodeRef<T>>; // not `RwLock<NodeDataRef<T>>` which would cause memory leak.
    /// Children relationship is one of ownership.
    type Children<T> = RwLock<Vec<Child<T>>>;
    type Child<T> = NodeDataRef<T>;

    /// This struct holds underlying data. It shouldn't be created directly, instead use:
    /// [`Node`](struct@Node).
    pub struct NodeData<T>
    where
        T: Display,
    {
        value: T,
        parent: Parent<T>,
        children: Children<T>,
    }

    /// This struct is used to own a [`NodeData`] inside an [`Arc`], which can be shared, so that it can
    /// have multiple owners. It also has getter methods for all of [`NodeData`]'s properties.
    #[derive(Debug)]
    pub struct Node<T: Display> {
        arc_ref: NodeDataRef<T>,
    }

    impl<T> Node<T>
    where
        T: Display,
    {
        pub fn new(value: T) -> Node<T> {
            let new_node = NodeData {
                value,
                parent: RwLock::new(Weak::new()),
                children: RwLock::new(Vec::new()),
            };
            let arc_ref = Arc::new(new_node);
            Node { arc_ref }
        }

        pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef<T> {
            Arc::clone(&self.arc_ref)
        }

        pub fn create_and_add_child(self: &Self, value: T) -> NodeDataRef<T> {
            let new_child = Node::new(value);
            self.add_child_and_update_its_parent(&new_child);
            new_child.get_copy_of_internal_arc()
        }

        /// 🔏 Write locks used.
        pub fn add_child_and_update_its_parent(self: &Self, child: &Node<T>) {
            {
                let mut my_children = self.arc_ref.children.write().unwrap();
                my_children.push(child.get_copy_of_internal_arc());
            } // `my_children` guard dropped.

            {
                let mut childs_parent = child.arc_ref.parent.write().unwrap();
                *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
            } // `my_parent` guard dropped.
        }

        pub fn has_parent(self: &Self) -> bool {
            self.get_parent().is_some()
        }

        /// 🔒 Read lock used.
        pub fn get_parent(self: &Self) -> Option<NodeDataRef<T>> {
            let my_parent_weak = self.arc_ref.parent.read().unwrap();
            if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
                Some(my_parent_arc_ref)
            } else {
                None
            }
        }
    }

    impl<T> Deref for Node<T>
    where
        T: Display,
    {
        type Target = NodeData<T>;

        fn deref(&self) -> &Self::Target {
            &self.arc_ref
        }
    }

    impl<T> fmt::Debug for NodeData<T>
    where
        T: Debug + Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut parent_msg = String::new();
            if let Some(parent) = self.parent.read().unwrap().upgrade() {
                parent_msg.push_str(format!("📦 {}", parent.value).as_str());
            } else {
                parent_msg.push_str("🚫 None");
            }
            f.debug_struct("Node")
                .field("value", &self.value)
                // .field("parent", &self.parent)
                .field("parent", &parent_msg)
                .field("children", &self.children)
                .finish()
        }
    }
}
