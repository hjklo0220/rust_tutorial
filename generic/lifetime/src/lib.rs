pub fn lifetime_main() {
    let r;

    let x = 5;
    r = &x;

    println!("r: {r}");

    let string1 = String::from("avbcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {result}");

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {result2}");
    }
    let string5 = String::from("long string is long long long");
    let result3;
    {
        let string6 = String::from("abc");
        result3 = longest(string5.as_str(), string6.as_str());
    }
    println!("{result3}");
}

fn longest<'lifetime>(x: &'lifetime str, y: &'lifetime str) -> &'lifetime str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}