use regex::Regex;

fn main() {
    // 数字とそれ以外を分解する正規表現
    let str = "123abc345---678!#$124";
    let regex = Regex::new("(?P<number>[0-9]+)(?P<other>[^0-9]+)").unwrap();
    println!("input: {}", str);
    println!("split <number> <others>");
    for capture in regex.captures_iter(str) {
        println!(
            "number: {}, other: {}",
            capture.name("number").unwrap().as_str(),
            capture.name("other").unwrap().as_str()
        );
    }
    println!("============================");

    // HTML っぽいやつを分解する正規表現
    let str = "<a>User Name</a>";
    let regex = Regex::new("<.+>(?P<element>.+)</.*>").unwrap();
    println!("input: {str}");
    println!("extract element");
    for capture in regex.captures_iter(str) {
        println!("element: {}", capture.name("element").unwrap().as_str(),);
    }
}
