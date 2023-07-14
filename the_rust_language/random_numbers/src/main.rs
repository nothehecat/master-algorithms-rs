// Generates random numbers with help of random-number generators 

use rand::{Rng, thread_rng};
use rand::distributions::{Uniform, Standard, Alphanumeric};
use rand_distr::{Distribution, Normal, NormalError};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    //////////////////////////////////////
    // Generate random numbers in a range
    //////////////////////////////////////
    rng = thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    //////////////////////////////////////////////////
    // Generate random numbers with a distribution
    // can be faster when repeated generation is needed
    /////////////////////////////////////////////////
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    ///////////////////////////////////////////////////////
    // Generate random numbers with a normal distribution
    // check distributions here 
    // https://docs.rs/rand_distr/0.4.3/rand_distr/index.html
    ///////////////////////////////////////////////////////
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);

    //////////////////////////////////////////////////////
    // Randomly generates a tuple of (i32, bool, f64) and
    // a variable of type Point
    //////////////////////////////////////////////////////
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

    //////////////////////////////////////////////////////////
    // Randomly generates string of given length ASCII chars
    //////////////////////////////////////////////////////////
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

    Ok(())
}

