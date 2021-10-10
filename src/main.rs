use std::mem;

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]

fn core_datatypes(){
    let x:i64= 0;
    let x= 3;
    println!("Hello, world!");
    println!("x = {}", x); // immutable, cannot reassign
    let mut y:isize = 0;
    y = 22222222;
    println!("y mut ={} and has a size of {} byte in {} op sys",
     y, mem::size_of_val(&y), mem::size_of_val(&y)*8);
    let a = true;
    let b = 3.4;
    let c = 'a';
    let s = "sachin";
}

fn operators () {
    //Operators
    let mut a = 2;
    a+=2; //4
    a-=1; //3
    a*=3; //9
    a/=2; //4 (rounds to floor)
    // a%=2; // 0
    println!("a {} ", a);
    let a_pow = i32::pow(a, 4);
    println!("a pow {} ", a_pow);
    //Integer power
    let b = 2.5;
    let a_int_pow = f64::powi(b, 3);
    let a_float_pow = f64::powf(b, std::f64::consts::PI);
    println!("a_int {}, a_float {}", a_int_pow, a_float_pow);
}

fn main() {
    
}
