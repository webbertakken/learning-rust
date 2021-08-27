// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables, dead_code)]

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // Signature only
    fn summarise_author(&self) -> String;

    // Default implementation
    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }
}

pub trait OtherTrait {}

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("{}", self.username)
    }

    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise())
}

// Identical to, and could also be written as:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarise())
// }

// Can also make constrains on multiple traits, like so
pub fn advanced_notify(item: &(impl Summary + OtherTrait)) {
    println!("Breaking news! {}", item.summarise())
}

// Or when the types become long we can use the where class instead
pub fn long_advanced_notify<T, U>(t: &T, u: &U)
where
    T: Summary + OtherTrait,
    U: OtherTrait,
{
    // Do stuff
}

// when traits

pub fn main() {
    println!("\ntraits...");

    let article = NewsArticle {
        author: String::from("Webber"),
        headline: String::from("Amazing story about hello world!"),
        content: String::from(
            "Hello world is the first thing we output as developers, when we learn a new language.",
        ),
    };

    let tweet = Tweet {
        username: String::from("@webbertakken"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    println!("Article summary: {}", article.summarise());

    notify(&tweet);
}
