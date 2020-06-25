
use std::f64::consts::PI;

mod nation {
    pub mod government {
        pub fn govern() {
            println!("govern test function");
        }
    }

    mod congress {
        pub fn legislate() {
            println!("legislate func");
        }
    }

    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}
// use crate::nation::congress::legislate;

pub fn mod_test(){
    nation::government::govern();
    println!("PI:{}",PI);
}