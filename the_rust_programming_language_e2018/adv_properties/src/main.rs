type Kilometers = i32;
type Test = Kilometers;

fn main() {
    let mut num = 5;
    let test = vec![1, 2, 3];

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 10;
        println!("{}", *r1);
        println!("{}", *r2);
        println!("{}", abs(-3));
    }
    call_from_c();
    println!("{}", HELLO_WORLD);

    let x = 5;
    let y: Test = 5;
    assert_eq!(x, y);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just test");
}

static HELLO_WORLD: &'static str = "Hello, world";
