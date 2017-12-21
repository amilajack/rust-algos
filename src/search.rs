pub fn binary_search(list: Vec<i32>, item: i32) -> bool {
    let mut first_index = 0;
    let mut last_index = list.len() - 1;

    while first_index != last_index {
        let middle_index = (list.len() - 1) / 2;
        let middle_value = list.get(middle_index).expect("Index out of bounds");

        if middle_value == &item {
            return true;
        }

        if middle_value < &item {
            first_index = middle_index + 1;
        } else {
            last_index = middle_index - 1;
        }
    }

    false
}
