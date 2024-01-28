pub mod game;
pub mod inputs;

pub fn create_progress_bar(progress: i32, goal: i32) {
    let progress = progress.min(goal);
    let progress = (progress as f32 / goal as f32) * 100.0;

    let progress_bar_length = 20;
    let progress_bar_fill = (progress / 100.0 * progress_bar_length as f32) as usize;

    let mut progress_bar = String::from("[");

    for i in 0..progress_bar_length {
        if i < progress_bar_fill {
            progress_bar.push('=');
        } else if i == progress_bar_fill {
            progress_bar.push('>');
        } else {
            progress_bar.push(' ');
        }
    }

    progress_bar.push(']');
    progress_bar.push_str(&format!(" {:.2}%", progress));

    print!("\r{}", progress_bar);
}
