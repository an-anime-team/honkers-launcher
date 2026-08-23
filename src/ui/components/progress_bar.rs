use relm4::prelude::*;
use adw::prelude::*;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::sophon::installer::Update as SophonInstallerUpdate;
use anime_launcher_sdk::anime_game_core::sophon::updater::Update as SophonPatcherUpdate;

use crate::*;

pub struct ProgressBarInit {
    pub caption: Option<String>,

    /// Add progress percentage (`XX.YY%`) suffix
    pub display_progress: bool,

    /// Add `(XX MB of YY MB)` suffix
    pub display_fraction: bool,

    pub visible: bool
}

pub struct ProgressBar {
    pub fraction: f64,
    pub caption: Option<String>,

    /// e.g. (53.21 MB, 10 GB)
    pub downloaded: Option<(String, String)>,

    /// Add progress percentage (`XX.YY%`) suffix
    pub display_progress: bool,

    /// Add `(XX MB of YY MB)` suffix
    pub display_fraction: bool,

    pub visible: bool
}

#[derive(Debug)]
pub enum ProgressBarMsg {
    Reset,
    UpdateCaption(Option<String>),
    DisplayProgress(bool),
    DisplayFraction(bool),

    /// (current bytes, total bytes)
    UpdateProgress(u64, u64),
    /// (items done, total items)
    UpdateProgressCounter(u64, u64),

    UpdateFromState(InstallerUpdate),
    UpdateFromDiffState(DiffUpdate),
    SetVisible(bool)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ProgressBar {
    type Init = ProgressBarInit;
    type Input = ProgressBarMsg;
    type Output = ();

    view! {
        #[root]
        gtk::ProgressBar {
            set_valign: gtk::Align::Center,

            #[watch]
            set_visible: model.visible,

            #[watch]
            set_fraction: model.fraction,

            #[watch]
            set_show_text: model.caption.is_some(),

            #[watch]
            set_text: Some(&match model.caption.clone() {
                Some(mut caption) => {
                    if model.display_progress {
                        caption = format!("{caption}: {:.2}%", model.fraction * 100.0);
                    }

                    if model.display_fraction {
                        if let Some((curr, total)) = &model.downloaded {
                            caption = format!("{caption} ({curr} of {total})");
                        }
                    }

                    caption
                },
                None => String::new()
            })
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let model = ProgressBar {
            fraction: 0.0,
            caption: init.caption,
            downloaded: None,
            display_progress: init.display_progress,
            display_fraction: init.display_fraction,
            visible: init.visible
        };

        let widgets = view_output!();

        AsyncComponentParts {
            model,
            widgets
        }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            ProgressBarMsg::Reset => {
                self.fraction = 0.0;
                self.downloaded = None;
                self.caption = None;
            }

            ProgressBarMsg::UpdateCaption(caption) => self.caption = caption,
            ProgressBarMsg::DisplayProgress(value) => self.display_progress = value,
            ProgressBarMsg::DisplayFraction(value) => self.display_fraction = value,

            ProgressBarMsg::UpdateProgress(curr, total) => {
                self.fraction = curr as f64 / total as f64;

                self.downloaded = Some((prettify_bytes(curr), prettify_bytes(total)));
            }

            ProgressBarMsg::UpdateProgressCounter(curr, total) => {
                self.fraction = curr as f64 / total as f64;

                self.downloaded = Some((curr.to_string(), total.to_string()))
            }

            ProgressBarMsg::UpdateFromState(state) => match state {
                InstallerUpdate::CheckingFreeSpace(_) => {
                    self.caption = Some(tr!("checking-free-space"))
                }
                InstallerUpdate::DownloadingStarted(_) => self.caption = Some(tr!("downloading")),
                InstallerUpdate::UpdatingPermissionsStarted(_) => {
                    self.caption = Some(tr!("updating-permissions"))
                }
                InstallerUpdate::UnpackingStarted(_) => self.caption = Some(tr!("unpacking")),

                InstallerUpdate::DownloadingProgress(curr, total)
                | InstallerUpdate::UpdatingPermissions(curr, total)
                | InstallerUpdate::UnpackingProgress(curr, total) => {
                    self.fraction = curr as f64 / total as f64;

                    self.downloaded = Some((prettify_bytes(curr), prettify_bytes(total)));
                }

                InstallerUpdate::DownloadingFinished => tracing::info!("Downloading finished"),
                InstallerUpdate::UpdatingPermissionsFinished => {
                    tracing::info!("Updating permissions finished")
                }
                InstallerUpdate::UnpackingFinished => tracing::info!("Unpacking finished"),

                InstallerUpdate::DownloadingError(err) => {
                    tracing::error!("Downloading error: {:?}", err)
                }
                InstallerUpdate::UnpackingError(err) => {
                    tracing::error!("Unpacking error: {:?}", err)
                }
            },

            ProgressBarMsg::UpdateFromDiffState(state) => match state {
                // checking free space
                DiffUpdate::Installer(SophonInstallerUpdate::CheckingFreeSpace(_))
                | DiffUpdate::Patcher(SophonPatcherUpdate::CheckingFreeSpace(_)) => {
                    self.caption = Some(tr!("checking-free-space"))
                }

                // checking files
                DiffUpdate::Installer(SophonInstallerUpdate::CheckingFiles {
                    ..
                })
                | DiffUpdate::Patcher(SophonPatcherUpdate::CheckingFilesStarted) => {
                    self.caption = Some(tr!("verifying-files"));
                    self.display_fraction = false;
                }

                DiffUpdate::Installer(SophonInstallerUpdate::CheckingFilesProgress {
                    passed,
                    total
                }) => self.fraction = passed as f64 / total as f64,

                // download started
                DiffUpdate::Installer(SophonInstallerUpdate::DownloadingStarted {
                    ..
                })
                | DiffUpdate::Patcher(SophonPatcherUpdate::DownloadingStarted(_)) => {
                    self.caption = Some(tr!("downloading"));
                    self.display_fraction = true;
                }

                // download progress
                DiffUpdate::Installer(SophonInstallerUpdate::DownloadingProgressBytes {
                    downloaded_bytes,
                    total_bytes
                })
                | DiffUpdate::Patcher(SophonPatcherUpdate::DownloadingProgressBytes {
                    downloaded_bytes,
                    total_bytes
                }) => {
                    self.fraction = downloaded_bytes as f64 / total_bytes as f64;

                    self.downloaded = Some((
                        prettify_bytes(downloaded_bytes),
                        prettify_bytes(total_bytes)
                    ));
                }

                // finish
                DiffUpdate::Installer(SophonInstallerUpdate::DownloadingFinished)
                | DiffUpdate::Patcher(SophonPatcherUpdate::DownloadingFinished) => {
                    tracing::info!("Downloading finished")
                }
                DiffUpdate::Patcher(SophonPatcherUpdate::DeletingFinished) => {
                    tracing::info!("Finished deleting unused files")
                }
                DiffUpdate::Patcher(SophonPatcherUpdate::PatchingFinished) => {
                    tracing::info!("Patching finished")
                }

                // error
                DiffUpdate::Installer(SophonInstallerUpdate::DownloadingError(err))
                | DiffUpdate::Patcher(SophonPatcherUpdate::DownloadingError(err)) => {
                    tracing::error!("Downloading error: {:?}", err)
                }
                DiffUpdate::Patcher(SophonPatcherUpdate::PatchingError(err)) => {
                    tracing::error!("Patching error: {err}")
                }
                DiffUpdate::Patcher(SophonPatcherUpdate::FileHashCheckFailed(path)) => {
                    tracing::error!("Failed to perform hash check for file {}", path.display())
                }

                // other progress metrics
                DiffUpdate::Installer(SophonInstallerUpdate::DownloadingProgressFiles {
                    downloaded_files,
                    total_files
                }) => {
                    tracing::info!("Downloaded {downloaded_files} files out of {total_files}")
                }
                DiffUpdate::Patcher(SophonPatcherUpdate::PatchingProgress {
                    patched_files,
                    total_files
                }) => {
                    tracing::info!("Patched {patched_files} files out of {total_files}")
                }

                // explicitly ignored
                DiffUpdate::Patcher(SophonPatcherUpdate::DeletingProgress {
                    ..
                })
                | DiffUpdate::Patcher(SophonPatcherUpdate::DeletingStarted)
                | DiffUpdate::Patcher(SophonPatcherUpdate::PatchingStarted) => {}
            },

            ProgressBarMsg::SetVisible(visible) => self.visible = visible
        }
    }
}
