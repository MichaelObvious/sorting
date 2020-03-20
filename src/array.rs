use console_gfx::rendering::{ colour::Colour, renderer::RenderCommand };
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array {
    array: Vec<u32>,
    i:     usize,
    j:     usize,
}

impl Array {
    
    pub fn new(width: u32) -> Array {
        Array { array: (1..width + 1).collect(), i: 0, j: 0 }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn render(&self, height: usize) -> Vec<RenderCommand> {
        let mut commands = Vec::new();
        commands.append(&mut vec![
            RenderCommand::SetBackground(Colour::black()),
            RenderCommand::SetColour(Colour::white()),
        ]);
        for x in 0..self.len() {
            commands.push(RenderCommand::DrawLine(x, height - (self.array[x] as f64 / self.len()as f64 * height as f64).ceil() as usize, x, height, '|'));
        }

        return commands;
    }

}

impl ops::Index<usize> for Array {
    
    type Output = u32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.array[i]
    }

}
