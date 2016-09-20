use clear::Clear;

#[derive(Debug)]
pub struct ObjectPool<T: Default + Clear> {
    objects: Vec<T>,
    length: usize,
}

impl<T: Default + Clear> ObjectPool<T> {
    pub fn new() -> Self {
        ObjectPool {
            objects: Vec::new(),
            length: 0,
        }
    }

    pub fn clear(&mut self) {
        self.length = 0;
    }

    fn expand(&mut self) {
        self.objects.push(T::default());
    }

    pub fn alloc(&mut self) -> &mut T {
        if self.length == self.objects.len() {
            self.expand();
        }
        let ret = &mut self.objects[self.length];
        self.length += 1;

        ret.clear();
        ret
    }
}
