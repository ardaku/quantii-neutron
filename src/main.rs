use quantii_neutron::{start_quantii_desktop, QuantiiService};

fn main() {
    // launch quantii services
    let services: Vec<QuantiiService> = vec![];

    services.iter().for_each(|f| {
        f.run_service();
    });

    // launch quantii desktop
    start_quantii_desktop();

    // create
    let s = ServiceResult {
        data: Some(0),
        status_code: 0,
        message: None,
    };

    println!("s.data = {}", s.data.unwrap());

    let b = Box::new(2);

    fn take_b_and_dont_return(b: Box<u64>) {

    }

    take_b_and_dont_return(b);
}

pub struct ServiceResult<T> {
    data: Option<T>,
    status_code: u64,
    message: Option<String>,
}

pub struct SomeStruct<'some_lifetime> {
    data: &'some_lifetime str,
}

pub fn return_something<'some_lifetime>() -> &'some_lifetime str {
    // create a string on the stack (2 bytes) and a ref to its starting addr and bookkeeping on its size
    // slices are just views of this reference from the address on the stack, even if it is in a different stack frame above (or below? no)
    "hi"
}

pub fn do_something() {
    {
        // move
        let x = return_something();

        println!("x = {}", x);
    }

    // x goes out of scope and so "hi" goes out of lifetime here
}
