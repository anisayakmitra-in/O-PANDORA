use std::path::Path;

use libloading::{
    Library,
};

pub struct RuntimePlugin {

    pub name:
        String,

    pub library:
        Library,
}

pub struct PluginRuntime;

impl PluginRuntime {

    pub unsafe fn load(

        plugin_path:
            impl AsRef<Path>,

    ) -> Result<
        RuntimePlugin,
        String,
    > {

        let path =
            plugin_path
                .as_ref();

        let library =

            Library
                ::new(path)
                .map_err(
                    |error| {

                        error
                            .to_string()
                    }
                )?;

        Ok(
            RuntimePlugin {

                name:
                    path
                        .to_string_lossy()
                        .to_string(),

                library,
            }
        )
    }
}


