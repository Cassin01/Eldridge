mod make_list;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_code2() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("file not found");

    let mut texts = String::new();
    for line in BufReader::new(file).lines() {
        texts.push_str(&line.unwrap().trim_right().to_owned());
    }
    texts
}

/*
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
*/

fn is_space(word: &String) -> bool {
    match word.as_str() {
        "" => true,
        _  => false,
    }
}

fn is_num(texts: &String) -> bool {
    for c in texts.chars() {
        match c {
            '1' => (),
            '2' => (),
            '3' => (),
            '4' => (),
            '5' => (),
            '6' => (),
            '7' => (),
            '8' => (),
            '9' => (),
            _   => return false
        }
    }
    true
}

fn read_texts(texts: String) -> make_list::Tree {
    let mut string = String::new();
    let mut tree = make_list::Tree::new();

    for c in texts.chars() {
        if c == ','
        || c == ')'
        || c == '('
        || c == ' ' {
            let name = string.clone();
            let id: String;
            if !is_space(&name) {
                if is_num(&name) {
                    id = String::from("num");
                } else {
                    id = String::from("fn");
                }
                string.clear();

                tree = tree.cons(
                    make_list::Object {
                        id: id,
                        name: name
                    }
                );
            }
        } else {
            string.push(c);
            // println!("{:?}", string);
        }
    }
    tree = tree.cons(
        make_list::Object {
            id: String::from("fn"),
            name: string.clone(),
        }
    );
    string.clear();
    tree
}


fn main() {
    let i = read_code2();
    //println!("{}", i);
    println!("{:?}", read_texts(i));
}
