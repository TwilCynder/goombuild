use crate::override_yaml::Overrides;

#[derive(gumdrop::Options, Debug)]
pub struct Options {
    pub help: bool,

    pub out_file: Option<String>,

    pub input_file: Option<String>,

    #[options(multi="add", long="config-override")]
    pub config_overrides: Overrides
}

