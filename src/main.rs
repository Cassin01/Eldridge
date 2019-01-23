mod make_list;
mod read_code;
mod implement;

fn add(opiton: Option<String>) -> String {
    match opiton {
        Some(m) => m,
        None    => panic!("Err: element is not exist."),
    }
}

fn code_stack(mut texts: Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = Vec::new();
    while !texts.is_empty() {
        let text = texts.pop();
        stack.push(add(text));
    }
    stack
}

fn tree(mut stack: Vec<String>) -> make_list::Tree {
    let mut tree = make_list::Tree::new();
    while !stack.is_empty() {
        let id   = stack.pop();
        let name = stack.pop();
        if id == Some(String::from("fn")) || id == Some(String::from("val")) {
            tree = tree.cons(
                make_list::Object {
                    id: add(id),
                    name: add(name)
                }
            );
        } else {

        }
    }
    tree
}

fn main() {
    // 単語ずつ読み込み
    let texts: Vec<String> = read_code::read();
    // println!("text: {:?}", texts);

    // スタックに貯める
    let stack: Vec<String> = code_stack(texts);
    // println!("stack: {:?}", stack);

    // スタックをポップしてB木を作る
    let tree : make_list::Tree = tree(stack);
    println!("B tree: {:?}", tree);

    implement::implement(tree);
}
