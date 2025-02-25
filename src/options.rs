#[derive(gumdrop::Options, Debug)]
pub struct Options {
    pub help: bool,

    #[options(default="./Makefile")]
    pub out_file: String,

    pub input_file: Option<String>
}

