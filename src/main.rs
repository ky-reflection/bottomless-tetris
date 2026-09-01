mod engine;
mod view;

use engine::Game;
use macroquad::prelude::*;
use macroquad::rand as mqrand;
use std::time::{SystemTime, UNIX_EPOCH};
use view::{VIEW_H, VIEW_W};

fn icon() -> miniquad::conf::Icon {
    fn arr<const N: usize>(b: &[u8]) -> [u8; N] {
        b.try_into().expect("icon")
    }
    miniquad::conf::Icon {
        small: arr(include_bytes!("../assets/icon16.rgba")),
        medium: arr(include_bytes!("../assets/icon32.rgba")),
        big: arr(include_bytes!("../assets/icon64.rgba")),
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: VIEW_W as i32,
        window_height: VIEW_H as i32,
        high_dpi: true,
        window_resizable: false,
        icon: Some(icon()),
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    mqrand::srand(seed);
    let mut g = Game::new();
    loop {
        g.tick(get_frame_time());
        view::draw(&g);
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::engine::*;

    fn p(k: Kind, x: i32, y: i64, r: u8) -> Fall {
        Fall { k, x, y, r }
    }

    #[test]
    fn no_floor() {
        let w = Well::new();
        if FLOOR.is_none() {
            let f = p(Kind::T, 3, 9_007_199_254_740_991, 0);
            assert!(w.fits(&f));
            assert_eq!(w.gap(&f), None);
        } else {
            assert!(w.gap(&p(Kind::T, 3, 0, 0)).is_some());
        }
    }

    #[test]
    fn walls_only() {
        let w = Well::new();
        assert!(!w.fits(&p(Kind::I, -1, 0, 0)));
        assert!(!w.fits(&p(Kind::I, 8, 0, 0)));
        assert!(w.fits(&p(Kind::O, 3, -1000, 0)));
    }

    #[test]
    fn gap_on_stack() {
        let mut w = Well::new();
        w.stamp(&p(Kind::I, 3, 50, 0));
        let d = w.gap(&p(Kind::T, 3, 0, 0)).unwrap();
        assert!(d > 0 && d < 50);
        assert!(w.fits(&p(Kind::T, 3, d, 0)));
        assert!(!w.fits(&p(Kind::T, 3, d + 1, 0)));
    }

    #[test]
    fn clear_falls_down() {
        let mut w = Well::new();
        w.rows.insert(10, 0x03FF);
        w.rows.insert(9, 0x0001);
        assert_eq!(w.clear(&[10]), 1);
        assert!(w.rows.get(&10).is_some_and(|&m| m == 0x0001));
        assert!(!w.rows.contains_key(&9));
    }

    #[test]
    fn clear_two_lines_compacts() {
        let mut w = Well::new();
        w.rows.insert(8, 0x0001);
        w.rows.insert(9, 0x03FF);
        w.rows.insert(10, 0x03FF);
        w.rows.insert(11, 0x0002);
        assert_eq!(w.clear(&[9, 10]), 2);
        assert_eq!(w.rows.get(&10).copied(), Some(0x0001));
        assert_eq!(w.rows.get(&11).copied(), Some(0x0002));
        assert!(!w.rows.contains_key(&8));
        assert!(!w.rows.contains_key(&9));
    }

    #[test]
    fn clear_split_lines_compacts() {
        let mut w = Well::new();
        w.rows.insert(7, 0x0004);
        w.rows.insert(8, 0x03FF);
        w.rows.insert(9, 0x0001);
        w.rows.insert(10, 0x03FF);
        w.rows.insert(11, 0x0002);
        assert_eq!(w.clear(&[]), 2);
        assert_eq!(w.rows.get(&9).copied(), Some(0x0004));
        assert_eq!(w.rows.get(&10).copied(), Some(0x0001));
        assert_eq!(w.rows.get(&11).copied(), Some(0x0002));
        assert_eq!(w.rows.len(), 3);
    }

    #[test]
    fn clear_four_drops_above() {
        let mut w = Well::new();
        w.rows.insert(14, 0x0008);
        for y in 15..19 {
            w.rows.insert(y, 0x03FF);
        }
        assert_eq!(w.clear(&[]), 4);
        assert_eq!(w.rows.get(&18).copied(), Some(0x0008));
        assert_eq!(w.rows.len(), 1);
    }

    #[test]
    fn shift_keeps_shape() {
        let mut w = Well::new();
        w.stamp(&p(Kind::O, 3, SHIFT + 8, 0));
        w.shift(SHIFT);
        assert!(w.solid(4, 8) || w.solid(3, 8) || w.solid(4, 9));
        assert!(!w.rows.keys().any(|y| y.abs() > 20));
    }

    #[test]
    fn spin_reads_corners() {
        let mut g = Game::new();
        g.now = p(Kind::T, 3, 5, 0);
        let (cx, cy) = (4, 6);
        *g.well.rows.entry(cy - 1).or_insert(0) |= 1 << (cx - 1);
        *g.well.rows.entry(cy - 1).or_insert(0) |= 1 << (cx + 1);
        *g.well.rows.entry(cy + 1).or_insert(0) |= 1 << (cx - 1);
        g.turned = true;
        g.kick = 0;
        assert!(matches!(g.spin(), Spin::Full | Spin::Mini));
    }

    #[test]
    fn kick_off_wall() {
        let w = Well::new();
        let f = p(Kind::T, -1, 0, 3);
        assert!(!w.fits(&f));
        assert!(w.fits(&f.at(1, 0, 0)));
    }

    #[test]
    fn settle_pays_and_clears() {
        let mut g = Game::new();
        g.well.rows.insert(5, 0x03FF);
        g.now = p(Kind::I, 3, 4, 0);
        g.settle();
        assert!(g.ledger.score >= 100);
        assert!(g.ledger.lines >= 1);
        assert!(!g.well.rows.contains_key(&5));
    }

    #[test]
    fn never_rests_in_void() {
        let g = Game::new();
        assert!(!g.resting());
        if FLOOR.is_none() {
            assert_eq!(g.well.gap(&g.now), None);
        } else {
            assert!(g.well.gap(&g.now).is_some());
        }
    }
}
