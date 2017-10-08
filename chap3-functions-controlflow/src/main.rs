fn add(x: i32, y: i32) {
    println!("lame addem up {}", x + y);
}

fn expressed () -> i32 {
    42
}

fn main() {
    add(5,4);
    let everything = expressed();
    println!("The anser to everything is {0}", everything);

    // control flow...
    let rust = true;
    if rust {
      println!("Just like other languages, but no parens needed");
    }

    // if is an expression, can use it with let
    // each arm must eval to the same data type
    let dog =  if rust {
        "goofy"
    } else {
        "big red"
    };
    println!("Dog's name is {}", dog);

    // loop
    let mut x = 0;
    loop {
        x = x+1;
        if x == 100 {
            println!("I looped {} times", x);
            break;
        }
    }

    // while
    while x != -1 {
        println!("Counting down...{}", x);
        x = x-1;
    }

    // for
    let arr = [1,3,5,7,9,11,13,17,19];
    for element in arr.iter() {
        println!("Primes are ... {}", element);
    }
    
    // AWESOME...ranges like Ruby...love it!
    for number in (0..11).rev() {
        if number > 0 {
            println!("Lifing off in {}", number);
        } else {
            println!("LIFTOFF");
        }
    }
}
