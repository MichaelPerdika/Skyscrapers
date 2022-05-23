use crate::Cell;
pub enum WhichRule {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Board {
    number: usize,
    rules_up: Vec<usize>,
    rules_down: Vec<usize>,
    rules_left: Vec<usize>,
    rules_right: Vec<usize>,
    cells: Vec<Vec<Cell>>, // first is horizontal second is vertical
}

impl Board {
    pub fn new_board(number: usize) -> Board {
        Board {
            number,
            cells: vec![vec![Cell::new_cell(number); number]; number],
            rules_up: vec![0; number],
            rules_down: vec![0; number],
            rules_left: vec![0; number],
            rules_right: vec![0; number],
        }
    }

    pub fn print_board(&self) {
        let adj_spaces = self.number / 2;
        let spaces_left = " ".repeat(adj_spaces);
        let spaces_right;
        if self.number % 2 == 0 {
            spaces_right = " ".repeat(adj_spaces - 1);
        } else {
            spaces_right = spaces_left.clone();
        }
        // print rules top
        print!("     "); // 4 spaces + one more space for first |
        for ru in &self.rules_up {
            print!("{}", spaces_left);
            if ru > &0 {
                print!("{}", ru);
            } else {
                print!(" ");
            }
            print!("{}", spaces_right);
            print!(" "); // |
        }
        println!("");
        for row in 0..self.number {
            print!("  "); // 2 spaces
            if self.rules_left[row] > 0 {
                print!("{} ", self.rules_left[row]); // Number + one space
            } else {
                print!("  "); // no number + one space
            }
            for col in 0..self.number {
                print!("|");
                self.cells[row][col].print_cell();
            }
            print!("|");
            if self.rules_right[row] > 0 {
                println!(" {}", self.rules_right[row]);
            } else {
                println!("  ");
            }
        }

        // print rules down
        print!("     "); // One more space for first |
        for rd in &self.rules_down {
            print!("{}", spaces_left);
            if rd > &0 {
                print!("{}", rd);
            } else {
                print!(" ");
            }
            print!("{}", spaces_right);
            print!(" "); // |
        }
        println!("");
    }

    pub fn update_rule_x(&mut self, args: &str, which_rule: WhichRule) {
        let mut index = 0;
        for c in args.chars() {
            if index >= self.number {
                break;
            }
            if let Some(mjk) = c.to_digit(10) {
                if mjk as usize > self.number {
                    continue;
                }
                println!("Radix = {}", mjk);
                match which_rule {
                    WhichRule::Up => self.rules_up[index] = mjk as usize,
                    WhichRule::Down => self.rules_down[index] = mjk as usize,
                    WhichRule::Left => self.rules_left[index] = mjk as usize,
                    WhichRule::Right => self.rules_right[index] = mjk as usize,
                }
            }
            index += 1;
        }
    }

    pub fn restore_cell(&mut self, args: &str) {
        if args.len() >= 2 {
            if let Some(row_c) = args.chars().nth(0) {
                if let Some(row_n) = row_c.to_digit(10) {
                    if let Some(col_c) = args.chars().nth(1) {
                        if let Some(col_n) = col_c.to_digit(10) {
                            if row_n <= self.number as u32 && col_n <= self.number as u32 {
                                self.cells[row_n as usize - 1][col_n as usize - 1].restore();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_ok_cell(&mut self, args: &str) {
        // user should give 1-index based instead of 0
        if let Some(hor_line_c) = args.chars().nth(0) {
            if let Some(ver_line_c) = args.chars().nth(1) {
                if let Some(number_c) = args.chars().nth(2) {
                    if let Some(hor_line_i) = hor_line_c.to_digit(10) {
                        if let Some(ver_line_i) = ver_line_c.to_digit(10) {
                            if let Some(number_i) = number_c.to_digit(10) {
                                if hor_line_i > 0
                                    && hor_line_i < 1 + self.number as u32
                                    && ver_line_i > 0
                                    && ver_line_i < 1 + self.number as u32
                                {
                                    self.cells[hor_line_i as usize - 1][ver_line_i as usize - 1]
                                        .replace_cell_with_number(number_i as usize);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_all(&mut self) {
        self.check_integrity(); // TODO:
        self.check_line(WhichRule::Up);
        self.check_line(WhichRule::Down);
        self.check_line(WhichRule::Left);
        self.check_line(WhichRule::Right);
        self.check_compl_cell_excl_hor_and_ver();
    }

    fn check_integrity(&self) {
        // let mut i = 0;
        // for hor_cells in &self.cells.clone() {
        //     let mut j = 0;
        //     for cell in hor_cells {
        //         if cell.numbers.len() == 1 {
        //             let number_to_check = cell.numbers[0];
        //             for inner_range_num in 0..self.number {
        //                 let cell_row = &self.cells[inner_range_num][j];
        //                 let cell_col = &self.cells[i][inner_range_num];
        //                 // check if cell_row and cell_col don't have the number_to_check
        //             }
        //         }
        //         j += 1;
        //     }
        //     i += 1;
        // }
    }

    fn check_compl_cell_excl_hor_and_ver(&mut self) {
        let mut i = 0;
        for hor_cells in &self.cells.clone() {
            let mut j = 0;
            for cell in hor_cells {
                if cell.numbers.len() == 1 {
                    let number_to_erase = cell.numbers[0];
                    for inner_range_num in 0..self.number {
                        self.cells[inner_range_num][j].remove_number(number_to_erase);
                        self.cells[i][inner_range_num].remove_number(number_to_erase);
                    }
                }
                j += 1;
            }
            i += 1;
        }
    }
    fn check_line(&mut self, which_rule: WhichRule) {
        match which_rule {
            WhichRule::Up => {
                for col in 0..self.number {
                    let rule = self.rules_up[col];
                    if rule > 0 {
                        let mut cells: Vec<Cell> = vec![];
                        for row in 0..self.number {
                            cells.push(self.cells[row][col].clone());
                        }
                        check_cell_line_by_rule(rule, &mut cells);
                        for row in 0..self.number {
                            if self.cells[row][col] != cells[row] {
                                self.cells[row][col] = cells[row].clone();
                            }
                        }
                    }
                }
            }
            WhichRule::Down => {
                for col in 0..self.number {
                    let rule = self.rules_down[col];
                    if rule > 0 {
                        let mut cells: Vec<Cell> = vec![];
                        for row in (0..self.number).rev() {
                            cells.push(self.cells[row][col].clone());
                        }
                        check_cell_line_by_rule(rule, &mut cells);
                        for row in (0..self.number).rev() {
                            let reverse_row = self.number - row - 1;
                            if self.cells[row][col] != cells[reverse_row] {
                                self.cells[row][col] = cells[reverse_row].clone();
                            }
                        }
                    }
                }
            }
            WhichRule::Left => {
                for row in 0..self.number {
                    let rule = self.rules_left[row];
                    if rule > 0 {
                        let mut cells: Vec<Cell> = vec![];
                        for col in 0..self.number {
                            cells.push(self.cells[row][col].clone());
                        }
                        check_cell_line_by_rule(rule, &mut cells);
                        for col in 0..self.number {
                            if self.cells[row][col] != cells[col] {
                                self.cells[row][col] = cells[col].clone();
                            }
                        }
                    }
                }
            }
            WhichRule::Right => {
                for row in 0..self.number {
                    let rule = self.rules_right[row];
                    if rule > 0 {
                        let mut cells: Vec<Cell> = vec![];
                        for col in (0..self.number).rev() {
                            cells.push(self.cells[row][col].clone());
                        }
                        check_cell_line_by_rule(rule, &mut cells);
                        for col in (0..self.number).rev() {
                            let reverse_col = self.number - col - 1;
                            if self.cells[row][col] != cells[reverse_col] {
                                self.cells[row][col] = cells[reverse_col].clone();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn check_cell_line_by_rule(rule: usize, cells: &mut Vec<Cell>) {
    let max_number = cells[0].n;
    if max_number <= 2 {
        return;
    }

    if rule == 1 {
        cells[0].replace_cell_with_number(max_number);
    } else if rule == 2 {
        cells[0].remove_number(max_number);
        cells[1].remove_number(max_number - 1);
    } else if rule == max_number {
        for i in 0..max_number {
            cells[i].replace_cell_with_number(i + 1);
        }
    }
}