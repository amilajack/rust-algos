pub struct Node<T> {
    value: Option<T>,
    left: Node<T>,
    right: Node<T>,
}

pub fn binary_search_tree<T>(root: Node<T>, item: T) -> bool {
    match root.value {
        Some(value) => {
            if item == value {
                return true;
            }
            if item < value {
                binary_search_tree(root.left, item)
            } else {
                binary_search_tree(root.right, item)
            }
        },
        None => false,
    }
}
