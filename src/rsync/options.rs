/// Rsync command options
#[derive(Debug, Clone)]
pub struct RsyncOptions {
    pub archive: bool,           // -a
    pub verbose: bool,           // -v
    pub compress: bool,          // -z
    pub dry_run: bool,           // -n
    pub per_file_progress: bool, // --progress
    pub delete: bool,            // --delete
    pub human_readable: bool,    // -h
    pub use_ssh: bool,           // -e ssh
    pub delete_source: bool,     // --remove-source-files
    pub global_progress: bool,   // --info=progress2
    pub exclude: Vec<String>,
}

/// A toggleable option: key binding, display texts, and field accessors.
/// Single source of truth driving toggling, option pills, the help bar,
/// and destructive-run detection.
pub struct OptionDef {
    pub key: char,
    pub label: &'static str,
    pub flag: &'static str,
    pub destructive: bool,
    pub get: fn(&RsyncOptions) -> bool,
    pub get_mut: fn(&mut RsyncOptions) -> &mut bool,
}

pub const OPTIONS: &[OptionDef] = &[
    OptionDef { key: 'a', label: "Archive", flag: "-a", destructive: false, get: |o| o.archive, get_mut: |o| &mut o.archive },
    OptionDef { key: 'v', label: "Verbose", flag: "-v", destructive: false, get: |o| o.verbose, get_mut: |o| &mut o.verbose },
    OptionDef { key: 'z', label: "Compress", flag: "-z", destructive: false, get: |o| o.compress, get_mut: |o| &mut o.compress },
    OptionDef { key: 'n', label: "Dry-run", flag: "-n", destructive: false, get: |o| o.dry_run, get_mut: |o| &mut o.dry_run },
    OptionDef { key: 'p', label: "Progress", flag: "--progress", destructive: false, get: |o| o.per_file_progress, get_mut: |o| &mut o.per_file_progress },
    OptionDef { key: 'd', label: "Delete", flag: "--delete", destructive: true, get: |o| o.delete, get_mut: |o| &mut o.delete },
    OptionDef { key: 'h', label: "Human", flag: "-h", destructive: false, get: |o| o.human_readable, get_mut: |o| &mut o.human_readable },
    OptionDef { key: 'e', label: "SSH", flag: "-e ssh", destructive: false, get: |o| o.use_ssh, get_mut: |o| &mut o.use_ssh },
    OptionDef { key: 'r', label: "DelSrc", flag: "--remove-source-files", destructive: true, get: |o| o.delete_source, get_mut: |o| &mut o.delete_source },
    OptionDef { key: 'f', label: "Global", flag: "--info=progress2", destructive: false, get: |o| o.global_progress, get_mut: |o| &mut o.global_progress },
];

impl Default for RsyncOptions {
    fn default() -> Self {
        Self {
            archive: true,
            verbose: true,
            compress: false,
            dry_run: false,
            per_file_progress: true,
            delete: false,
            human_readable: true,
            use_ssh: false,
            delete_source: false,
            global_progress: false,
            exclude: Vec::new(),
        }
    }
}

impl RsyncOptions {
    /// True when the post-run source cleanup should execute.
    /// Never true during a dry-run: a preview must not touch the filesystem.
    pub fn should_cleanup_source(&self) -> bool {
        self.delete_source && !self.dry_run
    }

    /// Toggle the option bound to `key`; returns false when no option matches
    pub fn toggle_key(&mut self, key: char) -> bool {
        for def in OPTIONS {
            if def.key == key {
                let field = (def.get_mut)(self);
                *field = !*field;
                return true;
            }
        }
        false
    }

    /// True when any enabled option can destroy data
    pub fn has_destructive(&self) -> bool {
        OPTIONS.iter().any(|def| def.destructive && (def.get)(self))
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
        assert!(opts.per_file_progress);
        assert!(!opts.delete);
        assert!(opts.human_readable);
        assert!(!opts.use_ssh);
        assert!(!opts.delete_source);
        assert!(!opts.global_progress);
        assert!(opts.exclude.is_empty());
    }

    #[test]
    fn test_toggle_key_flips_every_option() {
        let mut opts = RsyncOptions::default();

        for def in OPTIONS {
            let before = (def.get)(&opts);
            assert!(opts.toggle_key(def.key), "no toggle for key {}", def.key);
            assert_eq!((def.get)(&opts), !before, "key {} did not flip", def.key);
        }
    }

    #[test]
    fn test_toggle_key_unknown_is_noop() {
        let mut opts = RsyncOptions::default();
        let before = opts.clone();

        assert!(!opts.toggle_key('x'));
        assert_eq!(opts.archive, before.archive);
        assert_eq!(opts.delete, before.delete);
    }

    #[test]
    fn test_options_table_has_unique_keys() {
        let mut keys: Vec<char> = OPTIONS.iter().map(|def| def.key).collect();
        keys.sort_unstable();
        keys.dedup();

        assert_eq!(keys.len(), OPTIONS.len());
    }

    #[test]
    fn test_has_destructive() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.has_destructive());

        opts.delete = true;
        assert!(opts.has_destructive());

        opts.delete = false;
        opts.delete_source = true;
        assert!(opts.has_destructive());
    }

    #[test]
    fn test_should_cleanup_source_requires_delete_source() {
        let mut opts = RsyncOptions::default();
        assert!(!opts.should_cleanup_source());
        opts.delete_source = true;
        assert!(opts.should_cleanup_source());
    }

    #[test]
    fn test_should_cleanup_source_blocked_by_dry_run() {
        let mut opts = RsyncOptions::default();
        opts.delete_source = true;
        opts.dry_run = true;
        assert!(!opts.should_cleanup_source());
    }
}
