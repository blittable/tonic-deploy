use log::{info, LevelFilter, Log, Metadata, Record};
use rustwide::logging::{self, LogStorage};
use rustwide::{cmd::SandboxBuilder, Crate, Toolchain, WorkspaceBuilder};
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logs();

    // Create a new workspace in .workspaces/tonic_server
    let server_workspace = WorkspaceBuilder::new(Path::new(".workspaces/tonic"), "tonic").init()?;

    // Run on beta toolchain
    let toolchain = Toolchain::Dist {
        name: "beta".into(),
    };

    toolchain.install(&server_workspace)?;

    //Assure rustfmt is installed - required by tonic-build
    toolchain.add_component(&server_workspace, "rustfmt")?;

    //Fetching from a github repository
    //let krate = Crate::git("https://github.com/...");

    //if we are using local code
    let krate = Crate::local(Path::new("../hello-tonic"));

    // Fetch the source
    krate.fetch(&server_workspace)?;

    // Configure a sandbox build with 1GB of RAM and no network access
    let doc_sandbox = SandboxBuilder::new()
        .memory_limit(Some(1024 * 1024 * 1024))
        .enable_networking(true);

    // Test build the doc for the project
    let mut build_dir = server_workspace.build_dir("docs");
    build_dir
        .build(&toolchain, &krate, doc_sandbox)
        .run(|build| {
            build
                .cargo()
                .args(&["doc", "--no-deps"])
                .log_output(true)
                .run()?;
            Ok(())
        })?;

    let mut build_dir = server_workspace.build_dir("hellotonic-server");

    // Configure a sandbox build with 1GB of RAM and no network access
    let server_sandbox = SandboxBuilder::new()
        .memory_limit(Some(1024 * 1024 * 1024))
        .enable_networking(true);

    // Build all code and start the server
    build_dir
        .build(&toolchain, &krate, server_sandbox)
        .run(|build| {
            build.cargo().args(&["build"]).run()?;
            build
                .cargo()
                .args(&["run", "--bin", "hellotonic-server"])
                .log_output(true)
                .run()?;
            Ok(())
        })?;

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
