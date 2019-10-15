//! Build tonic and a minimal client-server conversation.
use rustwide::{cmd::Command, cmd::SandboxBuilder, Crate, Toolchain, WorkspaceBuilder};

use failure::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    setup_logs();

    // Create a new workspace in .workspaces/tonic_server
    let server_workspace = WorkspaceBuilder::new(Path::new(".workspaces/tonic"), "tonic")
        .running_inside_docker(true)
        .init()?;

    // Run on beta toolchain
    let toolchain = Toolchain::Dist {
        name: "beta".into(),
    };

    toolchain.install(&server_workspace)?;

    //Assure rustfmt is installed - required by tonic-build
    toolchain.add_component(&server_workspace, "rustfmt")?;

    //If fetching from a github repository
    //let krate = Crate::git("https://github.com/...");

    //if we are using local code
    let krate = Crate::local(Path::new("../hello-tonic"));

    // Fetch the source
    krate.fetch(&server_workspace)?;

    let mut build_dir = server_workspace.build_dir("hellotonic-client-server");

    // Configure a sandbox build with 1GB of RAM and network access
    let server_sandbox = SandboxBuilder::new()
        .memory_limit(Some(1024 * 1024 * 1024))
        .enable_networking(true);

    // In the sandbox, start the server and client, passing a minimal message
    let message_sink = || -> Result<Vec<String>, Error> {
        Ok(build_dir
            .build(&toolchain, &krate, server_sandbox)
            .run(|build| {
                Ok(Command::new(&server_workspace, "cargo")
                    .args(&["run", "--bin", "hellotonic-pair"])
                    .cd(build.host_source_dir())
                    .run_capture()?
                    .stdout_lines()
                    .to_owned())
            })?)
    };

    //Calc stats on the run
    for l in message_sink()? {
        println!("Message {}", l);
    }

    Ok(())
}

fn setup_logs() {
    let mut env = env_logger::Builder::new();
    env.filter_module("rustwide", log::LevelFilter::Info);
    if let Ok(content) = std::env::var("RUST_LOG") {
        env.parse_filters(&content);
    }
    rustwide::logging::init_with(env.build());
}
