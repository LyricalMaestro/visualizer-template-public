#![allow(non_snake_case, unused_macros)]
use std::{char::MAX, fs::{self}};

use proconio::input;
use rand::prelude::*;
use svg::node::element::{Circle, Rectangle, Definitions, Group, Image, Line, Style, Use};
use web_sys::console::log_1;

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub sx: i64,
    pub sy: i64,
    pub targets: Vec<Vec<i64>>,
    // pub walls: [[usize; 4]; m]
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.m)?;
        // for i in 0..self.n {
        //     writeln!(f, "{} {}", self.a[i], self.b[i])?;
        // }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize,
        m: usize,
        eps: f32,
        delta: f32,
        sx: i64,
        sy: i64,
        targets: [[i64; 2]; n],
        walls: [[i64; 4]; m]
    }

    // let tmp = vec![1; 1];
    // let a = abs.iter().map(|(n, _)| *n).collect::<Vec<_>>();
    // let b = abs.iter().map(|(_, m)| *m).collect::<Vec<_>>();

    let tvec = targets.to_vec();

    Input { n, m, sx, sy, targets: tvec }
}

pub struct Output {
    pub val: Vec<OutputCore>
}

pub struct OutputCore {
    pub x: i64,
    pub y: i64,
    pub v_x: f64,
    pub v_y: f64,
    pub score: i64
}

pub fn parse_output(f: &str, m: usize) -> Output {
    let mut outputs = Output {val: Vec::new() };
    let mut current_output = OutputCore {
        x: 0,
        y: 0,
        v_x: 0.0,
        v_y: 0.0,
        score: 0,
    };

    for line in f.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        if parts[0].starts_with('#') {
            match parts[0] {
                "#p" => {
                    current_output.x = parts[1].parse().unwrap();
                    current_output.y = parts[2].parse().unwrap();
                }
                "#v" => {
                    current_output.v_x = parts[1].parse().unwrap();
                    current_output.v_y = parts[2].parse().unwrap();
                }
                "#s" => {
                    current_output.score = parts[1].parse().unwrap();
                    outputs.val.push(current_output);
                    current_output = OutputCore {
                        x: 0,
                        y: 0,
                        v_x: 0.0,
                        v_y: 0.0,
                        score: 0,
                    };
                }
                _ => {}
            }
        }
    }

    outputs
}

pub fn gen(seed: u64) -> Input {

    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);

    // let a = Vec::from([0, 200]);
    // let b = Vec::from([0, 200]);
    // Input { n: 2, m: 1,a, b }
    Input { n: 2, m: 0, sx: 43722, sy: -75332, targets: [[79243, 32532].to_vec(), [44002, -77034].to_vec()].to_vec() }
}

// fn get_pos(input: &Input, output: &Output, turn: usize) -> (usize, usize) {
//     if output.t[turn] == 1 {
//         // 惑星の場所を返す
//         return (input.a[output.r[turn] - 1], input.b[output.r[turn] - 1]);
//     } else {
//         // 宇宙ステーションの場所を返す
//         return (output.c[output.r[turn] - 1], output.d[output.r[turn] - 1]);
//     }
// }

fn calculate_score(input: &Input, output: &Output) -> i64 {
    // let mut result: f64 = 0.;

    // let vv = output.v;
    // for i in 0..(output.v - 1) {
    //     let base1 = output.t[i];
    //     let base2 = output.t[i + 1];
    //     let mut coeff = 1;
    //     if base1 == 1 && base2 == 1 {
    //         // 両方とも惑星
    //         coeff = 25;
    //     } else if (base1 == 1 && base2 == 2) || (base1 == 2 && base2 == 1) {
    //         coeff = 5;
    //     } else if base1 == 2 && base2 == 2 {
    //         coeff = 1;
    //     };
    //     let (x1, y1) = get_pos(input, output, i);
    //     let (x2, y2) = get_pos(input, output, i + 1);
    //     let euclid = ((x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)) as i32;
    //     result += (euclid * coeff) as f64;
    // }
    // // log_1(&format!("result: {}, v:{}", result, output.v).into());

    // let mut ret = 1000 * 1000 * 1000 / (1000 as f64 + (result as f64).powf(0.5)) as i64;
    // return ret;

    return 23456;
}

pub fn pos_convert(x: i64, y: i64, scale: f32) -> (f32, f32) {
    let x_f32: f32 = x as f32 + 1e5;
    let y_f32: f32 = y as f32 + 1e5;

    (x_f32 * scale, y_f32 * scale)
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let mut score = 0;

    // Canvasの設定
    let scale = 0.005;
    let W = 200000.0 * scale;
    let H = 200000.0 * scale;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10.0, H + 10.0))
        .set("width", W + 10.0)
        .set("height", H + 10.0)
        .set("style", "background-color:white");

    let drone_size = 10.0;

    let mut droneX = 0.0;
    let mut droneY = 0.0;
    if turn == 0 {
        let (drone_start_x, drone_start_y) = pos_convert(input.sx, input.sy, scale);
        droneX = drone_start_x;
        droneY = drone_start_y;
    }
    else {
        if let Some(o) = output.val.get(turn-1) {
            let (drone_start_x, drone_start_y) = pos_convert(o.x, o.y, scale);
            droneX = drone_start_x;
            droneY = drone_start_y;
            score = o.score;
        }
    }

    // ドローンの要素を追加
    doc = doc.add(
        Rectangle::new()
            .set("width", drone_size)
            .set("height", drone_size)
            .set("fill", "#ff0000")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("x", droneX)
            .set("y", droneY)
            .set("id", "drone")
    );

    // 目的地を追加
    for trow in input.targets.iter() {
        let (ftx, fty) = pos_convert(trow[0], trow[1], scale);
        doc = doc.add(
            Rectangle::new()
                .set("width", 5)
                .set("height", 5)
                .set("fill", "#00ff00")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("class", "target")
                .set("x", ftx)
                .set("y", fty)
        )
    }

    // 現在の場所にロケットの画像を埋め込む
    // let (x, y) = get_pos(input, output, turn);
    // doc = doc.add(Group::new().add(
    //     Use::new()
    //     .set("x", drone_start_x)
    //     .set("y", drone_start_y)
    //     .set("href", "#drone")
    // ));
    (score as i64, "".to_string(), doc.to_string())
}
