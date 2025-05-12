#[derive(Debug)]
pub struct TodoItem {
    id: u32,
    title: String,
    done: bool,
}

impl TodoItem {
    pub fn new(id: u32, title: String) -> Self {
        Self {id, title, done=false}
    }

    pub fn format(&self) -> String {
        format!("{}|{}|{}", self.id, self.title, self.done)
    }
    
}