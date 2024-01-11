pub use tqdm::{refresh, tqdm as qtmd, Style, Tqdm as Qtmd};

pub trait Iter<Item>: Iterator<Item = Item> {
    fn qtmd(self) -> Qtmd<Item, Self>
    where
        Self: Sized,
    {
        qtmd(self)
    }
}

impl<Item, Iter: Iterator<Item = Item>> crate::Iter<Item> for Iter {}
