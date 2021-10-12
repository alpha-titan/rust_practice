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


fn structs () {

    struct Human {
        name:String,
        age:i32
    }

    let person = Human{name:"sachin".to_string(), age:21};
    println!("Hi I am {}! and I am {} years old.", person.name ,person.age);
}

fn enums () {
    #[derive(Debug)]
    enum Language {
        Javascript,
        Python,
        Rust
    }

    let primary_lang = Language::Javascript;

    let new_lang = match primary_lang {
        Language::Javascript => "All Rounder",
        Language::Python => "Good For Scripting",
        Language::Rust => "Awesome C Alternative"
    };

    println!("{:#?} is a {}", primary_lang, new_lang);
}

fn inlineIf () {

    let age =19;
    let is_medical_condition = false;
    let is_eligible_for_drivers_license =  if age >= 19 {
        if !is_medical_condition {
            true
        }
        else {
            false
        }
    }else{
        false
    };

    println!("I am {} years old and my drivers license status is {}", age, is_eligible_for_drivers_license);

}

fn options () {
    #[derive(Debug)]
    let status_code = Some(200);

    // let status_description = if status_code == 200 {Some(status_code)}else {None}

    let response = match status_code {
        
        Some(200) => "Success",
        None => "Not Success",
        _ => "Not valid"
    };

    println!("status_code {:#?} means {}", status_code, response);

}

fn main() {
    options();
}
