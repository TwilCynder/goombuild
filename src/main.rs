use std::fs;

use yaml_rust2::YamlLoader;

fn main() {
    let res = fs::read_to_string("./test/test.yaml").expect("Couldn't read file");
    let doc = YamlLoader::load_from_str(&res);

    println!("{doc:?}");
}
