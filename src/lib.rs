use zed_extension_api as zed;

struct Json5Extension;

impl zed::Extension for Json5Extension {
    fn new() -> Self {
        Json5Extension
    }
}

zed::register_extension!(Json5Extension);
