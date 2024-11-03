use crate::*;

pub trait Task {
    fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()>;
}

