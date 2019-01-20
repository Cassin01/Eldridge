use std::fmt;

#[derive(Debug)]
struct Object {
    id: String,
    name: String,
}

#[derive(Debug)]
enum Tree {
    Nil,
    Cons(Object, Box<Tree>),
    List(Object, Box<Tree>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object({}, {})", self.id, self.name)
    }
}

impl Tree {
    // 空Tree を返す
    fn new() -> Tree {
        Tree::Nil
    }

    // データを追加する
    fn cons(self, object: Object) -> Tree {
        Tree::Cons(object, Box::new(self))
    }

    fn list(self, object: Object) -> Tree {
        Tree::List(object, Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
