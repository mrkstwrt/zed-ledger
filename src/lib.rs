use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct LedgerExtension {
    cached_binary_path: Option<String>,
}

impl LedgerExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "claytonrcarter/ledger-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let version = release.version.replace("v", "");
        let (platform, arch) = zed::current_platform();

        let asset_name = format!(
            "ledger-language-server-{version}-{os}-{arch}",
            os = match platform {
                zed::Os::Mac => "apple",
                zed::Os::Linux => "linux",
                zed::Os::Windows =>
                    return Err(
                        "unsupported os: ledger-language-server is not released for windows"
                            .to_string()
                    ),
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 => "x86_64",
                zed::Architecture::X86 =>
                    return Err(
                        "unsupported arch: ledger-language-server is not released for x86"
                            .to_string()
                    ),
            },
        );
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("ledger-language-server-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;

        if !fs::metadata(&asset_name).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &asset_name,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|err| format!("failed to download file: {err}"))?;

            zed::make_file_executable(&asset_name)?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;
            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(asset_name.clone());
        Ok(asset_name)
    }
}

impl zed::Extension for LedgerExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        LedgerExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        #[allow(dead_code)]
        enum Binary {
            // Helpful for testing local builds of the language server.
            // false = release build, true = debug build (w/ addl logging)
            Local(bool),

            Release,
        }
        let build = Binary::Release;

        let command = if let Binary::Local(debug) = build {
            let home = worktree
                .shell_env()
                .iter()
                .filter_map(|env| match env.0.as_str() {
                    "HOME" => Some(env.1.clone()),
                    _ => None,
                })
                .nth(0)
                .ok_or("couldn't find $HOME in env")?;
            format!(
                "{home}/src/ledger-language-server/target/{debug}/ledger-language-server",
                debug = if debug { "debug" } else { "release" }
            )
        } else {
            self.language_server_binary_path(language_server_id)?
        };

        Ok(zed::Command {
            command,
            args: vec!["lsp".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(LedgerExtension);
