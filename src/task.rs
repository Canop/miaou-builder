use crate::*;

pub enum Task {
    Scss,
}

impl Task {
    pub fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        match self {
            Self::Scss => {
                let task = ScssTask::new()?;
                task.execute(project)
            }
        }
    }
}
