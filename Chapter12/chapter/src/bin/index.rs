use std::default::Default;
use std::fmt;
use std::ops::{Index, IndexMut};

//trait Index<Idx> {
//    type Output: ?Sized;
//
//    fn index(&self, index: Idx) -> &Self::Output;
//}
//
//trait IndexMut<Idx>: Index<Idx> {
//    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
//}

#[derive(Debug)]
struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P> fmt::Display for Image<P>
where
    P: fmt::Debug + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for chunk in self.pixels.chunks(self.width) {
            let _ = writeln!(f, "{:?}", chunk);
        }
        Ok(())
    }
}

impl<P: Default + Copy + Clone> Image<P> {
    fn new(width: usize, height: usize) -> Self {
        Image {
            width: width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    let mut image = Image::<u8>::new(5, 7);
    println!("{}", image);

    image[0][4] = 100;
    image[6][0] = 200;
    println!("{}", image);
}