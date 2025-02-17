#![allow(non_snake_case, unused_macros)]
use proconio::input;
use rand::prelude::*;
use svg::node::element::{Circle, Definitions, Group, Image, Line, Style, Use};

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub a: Vec<usize>,
    pub b: Vec<usize>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.m)?;
        for i in 0..self.n {
            writeln!(f, "{} {}", self.a[i], self.b[i])?;
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
        abs: [(usize, usize); n]
    }
    let a = abs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
    let b = abs.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    Input { n, m, a, b }
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
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);

    let a = Vec::from([0, 200]);
    let b = Vec::from([0, 200]);
    Input { n: 2, m: 1,a, b }
}

fn get_pos(input: &Input, output: &Output, turn: usize) -> (usize, usize) {
    if output.t[turn] == 1 {
        // 惑星の場所を返す
        return (input.a[output.r[turn] - 1], input.b[output.r[turn] - 1]);
    } else {
        // 宇宙ステーションの場所を返す
        return (output.c[output.r[turn] - 1], output.d[output.r[turn] - 1]);
    }
}

fn calculate_score(input: &Input, output: &Output) -> i64 {
    let mut result: f64 = 0.;

    let vv = output.v;
    for i in 0..(output.v - 1) {
        let base1 = output.t[i];
        let base2 = output.t[i + 1];
        let mut coeff = 1;
        if base1 == 1 && base2 == 1 {
            // 両方とも惑星
            coeff = 25;
        } else if (base1 == 1 && base2 == 2) || (base1 == 2 && base2 == 1) {
            coeff = 5;
        } else if base1 == 2 && base2 == 2 {
            coeff = 1;
        };
        let (x1, y1) = get_pos(input, output, i);
        let (x2, y2) = get_pos(input, output, i + 1);
        let euclid = ((x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)) as i32;
        result += (euclid * coeff) as f64;
    }
    // log_1(&format!("result: {}, v:{}", result, output.v).into());

    let mut ret = 1000 * 1000 * 1000 / (1000 as f64 + (result as f64).powf(0.5)) as i64;
    return ret;

    // return ((10.).powf(9) / (1000 + (result as f64).powf(0.5))) as usize;
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
