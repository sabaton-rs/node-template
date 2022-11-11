#![forbid(unsafe_code)]
use clap::{arg, command};
use {{crate_name}}::{example_node_main,Params};
use tracing::Level;
use tracing_subscriber::prelude::*;

#[cfg(feature = "dlt")]
use dlt_tracing_subscriber::DltLayer;

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom configuration file for {{project-name}}"
            )
            .required(false)
            .allow_invalid_utf8(false),
        )
        .arg(
            arg!(
                -d --debug ... "Turn on debugging"
            )
            .required(false)
        )
        .arg(
            arg!(
                -g --groupname ... "Set the group name"
            )
            .required(false)
        )
        .arg(
            arg!(
                -i --instancename ... "Set the instance name"
            )
            .required(false)
        )
        .arg(
            arg!(
                -f --debugfilter "List of debug spans to filter on. The default is to show all spans"
            )
            .required(false)
        )
        .get_matches();

    let trace_filter = match matches.occurrences_of("debug") {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE
    };

    // Use DLT tracing subscriber if enabled
    #[cfg(feature = "dlt")]
    tracing_subscriber::registry()
        .with(DltLayer::new("EXND","Example Application Node"))
        .init();
    #[cfg(not(feature = "dlt"))]    
    tracing_subscriber::fmt()
    // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
    // will be written to stdout.
    .with_max_level(trace_filter)
    // sets this to be the default, global collector for this application.
    .with_target(true)
    .init();


    /*
        Process the command line arguments here and create a configuration structure that is then passed into the main function.
     */
    let params = Params {
        maybe_group: matches.value_of("groupname"),
        maybe_instance: matches.value_of("instancename"),
    };

    example_node_main(&params).expect("Error running main");
}
