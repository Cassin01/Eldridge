use super::make_list;

pub fn implement(tree: make_list::Tree) {
    make_list::Tree::read(tree);
}

impl make_list::Tree {
    fn out(mut stream: Vec<String>) -> Vec<String> {
        if let Some(x) = stream.pop() {
            println!("{}", x);
        }
        stream
    }

    fn add(mut stream: Vec<String>) -> Vec<String> {
        let r: String;
        let l: String;
        if let Some(x) = stream.pop() {
            r =  x;
        } else {
            r = String::from("parameter is not exit");
        }

        if let Some(x) = stream.pop() {
            l = x
        } else {
            l = String::from("parameter is not exit")
        }

        let ri32: i64 = r.parse().unwrap();
        let li32: i64 = l.parse().unwrap();

        let ans = (ri32 + li32).to_string();

        stream.push(ans);
        stream
    }

    pub fn read(mut tree: make_list::Tree) {
        let mut stream: Vec<String> = Vec::new();
        loop {
            match tree {
                make_list::Tree::Nil => break,
                make_list::Tree::Cons(object, y) => {
                    match object.id.as_ref() {
                        "val" => stream.push(object.name),
                        "num" => stream.push(object.name),
                        "fn"  => {
                            match object.name.as_ref() {
                                "add" => stream = make_list::Tree::add(stream),
                                "out" => stream = make_list::Tree::out(stream),
                                _     => (),
                            }
                        },
                        _                   => (),
                    }
                    tree = make_list::Tree::extract_from_box(y);
                },
            }
        }

    }

    fn extract_from_box(box_tree: Box<make_list::Tree>) -> make_list::Tree {
        match *box_tree {
                make_list::Tree::Nil => make_list::Tree::Nil,
                y => {
                    y
                },
        }
    }
}
