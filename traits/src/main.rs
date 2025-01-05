// traits in rust

pub trait Summary {
    fn summarize(&self) -> String {
        // default behavior if summarize is not implemented
        String::from("(read more ...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// above function same as:
pub fn notity_copy<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

pub fn s_f<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// is the same as: 
pub fn s_f2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// returns any time that implements the Summary trait: 
pub fn ret_sum() -> impl Summary {
    Tweet {
        username: String::from("foo");,
        content: String::from("bar");,
        reply: false,
        retweet: false,
    }
}

fn main() {
}
