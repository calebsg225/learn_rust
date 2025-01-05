#![allow(unused)]
// generics in rust

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &U {
        &self.y
    }
    fn mixup(self, other: Point<T2, U2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

// methods that only apply to Point when both T and U generic types are f32
impl Point<f32, f32> {
    fn calc_distance_from_origin(&self) -> {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let nums1 = vec![23, 65, 9, 12, 234, 439, 3, 45];
    let nums2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let chars1 = vec!['a', 'b', 'z', 'y', 'q', 'g', 'h'];
    let res1 = largest(&nums1);
    let res2 = largest(&nums2);
    let res3 = largest(&chars1);
    println!("The larges num is: {}", res1);
    println!("The larges num is: {}", res2);
    println!("The larges num is: {}", res3);

    let integer = Point { x = 69, y = 420 };
    let float = Point { x = 6.9, y = 42.0 };
    let both = Point { x = 69, y = 420.00 };
}

fn find_largest(nums: &[i32]) -> &i32 {

    let mut largest_num = &nums[0];

    for num in nums {
        if num > largest_num {
            largest_num = num;
        }
    }

    largest_num
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
