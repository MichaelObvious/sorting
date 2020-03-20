extern crate console_gfx;
extern crate rand;

mod algorithms;
mod array;

use console_gfx::rendering::{ colour::Colour, renderer::{ RenderCommand, Renderer } };
use std::{ thread, time };

fn main() {
    let width = 80;
    let height = 40;
    let ups = 10;
    let step = time::Duration::from_millis(1000 / ups);

    let mut r = Renderer::new((width, height));
    r.push_cmds(vec![
        RenderCommand::Clear(Colour::black()),
        RenderCommand::SetBackground(Colour::black()),
        RenderCommand::SetColour(Colour::white()),
        RenderCommand::DrawBorder('#'),
    ]);

    let mut arr = (false, array::Array::new(width as u32));
    let algs = vec![algorithms::Algorithm::Shuffle, algorithms::Algorithm::Shuffle];
    println!("algs: {}", algs.len());
    
    let mut last_time = time::Instant::now();
    
    for algorithm in algs.iter() {
        while !arr.0 {
            arr = algorithm.sort(arr.1);
            r.push_cmds(arr.1.render(height));
            r.update();
            
            thread::sleep(get_sleep_time(&last_time, &step));
            last_time = time::Instant::now();
        }
        arr = (false, arr.1.reset());
    }

    // make the red bars white
    arr.1 = arr.1.clear_changes();
    r.push_cmds(arr.1.render(height));
    r.update();

    // wait to exit to show the result
    thread::sleep(time::Duration::from_secs(2));
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
