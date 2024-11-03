use {
    crate::*,
    lazy_regex::*,
    std::{
        fmt,
        str::FromStr,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskRef {
    /// Clean then build everything
    All,
    /// Remove the build directory
    Clean,
    /// Build client side JS
    ClientJs,
    /// Copy resources to static directory
    Resources,
    /// Do all SASS/SCSS transfomations to CSS
    Css,
}

impl fmt::Display for TaskRef {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Clean => write!(f, "clean"),
            Self::ClientJs => write!(f, "client-js"),
            Self::Resources => write!(f, "resources"),
            Self::Css => write!(f, "css"),
        }
    }
}

impl FromStr for TaskRef {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        regex_switch!(s,
            "^all$"i => Self::All,
            "^clean$"i => Self::Clean,
            "^client-js$"i => Self::ClientJs,
            "^resources$"i => Self::Resources,
            "^css$"i => Self::Css,
        )
        .ok_or_else(|| "Not a valid task reference")
    }
}

impl TaskRef {
    pub fn add_to_set(self, task_set: &mut TaskSet, _project: &Project) {
        match self {
            Self::All => {
                task_set.add(Box::new(CleanTask::default()));
                task_set.add(Box::new(ClientJsTask::default()));
                task_set.add(Box::new(ResourcesTask::default()));
                task_set.add(Box::new(CssTask::default()));
            }
            Self::Clean => {
                task_set.add(Box::new(CleanTask::default()));
            }
            Self::ClientJs => {
                task_set.add(Box::new(ClientJsTask::default()));
            }
            Self::Resources => {
                task_set.add(Box::new(ResourcesTask::default()));
            }
            Self::Css => {
                task_set.add(Box::new(CssTask::default()));
            }
        }
    }
}
