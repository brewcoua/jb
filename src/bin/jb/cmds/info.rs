use clap::Command;
use console::style;
use jb::tool::{Kind,Type};

pub(crate) fn command() -> Command {
    Command::new("info")
        .about("Print information about the current environment and available tools")
}

pub(crate) fn dispatch() {
    let kinds = Kind::list();

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    let verbose = jb::env::Variable::Verbose.get_bool();
    let notify = jb::env::Variable::Notify.get_bool();
    let tools_dir = jb::env::Variable::ToolsDirectory.get::<std::path::PathBuf>();
    let icons_dir = jb::env::Variable::IconsDirectory.get::<std::path::PathBuf>();
    let bin_dir = jb::env::Variable::BinariesDirectory.get::<std::path::PathBuf>();

    println!(
        "{} {}\n",
        style(name).bold(),
        style(format!("v{version}")).dim().italic(),
    );

    println!("{}", style("Environment:").bold().underlined());
    println!("- Verbose: {}", if verbose { style("true").green() } else { style("false").red() });
    println!("- Notify: {}", if notify { style("true").green() } else { style("false").red() });
    println!("- Tools Directory: {}", style(tools_dir.display()).dim());
    println!("- Icons Directory: {}", style(icons_dir.display()).dim());
    println!("- Binaries Directory: {}", style(bin_dir.display()).dim());

    println!("\n{}", style("Tools:").bold().underlined());
    for kind in kinds {
        println!("- {}: {} ({})",
                 style(kind).cyan(),
                 style(kind.as_str()).italic(),
                 style(Type::kind_default(*kind)).dim(),
        );
    }
}