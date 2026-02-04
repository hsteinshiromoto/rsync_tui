/// Rsync command options
#[derive(Debug, Clone)]
pub struct RsyncOptions {
    pub archive: bool,           // -a
    pub verbose: bool,           // -v
    pub compress: bool,          // -z
    pub dry_run: bool,           // -n
    pub progress: bool,          // --progress
    pub delete: bool,            // --delete
    pub human_readable: bool,    // -h
    pub use_ssh: bool,           // -e ssh
    pub delete_source: bool,     // --remove-source-files
    pub delete_excluded: bool,   // --delete-excluded
    pub progress_per_file: bool, // --info=progress2
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
            delete_source: false,
            delete_excluded: false,
            progress_per_file: false,
            exclude: Vec::new(),
        }
    }
}

impl RsyncOptions {
    /// Toggle an option by index (0-10)
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
            8 => self.delete_source = !self.delete_source,
            9 => self.delete_excluded = !self.delete_excluded,
            10 => self.progress_per_file = !self.progress_per_file,
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
        assert!(!opts.delete_source);
        assert!(!opts.delete_excluded);
        assert!(!opts.progress_per_file);
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
    fn test_toggle_delete_source() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.delete_source);
        opts.toggle(8);
        assert!(opts.delete_source);
    }

    #[test]
    fn test_toggle_delete_excluded() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.delete_excluded);
        opts.toggle(9);
        assert!(opts.delete_excluded);
    }

    #[test]
    fn test_toggle_progress_per_file() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.progress_per_file);
        opts.toggle(10);
        assert!(opts.progress_per_file);
    }

    #[test]
    fn test_toggle_invalid_index() {
        let mut opts = RsyncOptions::default();
        let original = opts.clone();
        opts.toggle(11); // Invalid index
        opts.toggle(100); // Invalid index

        // All values should remain unchanged
        assert_eq!(opts.archive, original.archive);
        assert_eq!(opts.verbose, original.verbose);
        assert_eq!(opts.compress, original.compress);
    }
}
