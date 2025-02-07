use crate::*;

#[derive(Debug, Default)]
pub struct ResourcesTask {}

impl ResourcesTask {
    pub fn do_main_resources(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        for src in project.list_files("/src/rsc/*")? {
            let file_name = files::name_of(&src)?;
            let dest = project.build_dir.join("static").join(file_name);
            files::copy(&src, &dest)?;
        }
        Ok(())
    }
    pub fn do_plugin_resources(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        for plugin_dir in project.list_dirs("/plugins/*")? {
            let plugin = files::name_of(&plugin_dir)?;
            info!("plugin: {}", plugin);
            let rsc_dir = plugin_dir.join("rsc");
            if rsc_dir.exists() {
                files::copy(
                    &rsc_dir,
                    &project.build_dir.join("static").join(plugin).join("rsc"),
                )?;
            }
        }
        Ok(())
    }
}
impl Task for ResourcesTask {
    fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        self.do_main_resources(project)?;
        self.do_plugin_resources(project)?;
        println!("resources done");
        Ok(())
    }
}
