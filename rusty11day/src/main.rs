use std::cell::RefCell;
use std::rc::Rc;

mod assignment;
use assignment::task;

struct Flagger {
    is_true: RefCell<bool>,
}

struct Swagger {
    is_true: Rc<RefCell<bool>>,
}
fn main() {
    // created a new tuple ; on the stack
    let t = (12, "eggs");
    // created a new box in the heap, but b remains in the stack
    let b = Box::new(t);
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 10;
    let y = Box::new(x);

    assert_eq!(x, 10);
    assert_eq!(*y, 10);

    println!("{:?}", y);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    let flag = Flagger {
        is_true: RefCell::new(true),
    };

    // let refe = flag.is_true.borrow();
    // println!("{}", refe);

    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false;
    println!("{}", mut_ref);

    let swag = Swagger {
        is_true: Rc::new(RefCell::new(true)),
    };

    let reference = Rc::new(swag.is_true.clone());

    let mut mutable_ref = reference.borrow_mut();
    *mutable_ref = false;

    task()
}
