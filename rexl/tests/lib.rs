extern crate rexl;

#[cfg(test)]
mod cli;
#[cfg(test)]
mod interpolate;
#[cfg(test)]
mod io;
#[cfg(test)]
mod sort;
#[cfg(test)]
mod text;
#[cfg(test)]
mod time;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
