mod common;
mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;

use crate::day_one::get_password;
use crate::day_two::get_invalids_in_range;
use crate::day_three::get_banks;
use crate::day_four::count_rolls_of_paper;
use crate::day_five::count_fresh_items;

fn main() {
    get_password();
    get_invalids_in_range();
    get_banks();
    count_rolls_of_paper();
    count_fresh_items();
}
