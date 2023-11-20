use std::path::PathBuf;
use anyhow::format_err;
use super::BuildResult;
use crate::config::AndroidConfig;
use crate::ops::build;
use cargo::core::Workspace;
use cargo_util::ProcessBuilder;
use cargo::util::CargoResult;
use clap::ArgMatches;

pub fn install(
    workspace: &Workspace,
    config: &AndroidConfig,
    options: &ArgMatches,
) -> CargoResult<BuildResult> {
    let build_result = build::build(workspace, config, options)?;

    let adb = match which::which("adb") {
        Ok(tool_path) => PathBuf::from(tool_path),
        _ => return Err(format_err!("command not found: adb")),
    };

    for apk_path in build_result.target_to_apk_map.values() {
        drop(writeln!(
            workspace.config().shell().err(),
            "Installing apk '{}' to the device",
            apk_path.file_name().unwrap().to_string_lossy()
        ));

        ProcessBuilder::new(&adb)
            .arg("install")
            .arg("-r")
            .arg(apk_path)
            .exec()?;
    }

    Ok(build_result)
}
