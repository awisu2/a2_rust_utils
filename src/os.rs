use std::env::consts;
use std::process::Command;

const ENV_CONSTS_OS_LINUX: &str = "linux";
// const ENV_CONSTS_OS_MACOS: &str = "macos";
// const ENV_CONSTS_OS_IOS: &str = "ios";
// const ENV_CONSTS_OS_FREEBSD: &str = "freebsd";
// const ENV_CONSTS_OS_DRAGONFLY: &str = "dragonfly";
// const ENV_CONSTS_OS_NETBSD: &str = "netbsd";
// const ENV_CONSTS_OS_OPENBSD: &str = "openbsd";
// const ENV_CONSTS_OS_SOLARIS: &str = "solaris";
// const ENV_CONSTS_OS_ANDROID: &str = "android";
const ENV_CONSTS_OS_WINDOWS: &str = "windows";

pub fn open_filer(dir: &str) -> Result<(), String> {
    match consts::OS {
        ENV_CONSTS_OS_WINDOWS => Command::new("explorer").arg(dir).spawn(),
        // TODO: check
        ENV_CONSTS_OS_LINUX => Command::new("xdg-open").arg(dir).spawn(),
        &_ => return Ok(()),
    }
    .map_err(|e| e.to_string())?;

    Ok(())
}
