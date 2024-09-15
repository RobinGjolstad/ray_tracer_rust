
    pub(crate) fn build(self) -> Object {
        let mut children = self.children;
        children.iter_mut().for_each(|child| {
            // Apply the inverse of the group's transform to each child.
            // This will allow space conversions directly between the child and the world
            // without having to go through the group.

            let mut new_child_transform = self.transform * child.get_transform();
            new_child_transform = new_child_transform
                .calculate_inverse()
                .expect("Failed to calculate inverse.");
            child.set_transform(&new_child_transform);
            if let Some(m) = self.material {
                child.set_material(&m);
            }
        });

        Object::Group(Group {
            position: Point::new_point(0.0, 0.0, 0.0),
            transform: self.transform,
            material: self.material,
            children: Some(children.into()),
        })
    }
