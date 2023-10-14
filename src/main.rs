#![feature(let_chains)]

use std::io::{stdout, Stdout};
use std::{env, fs::File, io::Write};

use gif::Frame;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut decoder = gif::DecodeOptions::new();
    // Configure the decoder such that it will expand the image to RGBA.
    decoder.set_color_output(gif::ColorOutput::RGBA);

    // Read the file header
    let file = File::open(&args[1]).unwrap();
    let mut decoder = decoder.read_info(file).unwrap();

    let mut stdout = stdout();

    let mut last_frame: Option<Frame> = None;

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        // if the disposal method is 'background', we clear the area of the last frame
        if let Some(last_frame) = last_frame && last_frame.dispose == gif::DisposalMethod::Background {
            clear_area(
                &mut stdout,
                last_frame.left,
                last_frame.top,
                last_frame.width,
                last_frame.height,
            );
        }

        if frame.interlaced {
            panic!("interlaced gifs are not supported");
        }

        draw_frame(&mut stdout, &frame);

        std::thread::sleep(std::time::Duration::from_millis(75));

        last_frame = Some(frame.clone());
    }
}

fn clear_area(stdout: &mut Stdout, left: u16, top: u16, width: u16, height: u16) {
    for y in top..top + height {
        for x in left..left + width {
            // we need to add 1 to the x coordinate to account for terminal cursors being 1-indexed
            write!(stdout, "\x1b[{};{}H ", y + 1, (x + 1) * 2).unwrap();
        }
    }
    stdout.flush().unwrap();
}

fn draw_frame(stdout: &mut Stdout, frame: &gif::Frame) {
    // loop over 4 at a time
    for (i, pixel) in frame.buffer.chunks_exact(4).enumerate() {
        // pixel is a slice of 4 u8s, representing RGBA
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        let x = frame.left + (i as u16 % frame.width);
        let y = frame.top + (i as u16 / frame.width);

        // as terminal character is taller than it is wide, print two spaces for each pixel
        write!(
            stdout,
            "\x1b[{};{}H\x1b[48;2;{r};{g};{b}m  ",
            y + 1,
            (x + 1) * 2,
        )
        .expect("failed to write to stdout");
    }
    stdout.flush().expect("failed to flush stdout");
}
