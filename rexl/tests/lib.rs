extern crate rexl;

#[cfg(test)]
mod cli;
#[cfg(test)]
mod interpolate;
#[cfg(test)]
mod sort;
#[cfg(test)]
mod time;
#[cfg(test)]
mod text;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
