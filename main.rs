fn main(){
    // const MAX_POINTS: u32 = 100_000;
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The vlaue of x is: {}", x);
    // println!("The value of const MAX_POINTS is: {}", MAX_POINTS)
    // /////////////     SHADOWING  ///////////////
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // // println!("The value of x is: {}", x)
    // let spaces = "     ";
    // let spaces = spaces.len();
    // println!("length of spaces is: {}", spaces);
    // let guess: u32 = "42".parse().expect("Not a number");
    // println!("{}", guess);
    ////////// TUPLE /////////// tuple have multiple data types and with varied length
    //let tup: (i32, f64, u8, char) = (400, 4.56, 1, 'a');
    //let (x, y, z, c) = tup;
    // let a = tup.0;
    // let b = tup.1;
    // let c = tup.2;
    // let d = tup.3;
    // println!("{}{}{}{}", a, b, c, d)

    /////// ARRAY //////////////// array are fixed length and must have same data type, RUST will panic if we 
    // use variable for index number.
    // let a = [1, 2, 3, 4, 5];
    // let index = 3;
    // let element = a[index];
    // println!("{}", element)

    // FUNCTION ............
    //     println!("Hello World!");
    //     another_function(5, 6);
 
    // }
    // fn another_function(x: i32, y: i32){
    //     println!("{}", x);
    //     println!("{}", y);
    // }
    // let x = 5;
    // let y = {
    //     let x = 3;
    //     x +1
    // };
    // println!("value of y is: {}", y);
    // let mut s = String::from("Hello");
    // s.push_str(" World!");
    // println!("{}", s)
    ///// 4.1
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}.", s2, len);

}
fn calculate_length(s: String) -> (String: usize) {
    let length = s.len();
    (s, length)
}

