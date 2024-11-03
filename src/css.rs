use {
    crate::*,
    std::{
        fs,
        io::Read,
        path::{
            Path,
            PathBuf,
        },
    },
};


#[derive(Debug, Default)]
pub struct CssTask {}

impl CssTask {
    fn do_pages(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        let options = grass::Options::default();
        let pages = project.list_files("/src/page-scss/*.scss")?;
        for page in pages {
            let scss = fs::read_to_string(&page)?;
            let css = grass::from_string(&scss, &options)?;
            //println!("css: {}", css);
            let css_path: PathBuf =
                files::dest_path(&page, &project.build_dir.join("static"), "css")?;
            files::write(&css_path, &css)?;
        }
        Ok(())
    }

    /// Build the miaou.css file for each theme.
    ///
    /// Each theme CSS file is built by
    /// - compiling the main.scss file in src/main-scss, with reference both to
    ///     src/main-scss/*.scss and the theme directory's SCSS files
    /// - adding the CSS files of all plugins
    /// - adding the result of compiling the SCSS files of all plugins, which can
    ///     refer to SCSS files in src/main-scss and the theme directory
    fn do_themes(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        let main_scss_dir = project.root_dir.join("src/main-scss");
        let theme_dirs = project.list_dirs("/themes/*")?;
        let plugin_scss_files = project.list_files("/plugins/*/scss/*.scss")?;
        for theme_dir in theme_dirs {
            let theme = theme_dir.file_name().unwrap().to_str().unwrap();
            let mut dirs = Dirs::default();
            dirs.add(&main_scss_dir);
            dirs.add(&theme_dir);
            println!("Theme: {}", theme);
            let options = grass::Options::default().fs(&dirs);

            //-- compile the main.scss file
            let mut theme_css = grass::from_path("main.scss", &options)?;

            //-- add the plugin CSS files
            project.read_files_to_string("/plugins/*/css/*.css", &mut theme_css)?;

            //-- compile and add the plugin SCSS files
            for plugin_scss_file in &plugin_scss_files {
                let mut scss = String::new();
                scss.push_str("@import \"variables\";\n");
                scss.push_str("@import \"variables-default\";\n");
                files::read_file_to_string(&plugin_scss_file, &mut scss)?;
                let plugin_css = grass::from_string(&scss, &options)?;
                theme_css.push_str(&plugin_css);
            }

            let css_path = project.build_dir
                .join("static").join("themes").join(theme)
                .join("miaou.css");
            files::write(&css_path, &theme_css)?;
        }

        Ok(())
    }
}
impl Task for CssTask {
    fn execute(
        &self,
        project: &Project,
    ) -> anyhow::Result<()> {
        self.do_pages(project)?;
        self.do_themes(project)?;
        println!("sass done");
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Dirs {
    pub dirs: Vec<PathBuf>,
}
impl Dirs {
    fn add<P: Into<PathBuf>>(&mut self, dir: P) {
        let dir = dir.into();
        if !self.dirs.contains(&dir) {
            self.dirs.push(dir);
        }
    }
    fn path(&self, path: &Path) -> Option<PathBuf> {
        if path.is_absolute() {
            return Some(PathBuf::from(path));
        }
        for dir in &self.dirs {
            let path = dir.join(path);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }
}

impl grass::Fs for Dirs {
    fn is_dir(&self, path: &Path) -> bool {
        self.path(path).map_or(false, |path| path.is_dir())
    }
    fn is_file(&self, path: &Path) -> bool {
        self.path(path).map_or(false, |path| path.is_file())
    }
    // we read all the files having this name, in order of directories
    fn read(&self, path: &Path) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        let mut found = false;
        for dir in &self.dirs {
            let path = dir.join(path);
            if !path.exists() || !path.is_file() {
                continue;
            }
            fs::File::open(path)?.read_to_end(&mut buf)?;
            found = true;
        }
        if found {
            Ok(buf)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "not found"))
        }
    }

}
