use crate::engine::{cells, Fall, Game, Kind, COLS, FLOOR, ROWS};
use macroquad::prelude::*;

pub const PINK: Color = Color {
    r: 1.0,
    g: 0.07,
    b: 0.53,
    a: 1.0,
};
pub const VIEW_W: f32 = 700.0;
pub const VIEW_H: f32 = 740.0;
const UNITS: f32 = 36.0;

struct Lay {
    cell: f32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    sx: f32,
    sw: f32,
    px: f32,
}

fn frame() -> Rect {
    let sw = screen_width();
    let sh = screen_height();
    let s = (sw / VIEW_W).min(sh / VIEW_H);
    let w = VIEW_W * s;
    let h = VIEW_H * s;
    Rect::new((sw - w) * 0.5, (sh - h) * 0.5, w, h)
}

fn lay() -> Lay {
    let v = frame();
    let pad = (v.h * 0.035).max(16.0);
    let mut cell = ((v.h - pad * 2.0) / ROWS as f32).floor().max(8.0);
    let mut px = (cell * 0.18).clamp(3.0, 6.0);
    let mut side = (UNITS * px + pad).max(cell * 4.6);
    if pad * 3.0 + cell * COLS as f32 + side > v.w {
        cell = ((v.w - pad * 3.0) * 0.62 / COLS as f32).floor().max(8.0);
        side = (v.w - cell * COLS as f32 - pad * 3.0).max(72.0);
        px = ((side - 8.0) / UNITS).floor().clamp(2.0, 6.0);
    }
    let w = cell * COLS as f32;
    let h = cell * ROWS as f32;
    let ox = ((v.w - w - pad - side) * 0.5).max(pad * 0.5);
    Lay {
        cell,
        x: v.x + ox,
        y: v.y + (v.h - h) * 0.5,
        w,
        h,
        sx: v.x + ox + w + pad,
        sw: side,
        px,
    }
}

fn xy(wx: i32, wy: i64, l: &Lay) -> Option<(f32, f32)> {
    if !(0..ROWS as i64).contains(&wy) {
        return None;
    }
    Some((l.x + wx as f32 * l.cell, l.y + wy as f32 * l.cell))
}

fn skin(x: f32, y: f32, s: f32) {
    if s < 4.0 {
        return;
    }
    draw_rectangle(x, y, s, s, PINK);
    let a = (s * 0.12).max(1.0);
    draw_rectangle(x + a, y + a, s - a * 2.0, s - a * 2.0, BLACK);
    let b = (s * 0.28).max(2.0);
    draw_rectangle(x + b, y + b, s - b * 2.0, s - b * 2.0, PINK);
    let c = (s * 0.40).max(3.0);
    draw_rectangle(x + c, y + c, s - c * 2.0, s - c * 2.0, BLACK);
    let d = (s * 0.12).max(1.0).min(s * 0.2);
    draw_rectangle(x + (s - d) * 0.5, y + (s - d) * 0.5, d, d, PINK);
}

fn round(x: f32, y: f32, w: f32, h: f32, r: f32, c: Color) {
    let r = r.min(w * 0.5).min(h * 0.5);
    draw_rectangle(x + r, y, w - 2.0 * r, h, c);
    draw_rectangle(x, y + r, w, h - 2.0 * r, c);
    draw_circle(x + r, y + r, r, c);
    draw_circle(x + w - r, y + r, r, c);
    draw_circle(x + r, y + h - r, r, c);
    draw_circle(x + w - r, y + h - r, r, c);
}

fn glyph(c: char) -> [u8; 7] {
    match c {
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b01110, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        ' ' => [0; 7],
        _ => [0b11111; 7],
    }
}

fn text(s: &str, x: f32, y: f32, px: f32, c: Color) {
    let mut ox = x;
    for ch in s.chars() {
        let g = glyph(ch);
        for (row, bits) in g.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    draw_rectangle(ox + col as f32 * px, y + row as f32 * px, px, px, c);
                }
            }
        }
        ox += 6.0 * px;
    }
}

fn box_of(k: Kind, x: f32, y: f32, s: f32, cell: f32) {
    round(x, y, s, s, cell * 0.35, BLACK);
    let c = cells(k, 0);
    let minx = c.iter().map(|p| p.0).min().unwrap();
    let maxx = c.iter().map(|p| p.0).max().unwrap();
    let miny = c.iter().map(|p| p.1).min().unwrap();
    let maxy = c.iter().map(|p| p.1).max().unwrap();
    let cs = cell * 0.82;
    let px0 = x + (s - (maxx - minx + 1) as f32 * cs) * 0.5 - minx as f32 * cs;
    let py0 = y + (s - (maxy - miny + 1) as f32 * cs) * 0.5 - miny as f32 * cs;
    for (dx, dy) in c {
        skin(px0 + dx as f32 * cs, py0 + dy as f32 * cs, cs);
    }
}

fn ghost(p: &Fall, d: i64, l: &Lay) {
    for (x, y) in p.cells() {
        if let Some((sx, sy)) = xy(x, y.saturating_add(d), l) {
            draw_rectangle_lines(sx + 2.0, sy + 2.0, l.cell - 4.0, l.cell - 4.0, 2.0, PINK);
        }
    }
}

fn stat(l: &Lay, y: &mut f32, name: &str, val: &str) {
    text(name, l.sx, *y, l.px, BLACK);
    *y += 8.0 * l.px;
    text(val, l.sx, *y, l.px, BLACK);
    *y += 10.0 * l.px;
}

pub fn draw(g: &Game) {
    clear_background(PINK);
    let l = lay();
    draw_rectangle(l.x, l.y, l.w, l.h, BLACK);

    for row in 0..=ROWS {
        let sy = l.y + row as f32 * l.cell;
        let t = if row % 10 == 0 {
            l.cell * 0.35
        } else {
            l.cell * 0.12
        };
        draw_rectangle(l.x, sy, t, 2.0, PINK);
        draw_rectangle(l.x + l.w - t, sy, t, 2.0, PINK);
    }

    for (&y, &mask) in &g.well.rows {
        for x in 0..COLS {
            if mask & (1 << x) != 0 {
                if let Some((sx, sy)) = xy(x, y, &l) {
                    skin(sx, sy, l.cell);
                }
            }
        }
    }

    if let Some(d) = g.well.gap(&g.now) {
        if d > 0 {
            ghost(&g.now, d, &l);
        }
    }

    if !g.over {
        for (x, y) in g.now.cells() {
            if let Some((sx, sy)) = xy(x, y, &l) {
                skin(sx, sy, l.cell);
            }
        }
    }

    if let Some(f) = &g.flash {
        let w = f.text.len() as f32 * 6.0 * l.px;
        text(
            &f.text,
            l.x + (l.w - w) * 0.5,
            l.y + l.h * 0.42,
            l.px,
            Color {
                r: 1.0,
                g: 0.07,
                b: 0.53,
                a: (f.t / 1.2).clamp(0.0, 1.0),
            },
        );
    }

    let mut y = l.y + 8.0;
    stat(
        &l,
        &mut y,
        "SCORE",
        &format!("{:06}", g.ledger.score % 1_000_000),
    );
    stat(
        &l,
        &mut y,
        "LINES",
        &format!("{:03}", g.ledger.lines % 1000),
    );
    if FLOOR.is_some() {
        stat(
            &l,
            &mut y,
            "LEVEL",
            &format!("{:02}", g.ledger.lv().min(99)),
        );
    }

    text("NEXT", l.sx, y, l.px, BLACK);
    y += 8.0 * l.px;
    let s = (l.cell * 4.2).min(l.sw);
    box_of(g.next, l.sx, y, s, l.cell);

    if g.over {
        let m = "R  RESTART";
        let w = m.len() as f32 * 6.0 * l.px;
        text(m, l.x + (l.w - w) * 0.5, l.y + l.h * 0.55, l.px, PINK);
    }
}
