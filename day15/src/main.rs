use std::fs;

#[derive(Debug)]
enum Field {
    ROBOT,
    WALL,
    EMPTY,
    BOX,
}

#[derive(Debug)]
enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Grid {
    fields: Vec<Vec<Field>>,
    robot_position: (usize, usize),
}

fn get_next_pos(current_pos: (usize, usize), movement: &Movement) -> (usize, usize) {
    let next_pos = match movement {
        Movement::DOWN => (current_pos.0 + 1, current_pos.1),
        Movement::UP => (current_pos.0 - 1, current_pos.1),
        Movement::RIGHT => (current_pos.0, current_pos.1 + 1),
        Movement::LEFT => (current_pos.0, current_pos.1 - 1),
    };
    return next_pos;
}

impl Grid {
    fn coordinate_sum(&self) -> usize {
        let mut total_score: usize = 0;
        for (i, row) in self.fields.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                if matches!(x, Field::BOX) {
                    total_score += 100 * i + j;
                }
            }
        }
        return total_score;
    }

    fn draw(&self) {
        for row in self.fields.iter() {
            for x in row.iter() {
                match x {
                    Field::BOX => print!("O"),
                    Field::WALL => print!("#"),
                    Field::EMPTY => print!("."),
                    Field::ROBOT => print!("@"),
                }
            }
            print!("\n")
        }
        print!("\n")
    }

    fn get_field(&self, position: &(usize, usize)) -> &Field {
        return &self.fields[position.0][position.1];
    }

    fn set_field(&mut self, position: &(usize, usize), field: Field) {
        self.fields[position.0][position.1] = field;
    }

    fn apply_movement(&mut self, movement: &Movement) {
        let current_pos = self.robot_position;
        let next_pos = get_next_pos(current_pos, movement);
        let next_field = self.get_field(&next_pos);
        match next_field {
            Field::WALL => {
                return;
            }
            Field::EMPTY => {
                self.set_field(&current_pos, Field::EMPTY);
                self.set_field(&next_pos, Field::ROBOT);
                self.robot_position = next_pos;
                return;
            }
            Field::BOX => {
                self._handle_box_case(current_pos, movement);
            }
            Field::ROBOT => unreachable!(),
        }
    }

    fn _handle_box_case(&mut self, current_pos: (usize, usize), movement: &Movement) {
        let mut positions_on_line: Vec<(usize, usize)> = Vec::new();
        let mut current_pos = current_pos;
        positions_on_line.push(current_pos);
        loop {
            let next_pos = get_next_pos(current_pos, movement);
            positions_on_line.push(next_pos);
            let next_field = self.get_field(&next_pos);
            if !matches!(next_field, Field::BOX) {
                break;
            } else {
                current_pos = next_pos;
            }
        }
        let final_pos = positions_on_line.last().unwrap();
        let final_field = self.get_field(final_pos);
        match final_field {
            Field::WALL => return,
            Field::EMPTY => {
                // need to change first, second and last element in line:
                // robot - box - ... - box - empty -> empty - robot - box - ... - box
                let first_pos = &positions_on_line[0];
                let second_pos = &positions_on_line[1];
                let final_pos = positions_on_line.last().unwrap();
                self.robot_position = *second_pos;
                self.set_field(first_pos, Field::EMPTY);
                self.set_field(second_pos, Field::ROBOT);
                self.set_field(final_pos, Field::BOX);
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn new_from_string(grid_string: &str) -> Self {
        let mut robot_position: (usize, usize) = (0, 0);
        let mut fields: Vec<Vec<Field>> = Vec::new();
        for (i, line) in grid_string.lines().enumerate() {
            let mut row: Vec<Field> = Vec::new();
            for (j, x) in line.chars().enumerate() {
                let field = match x {
                    '@' => Field::ROBOT,
                    '.' => Field::EMPTY,
                    '#' => Field::WALL,
                    'O' => Field::BOX,
                    _ => unreachable!(),
                };
                if matches!(field, Field::ROBOT) {
                    robot_position = (i, j);
                }
                row.push(field);
            }
            fields.push(row);
        }

        return Self {
            fields,
            robot_position,
        };
    }
}

fn movements_string_to_movement_vec(movement_str: &str) -> Vec<Movement> {
    let movement_string = movement_str.replace("\n", "");
    let mut movements: Vec<Movement> = Vec::new();
    for x in movement_string.chars() {
        let mv: Movement = match x {
            'v' => Movement::DOWN,
            '^' => Movement::UP,
            '<' => Movement::LEFT,
            '>' => Movement::RIGHT,
            _ => unreachable!(),
        };
        movements.push(mv);
    }
    return movements;
}

fn main() {
    let input: String = fs::read_to_string("src/input.txt").unwrap();
    let mut input_block_iter = input.split("\n\n");
    let grid_string = input_block_iter.next().unwrap();
    let movement_str = input_block_iter.next().unwrap();
    let movements: Vec<Movement> = movements_string_to_movement_vec(movement_str);
    let mut grid = Grid::new_from_string(grid_string);
    // grid.draw();
    for movement in movements.iter() {
        // print!("{:?}\n", movement);
        grid.apply_movement(movement);
        // grid.draw();
    }
    let coordinate_score = grid.coordinate_sum();
    // print!("{:?}", grid);
    // print!("{:?}", movements);
    print!("{:?}", coordinate_score);
}
