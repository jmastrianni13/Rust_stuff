
pub fn main() {

}

fn insertion_sort(items: &mut Vec<i32>) {
    let mut j;
    let mut key;
    for i in 1..items.len() {
        key = items[i];
        j = i;
        while j > 0 && items[j-1] > key {
            items[j] = items[j-1];
            j = j - 1;
        items[j] = key;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut items: Vec<i32> = vec![5, 4, 2, 6, 1, 3];
        insertion_sort(&mut items);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], items);
    }

}
        
