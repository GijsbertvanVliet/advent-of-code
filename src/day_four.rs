use crate::common::read_file_multiple_lines;

pub fn count_rolls_of_paper() {
    let mut grid: Grid = Grid::from_input_file();
    let mut count = 0;
    let mut rolled_picked_up_one_go = 1;
    while rolled_picked_up_one_go > 0 {
        rolled_picked_up_one_go = grid.pickup_all_available_rolls();
        count = count + rolled_picked_up_one_go;
    }
    println!("day four answer = {}", count);
}

struct Grid {
    elements: [[char; 140]; 140],
}

impl Grid {
    fn from_input_file() -> Grid {
        let file_input: Vec<String> = read_file_multiple_lines("resources/day4Input.txt").unwrap();

        let elements_vector: Vec<[char; 140]> =
            file_input.iter().map(Self::string_to_array).collect();
        let elements_array: [[char; 140]; 140] = elements_vector.try_into().unwrap();

        Grid {
            elements: elements_array,
        }
    }

    fn string_to_array(input: &String) -> [char; 140] {
        let vector: Vec<char> = input.chars().collect();
        vector.try_into().unwrap()
    }

    // fn print(&self) {
    //     for row in self.elements {
    //         for char in row {
    //             print!("{}", char);
    //         }
    //         println!();
    //     }
    // }

    // Since we want point coordinates to be unsigned, we actually want to start counting from one for once
    fn is_paper_roll(&self, point: &Point) -> bool {
        if point.x == 0 || point.y == 0 {
            false
        } else {
            let row: Option<&[char; 140]> = self.elements.get(point.x - 1);
            let char: Option<&char> = row.map(|r| r.get(point.y - 1)).flatten();
            char == Some(&'@')
        }
    }

    fn count_adjacent_rolls(&self, point: &Point) -> usize {
        let surrounding_points = [
            Point {
                x: point.x - 1,
                y: point.y - 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x - 1,
                y: point.y + 1,
            },
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x + 1,
                y: point.y - 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x + 1,
                y: point.y + 1,
            },
        ];
        surrounding_points
            .iter()
            .filter(|point| self.is_paper_roll(point))
            .count()
    }

    fn is_roll_which_can_be_picked_up(&self, point: &Point) -> bool {
        self.is_paper_roll(point) && self.count_adjacent_rolls(point) < 4
    }

    fn pick_up_roll(&mut self, point: &Point) -> bool {
        if self.is_roll_which_can_be_picked_up(point) {
            self.elements[point.x - 1][point.y - 1] = '.';
            true
        } else {
            false
        }
    }

    fn pickup_all_available_rolls(&mut self) -> usize {
        let mut count: usize = 0;
        for x in 1..=140usize {
            for y in 1..=140usize {
                if self.pick_up_roll(&Point { x, y }) {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Point {
    x: usize,
    y: usize,
}
