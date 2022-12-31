use ray_tracer::{matrices::Matrix, tuples::Tuple};

fn main() {
    //
    let mut id = Matrix::new_identity();
    let id_inv = id.get_inverted().unwrap();

    println!("\nInverse of ID");
    println!("{:?}", id);
    println!("{:?}", id_inv);

    let mat = Matrix::new(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![4.0, 5.0, 6.0, 7.0],
        vec![8.0, 9.0, 0.0, 1.0],
        vec![2.0, 3.0, 4.0, 4.0],
    ])
    .unwrap();
    let mut mat_trans = mat.clone().transpose().unwrap();
    let mat_inv = mat.clone().get_inverted().unwrap();
    let mat_mul_inverse = mat.clone() * mat_inv.clone();

    println!("\nMultiply inverse mat");
    println!("{:?}", mat);
    println!("{:?}", mat_mul_inverse);

    println!("\nInv trans trans inv");
    println!("{:?}", mat_trans.get_inverted().unwrap());
    println!("{:?}", mat_inv.transpose());

    let tup = Tuple::new(1.0, 2.0, 3.0, 4.0);
    let id1 = Matrix::new(vec![
        vec![1.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap();

    println!("\nTuplething");
    println!("{:?}", id * tup);
    println!("{:?}", id1 * tup);
}
