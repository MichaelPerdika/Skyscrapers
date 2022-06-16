#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub n: usize,
    pub numbers: Vec<usize>,
}

impl Cell {
    pub fn new_cell(number: usize) -> Cell {
        Cell {
            n: number,
            numbers: (1..number + 1).collect(),
        }
    }

    #[cfg(test)]
    pub fn new_cell_fixed(number: usize, fixed_numbers: Vec<usize>) -> Cell {
        Cell {
            n: number,
            numbers: fixed_numbers,
        }
    }

    pub fn print_cell(&self) {
        if self.numbers.len() == 1 && self.n >= 3 {
            let adj_spaces = self.n / 2;
            let spaces_left = " ".repeat(adj_spaces - 1);
            let spaces_right;
            if self.n % 2 == 0 {
                spaces_right = " ".repeat(adj_spaces - 2);
            } else {
                spaces_right = spaces_left.clone();
            }
            print!("{}({}){}", spaces_left, self.numbers[0], spaces_right);
        } else {
            for i in 1..self.n + 1 {
                if self.numbers.contains(&i) {
                    print!("{}", i);
                } else {
                    print!(" ");
                }
            }
        }
    }

    pub fn replace_cell_with_number(&mut self, number: usize) {
        if number > 0 && number <= self.n {
            self.numbers = vec![number];
        }
    }

    pub fn replace_cell_with_vec(&mut self, numbers: &Vec<usize>) {
        if numbers.len() > 0 && numbers.len() <= self.n {
            self.numbers = numbers.clone();
        }
    }

    pub fn contains_numbers(&self, numbers: &Vec<usize>) -> bool {
        for num in numbers {
            if !self.numbers.contains(num) {
                return false;
            }
        }
        true
    }

    pub fn restore(&mut self) {
        self.numbers = (1..self.n + 1).collect();
    }

    pub fn remove_number(&mut self, number: usize) {
        if self.numbers.contains(&number) && self.numbers.len() > 1 {
            // self.numbers.remove(number);
            self.numbers.retain(|&x| x != number);
        }
    }
}
