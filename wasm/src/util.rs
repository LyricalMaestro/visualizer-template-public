#![allow(non_snake_case, unused_macros)]
use std::{char::MAX, fs::{self}};

use proconio::input;
use svg::node::element::{Circle, Line};
use web_sys::console::{log, trace};

const MAX_TURN: u32 = 5000;


#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub epsilon: String,
    pub delta: String,
    pub start_pos: (i32, i32),
    pub target_pos: Vec<(i32, i32)>,
    pub walls: Vec<(i32, i32, i32, i32)>,
    pub noises: Vec<String>,
    pub v_noise: Vec<(i32, i32)>
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {} {}", self.n, self.m, self.epsilon, self.delta)?;
        writeln!(f, "{} {}", self.start_pos.0, self.start_pos.1)?;
        for i in 0..self.n {
            writeln!(f, "{} {}", self.target_pos[i].0, self.target_pos[i].1)?;
        }
        for i in 0..self.m {
            writeln!(f, "{} {} {} {}", self.walls[i].0, self.walls[i].1, self.walls[i].2, self.walls[i].3)?;
        }
        for i in 0.. MAX_TURN {
            writeln!(f, "{}", self.noises[i as usize])?;
        }
        for i in 0.. MAX_TURN {
            writeln!(f, "{} {}", self.v_noise[i as usize].0, self.v_noise[i as usize].1)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n:usize,
        m: usize,
        epsilon: String, 
        delta: String,
        start_pos: (i32, i32),
        target_pos: [(i32, i32); n],
        walls: [(i32, i32, i32, i32); m],
        noises: [String; MAX_TURN],
        v_noise: [(i32, i32); MAX_TURN]    
    }
    Input { n, m, epsilon, delta, start_pos, target_pos, walls, noises, v_noise }
}

pub struct Output {
    pub c: Vec<usize>,
    pub d: Vec<usize>,
    pub v: usize,
    pub t: Vec<usize>,
    pub r: Vec<usize>,
}

pub fn parse_output(f: &str, m: usize) -> Output {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        cd: [(usize, usize); m],
        v: usize,
        tr: [(usize, usize); v]
    }
    let c = cd.iter().map(|(c, _)| *c).collect::<Vec<_>>();
    let d = cd.iter().map(|(_, d)| *d).collect::<Vec<_>>();
    let t = tr.iter().map(|(t, _)| *t).collect::<Vec<_>>();
    let r = tr.iter().map(|(_, r)| *r).collect::<Vec<_>>();
    Output { c, d, v, t, r }
}

pub fn gen(seed: u64) -> Input {
//    eprintln!("--------");
/*
    let f = fs::read_to_string("./data/inA/0000.txt").unwrap();
    let f = proconio::source::once::OnceSource::from(f.as_str());
    input! {
        from f,
        n:usize,
        m: usize,
        epsilon: String, 
        delta: String,
        start_pos: (i32, i32),
        target_pos: [(i32, i32); n],
        walls: [(i32, i32, i32, i32); m],
        noises: [String; MAX_TURN],
        v_noise: [(i32, i32); MAX_TURN]    
    }
    Input { n, m, epsilon, delta, start_pos, target_pos, walls, noises, v_noise }
    */
    let x  = vec!["0.3".to_string(); MAX_TURN as usize];
    let y  = vec![(-3, 8); MAX_TURN as usize];
    Input {n:0, m:0, epsilon:"0.1".to_string(), delta:"10.0".to_string(), start_pos:(3, 8), target_pos:vec![], walls:vec![], noises:x, v_noise:y}
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let score = 3;
    println!("debug....");

    let W = 1000;
    let H = 1000;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");

    doc = doc.add(
        Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", W)
            .set("y2", 0)
            .set("stroke", "#000000"),
    );
    doc = doc.add(
        Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", 0)
            .set("y2", H)
            .set("stroke", "#000000"),
    );
    doc = doc.add(
        Line::new()
            .set("x1", W)
            .set("y1", 0)
            .set("x2", W)
            .set("y2", H)
            .set("stroke", "#000000"),
    );
    doc = doc.add(
        Line::new()
            .set("x1", 0)
            .set("y1", H)
            .set("x2", W)
            .set("y2", H)
            .set("stroke", "#000000"),
    );

    doc = doc.add(
        Circle::new()
            .set("cx", 240)
            .set("cy", 500)
            .set("r", 18)
            .set("fill", "#8700a3")
            .set("stroke", "black")
            .set("stroke-width", "2")
            .set("class", "box")
    );


    // 現在の場所にロケットの画像を埋め込む
//    let (x, y) = get_pos(input, output, turn);
//    doc = doc.add(Group::new().add(Use::new().set("x", x).set("y", y).set("href", "#robo")));
    (score as i64, "".to_string(), doc.to_string())
}
