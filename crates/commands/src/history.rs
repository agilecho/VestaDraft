use vesta_model::ProjectContent;
#[derive(Clone, Debug)]
pub struct History {
    pub snapshots: Vec<ProjectContent>,
    pub cursor: usize,
    pub generation: u64,
}
impl History {
    pub fn new(c: ProjectContent) -> Self {
        Self {
            snapshots: vec![c],
            cursor: 0,
            generation: 0,
        }
    }
    pub fn record(&mut self, c: ProjectContent) {
        self.snapshots.truncate(self.cursor + 1);
        self.snapshots.push(c);
        if self.snapshots.len() > 256 {
            self.snapshots.remove(0);
        }
        self.cursor = self.snapshots.len() - 1;
        self.generation += 1;
    }
}
