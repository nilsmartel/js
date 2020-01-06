use std::rc::Rc;

enum Value {
    Boolean(bool),
    String(Rc<String>),
    Number(f64),
    Reference(usize),
    // FunctionPointer(usize),
}

enum Object {
    Array(Vec<Object>),
    Class(std::collections::HashMap<String, Object>),
}

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
