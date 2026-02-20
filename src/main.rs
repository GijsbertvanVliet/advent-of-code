mod common;
mod day_one;
mod day_two;
mod day_three;

use crate::day_one::get_password;
use crate::day_two::get_invalids_in_range;
use crate::day_three::get_banks;

fn main() {
    get_password();
    get_invalids_in_range();
    get_banks();
}
