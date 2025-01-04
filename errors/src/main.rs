fn main() {
    it_that_panics();
}

fn it_that_panics() {
    let mut v = Vec::new();
    v.push(6);
    v.push(9);

    v[420];
}
