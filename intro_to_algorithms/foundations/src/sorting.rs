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

fn merge_sort(items: &mut Vec<i32>, p: usize, r: usize) {
    if p >= r {
        return;
    }

    let q = (p + r) / 2;
    merge_sort(items, p, q);
    merge_sort(items, q+1, r);
    merge(items, p, q, r);

}

fn merge(items: &mut Vec<i32>, p: usize, q: usize, r: usize) {
    let nl = q - p + 1; 
    let nr = r - q;

    let mut l_items = vec![0; nl];
    let mut r_items = vec![0; nr];

    for i in 0..nl-1 {
        l_items[i] = items[p+i];
    }
    for j in 0..nr-1 {
        r_items[j] = items[q + j + 1];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < nl && j < nr {
        if l_items[i] <= r_items[j] {
            items[k] = l_items[i];
            i += 1;
        } else {
            items[k] = r_items[j];
            j += 1;
        }
        k += 1;
    }

    while i < nl {
        items[k] = l_items[i];
        i += 1;
        k += 1;
    }

    while j < nr {
        items[k] = r_items[j];
        j += 1;
        k += 1;
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

    #[test]
    fn test_merge_sort() {
        // let mut items: Vec<i32> = vec![];
        // merge_sort(&mut items, 0, 0);
        // assert_eq!(vec![] as Vec<i32>, items);

        let mut items: Vec<i32> = vec![5, 4, 2, 6, 1, 3];
        let p = 0;
        let r = items.len();
        merge_sort(&mut items, p, r);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], items);
    }
}
