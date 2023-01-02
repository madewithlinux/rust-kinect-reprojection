use anyhow::Result;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

use kinect1::{get_sensor_count, start_frame_thread};

fn main() -> Result<()> {
    println!("Hello, world!");
    dbg!(get_sensor_count().unwrap());

    let receiver = start_frame_thread();

    // let rgb_output_dir = PathBuf::from("data/rgb_frames/");
    // create_dir_all(&rgb_output_dir)?;
    // let mut i = 0;
    // for frame in receiver.iter() {
    //     dbg!(i);
    //     let filename = format!("kinect_rgb_data_{}.png", i);
    //     frame.save(rgb_output_dir.join(filename))?;
    //     i += 1;
    // }

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Kinect RGB data - ESC to exit", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // // Limit to max ~30 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600 / 2)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // TODO: handle hangup case?
        if let Ok(frame) = receiver.try_recv() {
            let frame = frame.color_frame;
            assert_eq!(frame.len() / 3, buffer.len());
            for (i, pixel) in frame.pixels().enumerate() {
                buffer[i] = (pixel.0[0] as u32) << 16 | (pixel.0[1] as u32) << 8 | (pixel.0[2] as u32);
            }
            // for i in 0..(WIDTH*HEIGHT) {
            //     frame.get(i)
            //     buffer[i] =(
            //         (frame[i] as u32) << 24
            //     );
            // }
        }
        // for i in buffer.iter_mut() {
        //     *i = 0; // write something more funny here!
        // }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
