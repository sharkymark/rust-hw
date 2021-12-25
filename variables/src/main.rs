use std::slice;
use std::str;

fn main() {
    let mut x = 5;
    const Y_CONST:i32 = 10;

    println!("constant e.g.");
    println!("The value of Y_CONST is: {}", Y_CONST);




    println!("mutable variable e.g.");
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);



    println!("shadowed variable e.g.");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("changing the type, with shadowing: The value of spaces is: {}", spaces);


    // rust is statically typed, meaning it must know the types of variables at compile time
    println!("data types e.g.");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // scalar types represent a single value like integer, floating-point numbers, Booleans, and characters

    let markint = 42;
    println!("The value of markint is {}", markint);


    println!("float-point types e.g.");
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {} and y is {}", x,y);    


    println!(" Boolean e.g.");
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {} and f is {}", t,f);  



    println!(" operators e.g.");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4.0 * 31.543;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("The value of sum is: {} and difference is {}", sum,difference);  
    println!("The value of product is: {} and quotient is {}", product,quotient);  
    println!("The value of remainder is: {}", remainder);  


    println!(" char e.g.");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {} and z is {} and hear_eyed_cat is {}", c,z, heart_eyed_cat); 


    println!(" tuple type e.g.");
    let tup: (i32, f64, u8, char, &'static str) = (500, 6.4, 1, 'M',"Mark Milligan");
    let (x,y,z,c,s) = tup;
    println!("The value of x is: {} and y is {} and z is {} and c is {} and s is {}", x,y,z,c,s);  
    let ptr = s.as_ptr();
    let len = s.len();
    println!("The value of ptr is {:?} and of len is {}",ptr,len);
    let ss = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(ptr, len);
    
        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };

    println!(" Array - every element must have the same type - and have fixed lengths e.g.");
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [69;2];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("a: The value of element 1 is: {} and element 2 is {}", a[0],a[1]);  
    println!("b: The value of element 1 is: {} and element 2 is {}", b[0],b[1]);  
    println!("months: The value of element 1 is: {} and element 2 is {}", months[0],months[1]);  





    println!(" e.g.");

    println!("The value of x is: {} and y is {}", x,y);  

}
