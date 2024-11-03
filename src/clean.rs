use {
    crate::*,
};


#[derive(Debug, Default)]
pub struct CleanTask {
}

impl Task for CleanTask {
    fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        let static_dir = project.build_dir.join("static");
        if static_dir.exists() {
            std::fs::remove_dir_all(&static_dir)?;
        }
        println!("clean done");
        Ok(())
    }
}

