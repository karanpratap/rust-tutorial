/// Mergesort start
fn merge_helper<T>(mut vec: Vec<T>, left: usize, middle: usize, right: usize) -> Vec<T>
where T: PartialOrd + Copy {
    let mut left_ptr = left;
    let mut right_ptr = middle + 1;
    let mut temp_vec: Vec<T> = vec![];
    while left_ptr <= middle && right_ptr <= right {
        if vec[right_ptr] < vec[left_ptr] {
            temp_vec.push(vec[right_ptr]);
            right_ptr += 1;
        } else {
            temp_vec.push(vec[left_ptr]);
            left_ptr += 1;
        }
    }

    // Push remaining elements
    while left_ptr <= middle {
        temp_vec.push(vec[left_ptr]);
        left_ptr += 1;
    }
    while right_ptr <= right {
        temp_vec.push(vec[right_ptr]);
        right_ptr += 1;
    }

    // Copy back sorted elements
    let mut counter = 0; 
    for i in left..right + 1 {
        vec[i] = temp_vec[counter];
        counter += 1;
    } 

    vec
}

/// Function to merge-sort an array in-place
/// @param[in] left The starting index of the vector inclusive
/// @param[in] right The end index of the vector inclusive
fn merge_sort<T>(mut vec: Vec<T>, left: usize, right: usize) -> Vec<T>
where T: PartialOrd + Copy {
    if right <= left {
        return vec;
    } else if right - left == 1 {
        if vec[left] > vec[right] {
            vec.swap(left, right);
        }
        return vec;
    }
    let middle = (left + right) / 2;
    let vec = merge_sort(vec, left, middle);
    let vec = merge_sort(vec, middle + 1, right);
    let vec = merge_helper(vec, left, middle, right);
    vec
}
/// Mergesort end

/// Quicksort start
fn partition_helper<T>(mut vec: Vec<T>, left: usize, right: usize) -> (Vec<T>, usize)
where T: PartialOrd + Copy {
    let pivot_index = right;
    let mut store_index = left;
    for i in left..right {
        if vec[i] < vec[pivot_index] {
            vec.swap(i, store_index);
            store_index += 1;
        }
    }
    vec.swap(store_index, pivot_index);
    (vec, store_index)
}

fn quick_sort<T>(vec: Vec<T>, left: usize, right: usize) -> Vec<T>
where T: PartialOrd + Copy {
    if left < right {
        let (vec, partition) = partition_helper(vec, left, right);
        let vec = quick_sort(vec, left, partition - 1);
        let vec = quick_sort(vec, partition + 1, right);
        vec
    } else {
        vec
    }
}
/// Quicksort end

fn main() {
    let vec = vec![1, 5, 8, 2, 3, 4, 9, 210, 100];
    let len = vec.len();
    let vec = merge_sort(vec, 0, len - 1);

    println!("Int Vector after merge sorting: {:?}", vec);

    let vec = vec![1.1, 5.8, 8.2, 2.3, 3.5, 4.9, 9.1, 21.0, 10.1];
    let len = vec.len();
    let vec = quick_sort(vec, 0, len - 1);

    println!("Float Vector after quick sorting: {:?}", vec);

    let vec = vec![1, 5, 8, 2, 3, 4, 9, 210, 100];
    let len = vec.len();
    let vec = quick_sort(vec, 0, len - 1);

    println!("Int Vector after quick sorting: {:?}", vec);

    let vec = vec![1.1, 5.8, 8.2, 2.3, 3.5, 4.9, 9.1, 21.0, 10.1];
    let len = vec.len();
    let vec = merge_sort(vec, 0, len - 1);

    println!("Float Vector after merge sorting: {:?}", vec);
}

