//! many examples with matrices

use ndarray::{arr1, arr2, Array1, array, ArrayView1};
use nalgebra::Matrix3;


fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}


fn main() {

    // Adding matrices
    println!("Adding matrices");
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    let b = arr2(&[[6, 5, 4],
                   [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}\n", sum);
    
    // Multiplying matrices
    println!("Multiplying matrices");
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    let b = arr2(&[[6, 3],
                   [5, 2],
                   [4, 1]]);

    println!("{}\n", a.dot(&b));

    // Multiplying a matrix by a scalar
    println!("Multiplying a matrix by a scalar");
    let scalar = 4;
    let vector = arr1(&[1, 2, 3]);
    let matrix = arr2(&[[4, 5, 6],
                        [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}\n", new_matrix);

    // Calculating L2 and L1 norms
    println!("Calculating L2 and L1 norms");

    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));

    // Inverting a matrix
    println!("Inverting a matrix");
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m1 = {}", m1);
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }

}

