fn unsafe_1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {
    println!("do something dangerous");
}

fn unsafe_2() {
    unsafe {
        dangerous();
    }
}

fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // ERROR: cannot borrow `*values` as mutable more than once at a time
    //(&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_3() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut  v[..];

    //let (a, b) = r.split_at_mut(3);
    let (a, b) = my_split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn unsafe_4() {
    let address = 0x01234usize;

    let r = address as *mut i32;
    let values: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(r, 10000) };

    // CRASH: Segmentation fault
    // println!("values: {:?}", values);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn unsafe_5() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}


fn main() {
    unsafe_1();
    unsafe_2();
    unsafe_3();
    unsafe_4();
    unsafe_5();
}
