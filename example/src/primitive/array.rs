pub fn it_works() {
    let array = [1; 5];
    println!("array is {:?}", array);
    analyze_slice(&array[..]);
    analyze_slice(&array[1..]);
    let array = ["aa".to_string(), "bb".to_string()];
    for i in array.iter() {
        println!("{}", i);
    }
    // impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
    let array = &[1, 5];
    for i in array.into_iter() {
        println!("{}", i)
    }

    let array: &[i32; 3] = &[666; 3];
    for i in array.into_iter() {
        println!("{}", i)
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("slice is {:?}", slice);
    println!("first elememt of the slice is {}", slice[0]);
    println!("the slice has {} elements", slice.len());
    println!("array occupies {} bytes", std::mem::size_of_val(&slice));
}
