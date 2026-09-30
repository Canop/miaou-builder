use {
    clap::{
        CommandFactory,
        Parser,
    },
    miaou_builder::*,
    std::io::{
        self,
        Write,
    },
};

fn main() -> anyhow::Result<()> {
    cli_log::init_cli_log!();
    if let Err(e) = run() {
        if e.downcast_ref::<io::Error>()
            .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe)
        {
            std::process::exit(141);
        }
        return Err(e);
    }
    cli_log::info!("bye");
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut out = io::stdout();
    if args.help {
        clap_help::Printer::new(Args::command()).write_help(&mut out)?;
        return Ok(());
    }
    if args.version {
        writeln!(out, "miaou-builder {}", env!("CARGO_PKG_VERSION"))?;
        return Ok(());
    }
    let project = Project::new()?;
    let mut task_set = TaskSet::default();
    args.task.add_to_set(&mut task_set, &project);
    task_set.execute(&project)
}
