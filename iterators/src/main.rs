// iterators in rust
#![allow(unused)]

fn main() {
    let v1 = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    for (i, val) in v1_iter.enumerate() {
        println!("Got: {}, {}", val, i);
    }

    let v2 = vec![2, 3, 4, 5];
    let mut v2_iter = v2.iter();
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    // calling Iterator.next() changes the internal state the iterator uses to keep track of where
    // it is in the sequence

    let v3: Vec<i32> = vec![3,4,5,6];
    let mut v4_iter = v3.iter().map(|x| x*2);
    // call collect() at the end of map() to return v4 as a Vec instead of an Iterator
    println!("{:?}", v4_iter.next());

    let v5: Vec<i32> = vec![5, 6, 7, 8, 9, 10, 11, 12, 13];
    let v6: Vec<_> = v5.iter().filter(|x| x >= &&9).collect();
    // && needs to preceed 9 because filter() only uses a reference of each value in the vec
    println!("{:?}", v6);
    
}
