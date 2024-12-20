extern crate sdl2;

use std::time::Instant;

use canvas::{Content, Grid};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect};

mod canvas;
mod controller;

const WINDOW_RESOLUTION: (u32, u32) = (1200, 600);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let (width, height) = WINDOW_RESOLUTION;

    let window = video_subsystem
        .window("water sim", width, height)
        .position(0, 0)
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let mut grid = Grid {
        width,
        height,
        cols_num: 60,
        rows_num: 30,
        contents: Content::get_preset(60, 30),
        entities: [Vec::new(), Vec::new(), Vec::new()],
        t: Instant::now(),
    };

    let mut controller = controller::Controller::default();

    let mut event_pump = sdl_context.event_pump()?;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'run,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => controller.toggle_pause(),
                Event::KeyDown {
                    keycode: Some(Keycode::LSHIFT),
                    ..
                } => controller.shiftmod = true,
                Event::KeyUp {
                    keycode: Some(Keycode::LSHIFT),
                    ..
                } => controller.shiftmod = false,
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    //controller.mouse_pressed = true;
                    let i = (x / 20) as usize;
                    let j = (y / 20) as usize;
                    //println!("{}: {} {}: {}", x, i, y, j);

                    let x = (i * 20) as i32;
                    let y = (j * 20) as i32;

                    match mouse_btn {
                        MouseButton::Left => {
                            if *grid.contents.get(j).unwrap().get(i).unwrap() == Content::Void {
                                if controller.shiftmod {
                                    grid.contents[j][i] = Content::Rock;
                                    grid.entities[1].push((x, y));
                                } else {
                                    grid.contents[j][i] = Content::Water;
                                    grid.entities[0].push((x, y));
                                }
                            }
                        }
                        MouseButton::Right => {
                            if *grid.contents.get(j).unwrap().get(i).unwrap() == Content::Void {
                                grid.contents[j][i] = Content::Lava;
                                grid.entities[2].push((x, y));
                            }
                        }
                        _ => (),
                    }
                }
                Event::MouseButtonUp { .. } => {
                    //controller.mouse_pressed = false;
                }
                _ => (), // println!("{:?}", event),
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        grid.render(&mut canvas, false);

        if controller.paused {
            canvas.set_draw_color(Color::RGB(127, 0, 0));
            canvas.fill_rect(Rect::new(width as i32 - 150, 50, 100, 100))?;
        } else {
            if grid.t.elapsed().as_millis() >= 500 {
                grid.update();
                grid.t = Instant::now();
            }
        }
        canvas.present();
    }

    Ok(())
}
