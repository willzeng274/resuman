// #[derive(Debug)]
// pub enum ResumeManagerError {
//     IoError(std::io::Error),
//     ParseError(String),
// }
//
// impl std::fmt::Display for ResumeManagerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             ResumeManagerError::IoError(err) => write!(f, "I/O Error: {}", err),
//             ResumeManagerError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
//         }
//     }
// }
//
// impl From<std::io::Error> for ResumeManagerError {
//     fn from(err: std::io::Error) -> Self {
//         ResumeManagerError::IoError(err)
//     }
// }
