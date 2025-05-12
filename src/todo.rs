#[derive(Debug)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl TodoItem {
    pub fn new(id: u32, title: String) -> Self {
        Self {id, title, done: false}
    }

    pub fn format(&self) -> String {
        format!("{}|{}|{}", self.id, self.title, self.done)
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 { return None; }

        Some(Self {
            id: parts[0].parse().ok()?,
            title: parts[1].to_string(),
            done: parts[2].parse().ok()?,
        })
    }

}