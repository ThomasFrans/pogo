pub mod mines {
    use rand::{thread_rng, Rng};
    use std::fmt;
    use std::fmt::{Display, Formatter};
    use std::io::{stdout, Write};
    use std::ptr::write;

    const MINE_MASK: u8 = 0b00000100; // has mine
    const COVER_MASK: u8 = 0b00000010; // is covered
    const FLAG_MASK: u8 = 0b00000001; // has flag

    pub struct Board {
        cells: Vec<Vec<Cell>>,
        cells_amount: i32,
        width: i32,
        height: i32,
    }

    impl Display for Board {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, " 01234567\n");
            let mut m = 0;
            for i in &self.cells {
                write!(f, "{}", m);
                m += 1;
                for j in i {
                    if !j.is_covered() && !j.has_mine() && j.neighbor_mines == 0 {
                        write!(f, " ")?
                    } else if !j.is_covered() && j.neighbor_mines > 0 {
                        write!(f, "{}", j.neighbor_mines)?
                    } else if !j.is_covered() && j.has_mine() {
                        write!(f, "O")?
                    } else {
                        write!(f, "#")?
                    }
                }
                write!(f, "\n")?
            }
            Ok(())
        }
    }

    impl Board {
        /// Generate an empty, fully covered board and generate a set amount of mines on it randomly
        pub fn new(
            width: i32,
            height: i32,
            mines: i32,
        ) -> Result<Self, &'static str> {
            if mines > width * height {
                return Err("Too many mines");
            }
            let mut rng = thread_rng();
            let mut board = Board {
                cells: vec![
                    vec![Cell::new(); height.try_into().unwrap()];
                    width.try_into().unwrap()
                ],
                cells_amount: width * height,
                width: width,
                height: height,
            };
            for i in 0..mines {
                let mut cell = board.serial_to_coord(rng.gen_range(0..board.cells_amount));
                while board.cells[cell.1 as usize][cell.0 as usize].has_mine() {
                    cell = board.serial_to_coord(rng.gen_range(0..board.cells_amount))
                }

                board.cells[cell.1 as usize][cell.0 as usize].place_mine();
                for i in board.get_all_neighbors(cell.0, cell.1) {
                    board.cells[i.1 as usize][i.0 as usize].increment();
                }
            }
            Ok(board)
        }

        pub fn get_state(&self) -> &Vec<Vec<Cell>> {
            &self.cells
        }

        pub fn uncover(&mut self, x: i32, y: i32) {
            let cell = &mut self.cells[y as usize][x as usize];
            cell.uncover();
            /*
               player has lost the game
            */
            if cell.has_mine() {
                for i in &mut self.cells {
                    for j in i {
                        j.uncover();
                    }
                }
            } else {
                for i in self.get_neighbors(x, y) {
                    let next_cell = &mut self.cells[i.1 as usize][i.0 as usize];
                    if self.cells[y as usize][x as usize].neighbor_mines == 0 {
                        self.uncover(i.0, i.1);
                    }
                }
            }
        }

        fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
            let mut neighbors: Vec<(i32, i32)> = Vec::new();
            if x - 1 >= 0 && self.cells[y as usize][(x - 1) as usize].is_covered() {
                neighbors.push((x - 1, y));
            }
            if x + 1 < self.width && self.cells[y as usize][(x + 1) as usize].is_covered() {
                neighbors.push((x + 1, y));
            }
            if y - 1 >= 0 && self.cells[(y - 1) as usize][(x) as usize].is_covered() {
                neighbors.push((x, y - 1));
            }
            if y + 1 < self.height && self.cells[(y + 1) as usize][(x) as usize].is_covered() {
                neighbors.push((x, y + 1));
            }
            neighbors
        }

        fn get_all_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
            let mut neighbors: Vec<(i32, i32)> = Vec::new();
            let neighbor_coords = [
                (-1, -1), (0, -1), (1, -1),
                (-1, 0) ,          (1, 0) ,
                (-1, 1) , (0, 1) , (1, 1)
            ];
            for neighbor in neighbor_coords {
                if x + neighbor.0 >= 0
                    && x + neighbor.0 < self.width
                    && y + neighbor.1 >= 0
                    && y + neighbor.1 < self.height {
                    neighbors.push((neighbor.0 + x, neighbor.1 + y));
                }
            }
            neighbors
        }

        fn serial_to_coord(&self, serial: i32) -> (i32, i32) {
            (serial % self.width, serial / self.width)
        }
    }

    #[derive(Clone)]
    pub struct Cell {
        state: u8,
        neighbor_mines: u8,
    }

    impl Cell {
        /// Generate a covered cell without mine or flag
        #[inline]
        fn new() -> Self {
            Cell {
                state: 0b00000010,
                neighbor_mines: 0
            }
        }

        #[inline]
        fn uncover(&mut self) {
            self.state &= !COVER_MASK;
        }

        #[inline]
        fn cover(&mut self) {
            self.state |= COVER_MASK;
        }

        #[inline]
        fn place_mine(&mut self) {
            self.state |= MINE_MASK;
        }

        #[inline]
        fn remove_mine(&mut self) {
            self.state &= !MINE_MASK;
        }

        #[inline]
        fn place_flag(&mut self) {
            self.state |= FLAG_MASK;
        }

        #[inline]
        fn remove_flag(&mut self) {
            self.state &= FLAG_MASK;
        }

        #[inline]
        pub fn has_flag(&self) -> bool {
            (self.state & FLAG_MASK) == FLAG_MASK
        }

        #[inline]
        pub fn has_mine(&self) -> bool {
            (self.state & MINE_MASK) == MINE_MASK
        }

        #[inline]
        pub fn is_covered(&self) -> bool {
            (self.state & COVER_MASK) == COVER_MASK
        }

        #[inline]
        pub fn get_neighbor_amount(&self) -> i32 {
            self.neighbor_mines as i32
        }

        #[inline]
        fn increment(&mut self) {
            self.neighbor_mines += 1;
        }
    }
}
