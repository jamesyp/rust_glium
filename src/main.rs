// #[macro_use]
extern crate glium;

use std::time::{Duration,SystemTime};

fn main() {
    use glium::glutin;

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut closed = false;

    let mut prev_time = SystemTime::now();
    while !closed {
        draw(&display);

        events_loop.poll_events(|ev| {
            match ev {
              glutin::Event::WindowEvent { event, .. } => match event {
                  glutin::WindowEvent::CloseRequested => closed = true,
                  _ => (),
              },
              _ => (),
            }
        });

        match prev_time.elapsed() {
          Ok(duration) => sleep_framerate(duration),
          Err(_)   => panic!("Something bust!"),
        };

        match prev_time.elapsed() {
          Ok(duration) => print_framerate(duration),
          Err(_)   => panic!("Something bust!"),
        };

        prev_time = SystemTime::now();
    }
}

fn draw(display: &glium::Display) {
    use glium::Surface;

    let mut target = display.draw();
    target.clear_color(0.0, 0.25, 0.7, 1.0);
    target.finish().unwrap();
}

fn sleep_framerate(duration: std::time::Duration) {
    use std::thread::sleep;

    let target_framerate = 100;
    let micros_per_frame = Duration::new(1, 0).as_micros() / target_framerate;
    let duration_as_micros = duration.as_micros();

    if (micros_per_frame > duration_as_micros) {
      let sleep_time = micros_per_frame - duration_as_micros;
      sleep(Duration::from_micros(sleep_time as u64));
    }
}

fn print_framerate(duration: std::time::Duration) {
    let one_second = Duration::new(1, 0);

    println!("{}", one_second.as_micros() / duration.as_micros());
}
