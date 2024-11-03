use {
    clap::{
        CommandFactory,
        Parser,
    },
    miaou_builder::*,
};

fn main() -> anyhow::Result<()> {
    cli_log::init_cli_log!();
    let args = Args::parse();
    if args.help {
        clap_help::Printer::new(Args::command()).print_help();
        return Ok(());
    }
    if args.version {
        println!("bacon {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let project = Project::new()?;
    let mut task_set = TaskSet::default();
    args.task.add_to_set(&mut task_set, &project);
    task_set.execute(&project)?;
    cli_log::info!("bye");
    Ok(())
}
