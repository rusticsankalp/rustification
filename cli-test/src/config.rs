#[derive(PartialEq,Debug)]
pub enum Loglevel {
    Info,
    Debug,
    Error,
}

#[derive(PartialEq,Debug)]
pub enum LogOut {
    Stdout,
    Stderr,
}


/// This struct is for controlloing logging
/// It has two fields level and output
/// level is of type Loglevel and output is of type LogOut
/// '''
/// let config = LogginConfig {
///    level: Loglevel::Info,
///   output: LogOut::Stdout,
/// };
/// '''

#[derive(PartialEq,Debug)]
pub struct LogginConfig {
    pub level: Loglevel,
    pub output: LogOut,
}

impl LogginConfig{
    /// This function returns a new instance of LogginConfig
    /// It takes two arguments level and output
    /// '''
    /// let config = LogginConfig::new(Loglevel::Info, LogOut::Stdout);
    /// '''
    pub fn new(level: Loglevel, output: LogOut) -> Self {
        LogginConfig {
            level,
            output,
        }
    }    
}
