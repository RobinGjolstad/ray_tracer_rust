use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    colors::Color,
    lights::Light,
    materials::Material,
    shapes::{sphere::Sphere, Object, Shapes},
    transformations::Transform,
    tuples::Tuple,
    world::World,
};

fn main() {
    let mut floor = Sphere::new();
    floor.set_transform(&Transform::scaling(10.0, 0.01, 10.0));
    let mut material = Material::new();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    floor.set_material(&material);

    let mut left_wall = Sphere::new();
    let trans = Transform::translate(0.0, 0.0, 5.0)
        * Transform::rotation_y(-PI / 4.0)
        * Transform::rotation_x(PI / 2.0)
        * Transform::scaling(10.0, 0.01, 10.0);
    left_wall.set_transform(&trans);
    left_wall.set_material(&floor.get_material());

    let mut right_wall = Sphere::new();
    let trans = Transform::translate(0.0, 0.0, 5.0)
        * Transform::rotation_y(PI / 4.0)
        * Transform::rotation_x(PI / 2.0)
        * Transform::scaling(10.0, 0.01, 10.0);
    right_wall.set_transform(&trans);
    right_wall.set_material(&floor.get_material());

    let mut middle = Sphere::new();
    middle.set_transform(&Transform::translate(-0.5, 1.0, 0.5));
    material = Material::new();
    material.color = Color::new(0.1, 1.0, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    middle.set_material(&material);

    let mut right = Sphere::new();
    let trans = Transform::translate(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5);
    right.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(0.5, 1.0, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    right.set_material(&material);

    let mut left = Sphere::new();
    let trans = Transform::translate(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33);
    left.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(1.0, 0.8, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    left.set_material(&material);

    let mut world = World::new();
    world.lights.push(Light::point_light(
        &Tuple::new_point(-10.0, 10.0, -10.0),
        &Color::new(1.0, 1.0, 1.0),
    ));
    world.objects = vec![
        Object::Sphere(floor),
        Object::Sphere(left_wall),
        Object::Sphere(right_wall),
        Object::Sphere(left),
        Object::Sphere(middle),
        Object::Sphere(right),
    ];

    let mut camera = Camera::new(240, 240, PI / 3.0);
    camera.set_transform(Transform::view_transform(
        &Tuple::new_point(0.0, 1.5, -5.0),
        &Tuple::new_point(0.0, 1.0, 0.0),
        &Tuple::new_vector(0.0, 1.0, 0.0),
    ));

    let mut img = camera.render(&world);

    img.save(&String::from(format!(
        "images/ch8_pit/ch8_pit_{}x{}.ppm",
        img.width(),
        img.height()
    )));
}
