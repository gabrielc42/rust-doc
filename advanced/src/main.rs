fn main() {
    // unsafe
    // dereference a raw pointer
    // call an unsafe function or method
    // access or modify a mutable static variable
    // implement an unsafe trait
    // access fields of union

    // raw pointers
    // are allowed to ignore borrowing rules by having
    // both immutable and mutable pointers or
    // multiple mutable pointers to same location
    // aren't guaranteed to point to valid memory
    // are allowed to be null
    // don't implement any automatic cleanup

    // immutable and mutable raw pointer from references
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // raw pointer to arbitrary location in memory
    // usually no good reason to write code like this
    // compiler might optimize the code so no memory access,
    // or program might error w/ a segmentation fault
    let address = 0x012345usize;
    let r address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // with raw pointers, we can create a mutable pointer and an immutable pointer to the
    // same location and change data through the mutable pointer
    // potentially creating a data race

    // why raw pointers ever?
    // building up safe abstractions that borrow checker doesn't understand
    // or, calling an unsafe function or method:
    
}
