fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        dangerous();
    }
}

unsafe fn dangerous() {}

pub fn unsafe_fn() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    println!("v is {:?}", v);
    let r = &mut v[..];
    // let (a, b) = r.split_as_mut(3);
    let (a, b) = split_as_mut(r, 3);
    println!("a is {:?}, b is {:?}", a, b);
    raw_pointer();
    static_variable(3);
}
use std::slice;
fn split_as_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..])
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

static mut COUNTER: u32 = 0;
fn static_variable(inc: u32) {
    unsafe {
        println!("COUNTER: {}", COUNTER);
        println!("COUNTER ADD {}", inc);
        COUNTER += inc;
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {

}

unsafe impl Foo for i32 {
    
}
