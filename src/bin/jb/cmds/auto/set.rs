use std::fmt::Display;
use std::str::FromStr;
use clap::{arg, Command, value_parser};
use jb::env::Variable;
use crate::emoji::CHECK;

pub(super) fn command() -> Command {
    Command::new("set")
        .about("Set the frequency of automatic updates for tools")
        .long_about("This command will set the frequency of automatic updates for JetBrains tools. It is required to run this command before enabling automatic updates.")
        .arg(
            arg!(frequency: <FREQUENCY> "The frequency of the updates")
                .value_parser(value_parser!(Frequency))
                .required(true)
                .num_args(1)
        )
}

pub(super) fn dispatch(args: &clap::ArgMatches) -> jb::Result<()> {
    let frequency = args.get_one::<Frequency>("frequency").unwrap();

    // Generate the systemd service AND timer files
    let systemd_path = Variable::SystemdDirectory.get::<std::path::PathBuf>();

    if !systemd_path.exists() {
        jb::catch!(std::fs::create_dir(&systemd_path));
    }

    let current_exe = jb::catch!(std::env::current_exe());

    let description = "Check and update JetBrains tools";

    // Create the service file
    let service_content = format!(
        "[Unit]\n\
        Description={description}\n\
        \n\
        [Service]\n\
        Type=simple\n\
        ExecStart={executable} refresh --all -n\n\
        \n\
        [Install]\n\
        WantedBy=multi-user.target\n",
        executable = current_exe.display(),
    );

    let service_path = systemd_path.join("jb.service");
    jb::debug!("Writing to {}", service_path.display());

    jb::catch!(std::fs::write(&service_path, service_content));

    // Create the timer file
    let timer_content = format!(
        "[Unit]\n\
        Description={description}\n\
        \n\
        [Timer]\n\
        OnCalendar={frequency}\n\
        AccuracySec=12h\n\
        Persistent=true\n\
        \n\
        [Install]\n\
        WantedBy=timers.target\n",
        frequency = match frequency {
            Frequency::Daily => "daily",
            Frequency::Weekly => "weekly",
            Frequency::Monthly => "monthly",
        },
    );

    let timer_path = systemd_path.join("jb.timer");
    jb::debug!("Writing to {}", timer_path.display());

    jb::catch!(std::fs::write(&timer_path, timer_content));

    jb::info!("{CHECK} Set automatic updates to {}", frequency);

    Ok(())
}

#[derive(Debug,Copy,Clone)]
enum Frequency {
    Daily,
    Weekly,
    Monthly,
}
impl FromStr for Frequency {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(Frequency::Daily),
            "weekly" => Ok(Frequency::Weekly),
            "monthly" => Ok(Frequency::Monthly),
            _ => anyhow::bail!("Invalid frequency"),
        }
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::Daily => write!(f, "Daily"),
            Frequency::Weekly => write!(f, "Weekly"),
            Frequency::Monthly => write!(f, "Monthly"),
        }
    }
}