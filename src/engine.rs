use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::HashMap;

pub const COLS: i32 = 10;
pub const ROWS: i32 = 20;
pub const FLOOR: Option<i64> = None;
const DAS: f32 = 0.12;
const ARR: f32 = 0.025;
const LOCK: f32 = 0.5;
const RESETS: u8 = 15;
const FALL: f32 = 0.8;
const SOFT: f32 = 0.04;
const KEEP: i64 = 192;
pub const SHIFT: i64 = 1_000_000;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

const BAG: [Kind; 7] = [
    Kind::I,
    Kind::O,
    Kind::T,
    Kind::S,
    Kind::Z,
    Kind::J,
    Kind::L,
];

pub fn cells(k: Kind, r: u8) -> [(i32, i32); 4] {
    match (k, r % 4) {
        (Kind::I, 0) => [(0, 1), (1, 1), (2, 1), (3, 1)],
        (Kind::I, 1) => [(2, 0), (2, 1), (2, 2), (2, 3)],
        (Kind::I, 2) => [(0, 2), (1, 2), (2, 2), (3, 2)],
        (Kind::I, 3) => [(1, 0), (1, 1), (1, 2), (1, 3)],
        (Kind::O, _) => [(1, 0), (2, 0), (1, 1), (2, 1)],
        (Kind::T, 0) => [(1, 0), (0, 1), (1, 1), (2, 1)],
        (Kind::T, 1) => [(1, 0), (1, 1), (2, 1), (1, 2)],
        (Kind::T, 2) => [(0, 1), (1, 1), (2, 1), (1, 2)],
        (Kind::T, 3) => [(1, 0), (0, 1), (1, 1), (1, 2)],
        (Kind::S, 0) => [(1, 0), (2, 0), (0, 1), (1, 1)],
        (Kind::S, 1) => [(1, 0), (1, 1), (2, 1), (2, 2)],
        (Kind::S, 2) => [(1, 1), (2, 1), (0, 2), (1, 2)],
        (Kind::S, 3) => [(0, 0), (0, 1), (1, 1), (1, 2)],
        (Kind::Z, 0) => [(0, 0), (1, 0), (1, 1), (2, 1)],
        (Kind::Z, 1) => [(2, 0), (1, 1), (2, 1), (1, 2)],
        (Kind::Z, 2) => [(0, 1), (1, 1), (1, 2), (2, 2)],
        (Kind::Z, 3) => [(1, 0), (0, 1), (1, 1), (0, 2)],
        (Kind::J, 0) => [(0, 0), (0, 1), (1, 1), (2, 1)],
        (Kind::J, 1) => [(1, 0), (2, 0), (1, 1), (1, 2)],
        (Kind::J, 2) => [(0, 1), (1, 1), (2, 1), (2, 2)],
        (Kind::J, 3) => [(1, 0), (1, 1), (0, 2), (1, 2)],
        (Kind::L, 0) => [(2, 0), (0, 1), (1, 1), (2, 1)],
        (Kind::L, 1) => [(1, 0), (1, 1), (1, 2), (2, 2)],
        (Kind::L, 2) => [(0, 1), (1, 1), (2, 1), (0, 2)],
        (Kind::L, 3) => [(0, 0), (1, 0), (1, 1), (1, 2)],
        _ => unreachable!(),
    }
}

fn kicks(k: Kind, a: u8, b: u8) -> [(i32, i32); 5] {
    match (k == Kind::I, a % 4, b % 4) {
        (true, 0, 1) => [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        (true, 1, 0) => [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        (true, 1, 2) => [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        (true, 2, 1) => [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        (true, 2, 3) => [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        (true, 3, 2) => [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        (true, 3, 0) => [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        (true, 0, 3) => [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        (false, 0, 1) => [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        (false, 1, 0) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        (false, 1, 2) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        (false, 2, 1) => [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        (false, 2, 3) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        (false, 3, 2) => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
        (false, 3, 0) => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
        (false, 0, 3) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        _ => [(0, 0); 5],
    }
}

#[derive(Clone, Copy)]
pub struct Fall {
    pub k: Kind,
    pub x: i32,
    pub y: i64,
    pub r: u8,
}

impl Fall {
    pub fn cells(&self) -> [(i32, i64); 4] {
        cells(self.k, self.r).map(|(dx, dy)| (self.x + dx, self.y.saturating_add(dy as i64)))
    }

    pub fn at(&self, dx: i32, dy: i64, r: u8) -> Fall {
        Fall {
            k: self.k,
            x: self.x + dx,
            y: self.y.saturating_add(dy),
            r,
        }
    }
}

pub struct Well {
    pub rows: HashMap<i64, u16>,
}

impl Well {
    pub fn new() -> Self {
        Self {
            rows: HashMap::new(),
        }
    }

    pub fn solid(&self, x: i32, y: i64) -> bool {
        if !(0..COLS).contains(&x) || FLOOR == Some(y) {
            return true;
        }
        self.rows.get(&y).is_some_and(|m| m & (1 << x) != 0)
    }

    pub fn fits(&self, p: &Fall) -> bool {
        p.cells().iter().all(|&(x, y)| !self.solid(x, y))
    }

    pub fn stamp(&mut self, p: &Fall) {
        for (x, y) in p.cells() {
            *self.rows.entry(y).or_insert(0) |= 1 << x;
        }
    }

    pub fn clear(&mut self, _ys: &[i64]) -> u32 {
        let mut hit: Vec<i64> = self
            .rows
            .iter()
            .filter(|(_, m)| *m & 0x03FF == 0x03FF)
            .map(|(&y, _)| y)
            .collect();
        hit.sort_unstable();
        for y in &hit {
            self.rows.remove(y);
        }
        let old = std::mem::take(&mut self.rows);
        for (ry, m) in old {
            let n = hit.iter().filter(|&&cy| cy > ry).count() as i64;
            *self.rows.entry(ry + n).or_insert(0) |= m;
        }
        hit.len() as u32
    }

    pub fn gap(&self, p: &Fall) -> Option<i64> {
        let mut best: Option<i64> = None;
        for (x, y) in p.cells() {
            if !(0..COLS).contains(&x) {
                return Some(0);
            }
            if let Some(f) = FLOOR {
                if f > y {
                    let d = f - y - 1;
                    best = Some(best.map_or(d, |b| b.min(d)));
                }
            }
            let bit = 1u16 << x;
            for (&ry, &mask) in &self.rows {
                if ry > y && mask & bit != 0 {
                    let d = ry - y - 1;
                    best = Some(best.map_or(d, |b| b.min(d)));
                }
            }
        }
        best
    }

    pub fn shift(&mut self, dy: i64) {
        let old = std::mem::take(&mut self.rows);
        self.rows = old.into_iter().map(|(y, m)| (y - dy, m)).collect();
    }

    pub fn prune(&mut self, y: i64) {
        if FLOOR.is_some() {
            return;
        }
        self.rows.retain(|&ry, _| (ry - y).abs() <= KEEP);
    }

    pub fn top(&self) -> Option<i64> {
        self.rows.keys().copied().min()
    }
}

struct Queue {
    rest: Vec<Kind>,
}

impl Queue {
    fn new() -> Self {
        Self { rest: Vec::new() }
    }

    fn pull(&mut self) -> Kind {
        if self.rest.is_empty() {
            let mut k = BAG.to_vec();
            for i in (1..k.len()).rev() {
                k.swap(i, gen_range(0, i + 1));
            }
            self.rest = k;
        }
        self.rest.pop().unwrap()
    }
}

#[derive(Clone, Copy)]
pub enum Spin {
    None,
    Mini,
    Full,
}

pub struct Ledger {
    pub score: u64,
    pub lines: u64,
    pub combo: i32,
    pub b2b: bool,
}

impl Ledger {
    fn new() -> Self {
        Self {
            score: 0,
            lines: 0,
            combo: -1,
            b2b: false,
        }
    }

    pub fn lv(&self) -> u64 {
        1 + self.lines / 10
    }

    fn pay(&mut self, spin: Spin, n: u32) -> &'static str {
        if n == 0 {
            self.combo = -1;
            return match spin {
                Spin::Full => {
                    self.score = self.score.saturating_add(400 * self.lv());
                    "T-SPIN"
                }
                Spin::Mini => {
                    self.score = self.score.saturating_add(100 * self.lv());
                    "T-SPIN MINI"
                }
                Spin::None => "",
            };
        }
        self.combo += 1;
        let lv = self.lv();
        self.lines = self.lines.saturating_add(n as u64);
        let hard = n == 4 || !matches!(spin, Spin::None);
        let mut pts: u64 = match (spin, n) {
            (Spin::Full, 1) => 800,
            (Spin::Full, 2) => 1200,
            (Spin::Full, 3) => 1600,
            (Spin::Mini, 1) => 200,
            (Spin::Mini, 2) => 400,
            (_, 1) => 100,
            (_, 2) => 300,
            (_, 3) => 500,
            (_, 4) => 800,
            _ => 0,
        };
        if hard && self.b2b {
            pts += pts / 2;
        }
        if self.combo > 0 {
            pts += 50 * self.combo as u64;
        }
        self.score = self.score.saturating_add(pts.saturating_mul(lv));
        self.b2b = hard;
        match (spin, n) {
            (Spin::Full, 1) => "T-SPIN SINGLE",
            (Spin::Full, 2) => "T-SPIN DOUBLE",
            (Spin::Full, 3) => "T-SPIN TRIPLE",
            (Spin::Mini, _) => "T-SPIN MINI",
            (_, 4) => "TETRIS",
            _ => "",
        }
    }
}

pub struct Flash {
    pub text: String,
    pub t: f32,
}

pub struct Game {
    pub well: Well,
    pub now: Fall,
    pub next: Kind,
    pub ledger: Ledger,
    pub over: bool,
    pub flash: Option<Flash>,
    bag: Queue,
    grav: f32,
    wait: f32,
    left: u8,
    das: f32,
    arr: f32,
    dir: i32,
    pub turned: bool,
    pub kick: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut bag = Queue::new();
        let now = Fall {
            k: bag.pull(),
            x: 3,
            y: Self::origin(),
            r: 0,
        };
        let next = bag.pull();
        Self {
            well: Well::new(),
            now,
            next,
            ledger: Ledger::new(),
            over: false,
            flash: None,
            bag,
            grav: 0.0,
            wait: 0.0,
            left: RESETS,
            das: 0.0,
            arr: 0.0,
            dir: 0,
            turned: false,
            kick: 0,
        }
    }

    fn origin() -> i64 {
        match FLOOR {
            Some(f) => f.saturating_sub(ROWS as i64),
            None => 0,
        }
    }

    fn birth(&mut self, k: Kind) {
        let y = match FLOOR {
            Some(_) => Self::origin(),
            None => self.well.top().unwrap_or(self.now.y).saturating_sub(4),
        };
        let p = Fall { k, x: 3, y, r: 0 };
        if !self.well.fits(&p) {
            self.over = true;
        }
        self.now = p;
        self.grav = 0.0;
        self.wait = 0.0;
        self.left = RESETS;
        self.turned = false;
    }

    pub fn resting(&self) -> bool {
        !self.well.fits(&self.now.at(0, 1, self.now.r))
    }

    fn slide(&mut self, dx: i32, dy: i64) -> bool {
        let p = self.now.at(dx, dy, self.now.r);
        if self.well.fits(&p) {
            self.now = p;
            self.turned = false;
            self.touch();
            true
        } else {
            false
        }
    }

    fn rot(&mut self, s: i8) -> bool {
        if self.now.k == Kind::O {
            self.turned = true;
            self.kick = 0;
            self.touch();
            return true;
        }
        let a = self.now.r;
        let b = (a as i8 + s).rem_euclid(4) as u8;
        for (i, &(kx, ky)) in kicks(self.now.k, a, b).iter().enumerate() {
            let p = self.now.at(kx, -(ky as i64), b);
            if self.well.fits(&p) {
                self.now = p;
                self.turned = true;
                self.kick = i;
                self.touch();
                return true;
            }
        }
        false
    }

    fn touch(&mut self) {
        if self.resting() && self.left > 0 {
            self.wait = 0.0;
            self.left -= 1;
        }
    }

    pub fn spin(&self) -> Spin {
        if self.now.k != Kind::T || !self.turned {
            return Spin::None;
        }
        let (cx, cy) = (self.now.x + 1, self.now.y + 1);
        let c = [
            self.well.solid(cx - 1, cy - 1),
            self.well.solid(cx + 1, cy - 1),
            self.well.solid(cx - 1, cy + 1),
            self.well.solid(cx + 1, cy + 1),
        ];
        if c.iter().filter(|&&x| x).count() < 3 {
            return Spin::None;
        }
        let face = match self.now.r % 4 {
            0 => [c[0], c[1]],
            1 => [c[1], c[3]],
            2 => [c[2], c[3]],
            _ => [c[0], c[2]],
        };
        if face.iter().all(|&x| x) || self.kick == 4 {
            Spin::Full
        } else {
            Spin::Mini
        }
    }

    fn say(&mut self, s: &str) {
        if !s.is_empty() {
            self.flash = Some(Flash {
                text: s.to_string(),
                t: 1.2,
            });
        }
    }

    pub fn settle(&mut self) {
        self.well.stamp(&self.now);
        let spin = self.spin();
        let mut ys: Vec<i64> = self.now.cells().iter().map(|c| c.1).collect();
        ys.sort_unstable();
        ys.dedup();
        let n = self.well.clear(&ys);
        let msg = self.ledger.pay(spin, n);
        self.say(msg);
        self.well.prune(self.now.y);
        let heir = self.next;
        self.next = self.bag.pull();
        self.birth(heir);
        self.rebase();
    }

    fn drop(&mut self) {
        if let Some(d) = self.well.gap(&self.now) {
            if d > 0 {
                self.now.y = self.now.y.saturating_add(d);
                self.ledger.score = self.ledger.score.saturating_add(2 * d as u64);
                self.turned = false;
            }
            self.settle();
        }
    }

    fn rebase(&mut self) {
        if FLOOR.is_some() {
            return;
        }
        let y = self.now.y;
        if y.abs() < SHIFT {
            return;
        }
        self.well.shift(y);
        self.now.y = 0;
    }

    fn keys(&mut self) {
        if is_key_pressed(KeyCode::R) {
            *self = Game::new();
            return;
        }
        if self.over {
            return;
        }

        let d = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        if d != self.dir {
            self.dir = d;
            self.das = 0.0;
            self.arr = 0.0;
            if d != 0 {
                self.slide(d, 0);
            }
        } else if d != 0 {
            self.das += get_frame_time();
            if self.das >= DAS {
                self.arr += get_frame_time();
                while self.arr >= ARR {
                    self.arr -= ARR;
                    if !self.slide(d, 0) {
                        break;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Z) {
            self.rot(-1);
        }
        if is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Up) {
            self.rot(1);
        }
        if FLOOR.is_some() && is_key_pressed(KeyCode::Space) {
            self.drop();
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if let Some(f) = &mut self.flash {
            f.t -= dt;
            if f.t <= 0.0 {
                self.flash = None;
            }
        }
        if self.over {
            self.keys();
            return;
        }
        self.keys();
        if self.over {
            return;
        }

        let g = if is_key_down(KeyCode::Down) {
            SOFT
        } else {
            (FALL / self.ledger.lv() as f32).max(1.0 / 60.0)
        };
        self.grav += dt;
        while self.grav >= g {
            self.grav -= g;
            let y0 = self.now.y;
            if self.slide(0, 1) {
                if FLOOR.is_some() && is_key_down(KeyCode::Down) {
                    self.ledger.score = self.ledger.score.saturating_add(1);
                }
                if self.now.y == y0 {
                    break;
                }
            } else {
                break;
            }
        }

        if self.resting() {
            self.wait += dt;
            if self.wait >= LOCK {
                self.settle();
            }
        } else {
            self.wait = 0.0;
            self.left = RESETS;
        }

        self.well.prune(self.now.y);
        self.rebase();
    }
}
