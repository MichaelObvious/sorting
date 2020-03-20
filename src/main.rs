extern crate console_gfx;

mod algorithms;
mod array;

use console_gfx::rendering::{ colour::Colour, renderer::{ RenderCommand, Renderer } };
use std::{ thread, time };

fn main() {
    let width = 20;
    let height = 20;
    let ups = 2;
    let step = time::Duration::from_millis(1000 / ups);

    let mut arr = array::Array::new(width as u32);

    let mut r = Renderer::new((width, height));
    r.push_cmds(vec![
        RenderCommand::Clear(Colour::black()),
        RenderCommand::SetBackground(Colour::black()),
        RenderCommand::SetColour(Colour::white()),
        RenderCommand::DrawBorder('#'),
    ]);

    let mut last_time = time::Instant::now();
    loop {
        r.push_cmds(arr.render(height));
        arr = arr.swap(1, 2);
        r.update();

        thread::sleep(get_sleep_time(&last_time, &step));
        last_time = time::Instant::now();
    }

    r.push_cmd(RenderCommand::Reset);
    r.update();

    ()
}

fn get_sleep_time(last: &time::Instant, step: &time::Duration) -> time::Duration {
    let elapsed = last.elapsed();
    if *step < elapsed {
        return time::Duration::from_millis(0);
    } else {
        return *step - elapsed;
    }
}
