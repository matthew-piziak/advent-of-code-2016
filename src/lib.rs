#![feature(conservative_impl_trait)]
#![feature(try_from)]

mod day_01;
mod day_02;

pub fn day_01() {
    let day_01_answer = day_01::blocks_away(include_str!("day_01_input"));
    assert_eq!(day_01_answer, Ok(279));
}
