fn main() {
    let s = String::from("pew");
    println!("1{s}");
    let s = take_and_give_back_ownership(s);
    println!("3{s}");

    let mut s1 = String::from("ssss");
    println!("1{}", s1);
    // let l = calc_len(&s1);
    let l = calc_len_mut(&mut s1);
    println!("3{}, {}", s1, l);

    let mut s2 = String::from("3rd string, i 0");

    let mut r1 = &mut s2;
    let r2 = &mut r1;

    println!("{}", r2);

    arr_messing();
}

fn take_and_give_back_ownership(new_string: String) -> String {
    println!("2{new_string}");
    new_string
}

fn calc_len(s: &String) -> usize {
    // s is a reference to the inputed string. It does not have ownership.
    // references are immutable by default.
    println!("2{}", s);
    s.len()
}

fn calc_len_mut(s: &mut String) -> usize {
    s.push('a');
    println!("mut{}", s);
    s.len()
}

fn arr_messing() {
   let a: [i32; 5] = [2, 3, 4, 5, 6];
   let slice = &a[1..4];
   println!("{:?} {:?}", a, slice);
}
