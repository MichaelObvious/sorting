use console_gfx::rendering::{ colour::Colour, renderer::RenderCommand };
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array {
    array:   Vec<u32>,
    changes: Vec<usize>,
    pub i:   isize,
    pub j:   isize,
}

impl Array {
    
    pub fn new(width: u32) -> Array {
        Array { array: (1..width + 1).collect(), changes: Vec::new(), i: -1, j: -1 }
    }

    pub fn new_ij(arr: Array, i_: isize, j_: isize) -> Array {
        Array { array: arr.array, changes: arr.changes, i: i_, j: j_ }
    }

    pub fn reset(self) -> Array {
        Array { array: self.array, changes: Vec::new(), i: -1, j: -1 }
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
            for x in &self.changes {
                let gb = match 255.0 * (i / 3.0) { x if x < 255.0 => x as u8, _ => 255 };
                commands.push(RenderCommand::SetColour(Colour::rgb(255, gb as u8, gb as u8)));
                commands.push(RenderCommand::DrawLine(*x, 0, *x, height - (self.array[*x] as f64 / self.len()as f64 * height as f64).ceil() as usize, ' '));
                commands.push(RenderCommand::DrawLine(*x, height - (self.array[*x] as f64 / self.len()as f64 * height as f64).ceil() as usize, *x, height, '|'));
                i += 1.0;
            }

            self.changes.truncate(5);
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
        Array { array: a, changes: c, i: self.i, j: self.j }
    }

    pub fn clear_changes(self) -> Array {
        Array { array: self.array, changes: Vec::new(), i: self.i, j: self.j }
    }

}

impl ops::Index<usize> for Array {
    
    type Output = u32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.array[i]
    }

}
