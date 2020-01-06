struct Arena {
    objects: Vec<Object>,
    free_blocks: Vec<usize>,
}

impl Arena {
    fn push(&mut self, o: Object) -> usize {
        if let Some(index) = self.free_blocks.pop() {
            self.objects[index] = o;
            index
        } else {
            let index = self.objects.len();
            self.objects.push(o);
            index
        }
    }

    unsafe fn free(&mut self, index: usize) {
        assert!(index < self.objects.len());

        if self.objects.len() == index + 1 {
            self.objects.pop();
            return;
        }

        self.free_blocks.push(index);
    }
}
