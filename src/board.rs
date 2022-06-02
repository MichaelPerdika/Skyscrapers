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
            if let Some(rule) = c.to_digit(10) {
                if rule as usize > self.number {
                    continue;
                }
                match which_rule {
                    WhichRule::Up => self.rules_up[index] = rule as usize,
                    WhichRule::Down => self.rules_down[index] = rule as usize,
                    WhichRule::Left => self.rules_left[index] = rule as usize,
                    WhichRule::Right => self.rules_right[index] = rule as usize,
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

    pub fn remove_number_from_cell(&mut self, args: &str) {
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
                                        .remove_number(number_i as usize);
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
        self.check_unique_number_left_all_cells();
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

    fn check_unique_number_left_all_cells(&mut self) {
        // Do the cols.
        for col in 0..self.number {
            let mut cells: Vec<Cell> = vec![];
            for row in 0..self.number {
                cells.push(self.cells[row][col].clone());
            }
            check_unique_number_left(&mut cells);
            for row in 0..self.number {
                if self.cells[row][col] != cells[row] {
                    self.cells[row][col] = cells[row].clone();
                }
            }
        }

        // // Do the rows.
        for row in 0..self.number {
            let mut cells: Vec<Cell> = vec![];
            for col in 0..self.number {
                cells.push(self.cells[row][col].clone());
            }
            check_unique_number_left(&mut cells);
            for col in 0..self.number {
                if self.cells[row][col] != cells[col] {
                    self.cells[row][col] = cells[col].clone();
                }
            }
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
    } else if rule > 2 && rule < max_number {
        let mut numbers_to_erase: Vec<usize> =
            (max_number - rule + 2..max_number + 1).rev().collect();
        let mut index = 0;
        while !numbers_to_erase.is_empty() {
            for m in &numbers_to_erase {
                cells[index].remove_number(*m);
            }
            numbers_to_erase.pop();
            index += 1
        }
    } else if rule == max_number {
        for i in 0..max_number {
            cells[i].replace_cell_with_number(i + 1);
        }
    }
}

// This doesn't work well ! Check it more.
fn check_unique_number_left(cells: &mut Vec<Cell>) {
    let mut number_count: Vec<usize> = vec![0; cells[0].n];

    for cell in cells.clone() {
        for num in cell.numbers {
            number_count[num - 1] += 1;
        }
    }
    let mut index = 0;
    for n_count in number_count {
        for cell in &mut *cells {
            if n_count == 1 {
                if cell.numbers.len() > 1 {
                    if cell.numbers.contains(&(&index + 1)) {
                        cell.replace_cell_with_number(index + 1)
                    }
                }
            }
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::board::check_cell_line_by_rule;
    use crate::Cell;
    #[test]
    fn test_check_cell_line_by_rule() {
        let n: usize = 5;
        // rule 1
        let mut rule_one = vec![Cell::new_cell(n); n];
        check_cell_line_by_rule(1, &mut rule_one);
        assert_eq!(
            rule_one,
            vec![
                Cell::new_cell_fixed(n, vec![5]),
                Cell::new_cell(n),
                Cell::new_cell(n),
                Cell::new_cell(n),
                Cell::new_cell(n),
            ]
        );

        // rule 2
        let mut rule_two = vec![Cell::new_cell(n); n];
        check_cell_line_by_rule(2, &mut rule_two);
        assert_eq!(
            rule_two,
            vec![
                Cell::new_cell_fixed(n, vec![1, 2, 3, 4]),
                Cell::new_cell_fixed(n, vec![1, 2, 3, 5]),
                Cell::new_cell(n),
                Cell::new_cell(n),
                Cell::new_cell(n),
            ]
        );

        // 2 < rule < max_number
        let mut rule_mid_n = vec![Cell::new_cell(n); n];
        check_cell_line_by_rule(4, &mut rule_mid_n);
        assert_eq!(
            rule_mid_n,
            vec![
                Cell::new_cell_fixed(n, vec![1, 2,]),
                Cell::new_cell_fixed(n, vec![1, 2, 3,]),
                Cell::new_cell_fixed(n, vec![1, 2, 3, 4]),
                Cell::new_cell(n),
                Cell::new_cell(n),
            ]
        );

        // rule max_number
        let mut rule_max_n = vec![Cell::new_cell(n); n];
        check_cell_line_by_rule(n, &mut rule_max_n);
        assert_eq!(
            rule_max_n,
            vec![
                Cell::new_cell_fixed(n, vec![1]),
                Cell::new_cell_fixed(n, vec![2]),
                Cell::new_cell_fixed(n, vec![3]),
                Cell::new_cell_fixed(n, vec![4]),
                Cell::new_cell_fixed(n, vec![5]),
            ]
        );
    }

    #[test]
    fn test_check_unique_number_left() {
        let n: usize = 4;
        // transform |123, 12, 23, 1234| --> |123, 12, 23, (4) |
        let mut keep_only_four = vec![
            Cell::new_cell_fixed(n, vec![1, 2, 3]),
            Cell::new_cell_fixed(n, vec![1, 2]),
            Cell::new_cell_fixed(n, vec![2, 3]),
            Cell::new_cell(n),
        ];
        check_unique_number_left(&mut keep_only_four);
        assert_eq!(
            keep_only_four,
            vec![
                Cell::new_cell_fixed(n, vec![1, 2, 3]),
                Cell::new_cell_fixed(n, vec![1, 2]),
                Cell::new_cell_fixed(n, vec![2, 3]),
                Cell::new_cell_fixed(n, vec![4]),
            ]
        );

        let n: usize = 3;
        // transform |123, 12, 23, 1234| --> |123, 12, 23, (4) |
        let mut keep_only_three_do_nothing = vec![
            Cell::new_cell_fixed(n, vec![2, 3]),
            Cell::new_cell_fixed(n, vec![1]),
            Cell::new_cell_fixed(n, vec![2, 3]),
        ];
        check_unique_number_left(&mut keep_only_three_do_nothing);
        assert_eq!(
            keep_only_three_do_nothing,
            vec![
                Cell::new_cell_fixed(n, vec![2, 3]),
                Cell::new_cell_fixed(n, vec![1]),
                Cell::new_cell_fixed(n, vec![2, 3]),
            ]
        );
    }
}
