fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;  // constants are always immutable, must have type annotation
    // constants are valid for the entire time a program runs, within the scope they were declared in.
    // This makes them a useful choice for values in your app domain that multiple parts of
    // the program might need to know about, such as the maximum number of points any player
    // of a game is allowed to earn or the speed of light
    // naming hardcoded values used throughout your program as constants is useful in conveying the
    // meaning of that value to future maintainers of the code.

    // **************************
    // SHADOWING
    // **************************
    let x = 4;
    let x = x + 1;  // creating new immutable variable called x from existing immutable variable
    let x = x * 2; // creating new immutable variable called x from existing immutable variable
    println!("The value of x is {}", x);

    /*
    every value in Rust is of a certain data type, which tells Rust what kind of
    data is being specified so it knows how to work with that data. We'll look at two
    data type subsets; scalar and compound.

    Keep in mind that Rust is a statically typed language, which means it must know the types
    of all variables at compile time. The compiler can usually infer what type we want to use based
    on the value and how we use it.

    In cases when many types are possible, such as when we converted a `String` to a numeric type
    using `parse`, we must add a type annotation, like this:
    */
    let guess: u32 = "42".parse().expect("Not a number!");

    // **************************
    // SCALAR TYPES
    // **************************
    // A scalar type represents a single value. Rust has four primary scalar types:
    // integer, floating-point numbers, Booleans, and characters.
    // You may recognize these from other programming languages.
    // Let's jump into how they work in Rust.
    /***************************
    INTEGER TYPES
    **************************
    An integer is a number without a fractional component. Each variant can be either
    signed or unsigned and has an explicit size.

    Signed and unsigned refer to whether it's possible for the number to be negative—in other words,
    whether the number needs to have a sign with it (signed) or whether it will only ever be
    positive and can therefore be represented without a sign (unsigned).

    Each signed variant can store numbers from -(2^(n-1)) to (2^(n-1)-1) inclusive, where n is the
    number of bits that variant uses.
    So an i8 can store numbers from -(2^7) to 2^7 -1, which equals -128 to 127

    Length	Signed	Unsigned
    ------------------------
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize



    # literal	    Example
    ------------------------
    Decimal	        98_222
    Hex	            0xff
    Octal	        0o77
    Binary	        0b1111_0000
    Byte (u8 only)	b'A'

    */
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    let t = true;
    let f = false;
    let c = 'z';
    let z = 'Ω';
    let heart_eyed_cat = '😻';

    /*
    COMPOUND TYPES

    Compound types can group multiple values into one type. Rust has two primitive compound types:
    tuples and arrays.

    The Tuple Type:
    A tuple is a general way of grouping together a number of values with a variety of types
    into one compound type. Tuples have fixed length: once declared, they cannot grow or shrink in size.

    We create a tuple by writing a comma-separated list of values inside parentheses. Each position
    in the tuple has a type, and the types of the different values in the tuple don't have to
    be the same.

    We've added optional type annotations in this example.

    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of (x, y, z) is: ({}, {}, {})", x, y, z);
    // println!("And here's the tup! {}", tup);
    println!("The value of tup.(0, 1, 2), {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    // every element in array must have the same type
    // arrays have fixed size in Rust!
    let arr = [0; 5];

    for x in &arr {
        print!("{}", x);
    }

}
