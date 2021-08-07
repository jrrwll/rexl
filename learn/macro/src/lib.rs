#![feature(log_syntax)]
#![feature(trace_macros)]
#![macro_use]

mod stringify;
mod trace;
mod vec;

macro_rules! four {
    () => {
        2 + 2
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, four!());
        assert_eq!(2 + 2, four![]);
        assert_eq!(2 + 2, four! {});
    }
}
