fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SPECIAL_NUMBER: i32 = 42;
    println!("The value of SPECIAL_NUMBER is: {}", SPECIAL_NUMBER);
    println!("The value of GLOBAL_CHAR is: {}", GLOBAL_CHAR);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let x = 'A';
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);

    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("The value of b is: {:?}", b);
    println!("The value of c is: {:?}", c);

    const THREE_AND_A_BIT:f32 = 3.4028236;
    println!("The value of THREE_AND_A_BIT is: {}", THREE_AND_A_BIT);
}

const GLOBAL_CHAR: u32 = 25;
