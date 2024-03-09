const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

fn hours_to_minutes(h: i32) -> i32 {
    60 * h
}

fn f_to_c(deg: i32) -> i32 {
    (deg - 32) * (5/9)
}

fn c_to_f(deg: i32) -> i32 {
    (deg * (9/5)) + 32
}

fn fibonnacci(num: u32) -> u32 {
    let mut res = if num <= 0 {0} else {1};

    let mut aux = res;
    let old = aux;
    for i in (1..num) {
        let old = res;
        res = res + aux;
        aux = old;
    }
    return res;
}

fn print_shadowing() {
    // TODO: check what addresses they take
    let y = 3;
    println!("y = {y}");
    let y = y + 3;
    println!("y = {y}");
    {
        let y = y * 3;
        println!("y = {y}");
    }
    println!("y = {y}");
}

fn variables_lesson() {
    let mut x = 3;
    println!("The value of x is {}", x);
    x = x + 1;
    println!("The value of x is {}", x);
    x = x + 1;
    println!("The value of x is {}", x);


    print_shadowing();

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
    // assigning new type is permitted with let
    // but gives errors when mutable
    
    let byte : u8 = b'a';
    let bin : u8 = 0b110_0001;
    println!("byte = {}", byte);
    println!("bin = {}", bin);

    // tuples
    let t1 : (u8, f32, char) = (33, 2.8, 'm');
    let t2 = ("str", 'c', 3);

    let (a, b, c) = t1;
    println!("{:#?}", t1); // Without the 'pretty' print {:#?} the tuple with multiple types won't
                           // print by default
    println!("{}", t2.2);
    println!("{}", c);

    // the array type 
    // default formatter cannot print also!
    let arr : [u8; 3] = [1, 2, 3]; // 3 of u8 type
    let arr2 = [0; 10]; // size 10 with default value 0
    println!("{:#?}", arr);
    // or print by accessing specific member
    println!("{}", arr2[9]);
    // TODO fix after learning slices !
    // println!("{}", arr2[1:1]);

    let minute_in_three_hours = hours_to_minutes(1) * 3;
    println!("{}", minute_in_three_hours);
}

fn main() {
    println!("{}", fibonnacci(7));
}
