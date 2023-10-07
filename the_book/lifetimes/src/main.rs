use std::fmt::Display;

fn main() {
    sample_lifetime_syntax();
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    list_lifetime_elision_rules();

    let s: &'static str = "I have a static lifetime.";

    let longest_str = longest_with_announcement("abc", "wxyz", "I have generics, trait bounds, and lifetimes!");

}

fn sample_lifetime_syntax() {
    println!("Sample lifetime syntax:");
    println!("  &i32 -> a reference");
    println!("  &'a i32 -> a reference with an explicit lifetime");
    println!("  &'a mut i32 -> a mutable reference with an explicit lifetime");
}

fn list_lifetime_elision_rules() {
    println!("Lifetime elision rules:");
    println!(" Rule 1: a lifetime parameter is applied to all inputs of fn or impl");
    println!(" Rule 2: if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters");
    println!(" Rule 3: if there are multiple input lifetime parameters, if one is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters"); 
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        return x
    } 
    
    return y;
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
 
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() >= y.len() {
        return x;
    }
    return y;
}

