use std::time::Instant;

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cols_num: u32,
    pub rows_num: u32,

    pub contents: Vec<Vec<Content>>,

    // water, rock, lava
    pub entities: [Vec<(i32, i32)>; 3],

    pub t: Instant,
}

impl Grid {
    pub fn render(&self, canvas: &mut WindowCanvas, grid: bool) {
        if grid {
            canvas.set_draw_color(Color::RGB(127, 127, 127));
            let pad = 0;

            let step = self.width / self.cols_num;
            let mut rows: Vec<Rect> = Vec::with_capacity(self.cols_num as usize + 1);
            for i in 0..=self.rows_num {
                let y = step * i;
                rows.push(Rect::new(0, y.saturating_sub(pad) as i32, self.width, pad));
            }

            let step = self.width / self.cols_num;
            let mut cols: Vec<Rect> = Vec::with_capacity(self.rows_num as usize + 1);
            for i in 0..=self.cols_num {
                let x = step * i;
                cols.push(Rect::new(x.saturating_sub(pad) as i32, 0, pad, self.height));
            }

            canvas.fill_rects(&rows).unwrap();
            canvas.fill_rects(&cols).unwrap();
        }

        let mut water_entities = Vec::new();
        let mut rock_entities = Vec::new();
        let mut lava_entities = Vec::new();

        for (x, y) in self.entities[0].iter() {
            water_entities.push(Rect::new(*x, *y, 20, 20));
        }
        for (x, y) in self.entities[1].iter() {
            rock_entities.push(Rect::new(*x, *y, 20, 20));
        }
        for (x, y) in self.entities[2].iter() {
            lava_entities.push(Rect::new(*x, *y, 20, 20));
        }

        canvas.set_draw_color(Color::RGB(201, 127, 68));
        canvas.fill_rects(&rock_entities).unwrap();

        canvas.set_draw_color(Color::RGB(11, 10, 68));
        canvas.fill_rects(&water_entities).unwrap();

        canvas.set_draw_color(Color::RGB(201, 10, 10));
        canvas.fill_rects(&lava_entities).unwrap();
    }

    pub fn update(&mut self) {
        let mut propagate = |i: usize, j: usize| -> (i32, i32) {
            let mut x = 0;
            let mut y = 0;

            if self.contents[j + 1][i] == Content::Void {
                self.contents[j][i] = Content::Void;
                self.contents[j + 1][i] = Content::Water;
                y += 20;
                return (x, y);
            } else if self.contents[j + 1][i - 1] == Content::Void {
                self.contents[j][i] = Content::Void;
                self.contents[j + 1][i - 1] = Content::Water;
                y += 20;
                x -= 20;
            } else if self.contents[j + 1][i + 1] == Content::Void {
                self.contents[j][i] = Content::Void;
                self.contents[j + 1][i + 1] = Content::Water;
                y += 20;
                x += 20;
            } else if self.contents[j][i - 1] == Content::Void {
                self.contents[j][i] = Content::Void;
                self.contents[j][i - 1] = Content::Water;
                x -= 20;
            } else if self.contents[j][i + 1] == Content::Void {
                self.contents[j][i] = Content::Void;
                self.contents[j][i + 1] = Content::Water;
                x += 20;
            }
            (x, y)
        };

        for (x, y) in self.entities[0].iter_mut() {
            let i = (*x / 20) as usize;
            let j = (*y / 20) as usize;

            let (vx, vy) = propagate(i, j);

            *x += vx;
            *y += vy;
        }

        for (x, y) in self.entities[2].iter_mut() {
            let i = (*x / 20) as usize;
            let j = (*y / 20) as usize;

            let (vx, vy) = propagate(i, j);

            *x += vx;
            *y += vy;
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Content {
    Water,
    Rock,
    Void,
    Lava,
}

impl Content {
    pub fn get_preset(x: usize, y: usize) -> Vec<Vec<Self>> {
        let mut res = Vec::with_capacity(x);

        res.push(vec![Self::Rock; x]);

        for _ in 1..y - 1 {
            let mut row = Vec::with_capacity(x);
            row.push(Self::Rock);
            for _ in 0..x - 2 {
                row.push(Self::Void);
            }
            row.push(Self::Rock);
            res.push(row);
        }

        res.push(vec![Self::Rock; x]);

        res
    }

    pub fn symtetize_water(&self, other: Self) -> Option<Self> {
        match other {
            Self::Lava => Some(Self::Rock),
            _ => None,
        }
    }
}
