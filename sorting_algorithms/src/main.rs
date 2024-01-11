/// Deprecated, moving to generic
fn merge_helper(mut vec: Vec<u32>, left: usize, middle:usize, right: usize) -> Vec<u32> {
    let mut left_ptr = left;
    let mut right_ptr = middle + 1;
    let mut temp_vec: Vec<u32> = vec![];
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
fn merge_sort(mut vec: Vec<u32>, left: usize, right: usize) -> Vec<u32> {
    if right <= left {
        return vec;
    } else if right - left == 1 {
        if vec[left] > vec[right] {
            let temp = vec[left];
            vec[left] = vec[right];
            vec[right] = temp;
        }
        return vec;
    }
    let middle = (left + right) / 2;
    let vec = merge_sort(vec, left, middle);
    let vec = merge_sort(vec, middle + 1, right);
    let vec = merge_helper(vec, left, middle, right);
    vec
}

fn main() {
    let vec = vec![1, 5, 8, 2, 3, 4, 9, 210, 100];
    let len = vec.len();
    let vec = merge_sort(vec, 0, len - 1);

    println!("Vector after sorting: {:?}", vec);
}
