mod tree;

pub fn main() {
    let root = super::args::root();
    tree::print(root, "".to_string(), true, 0);
}
