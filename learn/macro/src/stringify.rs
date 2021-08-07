macro_rules! capture_expr_then_stringify {
    ($e:expr) => {
        stringify!($e)
    };
}

#[test]
fn test() {
    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}
macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "got an addition"
    };
    (($i:ident)) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

#[test]
fn test2() {
    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );
    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );
}

macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!(#[$m])};
}
macro_rules! what_is {
    (#[no_mangle]) => {"no_mangle attribute"};
    (#[inline]) => {"inline attribute"};
    ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
}

#[test]
fn test3() {
    println!(
        "{}\n{}\n{}\n{}",
        what_is!(#[no_mangle]),
        what_is!(#[inline]),
        capture_then_what_is!(#[no_mangle]),
        capture_then_what_is!(#[inline]),
    );
}
