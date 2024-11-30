use crate::{
    error::RytmExternalError,
    file::{FilePathExt, RytmProjectFileType},
    traits::Post,
    types::{SaveTarget, SaveTargetIndex},
    RytmExternal,
};
use camino::Utf8PathBuf;
use median::{atom::Atom, object::MaxObj, symbol::SymbolRef};
use rytm_object::value::RytmValue;
use rytm_rs::SysexCompatible;
use tracing::{debug, error, instrument, warn};

impl RytmExternal {
    #[instrument]
    pub fn validate_and_get_save_target_index(
        target: SaveTarget,
        index: isize,
        min: usize,
        max: usize,
    ) -> Result<usize, RytmExternalError> {
        let index = index as usize;

        if index < min || index >= max {
            return Err(RytmExternalError::Custom(format!(
                "Save Error: Index out of bounds. {index} not in range {min}..{max} for {target}."
            )))
            .inspect_err(|err| {
                error!("{}", err);
            });
        }

        Ok(index)
    }

    #[instrument]
    pub fn expect_our_file_types(
        ext: Option<&str>,
    ) -> Result<RytmProjectFileType, RytmExternalError> {
        let Ok(Some(file_type)) = ext.map(str::parse).transpose() else {
            return Err(RytmExternalError::from(
                "File Error: Invalid file type. Only .rytm or .sysex files are allowed.",
            ))
            .inspect_err(|err| {
                error!("{}", err);
            });
        };

        Ok(file_type)
    }
}

impl RytmExternal {
    #[instrument(skip_all, fields(path = tracing::field::Empty))]
    pub fn load(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let span = tracing::Span::current();
        let maybe_path_symbol = atoms
            .last()
            .map(median::atom::Atom::get_symbol)
            .unwrap_or_default();
        let maybe_path = self
            .make_utf8_path_buf_respect_tilde(&maybe_path_symbol.to_string().unwrap_or_default());

        let path_is_provided = !maybe_path.as_str().is_empty();
        if path_is_provided {
            Self::expect_our_file_types(maybe_path.extension())?;
        }

        span.record("path", maybe_path.as_str());
        debug!("Trying to load from: {}.", maybe_path);

        // Check existence and file-ness.
        let path_exists = self
            .make_utf8_path_buf_respect_tilde(maybe_path.as_str())
            .exists();
        let path_is_file = self
            .make_utf8_path_buf_respect_tilde(maybe_path.as_str())
            .is_file();
        let path = if path_exists && path_is_file {
            maybe_path_symbol
        } else {
            self.send_status_warning();
            let warning = "Load Warning: The path provided does not exist or it is not a file. Opening dialog.";
            warning.obj_warn(self.max_obj());
            warn!("{}", warning);
            SymbolRef::default()
        };

        let Ok(file) = median::file::FilePath::find_with_dialog(&path, None).ok_or_else(|| {
            debug!("User cancelled open dialog.");
        }) else {
            return Ok(());
        };

        let file_name = file.file_name.to_string_lossy();
        let file_name_camino = Utf8PathBuf::from(file_name.as_ref());
        let maybe_ext = file_name_camino.extension();
        let file_type = Self::expect_our_file_types(maybe_ext)?;

        debug!("Loading project part from: {}.", file_name);

        match file_type {
            RytmProjectFileType::Sysex => {
                let absolute_path = file
                    .to_absolute_system_path()
                    .ok_or_else(|| {
                        RytmExternalError::from("Load Error: Failed to get absolute path.")
                    })?
                    .to_string_lossy()
                    .to_string();

                let Ok(bytes) = std::fs::read(&absolute_path) else {
                    return Err(RytmExternalError::from(
                        "Load Error: Failed to read sysex file.",
                    ))
                    .inspect_err(|err| {
                        error!("{}", err);
                    });
                };

                debug!("Sysex file loaded into memory.");

                // Because this load will load the file into the exact place where it was before.
                // If it was kit 2 then it will be kit 2 again. We can not change that.
                // TODO: If we implement copy and pasting with some sysex magic we can extend this behaviour.

                self.inner
                    .project
                    .lock()
                    .update_from_sysex_response(&bytes)
                    .map_err(|err| {
                        RytmExternalError::from(format!(
                            "Load Error: Failed to parse sysex file: {err:?}"
                        ))
                    })
                    .inspect_err(|err| {
                        error!("{}", err);
                    })?;

                debug!("Project part loaded from {} (sysex).", file_name);
            }
            RytmProjectFileType::Rytm => {
                let Ok(project_text) = file.read_text(median::file::TextLineBreak::Native, None)
                else {
                    return Err(RytmExternalError::from(
                        "Load Error: Failed to read project.",
                    ))
                    .inspect_err(|err| {
                        error!("{}", err);
                    });
                };

                debug!(
                    "Complete project loaded as text from: {}.",
                    file.file_name.to_string_lossy()
                );

                let project_text = project_text.to_str()?;

                let project: rytm_rs::RytmProject =
                    rytm_rs::RytmProject::try_from_str(project_text)
                        .map_err(|err| {
                            RytmExternalError::from(format!(
                                "Load Error: Failed to parse project: {err:?}"
                            ))
                        })
                        .inspect_err(|err| {
                            error!("{}", err);
                        })?;

                debug!("Complete project parsed.");

                *self
                    .inner
                    .project
                    .try_lock_for(std::time::Duration::from_secs(5))
                    .ok_or_else(|| {
                        RytmExternalError::from(
                            "Load Error: rytm is busy try again after some time.",
                        )
                    })
                    .inspect_err(|err| {
                        error!("{}", err);
                    })? = project;

                debug!("Complete project loaded (rytm).");
            }
        }
        self.send_status_success();
        Ok(())
    }

    // In save people should specify the file name.
    #[instrument(skip_all, fields(args = tracing::field::Empty))]
    pub fn save(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let span = tracing::Span::current();

        let values = self.get_rytm_values(atoms)?;
        span.record("args", format!("{values:?}"));
        let mut values_f = values.iter().peekable();
        let mut values_b = values.iter().peekable().rev();
        match values_f.peek() {
            Some(RytmValue::Symbol(path_or_save_target)) => {
                if values.len() == 1 {
                    // Consider this as a file path or user didn't pass anything in, act accordingly.
                    let maybe_valid_path =
                        self.make_utf8_path_buf_respect_tilde(path_or_save_target);
                    let save_file_type = if path_or_save_target.is_empty() {
                        RytmProjectFileType::Rytm
                    } else {
                        Self::expect_our_file_types(maybe_valid_path.extension())?
                    };
                    return match save_file_type {
                    RytmProjectFileType::Sysex => {
                        Err(RytmExternalError::from("Save Error: No save target or index found for a partial save through sysex. Either change your extension to .rytm or provide a save target and an index. Example: save kit 1 ~/Desktop/my_kit.sysex")).inspect_err(|err| error!("{}", err))
                    }
                    RytmProjectFileType::Rytm => {
                        // This might be a valid case..
                        let has_a_valid_parent = maybe_valid_path.parent().is_some_and(camino::Utf8Path::exists);
                        debug!("Path: {} has a valid parent: {}. And is empty: {}.",maybe_valid_path, has_a_valid_parent, path_or_save_target.is_empty());

                        let path = if has_a_valid_parent && !path_or_save_target.is_empty(){
                            maybe_valid_path
                        }
                        else {
                            match self.pick_from_save_dialog_for_entire_proj(maybe_valid_path, path_or_save_target.is_empty()) {
                                Ok(path) => path,
                                Err(err) => {
                                    if matches!(err, RytmExternalError::EarlyExitWithOk) {
                                        return Ok(());
                                    }
                                    return Err(err).inspect_err(|err| error!("{}", err));
                                }
                            }
                        };
                        self.save_entire_project(&path)
                    }
                };
                } else if values.len() > 1 && values.len() <= 3 {
                    // Check if the last argument is path like and has an extension.
                    if let RytmValue::Symbol(maybe_path) = values_b.next().unwrap() {
                        let maybe_valid_path = self.make_utf8_path_buf_respect_tilde(maybe_path);
                        if let Some(ext) = maybe_valid_path.extension() {
                            if ext.parse::<RytmProjectFileType>().is_err() {
                                return Err(RytmExternalError::from("File Error: Invalid file type. Only .rytm or .sysex files are allowed.")).inspect_err(|err| error!("{}", err));
                            }
                        }
                    }

                    let maybe_save_target = values_f.next();
                    let maybe_index = values_f.peek().copied().cloned();
                    #[allow(clippy::unnested_or_patterns)]
                let (save_target, index) = match  (maybe_save_target, &maybe_index) {
                    (Some(RytmValue::Symbol(maybe_save_target)), None | Some(RytmValue::Float(_)) | Some(RytmValue::Symbol(_))) => {
                        let save_target = maybe_save_target.parse::<SaveTarget>().inspect_err(|err| error!("{}", err))?;
                        let index = match save_target {
                            SaveTarget::Settings =>  {
                                Ok::<SaveTargetIndex, RytmExternalError>(SaveTargetIndex::NotNecessary)
                            },
                            SaveTarget::NotProvided => {
                                Ok::<SaveTargetIndex, RytmExternalError>(SaveTargetIndex::Ignore)
                            },
                            _ => {
                               return Err(RytmExternalError::from("Save Error: Invalid arguments the save target and index is not in their proper places for partial project saving (sysex).")).inspect_err(|err| error!("{}", err))?;
                            }
                        }?;

                        Ok((save_target, index))
                    }
                    (Some(RytmValue::Symbol(maybe_save_target)), Some(RytmValue::Int(maybe_valid_index))) =>  {
                        // TODO:
                        let save_target = maybe_save_target.parse::<SaveTarget>().inspect_err(|err| error!("{}", err))?;
                        let index = match save_target  {
                                SaveTarget::Pattern => Self::validate_and_get_save_target_index(save_target, *maybe_valid_index, 0, 127).map(|index| {
                                   SaveTargetIndex::Some(index)
                                }),
                                SaveTarget::Kit => Self::validate_and_get_save_target_index(save_target, *maybe_valid_index, 0, 127).map(|index| {
                                   SaveTargetIndex::Some(index)
                                }),
                                SaveTarget::Sound => Self::validate_and_get_save_target_index(save_target, *maybe_valid_index, 0, 127).map(|index| {
                                    SaveTargetIndex::Some(index)
                                }),
                                SaveTarget::Global => Self::validate_and_get_save_target_index(save_target, *maybe_valid_index, 0, 3).map(|index| {
                                    SaveTargetIndex::Some(index)
                                }),
                                SaveTarget::Settings => Ok::<SaveTargetIndex, RytmExternalError>(SaveTargetIndex::NotNecessary),
                                SaveTarget::NotProvided => Ok::<SaveTargetIndex, RytmExternalError>(SaveTargetIndex::Ignore),
                        }?;
                        values_f.next();
                        Ok((save_target, index))
                    }
                    _ => unreachable!()
                }.inspect_err(|err: &RytmExternalError| error!("{}", err))?;

                    if let Some(RytmValue::Symbol(must_be_path)) = values_f.next() {
                        // Consider this as a file path or user didn't pass anything in, act accordingly.
                        let maybe_valid_path = self.make_utf8_path_buf_respect_tilde(must_be_path);
                        let save_file_type = if must_be_path.is_empty() {
                            None
                        } else {
                            Some(Self::expect_our_file_types(maybe_valid_path.extension())?)
                        };

                        let path = match save_file_type {
                    None => {
                        match self.pick_from_save_dialog_for_partial_proj(save_target, index)  {
                            Ok(path) => Ok(path),
                            Err(err) => {
                                if matches!(err, RytmExternalError::EarlyExitWithOk) {
                                    return Ok(());
                                }
                                return Err(err).inspect_err(|err| error!("{}", err));
                            }
                        }
                    }
                    Some(RytmProjectFileType::Sysex) => {
                        // Save directly to the path after some checks.
                        let has_a_valid_parent = maybe_valid_path.parent().is_some_and(camino::Utf8Path::exists);

                        if !has_a_valid_parent {
                            return Err(RytmExternalError::from("Save Error: Please provide a valid file path or keep the path part empty to choose from the dialog.")).inspect_err(|err| error!("{}", err));
                        }

                        Ok(maybe_valid_path)
                    }
                    _ => {
                        Err(RytmExternalError::from("Save Error: Invalid file type. Since you've provided 3 arguments a partial save with a .sysex file is suitable.")).inspect_err(|err| error!("{}", err))
                    }
                }?;

                        return self.save_partial_project(
                            &path,
                            save_target,
                            index,
                            maybe_index.as_ref(),
                        );
                    }

                    let path = match self.pick_from_save_dialog_for_partial_proj(save_target, index)
                    {
                        Ok(path) => path,
                        Err(err) => {
                            if matches!(err, RytmExternalError::EarlyExitWithOk) {
                                return Ok(());
                            }
                            return Err(err).inspect_err(|err| error!("{}", err));
                        }
                    };

                    return self.save_partial_project(
                        &path,
                        save_target,
                        index,
                        maybe_index.as_ref(),
                    );
                }

                let path = match self
                    .pick_from_save_dialog_for_entire_proj(camino::Utf8PathBuf::default(), true)
                {
                    Ok(path) => path,
                    Err(err) => {
                        if matches!(err, RytmExternalError::EarlyExitWithOk) {
                            return Ok(());
                        }
                        return Err(err).inspect_err(|err| error!("{}", err));
                    }
                };

                return self.save_entire_project(&path);
            }
            None => {
                let path = match self
                    .pick_from_save_dialog_for_entire_proj(Utf8PathBuf::default(), true)
                {
                    Ok(path) => path,
                    Err(err) => {
                        if matches!(err, RytmExternalError::EarlyExitWithOk) {
                            return Ok(());
                        }
                        return Err(err).inspect_err(|err| error!("{}", err));
                    }
                };

                self.save_entire_project(&path)
            }
            _ => Err(RytmExternalError::from(
                "Save Error: Invalid arguments. Please check the structure of your command.",
            ))
            .inspect_err(|err| error!("{}", err)),
        }
    }

    #[instrument(skip(self))]
    pub fn pick_from_save_dialog_for_entire_proj(
        &self,
        mut maybe_valid_path: camino::Utf8PathBuf,
        path_was_empty: bool,
    ) -> Result<camino::Utf8PathBuf, RytmExternalError> {
        if !path_was_empty {
            // Now since here this is not the case open the dialog warning the user
            let warning = "Save Warning: File path is not valid for this context. Opening dialog.";
            self.send_status_warning();
            warning.obj_warn(self.max_obj());
            warn!("{}", warning);
        }

        if !path_was_empty {
            maybe_valid_path.set_extension("rytm");
        }

        let default_file_name = maybe_valid_path.file_name().unwrap_or("project.rytm");

        let Ok(file) =
            median::file::FilePath::save_dialog(default_file_name, None).ok_or_else(|| {
                debug!("User cancelled save dialog.");
            })
        else {
            // Manage early exit.
            return Err(RytmExternalError::EarlyExitWithOk);
        };

        let absolute_saving_path = file
            .to_absolute_system_path()
            .ok_or_else(|| RytmExternalError::from("Save Error: Failed to get absolute path."))
            .inspect_err(|err| error!("{}", err))?
            .to_string_lossy()
            .to_string();

        let camino_absolute_path = Utf8PathBuf::from(absolute_saving_path);
        let file_type = Self::expect_our_file_types(camino_absolute_path.extension())?;

        if file_type != RytmProjectFileType::Rytm {
            return Err(RytmExternalError::from(
                "Save Error: Invalid file type. For this case  a .rytm file is suitable.",
            ))
            .inspect_err(|err| error!("{}", err));
        }

        Ok(camino_absolute_path)
    }

    #[instrument(skip(self))]
    pub fn pick_from_save_dialog_for_partial_proj(
        &self,
        save_target: SaveTarget,
        index: SaveTargetIndex,
    ) -> Result<camino::Utf8PathBuf, RytmExternalError> {
        let warning = "Save Warning: File does not exist or it is not a file. Opening dialog.";
        self.send_status_warning();
        warning.obj_warn(self.max_obj());
        warn!("{}", warning);

        let default_file_name = match (save_target, index) {
            (SaveTarget::Pattern, SaveTargetIndex::Some(index)) => format!("pattern_{index}.sysex"),
            (SaveTarget::Kit, SaveTargetIndex::Some(index)) => format!("kit_{index}.sysex"),
            (SaveTarget::Sound, SaveTargetIndex::Some(index)) => format!("sound_{index}.sysex"),
            (SaveTarget::Global, SaveTargetIndex::Some(index)) => format!("global_{index}.sysex"),
            (SaveTarget::Settings, SaveTargetIndex::NotNecessary) => "settings.sysex".to_owned(),
            (SaveTarget::NotProvided, SaveTargetIndex::Ignore) => "project.rytm".to_owned(),
            _ => {
                return Err(RytmExternalError::from("Save Error: Invalid save target and index combination please check the structure of your message."));
            }
        };

        let Ok(file) =
            median::file::FilePath::save_dialog(&default_file_name, None).ok_or_else(|| {
                debug!("User cancelled save dialog.");
            })
        else {
            // Manage early exit.
            return Err(RytmExternalError::EarlyExitWithOk);
        };

        let absolute_path = file
            .to_absolute_system_path()
            .ok_or_else(|| RytmExternalError::from("Save Error: Failed to get absolute path."))
            .inspect_err(|err| error!("{}", err))?
            .to_string_lossy()
            .to_string();

        let camino_absolute_path = Utf8PathBuf::from(&absolute_path);
        let file_type = Self::expect_our_file_types(camino_absolute_path.extension())?;

        if file_type != RytmProjectFileType::Sysex {
            return Err(RytmExternalError::from(
                "Save Error: Invalid file type. For this case  a .sysex file is suitable.",
            ))
            .inspect_err(|err| error!("{}", err));
        }

        Ok(camino_absolute_path)
    }

    #[instrument(skip(self))]
    pub fn save_entire_project(&self, path: &camino::Utf8PathBuf) -> Result<(), RytmExternalError> {
        debug!("Saving complete project to: {}.", path);

        let project_text = self
            .inner
            .project
            .try_lock_for(std::time::Duration::from_secs(5))
            .ok_or_else(|| {
                RytmExternalError::from("Save Error: rytm is busy try again after some time.")
            })
            .inspect_err(|err| error!("{}", err))?
            .try_to_string()
            .map_err(|err| {
                RytmExternalError::from(format!(
                    "Save Error: Failed to serialize project for saving: {err:?}"
                ))
            })
            .inspect_err(|err| error!("{}", err))?;

        std::fs::write(path, project_text)
            .map_err(|err| {
                RytmExternalError::from(format!(
                    "Save Error: Failed to write project to file {path}: {err:?}"
                ))
            })
            .inspect(|()| {
                self.send_status_success();
                debug!("Project saved to: {}.", path);
            })
            .inspect_err(|err| error!("{}", err))
    }

    #[instrument(skip(self))]
    pub fn save_partial_project(
        &self,
        path: &camino::Utf8PathBuf,
        save_target: SaveTarget,
        index: SaveTargetIndex,
        maybe_index: Option<&RytmValue>,
    ) -> Result<(), RytmExternalError> {
        debug!("Saving project part to: {}. Index: {}", path, index);
        let payload = match (save_target, index) {
            (SaveTarget::Pattern, SaveTargetIndex::Some(index)) => self.inner.project.lock().patterns()[index].as_sysex(),
            (SaveTarget::Kit, SaveTargetIndex::Some(index)) =>self.inner.project.lock().kits()[index].as_sysex(),
            (SaveTarget::Sound, SaveTargetIndex::Some(index)) => self.inner.project.lock().pool_sounds()[index].as_sysex(),
            (SaveTarget::Global, SaveTargetIndex::Some(index)) => self.inner.project.lock().globals()[index].as_sysex(),
            (SaveTarget::Settings, SaveTargetIndex::NotNecessary) if matches!(maybe_index, Some(RytmValue::Float(_))) || matches!(maybe_index, Some(RytmValue::Int(_))) =>  {
                let warning = "Save Warning: Index is not necessary for settings. Ignoring index.";
                self.send_status_warning();
                warning.obj_warn(self.max_obj());
                warn!("{}",warning);
                self.inner.project.lock().settings().as_sysex()
            }
            (SaveTarget::Settings, SaveTargetIndex::NotNecessary) => self.inner.project.lock().settings().as_sysex(),
            _ => {
                return Err(RytmExternalError::from("Save Error: Invalid save target and index combination please check the structure of your message.")).inspect_err(|err| error!("{}", err));
            }
        }.map_err(
            |err| {
                RytmExternalError::from(format!("Save Error: Failed to serialize project part for saving: {err:?}"))
            }).inspect_err(|err| error!("{}", err))?;

        std::fs::write(path, payload)
            .map_err(|err| {
                RytmExternalError::from(format!(
                    "Save Error: Failed to write project part to file {path}: {err:?}"
                ))
            })
            .inspect(|()| {
                self.send_status_success();
                debug!("Project part saved to: {}.", path);
            })
            .inspect_err(|err| error!("{}", err))
    }
}
