use {
    crate::*,
};

#[derive(Default)]
pub struct ClientJsTask {
}

impl ClientJsTask {
    pub fn do_miaou_js(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        let mut js = String::new();
        for glob in [
            "/src/main-js/*.js",
            "/src/main-js/prettify/prettify.js",
            "/src/main-js/prettify/lang-*.js",
            "/plugins/*/client-scripts/*.js",
        ] {
            project.read_files_to_string(glob, &mut js)?;
        }
        let path = project.build_dir.join("static/miaou.concat.js");
        files::write(&path, &js)?;
        let minified = self.minify_js(js);
        let path = project.build_dir.join("static/miaou.min.js");
        files::write(&path, &minified)?;
        Ok(())
    }
    pub fn do_sw_js(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        let mut js = String::new();
        project.read_files_to_string("/src/sw-js/*.js", &mut js)?;
        let path = project.build_dir.join("static/miaou.sw.concat.js");
        files::write(&path, &js)?;
        let minified = self.minify_js(js);
        let path = project.build_dir.join("static/miaou.sw.min.js");
        files::write(&path, &minified)?;
        Ok(())
    }
    pub fn do_pages_js(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        let pages = project.list_files("/src/page-js/*.js")?;
        let dest_dir = project.build_dir.join("static");
        for page in &pages {
            let mut js = String::new();
            files::read_file_to_string(page, &mut js)?;
            let dest_path = files::dest_path(
                page,
                &dest_dir,
                "min.js"
            )?;
            files::write(&dest_path, &js)?;
        }
        Ok(())
    }
    pub fn minify_js(&self, input: String) -> String {
        input
    }
}
impl Task for ClientJsTask {
    fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        self.do_miaou_js(project)?;
        self.do_sw_js(project)?;
        self.do_pages_js(project)?;
        println!("client-js done");
        Ok(())
    }
}
