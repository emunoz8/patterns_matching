use std::slice;

pub fn run() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    //let address = 0x012345usize;
    //let r = address as *mut i32;

    //let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //this will print:
    //r1 is: 5
    //r2 is: 5
    //explanation:
    //we have two raw pointers pointing to the same data (which in rust should not work due to the
    //borrow checker); with unsafe keyword we are able to run unsafe code, a great use case is when
    //interfacing with C code.

    //Calling an Unsafe Function or Method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    //need to call the unsafe function within the unsafe block

    //Creating a Safe Abstraction over Unsafe code
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let mid = vector.len() / 2;
    let (left, right) = split_at_mut(&mut vector, mid);
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);
    // let r = &mut v[..];
    //
    // let (a, b) = r.split_at_mut(3);
    //
    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);
    //Created split_at_mut function and it returns two slices from one slice
}

fn split_at_mut(
    values: &mut [i32],
    mid: usize,
) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
