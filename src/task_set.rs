use crate::*;

#[derive(Default)]
pub struct TaskSet {
    tasks: Vec<Box<dyn Task>>,
}

impl TaskSet {
    pub fn add(
        &mut self,
        task: Box<dyn Task>,
    ) {
        self.tasks.push(task);
    }
    pub fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        for task in &self.tasks {
            task.execute(project)?;
        }
        Ok(())
    }
}
