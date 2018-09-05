extern crate aggregator;

use aggregator::Tweet;
use aggregator::Summarizable;

struct NewsArticle0 {}
impl Summarizable for NewsArticle0 {}

#[test]
fn test_module() {
    println!(">>>");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());
    aggregator::notify(tweet);

    let article = NewsArticle0{};
    println!("{}", article.summary());
}
