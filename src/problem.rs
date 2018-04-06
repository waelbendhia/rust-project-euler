pub trait Show {
    fn show(&self) -> String;
}

pub struct Problem {
    pub ind: u32,
    pub name: String,
    pub solution: fn() -> i64,
}

impl Show for Problem {
    fn show(&self) -> String {
        format!(
            "Problem {}: {} \nAnswer: {}",
            self.ind,
            self.name,
            (self.solution)(),
        )
    }
}
