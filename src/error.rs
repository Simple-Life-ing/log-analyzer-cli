use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("IO 错误: {0}")]
    IO(#[from] io::Error),
}
