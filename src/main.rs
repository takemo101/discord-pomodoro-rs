use clap::{App, Arg};
use serde::{Serialize, Deserialize};
use toml;
use std::fs;
use reqwest;
use chrono::{Local, DateTime, Duration};

#[derive(Debug, Deserialize)]
struct Setting {
    name: String,
    endpoint: String,
}

#[derive(Debug, Deserialize)]
struct Time {
    run: i64,
    rest: i64,
}

#[derive(Debug, Deserialize)]
struct Config {
    setting: Setting,
    time: Time,
}

#[derive(Debug, Serialize)]
struct Request {
    message: String,
}

/// create new request
fn to_request(message: String) -> Request {
    Request { message: message }
}

/// send request
fn sending<T: serde::Serialize>(endpoint: &String, data: T) -> Result<(), reqwest::Error>  {
    let res = reqwest::blocking::Client::new()
        .post(endpoint)
        .json(&data)
        .send()?
        .text()?;

    println!("response is: {}", res);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string("Config.toml").expect("ファイルが読み込めませんでした");
    let config: Config = toml::from_str(&config_text).expect("データが読み込めませんでした");

    let matches = App::new("discord_timer")
        .version("0.1.0") // バージョン情報
        .author("takemo <takemo101@gmail.com>") // 作者情報
        .about("Discord Timer CLI") // このアプリについて
        .arg(
            Arg::with_name("time") // 分数オプション
                .help("time minutes") // ヘルプメッセージ
                .short("t") // ショートコマンド
                .long("time") // ロングコマンド
                .takes_value(true) // 値あり
        )
        .get_matches();

    let mut minute = config.time.run; // タイマー分数

    // paが指定されていれば値を表示
    if let Some(o) = matches.value_of("time") {
        minute = o.parse::<i64>().expect("指定したオプションは数値ではありません！");
    }

    println!("minutes: {}", minute);

    sending(
        &config.setting.endpoint,
        to_request(format!("{}さん\n{}分間集中してください", config.setting.name, minute))
    )?;

    let stop_datetime: DateTime<Local> = Local::now() + Duration::minutes(minute);

    loop {
        std::thread::sleep(std::time::Duration::from_millis(60000));
        let now_datetime: DateTime<Local> = Local::now();

        if stop_datetime <= now_datetime {
            break;
        }

        println!("datetime: {:?}", now_datetime);
    }

    sending(
        &config.setting.endpoint,
        to_request(format!("{}さん\nお疲れ様でした\n休憩目安は{}分間です", config.setting.name, config.time.rest))
    )?;

    Ok(())
}
