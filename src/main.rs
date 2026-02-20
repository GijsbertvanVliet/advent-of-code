mod common;
mod day_one;
mod day_two;

use crate::day_one::get_password;
use crate::day_two::get_invalids_in_range;

fn main() {
    get_password();
    get_invalids_in_range();
}
