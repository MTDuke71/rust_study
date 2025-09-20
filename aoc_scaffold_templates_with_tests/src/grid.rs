
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(w: usize, h: usize, fill: T) -> Self {
        Self { w, h, data: vec![fill; w*h] }
    }
}

impl<T> Grid<T> {
    pub fn from_vec(w: usize, h: usize, data: Vec<T>) -> Self {
        assert_eq!(w*h, data.len());
        Self { w, h, data }
    }
    pub fn idx(&self, x: usize, y: usize) -> usize { y*self.w + x }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.w && y < self.h { Some(&self.data[self.idx(x,y)]) } else { None }
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.w && y < self.h { let i=self.idx(x,y); Some(&mut self.data[i]) } else { None }
    }
    pub fn neighbors4(&self, x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        let w=self.w; let h=self.h;
        let mut v = Vec::with_capacity(4);
        if x>0     { v.push((x-1,y)); }
        if y>0     { v.push((x,y-1)); }
        if x+1<w   { v.push((x+1,y)); }
        if y+1<h   { v.push((x,y+1)); }
        v.into_iter()
    }
}
