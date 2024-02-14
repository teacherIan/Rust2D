use std::thread;

const SIZE: usize = 11;

enum Step {
    Left,
    Right,
    Top,
    Bottom,
    TopLeftToBottomRight,
    BottomLeftToTopRight,
}

fn main() {
    let mut main_arr: [[char; SIZE]; SIZE] = [['-'; SIZE]; SIZE];
    spacer("Start");

    loop {
        step(Step::Left, &mut main_arr);
        step(Step::Right, &mut main_arr);
        step(Step::Top, &mut main_arr);
        step(Step::Bottom, &mut main_arr);
        step(Step::TopLeftToBottomRight, &mut main_arr);
        step(Step::BottomLeftToTopRight, &mut main_arr);
    }
}

fn spacer(title: &str) {
    let formatted_title = format!("{:-^1$}", title, SIZE * 3); // size * 3 so box appears square
    println!("\n{}\n", formatted_title);
}

fn view_arr(main_arr: [[char; SIZE]; SIZE]) {
    for row in main_arr.iter() {
        for col in row.iter() {
            print!(" {} ", col)
        }
        println!()
    }
}

fn left_side(main_arr: &mut [[char; SIZE]; SIZE]) {
    for i in 0..main_arr.len() {
        switch_value(&mut main_arr[i][0]);
    }
}

fn right_side(main_arr: &mut [[char; SIZE]; SIZE]) {
    for i in 0..main_arr.len() {
        switch_value(&mut main_arr[i][SIZE - 1]);
    }
}

fn top(main_arr: &mut [[char; SIZE]; SIZE]) {
    for i in 0..main_arr.len() {
        switch_value(&mut main_arr[0][i]);
    }
}

fn bottom(main_arr: &mut [[char; SIZE]; SIZE]) {
    for i in 0..main_arr.len() {
        switch_value(&mut main_arr[SIZE - 1][i]);
    }
}

fn top_left_to_bottom_right(main_arr: &mut [[char; SIZE]; SIZE]) {
    for i in 0..main_arr.len() {
        switch_value(&mut main_arr[i][i]);
    }
}

fn bottom_left_to_top_right(main_arr: &mut [[char; SIZE]; SIZE]) {
    for i in 0..main_arr.len() {
        switch_value(&mut main_arr[SIZE - i - 1][i]);
    }
}

fn switch_value(char: &mut char) -> char {
    if *char == '*' {
        *char = '-';
    } else {
        *char = '*';
    }
    *char
}

fn step(step: Step, main_arr: &mut [[char; SIZE]; SIZE]) {
    clearscreen::clear().expect("failed to clear screen");
    let ten_mills = std::time::Duration::from_millis(1000);
    match step {
        Step::Right => right_side(main_arr),
        Step::Left => left_side(main_arr),
        Step::Top => top(main_arr),
        Step::Bottom => bottom(main_arr),
        Step::TopLeftToBottomRight => top_left_to_bottom_right(main_arr),
        Step::BottomLeftToTopRight => bottom_left_to_top_right(main_arr),
    }
    view_arr(*main_arr);
    thread::sleep(ten_mills);
}
