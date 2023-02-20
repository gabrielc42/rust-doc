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
    unsafe fn dangerous() {} // must call dangerous in unsafe block

    unsafe {
        dangerous()
    }
    
    // creating safe abstraction over unsafe code
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    // can't implement split_at_mut w/ only safe Rust below
    // borrow checker does not understand that we are borrowing different
    // parts of slice, only that we are borrowing from same slice twice
    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     assert!(mid <= len);
    //     (&mut values[..mid], &mut values[mid..])
    // }

    use std::slice;
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
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

    let address = 0x01234usize;
    let r = address as *mut i32;
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // likely crash when slice is used
    // don't own memory at this arbitrary location
    // no guarantee the slice this code creates contains valid i32 values
    // attempting to use values as if it is a valid slice results in undefined behaviour

    // extern functions to call external code
    // Foreign Function Interface
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // extern to create interface that allows other languages to call Rust fn
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // accessing or modifying a mutable static variable
    // global variables are called static variables, similar to constants
    // fixed address in memory, always accessing same data
    // constants can duplicate their data when used
    
    static HELLO_WORLD: &str = "Hello, world! 🌁";

    println!("name is: {}", HELLO_WORLD);

    // static can be mutable
    // any code that read or writes from COUNTER must be w/in unsafe block
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    // w/ mutable data that is globally accessible, it is difficult to
    // ensure there are no data races, which is this is considered unsafe

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // unsafe trait implementation
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    fn stuff() {}
}
