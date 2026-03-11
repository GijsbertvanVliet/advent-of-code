use std::fmt::{Debug, Display, Formatter};
use crate::common::read_file_multiple_lines;

pub fn count_fresh_items() {
    let (ranges, _) = get_ranges_and_ingredients();
    let mut mutable_ranges = ranges;
    // let mut mutable_ranges = vec!(Range{start: 10, end: 13}, Range{start: 12, end: 18}, Range{start:20, end: 30});
    create_disjoint_ranges(&mut mutable_ranges);
    let mut total_size = 0;
    mutable_ranges.iter().for_each(|range| total_size += range.end - range.start + 1);
    println!("day five answer {}", total_size)
}

fn create_disjoint_ranges(ranges: &mut Vec<Range>) {
    let mut intersecting_range_found: bool = true;
    while intersecting_range_found {
        match find_intersecting_ranges(ranges) {
            None => intersecting_range_found = false,
            Some((index1, index2)) => {
                let range1 = ranges.get(index1).unwrap();
                let range2 = ranges.get(index2).unwrap();
                // println!("merging {} with {}", range1, range2);
                ranges.push(range1.merge(range2));
                ranges.remove(index2);
                ranges.remove(index1);
            }
        }
    }
}

fn find_intersecting_ranges(ranges: &Vec<Range>) -> Option<(usize, usize)> {
    let all_tuples = get_all_tuples(ranges);
    for tuple in all_tuples {
        let range1: &Range = ranges.get(tuple.0).unwrap();
        let range2: &Range = ranges.get(tuple.1).unwrap();
        if !range1.is_disjoint_from(range2) {
            return Some(tuple);
        }
    }
    return None
}

pub fn get_all_tuples<A>(vector: &Vec<A>) -> Vec<(usize, usize)> {
    let mut tuples: Vec<(usize, usize)> = vec!();
    for i in 0..vector.len() - 1 {
        for j in (i+1)..vector.len() {
            tuples.push((i, j));
        }
    }
    tuples
}

fn get_ranges_and_ingredients() -> (Vec<Range>, Vec<Ingredient>) {
    let input = read_file_multiple_lines("resources/day5Input.txt").unwrap();
    let mut ranges: Vec<Range> = vec![];
    let mut ingredients: Vec<Ingredient> = vec![];
    let mut empty_line_has_passed: bool = false;
    for line in input {
        if line.is_empty() {
            empty_line_has_passed = true
        } else if empty_line_has_passed {
            ingredients.push(Ingredient::from_line(&line));
        } else {
            ranges.push(Range::from_line(&line));
        }
    }
    (ranges, ingredients)
}

struct Range {
    start: u64,
    end: u64,
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Range")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl Range {
    fn from_line(line: &String) -> Range {
        let mut a = line.split('-');
        let part_one = a.next().unwrap().parse::<u64>().unwrap();
        let part_two = a.next().unwrap().parse::<u64>().unwrap();
        Range {
            start: part_one,
            end: part_two,
        }
    }

    fn contains(&self, ingredient: &Ingredient) -> bool {
        self.start <= ingredient.id && self.end >= ingredient.id
    }

    fn is_disjoint_from(&self, other: &Range) -> bool {
        other.end < self.start || self.end < other.start
    }

    fn merge(&self, other: &Range) -> Range {
        Range{
            start: self.start.min(other.start),
            end: self.end.max(other.end)
        }
    }
}

struct Ingredient {
    id: u64,
}

impl Ingredient {
    fn from_line(line: &String) -> Ingredient {
        Ingredient {
            id: line.parse::<u64>().unwrap(),
        }
    }

    fn is_in_range(&self, ranges: &Vec<Range>) -> bool {
        ranges.iter().find(|range| range.contains(self)).is_some()
    }
}
