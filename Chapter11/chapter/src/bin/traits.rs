#![allow(unused_variables)]

use std::any::Any;
use std::io::{Result, Write};
use std::ops::Range;

trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

pub struct Canvas;

impl Canvas {
    fn write_at(&mut self, x: i32, y: i32, c: char) {}
}

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..self.y
    }
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        true
    }
}

// generic impl

trait Shape {}

struct Circle {
    r: f64,
}

impl Shape for Circle {}

struct Rectangle {
    l: f64,
    b: f64,
}

impl Shape for Rectangle {}

struct Triangle {
    b: f64,
    h: f64,
}

impl Shape for Triangle {}

trait HasArea {
    fn area(&self) -> f64;
}

impl<S: Shape + 'static> HasArea for S {
    fn area(&self) -> f64 {
        if let Some(c) = (self as &dyn Any).downcast_ref::<Circle>() {
            std::f64::consts::PI * c.r * c.r
        } else if let Some(r) = (self as &dyn Any).downcast_ref::<Rectangle>() {
            r.l * r.b
        } else if let Some(t) = (self as &dyn Any).downcast_ref::<Triangle>() {
            0.5 * t.b * t.h
        } else {
            0.0
        }
    }
}

fn main() -> Result<()> {
    assert!('x'.is_emoji());

    let t = Triangle { b: 10.0, h: 20.0 };
    let c = Circle { r: 10.0 };
    let r = Rectangle { l: 10.0, b: 20.0 };
    println!("{}, {}, {}", t.area(), c.area(), r.area());

    let mut out = Sink;
    out.write_all(b"This is all for naught!")?;
    Ok(())
}
