use std::fmt;
use wasm_bindgen::prelude::*;

impl fmt::Display for Universe{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        for line in self.cells.as_slice().chunks(self.width as usize){
            for &cell in line{
                let symbol = if cell == Cell::Dead{'◻'}else {'◼'};
                write!(f,"{}",symbol)?;
            }
            write!(f,"\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe{
    width:u32,
    height:u32,
    cells:Vec<Cell>
}

#[wasm_bindgen]
impl Universe{
    fn get_index(&self, row:u32, column:u32)->usize{
        (row *  self.width + column) as usize
    }
    fn live_neignbor_count(&self, row:u32, column:u32) ->u8{
        let mut count = 0;
        for r in [self.height - 1,0,1].iter().cloned(){
            for c in [ self.width -1,0,1].iter().cloned(){
                if r == 0 && c == 0 {
                    continue;
                }
                let neighbor_row = (row + r) % self.height;
                let neighbor_col = (column + c )% self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn tick(&mut self){
        let mut next = self.cells.clone();
        for row in 0..self.height{
            for col in 0..self.width{
                let idx = self.get_index(row, col);
                let cells = self.cells[idx];
                let live_neighbors = self.live_neignbor_count(row,col);
                let next_cell = match(cells,live_neighbors){
                    // 规则1.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // 规则2.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // 规则3.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // 规则4.
                    (Cell::Dead, 3) => Cell::Alive,
                    // 其他.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        };
        self.cells = next;
    }
    pub fn new() -> Universe{
        let width = 64;
        let height = 64;
        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            }else{
                Cell::Dead
            }
        }).collect();
        Universe {
            width,
            height,
            cells
        }
    }
    pub fn render(&self) -> String{
        self.to_string()
    }
}