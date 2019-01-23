mod make_list;
mod implement;
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
        }
    }
    let name = string.clone();
    if !is_space(&name) {
        tree = tree.cons(
            make_list::Object {
                id: String::from("fn"),
                name: string.clone(),
            }
        );
    }
    string.clear();
    tree
}


fn main() {
    let i = read_code2();
    //println!("{}", i);
    let tree = read_texts(i);
    println!("{:?}", tree);

    implement::implement(tree);
}
