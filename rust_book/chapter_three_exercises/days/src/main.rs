use std::io;

fn ordinal(n: usize) -> String {
    match n {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 => "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        _ => "nth".to_string(),
    }
}

fn nth_day(n: usize, gift: String, gifts: &Vec<String>) {

    let collections : Vec<String> = vec![
        String::from("a partridge in a pear tree"),
        String::from("two turtle doves"),
        String::from("three french hens"),
        String::from("four calling birds"),
    ];

//    let n = n + 1;

    let start: String = String::from("On the ");
    let middle: String = String::from("day of Christmas, my true love gave to me");
    println!("{} {} {} {}", start, ordinal(n+1), middle, gifts[n]);

    for count in 1..n {
        let countdown = n - count;
        println!("{}, ",gifts[countdown]);
    }

    if n >= 1 { println!("and a {}, ",gifts[0]); }
}

fn main() {
    //let intro: String = String::from("On the first day of Christmas, my true love gave to me");
    let intro: String = String::from("On the {} day of Christmas, my true love gave to me");

    let collections : Vec<String> = vec![
        String::from("a partridge in a pear tree"),
        String::from("two turtle doves"),
        String::from("three french hens"),
        String::from("four calling birds"),
        String::from("five calling birds"),
        String::from("six calling birds"),
        String::from("seven calling birds"),
        String::from("eight calling birds"),
        String::from("nine calling birds"),
        String::from("ten calling birds"),
        String::from("eleven calling birds"),
        String::from("twelve calling birds"),
    ];

    let mut lyrics: Vec<String> = vec![intro];

    for (n, gift) in collections.iter().enumerate() {
        //        println!("{} {}", lyrics[0], gift);
        nth_day(n.try_into().unwrap(), gift.to_string(), &collections);
        println!("");
    }
}
