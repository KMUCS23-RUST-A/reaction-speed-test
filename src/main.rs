use std::io::{self, Write};
use std::time::{Duration, Instant};
use rand::Rng;
use colored::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut total_time = Duration::new(0, 0);
    let mut num_rounds = 0;

    loop {
        println!("게임을 시작하려면 '시작'을 입력하세요");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

        if input.trim() != "시작" {
            println!("{}", "잘못된 입력입니다.".bold());
            continue;
        }

        let delay = Duration::from_secs(rng.gen_range(1..6));
        std::thread::sleep(delay);

        let word = generate_random_word();
        println!("단어: {}", word);

        let start_time = Instant::now();
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("입력을 읽을 수 없습니다.");

        if user_input.trim() != word {
            println!("{}", "틀렸습니다!".bold().red());
            continue;
        }

        let end_time = Instant::now();

        let round_time = end_time.duration_since(start_time);
        let round_time_secs = round_time.as_secs_f32();
        let avg_time = total_time.as_secs_f32() / (num_rounds as f32);

        let time_str = format!("{:.2}초", round_time_secs);
        let time_colored = if round_time_secs > avg_time {
            time_str.bold().red()
        } else {
            time_str.bold().green()
        };
        println!("{} 걸렸습니다!", time_colored);

        total_time += round_time;
        num_rounds += 1;

        let avg_time_secs = total_time.as_secs_f32() / (num_rounds as f32);
        let avg_time_colored = format!("{:.2}초", avg_time_secs).yellow();
        println!("평균 시간: {}", avg_time_colored);
    }
}

fn generate_random_word() -> String {
    let words = vec![
        "빨간색", "노란색", "주황색", "파란색", "남색",
        "보라색", "검은색", "은색", "하얀색", "초록색",
        "분홍색", "하늘색", "살구색", "금색", "청록색",
    ];
    let index = rand::thread_rng().gen_range(0..words.len());
    words[index].to_owned()
}
