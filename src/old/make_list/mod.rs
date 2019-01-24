use std::fmt;

#[derive(Debug)]
pub struct Object {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Tree {
    Nil,
    Cons(Object, Box<Tree>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object({}, {})", self.id, self.name)
    }
}

impl Tree {
    // 空Tree を返す
    pub fn new() -> Tree {
        Tree::Nil
    }

    // データを追加する
    // fn 用(仮)
    pub fn cons(self, object: Object) -> Tree {
        Tree::Cons(object, Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
