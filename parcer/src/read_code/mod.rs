use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_code() -> String {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // {}というファイルの中
    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    // テキストは\n{}です
    //println!("With text:\n{}", contents);


    //read_word で単語が別れているかを確認
    //println!("{:?}", read_word(contents));

    // 末尾の改行文字を削る
    contents.trim_right().to_owned()
}

fn read_texts(texts: String) -> Vec<String> {
    //let a = String::from("fn main fn add val 1 val 2");
    let mut strings = Vec::new();
    let mut string = String::new();

    for c in texts.chars() {
        if c == ' ' {
            strings.push(string.clone());
            string.clear();
        } else {
            string.push(c);
            // println!("{:?}", string);
        }
    }
    strings.push(string.clone());
    string.clear();
    strings
}

pub fn read() -> Vec<String> {
    read_texts(read_code())
}