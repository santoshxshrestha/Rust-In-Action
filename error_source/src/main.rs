use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeroErrors {
    Msg(String),
    #[source]
    Source(std::io::Error),
}

fn main() {}
