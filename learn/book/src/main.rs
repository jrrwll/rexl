mod cp01_guessing_game;
#[cfg(test)]
mod cp02_basic;
#[cfg(test)]
mod cp03_struct;
#[cfg(test)]
mod cp04_match;
#[cfg(test)]
mod cp05_vec;
#[cfg(test)]
mod cp06_test;
#[cfg(test)]
mod cp07_io;
#[cfg(test)]
mod cp08_args;
#[cfg(test)]
mod cp09_trait;
#[cfg(test)]
mod cp10_closure;
#[cfg(test)]
mod cp11_iter;
#[cfg(test)]
mod cp12_oop;
#[cfg(test)]
mod cp13_unsafe;
#[cfg(test)]
mod cp14_lifetime;
#[cfg(test)]
mod cp15_rc;
#[cfg(test)]
mod cp16_thread;
mod cp17_socket;

fn main() {
    cp01_guessing_game::guessing_game();
    cp17_socket::listen();
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
