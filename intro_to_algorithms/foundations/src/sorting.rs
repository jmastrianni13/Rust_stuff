pub fn insertion_sort(items: &mut Vec<i32>) {
    let mut j;
    let mut key;
    for i in 1..items.len() {
        key = items[i];
        j = i;
        while j > 0 && items[j - 1] > key {
            items[j] = items[j - 1];
            j = j - 1;
        }
        items[j] = key;
    }
}

pub fn insertion_sort_dec(items: &mut Vec<i32>) {
    let mut j;
    let mut key;
    for i in (0..items.len()).rev() {
        key = items[i];
        j = i;
        while (j < items.len() - 1) && items[j + 1] > key {
            items[j] = items[j + 1];
            j = j + 1;
        }
        items[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut items: Vec<i32> = vec![];
        insertion_sort(&mut items);
        assert_eq!(vec![] as Vec<i32>, items);

        let mut items: Vec<i32> = vec![5, 4, 2, 6, 1, 3];
        insertion_sort(&mut items);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], items);

        let mut items: Vec<i32> = vec![31, 41, 59, 26, 41, 58];
        insertion_sort(&mut items);
        assert_eq!(vec![26, 31, 41, 41, 58, 59], items);
    }

    #[test]
    fn test_insertion_sort_dec() {
        let mut items: Vec<i32> = vec![];
        insertion_sort_dec(&mut items);
        assert_eq!(vec![] as Vec<i32>, items);

        let mut items: Vec<i32> = vec![5, 4, 2, 6, 1, 3];
        insertion_sort_dec(&mut items);
        assert_eq!(vec![6, 5, 4, 3, 2, 1], items);

        let mut items: Vec<i32> = vec![31, 41, 59, 26, 41, 58];
        insertion_sort_dec(&mut items);
        assert_eq!(vec![59, 58, 41, 41, 31, 26], items);
    }
}
