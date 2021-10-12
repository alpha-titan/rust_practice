use std::{collections::{HashMap, HashSet}, mem};

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

fn arrays () {

    let a = [1,2,4];
    println!("a = {:?}",a);
    for i in 0..a.len(){
        println!("{}", a[i]);
    }
   //Initialize a 2D array
   let matrix = [[false;3];3];
   println!("{:#?}", matrix);

   let target = 4;
   let found = a.binary_search(&target);
   match found {
       Ok(val) => println!("{} is at index {}", target, val),
       Err(_) => println!("Target not found")
   }
}

fn slices () {
    let mut a:[i16;4] = [1,2,3,4];
    
    let s = &a[1..3];

    for i in 0..s.len(){
        println!("{}", s[i]);
    }  
}

fn vectors () {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);

    for i in &vec{
        println!("{}", i);
    }
    // for safely accessing values use get to avoid index out of bounds
    let v = vec.get(0);

    match v{
        Some(x) => println!("{}", x),
        None => println!("Not found")
    }
    vec.push(200);
    vec.push(210);
    println!("Popping elements");
    while let Some(x) = vec.pop(){
        println!("{}", x);
    }
    let mut new_vec = vec![0;10];
    new_vec.fill(10);
    println!("{:?}", new_vec);

    let mut vec2 = Vec::new();
    vec2.extend(new_vec);
    println!("{:?}", vec2);
    //? Will panic because the new_vec has been moved to vec2 and destroyed, 
    //? uses into_iter() under the hood
    // println!("{:?}", new_vec);
}

fn hashMap () {
    let mut map:HashMap<&str, u16> = HashMap::new();
    //? add, overwrite existing is also possible for same key
    map.insert("sachin", 21);
    map.insert("shobhit", 23);
    map.insert("sushil", 23);
    println!("{:?}", map);
    //? Loop 

    for (key, value) in &map{
        println!("Key - {}, value - {}", key, value);
    }
    //? Will panic if key is not is found
    println!("Sachin's age is {}", map["sachin".into()]);

    //? check before modifying, if value exists then not modifies else it is added as a new entry
    let mut frequency = HashMap::new();
    for ch in "Sachin Ananthakumar".chars(){
        if ch != ' '{
            //? if key is not found then add 0 and return the address of the value else return the existing entry value
            let counter = frequency.entry(ch.to_lowercase().to_string()).or_insert(0);
            *counter+=1;
        }
    }
    println!("{:#?}", frequency );
}

fn hashSet () {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(3);
    //? Print set
    println!("{:#?}", set);
    
    let need = 1;
    println!("{} exists in set ? {}",need, set.contains(&need));
    let isRemoved = set.remove(&3);
    println!("Removed {}",isRemoved);

    let _1_to_10:HashSet<_> = (1..=10).collect();
    let _3_10: HashSet<_> = (3..=10).collect();
    let _11_20: HashSet<_> = (11..=20).collect();
    let _4_6: HashSet<_> = (4..=6).collect();

    println!("{:?} is subset of {:?}, {:?}", _3_10, _1_to_10, _3_10.is_subset(&_1_to_10));
    println!("union of {:?}, {:?}, {:?}", _3_10, _1_to_10, _3_10.union(&_1_to_10));
    println!("intersection of {:?}, {:?}, {:?}", _3_10, _1_to_10, _4_6.intersection(&_1_to_10));
    println!("are these disjoint {:?}, {:?}, {:?}", _3_10, _1_to_10, _3_10.is_disjoint(&_1_to_10));
    // parent set - subset
    println!("difference of {:?}, {:?}, {:?}", _4_6, _1_to_10, _1_to_10.difference(&_4_6));
 
}

fn Strings () {
    let mut string = String::new();
    string.push('a');
    println!("{}", string);
    let c_string = " bat";
    let mut new_string = string + c_string;
    println!("{}", new_string);
    new_string.remove(0);
    println!("{}", new_string);
    //? covert &str to String
    let con_string = String::from("Hello World");
    println!("{}", con_string);
    for char in con_string.chars(){
        print!("{}", char);
    }
    println!();
    let str_vec:Vec<&str> =  con_string.split(" ").collect();
    println!("{:?}", str_vec );
    for char in con_string.chars().rev(){
        print!("{}", char);
    }

    //? Static string
    println!();
    let s:&'static str = "I am Sachin";
    let replace = s.replace("Sachin", "Sachin AK");
    println!("{}", replace);
    if let Some(h) = s.chars().nth(100){
        println!("{}", h);
    }

}

fn main() {
    Strings()
}
