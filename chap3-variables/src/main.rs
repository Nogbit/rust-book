fn main() {
    // VARS
    let x = 5;
    println!("The value of x is {}", x);
    // must be mutable
    // x = 6;
    let mut y = 6;
    println!("The value of y is {}", y);
    y = 7;
    println!("The value of y is {}", y);

    // CONSTANTS
    const MAX_POINTS: u32 = 4294967295;
    //let z = u32::max_value();
    println!("const must have data type and set to constant expression {}", MAX_POINTS);

    // SHADOWING
    // hmmm, redeclare a var in same scope
    let num = 9;
    let num = 8;
    println!("The value of num int is {}", num);
    let num = "eight";
    println!("The value of num string is {}", num);

    // floats
    let s = 5.0; // f64...default
    let t: f32 = 3.0; // f32

    // chars...Unicode Scalar...cool
    let c = 'z';
    let d = '😎';
    let e = '\u{1F44F}'; //U+1F3FB
    println!("Unicode emojis mmm kay {} {}", d, e);

    // tuples...great!
    let tup = (9, 8, 7);
    let nine = tup.0;
    println!("tuples are quick and easy {}", nine);

    let tup = (6,5,4);
    let (six, five, four) = tup;
    println!("tuples can be named {}", five);

    // arrays, cant grow or shrink
    // vectors are also available to use
    let mut arr = [3,7,9];
    arr[0] = 1;
}
