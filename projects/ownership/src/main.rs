fn strings() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    // println!("full = {full}, first = {first}"); this would fail due ownership!
    println!("{full}");

}

fn boxes() {
    let b = Box::new(8);
    move_box(&b);
    println!("{b}");

}

fn vecs() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(3);
    let v2: &i32 = &v[3];
    println!("ori = {}, point = {}", v[3], v2);

    let mut c = vec![1, 2, 5];
    let mut c2 = &mut c[2];
    let c3 = &c2;
    println!("{}, {}", *c2, **c3);

    // this example was awful... created more questions and provides 0 answers!!
    // TOD
    // let mut test = vec![1, 2, 3, 4];
    // let test2 = &test;
    // give_and_take(&mut test, 5);
    // println!("{:#?}, {:#?}", test, test2)

}

fn give_and_take(v: &mut Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}

fn references() {
    let b1 = Box::new(3);
    let b2 = Box::new(&b1);
    println!("{:#?}",b2);
}

fn main() {
    vecs();
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn move_box(b: &Box<i32>) {
    println!("{b}");
}
