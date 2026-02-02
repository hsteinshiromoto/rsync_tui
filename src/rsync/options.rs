/// Rsync command options
#[derive(Debug, Clone)]
pub struct RsyncOptions {
    pub archive: bool,        // -a
    pub verbose: bool,        // -v
    pub compress: bool,       // -z
    pub dry_run: bool,        // -n
    pub progress: bool,       // --progress
    pub delete: bool,         // --delete
    pub human_readable: bool, // -h
    pub use_ssh: bool,        // -e ssh
    pub exclude: Vec<String>,
}

impl Default for RsyncOptions {
    fn default() -> Self {
        Self {
            archive: true,
            verbose: true,
            compress: false,
            dry_run: false,
            progress: true,
            delete: false,
            human_readable: true,
            use_ssh: false,
            exclude: Vec::new(),
        }
    }
}

impl RsyncOptions {
    /// Toggle an option by index (0-7)
    pub fn toggle(&mut self, index: usize) {
        match index {
            0 => self.archive = !self.archive,
            1 => self.verbose = !self.verbose,
            2 => self.compress = !self.compress,
            3 => self.dry_run = !self.dry_run,
            4 => self.progress = !self.progress,
            5 => self.delete = !self.delete,
            6 => self.human_readable = !self.human_readable,
            7 => self.use_ssh = !self.use_ssh,
            _ => {}
        }
    }
}
