use std::collections::HashMap;

fn main() {

    let mut vec = vec![9, 8, 5, 3, 1, 2, 4, 7, 6, 10];
    
    let mut h: HashMap<i32, u32> = HashMap::new();
    let mut max: (i32, u32) = (0, 0);

    // Sort the vector for median calculation
    for i in 0..vec.len() {
        for j in i..(vec.len()) {
            if vec[i] > vec[j] {
                let temp = vec[i];
                vec[i] = vec[j];
                vec[j] = temp;
            }
        }
    }

    // Create a Hash Map for mode calculation
    for item in vec.iter() {
        let count = h.entry(*item).or_insert(0);
        *count += 1;
        if *count > max.1 {
            max = (*item, *count);
        }
        // println!("{}", item);
    }

    println!("Sorted vector is : {:?}, len: {}", vec, vec.len());
    println!("HashMap is: {:?}", h);
    println!("Mode is: {}", max.0);

    // Median calculation
    let median: f32 = match vec.len() % 2 {
        0 => {
            let mid = vec.len() / 2;
            ((vec[mid - 1] + vec[mid])) as f32 / 2.0
        }
        1 => {
            vec[vec.len() / 2] as f32
        }
        _ => 0.0
    };

    println!("Median is: {}", median);

}
