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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let opts = RsyncOptions::default();

        assert!(opts.archive);
        assert!(opts.verbose);
        assert!(!opts.compress);
        assert!(!opts.dry_run);
        assert!(opts.progress);
        assert!(!opts.delete);
        assert!(opts.human_readable);
        assert!(!opts.use_ssh);
        assert!(opts.exclude.is_empty());
    }

    #[test]
    fn test_toggle_archive() {
        let mut opts = RsyncOptions::default();
        assert!(opts.archive);
        opts.toggle(0);
        assert!(!opts.archive);
        opts.toggle(0);
        assert!(opts.archive);
    }

    #[test]
    fn test_toggle_verbose() {
        let mut opts = RsyncOptions::default();
        assert!(opts.verbose);
        opts.toggle(1);
        assert!(!opts.verbose);
    }

    #[test]
    fn test_toggle_compress() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.compress);
        opts.toggle(2);
        assert!(opts.compress);
    }

    #[test]
    fn test_toggle_dry_run() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.dry_run);
        opts.toggle(3);
        assert!(opts.dry_run);
    }

    #[test]
    fn test_toggle_progress() {
        let mut opts = RsyncOptions::default();
        assert!(opts.progress);
        opts.toggle(4);
        assert!(!opts.progress);
    }

    #[test]
    fn test_toggle_delete() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.delete);
        opts.toggle(5);
        assert!(opts.delete);
    }

    #[test]
    fn test_toggle_human_readable() {
        let mut opts = RsyncOptions::default();
        assert!(opts.human_readable);
        opts.toggle(6);
        assert!(!opts.human_readable);
    }

    #[test]
    fn test_toggle_use_ssh() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.use_ssh);
        opts.toggle(7);
        assert!(opts.use_ssh);
    }

    #[test]
    fn test_toggle_invalid_index() {
        let mut opts = RsyncOptions::default();
        let original = opts.clone();
        opts.toggle(8); // Invalid index
        opts.toggle(100); // Invalid index

        // All values should remain unchanged
        assert_eq!(opts.archive, original.archive);
        assert_eq!(opts.verbose, original.verbose);
        assert_eq!(opts.compress, original.compress);
    }
}
