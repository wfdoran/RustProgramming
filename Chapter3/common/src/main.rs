use std::any::type_name;
use std::mem::size_of;

fn type__of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

fn size__of<T>(_: T) {
    println!("{}", std::mem::size_of::<T>())
}


const MAX_V : u32 = 100_000;

fn mutable() {
    let x = 5;
    println!("x is {}.", x);
    // x = 6;

    let mut y = 6;
    println!("y is {}.", y);
    y = 7;
    println!("y is {}.", y);
}

fn constants() {
    println!{"MAX_V is {}.", MAX_V}
}

fn shadowing() {
    let x = 5;
    println!("x is {}.", x);
    let x = 6;
    println!("x is {}.", x);

    let y = 10;
    let y = y + 1;
    let y = y * 2;
    println!("y is {}.", y);

    let z = "abcdef";
    println!("z is {}", z);
    type__of(z);

    let z = z.len();
    println!("z is {}", z);
    type__of(z);
}

fn int_types() {
    let a : i8 = 1;
    let b : i16 = 1;
    let c : i32 = 1;
    let d : i64 = 1;
    let e : i128 = 1;
    let f : usize = 1;

    println!("{} {} {} {} {} {}", a, b, c, d, e, f);
    size__of(a);
    size__of(b);
    size__of(c);
    size__of(d);
    size__of(e);
    size__of(f);
}

fn numeric_ops() {
    println!("{}", 5 + 10);
    println!("{}", 95.5 - 4.3);
    println!("{}", 4 * 30);
    println!("{}", 56.7 / 32.2);
    println!("{}", 43 % 5);
}

fn tuples() {
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    type__of(tup);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let (_x,y,_z) = tup;
    println!("y = {}", y);
}

fn arrays() {
    let x = [1,2,3,4,5];
    println!("{:?}", x);
    println!("len of x = {}", x.len());
    type__of(x);

    let y = [3; 5];
    println!("{:?}", y);
}

fn main() {
    mutable();
    constants();
    shadowing();
    int_types();
    numeric_ops();
    tuples();
    arrays();
}
