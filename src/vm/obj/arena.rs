pub struct Arena<T> {
    objects: Vec<T>,
    free_blocks: Vec<usize>,
}

impl<T> Arena<T> {
    pub fn push(&mut self, o: T) -> usize {
        if let Some(index) = self.free_blocks.pop() {
            self.objects[index] = o;
            index
        } else {
            let index = self.objects.len();
            self.objects.push(o);
            index
        }
    }

    pub fn free(&mut self, index: usize) {
        assert!(index < self.objects.len());

        if self.objects.len() == index + 1 {
            self.objects.pop();
            return;
        }

        self.free_blocks.push(index);
    }

    pub fn objects(&self) -> &Vec<T> {
        &self.objects
    }
}
