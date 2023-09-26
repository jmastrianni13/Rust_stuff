use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    fn default_behavior(&self) -> String {
        String::from("this is some default behavior")
    }
    fn summarize_author(&self) -> String;
    fn read_more(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

//pub fn notify<T: Summary>(item: &T) { // trait bound syntax
pub fn notify(item: &impl Summary) { // syntactic sugar
    // function uses a trait as a param
    // will accept any type that implements the Summary trait
    // this func can call any method from the Summary trait
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    // can return any type that implements the Summary trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
 
//pub fn switch_returns_summarizable(switch: bool) -> impl Summary{
    // this is not allowed - see Chapter 17 for details
//    if switch {
//        NewsArticle {
//            headline: String::from("Headline from summarizable NewArticle"),
//            location: String::from("returns_summarizable"),
//           author: String::from("some author"),
//            content: String::from("summarizable content from NewsArticle"),
//        }
//    } else {
//        Tweet {
//            username: String::from("twitter user"),
//            content: String::from("tweet from summarizable Tweet"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}
      
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x,y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x>=self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

