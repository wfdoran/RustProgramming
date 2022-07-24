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

fn main() {
    mutable();
    constants();
    shadowing();
    int_types();
}
