use console_gfx::rendering::{ colour::Colour, renderer::RenderCommand };
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array {
    array:      Vec<u32>,
    changes:    Vec<usize>,
    pub i:      isize,
    pub j:      isize,

    pub lowest: isize, // selection sort's lowest index
    pub sorted: bool,  // odd-even sort
}

impl Array {
    
    pub fn new(width: u32) -> Array {
        Array { array: (1..width + 1).collect(), changes: Vec::new(), i: -1, j: -1, lowest: -1, sorted: false }
    }

    pub fn set_ij(self, i_: isize, j_: isize) -> Array {
        Array { array: self.array, changes: self.changes, i: i_, j: j_, lowest: self.lowest, sorted: self.sorted }
    }

    pub fn set_lowest(self, l: isize) -> Array {
        Array { array: self.array, changes: self.changes, i: self.i, j: self.j, lowest: l, sorted: self.sorted }
    }

    pub fn set_sorted(self, s: bool) -> Array {
        Array { array: self.array, changes: self.changes, i: self.i, j: self.j, lowest: self.lowest, sorted: s }
    }

    pub fn reset(self) -> Array {
        Array { array: self.array, changes: Vec::new(), i: -1, j: -1, lowest: -1, sorted: false }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn render(&mut self, height: usize) -> Vec<RenderCommand> {
        let mut commands = Vec::new();
        commands.push(RenderCommand::SetBackground(Colour::black()));

        if self.changes.len() == 0 {
            commands.push(RenderCommand::SetColour(Colour::white()));
            for x in 0..self.len() {
                commands.push(RenderCommand::DrawLine(x, height - (self.array[x] as f64 / self.len()as f64 * height as f64).ceil() as usize, x, height, '|'));
            }
        } else {
            let mut i = 0.0;
            for x in self.changes.iter() {
                if *x < self.len() {
                    let gb = match 255.0 * (i / 5.0) { x if x < 255.0 => x as u8, _ => 255 };
                    commands.push(RenderCommand::SetColour(Colour::rgb(255, gb as u8, gb as u8)));
                    commands.push(RenderCommand::DrawLine(*x, 0, *x, height - (self.array[*x] as f64 / self.len()as f64 * height as f64).ceil() as usize, ' '));
                    commands.push(RenderCommand::DrawLine(*x, height - (self.array[*x] as f64 / self.len()as f64 * height as f64).ceil() as usize, *x, height, '|'));
                }
                i += 1.0;
            }
            if self.changes.len() > 5 {
                self.changes.truncate(5);
            } else {
                self.changes.insert(0, self.len());
            }
        }

        return commands;
    }

    pub fn swap(self, i: usize, j: usize) -> Array {
        println!();
        let mut c = self.changes;
        let mut a = self.array;
        c.insert(0, i);
        c.insert(0, j);
        a.swap(i, j);
        Array { array: a, changes: c, i: self.i, j: self.j, lowest: self.lowest, sorted: self.sorted }
    }

    pub fn check(self, i: usize) -> Array {
        let mut s = self;
        s.changes.insert(0, i);
        s
    }

    pub fn clear_changes(self) -> Array {
        Array { array: self.array, changes: Vec::new(), i: self.i, j: self.j, lowest: self.lowest, sorted: self.sorted }
    }

}

impl ops::Index<usize> for Array {
    
    type Output = u32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.array[i]
    }

}
