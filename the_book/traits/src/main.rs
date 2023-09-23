use traits::{NewsArticle, Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("twitter_user"),
        content: String::from("my first tweet"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", tweet.default_behavior());
    println!("{}", tweet.read_more());
  
    let newsarticle = NewsArticle{
        headline: String::from("news article headline!"),
        location: String::from("news article location"),
        author: String::from("news article author"),
        content: String::from("news article content"),
    };

    println!("{}", newsarticle.summarize());
    println!("{}", newsarticle.default_behavior());
    println!("{}", newsarticle.read_more());
}

