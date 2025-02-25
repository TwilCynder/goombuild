#[derive(gumdrop::Options, Debug)]
pub struct Options {
    pub help: bool,

    pub out_file: Option<String>,

    pub input_file: Option<String>
}

