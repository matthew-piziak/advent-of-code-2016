#![feature(conservative_impl_trait)]
#![feature(try_from)]

mod day_01;
mod day_02;

pub fn day_01() {
    let terminate_on_revisited_location = false;
    let day_01_answer_part_one = day_01::blocks_away(include_str!("day_01_input"),
                                                     terminate_on_revisited_location);
    assert_eq!(day_01_answer_part_one, Ok(279));

    let terminate_on_revisited_location = true;
    let day_01_answer_part_two = day_01::blocks_away(include_str!("day_01_input"),
                                                     terminate_on_revisited_location);
    assert_eq!(day_01_answer_part_two, Ok(163));
}

pub fn day_02() {
    let day_02_answer = day_02::code(include_str!("day_02_input"));
    assert_eq!(day_02_answer, Ok("69642".into()));
}
