#![feature(intrinsics)]

mod rusti {
    extern "rust-intrinsic" {
        pub fn cttz_nonzero<T>(x: T) -> T;
    }
}

pub fn main() {
    unsafe {
        use rusti::*;

        cttz_nonzero(0u8); //~ ERROR constant evaluation error [E0080]
        //~^ NOTE cttz_nonzero called on 0
    }
}
