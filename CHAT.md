# 초기 메인 로직을 짤 때 요청했던 생성문
rust를 이용해서 단어 기반 반응속도 게임을 만들어줘. 게임은 다음과 같은 제약조건을 만족해야 해. 그리고 코드만 생성해주고 설명은 하지말아줘.

- 사용자가 준비되었다고 입력하면 랜덤한 시간이 지난 후 단어가 나타나고, 이 단어를 사용자가 입력해서 맞춰야 해
- 단어를 맞추면 단어가 나타났을 때부터 맞췄을 때까지 얼마나 걸렸는지 표시해야 해.
- 게임을 계속 진행하기 위해서 무한루프를 돌아야해
- 그리고 지금까지 평균적으로 얼마나 걸렸는지도 알려줘

# ChatGPT의 생성코드
아래는 Rust로 작성된 단어 기반 반응속도 게임 코드입니다.

```rust
use std::io::{self, Write};
use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut total_time = Duration::new(0, 0);
    let mut num_rounds = 0;

    loop {
        println!("게임을 시작하려면 '시작'을 입력하세요");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

        if input.trim() != "시작" {
            println!("잘못된 입력입니다.");
            continue;
        }

        let delay = Duration::from_secs(rng.gen_range(1..5));
        println!("{}초 후에 단어가 나타납니다...", delay.as_secs());
        std::thread::sleep(delay);

        let word = generate_random_word();
        println!("단어: {}", word);

        let start_time = Instant::now();
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("입력을 읽을 수 없습니다.");
        let end_time = Instant::now();

        if user_input.trim() != word {
            println!("틀렸습니다!");
            continue;
        }

        let round_time = end_time.duration_since(start_time);
        println!("{}초 걸렸습니다!", round_time.as_secs());
        total_time += round_time;
        num_rounds += 1;

        let avg_time = total_time.as_secs_f32() / (num_rounds as f32);
        println!("평균 시간: {:.2}초", avg_time);
    }
}

fn generate_random_word() -> String {
    let words = vec![
        "apple", "banana", "cherry", "date", "elderberry",
        "fig", "grape", "honeydew", "kiwi", "lemon",
        "mango", "nectarine", "orange", "pear", "quince",
        "raspberry", "strawberry", "tangerine", "watermelon",
    ];
    let index = rand::thread_rng().gen_range(0..words.len());
    words[index].to_owned()
}
```
위의 코드를 실행하면, 사용자가 "시작"을 입력하면 게임이 시작됩니다. 게임은 랜덤한 지연 시간 후에 단어를 출력하고, 사용자가 단어를 입력하면 이전에 출력된 단어와 일치하는지 확인합니다. 일치하지 않으면 게임이 다시 시작됩니다. 일치하면 사용자가 단어를 맞춘데 걸린 시간을 출력하고, 이전에 맞춘 모든 단어의 평균 시간도 출력합니다.

# 가독성 강화 요청문
좋아 그럼 해당 코드에 이제 걸린 초를 시각적으로 좀 더 효과를 주기 위해 colored 크레이트를 이용해서 다음과 같은 사항을 적용해줘.
- 평균 시간은 노란색으로 표시해줘.
- 이번 라운드에서 걸린 시간이 평균 시간보다 길면 빨간색, 짧다면 초록색으로 표시해줘.
- 사용자가 올바른 입력을 하지 않았을 때는 표시하는 문자열에 bold를 적용해줘.

# ChatGPT의 수정 소스코드
아래는 colored 크레이트를 사용하여 걸린 시간과 평균 시간을 색상으로 표시하고, 잘못된 입력에 대한 문자열에 bold를 적용한 Rust 코드입니다.
```rust
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
            println!("{}", "잘못된 입력입니다.".bold().red());
            continue;
        }

        let delay = Duration::from_secs(rng.gen_range(1..5));
        println!("{}초 후에 단어가 나타납니다...", delay.as_secs());
        std::thread::sleep(delay);

        let word = generate_random_word();
        println!("단어: {}", word);

        let start_time = Instant::now();
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("입력을 읽을 수 없습니다.");
        let end_time = Instant::now();

        if user_input.trim() != word {
            println!("{}", "틀렸습니다!".bold().red());
            continue;
        }

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
        "apple", "banana", "cherry", "date", "elderberry",
        "fig", "grape", "honeydew", "kiwi", "lemon",
        "mango", "nectarine", "orange", "pear", "quince",
        "raspberry", "strawberry", "tangerine", "watermelon",
    ];
    let index = rand::thread_rng().gen_range(0..words.len());
    words[index].to_owned()
}
```
