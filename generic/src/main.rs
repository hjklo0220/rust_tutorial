mod largest;

use crate::largest::func;
use crate::largest::point;

use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = func::largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = func::largest(&char_list);
    println!("The largest char is {}", result);

    // let integer = point::Point { x: 5, y: 4 };
    // let float = point::Point { x: 5.5, y: 4.0 };
    // let integer_and_float = point::Point { x: 5, y: 9.9 };

    let p = point::Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.x());
    println!("{}", p.distance_from_origin());

    let p1 = point::PointV2 {
        x: 5,
        y: 10.4,
    };
    let p2 = point::PointV2{
        x: "hello",
        y: 'c',
    };
    
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Some(5);
    let float = Some(5.0);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

}


