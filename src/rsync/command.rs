use super::options::RsyncOptions;

/// Build rsync command from options
pub fn build_command(source: &str, destination: &str, options: &RsyncOptions) -> Vec<String> {
    let mut args = vec!["rsync".to_string()];

    if options.archive {
        args.push("-a".to_string());
    }
    if options.verbose {
        args.push("-v".to_string());
    }
    if options.compress {
        args.push("-z".to_string());
    }
    if options.dry_run {
        args.push("-n".to_string());
    }
    if options.progress {
        args.push("--progress".to_string());
    }
    if options.delete {
        args.push("--delete".to_string());
    }
    if options.human_readable {
        args.push("-h".to_string());
    }
    if options.use_ssh {
        args.push("-e".to_string());
        args.push("ssh".to_string());
    }
    if options.delete_source {
        args.push("--remove-source-files".to_string());
    }
    if options.delete_excluded {
        args.push("--delete-excluded".to_string());
    }
    if options.progress_per_file {
        args.push("--info=progress2".to_string());
    }

    for pattern in &options.exclude {
        args.push("--exclude".to_string());
        args.push(pattern.clone());
    }

    args.push(source.to_string());
    args.push(destination.to_string());

    args
}

/// Format command as display string
pub fn format_command(source: &str, destination: &str, options: &RsyncOptions) -> String {
    build_command(source, destination, options).join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = RsyncOptions::default();
        let cmd = build_command("/src", "/dest", &opts);

        assert!(cmd.contains(&"-a".to_string()));
        assert!(cmd.contains(&"-v".to_string()));
        assert!(cmd.contains(&"--progress".to_string()));
        assert!(cmd.contains(&"-h".to_string()));
        assert!(!cmd.contains(&"-z".to_string()));
        assert!(!cmd.contains(&"-n".to_string()));
    }

    #[test]
    fn test_source_destination_appended() {
        let opts = RsyncOptions::default();
        let cmd = build_command("/home/user", "server:/backup", &opts);

        assert_eq!(cmd[cmd.len() - 2], "/home/user");
        assert_eq!(cmd[cmd.len() - 1], "server:/backup");
    }

    #[test]
    fn test_ssh_option() {
        let mut opts = RsyncOptions::default();
        opts.use_ssh = true;
        let cmd = build_command("/src", "/dest", &opts);

        let ssh_idx = cmd.iter().position(|x| x == "-e").unwrap();
        assert_eq!(cmd[ssh_idx + 1], "ssh");
    }

    #[test]
    fn test_exclude_patterns() {
        let mut opts = RsyncOptions::default();
        opts.exclude = vec!["*.log".to_string(), "tmp/".to_string()];
        let cmd = build_command("/src", "/dest", &opts);

        assert!(cmd.contains(&"--exclude".to_string()));
        assert!(cmd.contains(&"*.log".to_string()));
        assert!(cmd.contains(&"tmp/".to_string()));
    }

    #[test]
    fn test_dry_run_flag() {
        let mut opts = RsyncOptions::default();
        opts.dry_run = true;
        let cmd = build_command("/src", "/dest", &opts);

        assert!(cmd.contains(&"-n".to_string()));
    }

    #[test]
    fn test_format_command() {
        let mut opts = RsyncOptions::default();
        opts.archive = true;
        opts.verbose = false;
        opts.progress = false;
        opts.human_readable = false;
        let formatted = format_command("/src", "/dest", &opts);

        assert_eq!(formatted, "rsync -a /src /dest");
    }

    #[test]
    fn test_all_options_disabled() {
        let opts = RsyncOptions {
            archive: false,
            verbose: false,
            compress: false,
            dry_run: false,
            progress: false,
            delete: false,
            human_readable: false,
            use_ssh: false,
            delete_source: false,
            delete_excluded: false,
            progress_per_file: false,
            exclude: vec![],
        };
        let cmd = build_command("/src", "/dest", &opts);

        assert_eq!(cmd, vec!["rsync", "/src", "/dest"]);
    }

    #[test]
    fn test_delete_source_flag() {
        let mut opts = RsyncOptions::default();
        opts.delete_source = true;
        let cmd = build_command("/src", "/dest", &opts);

        assert!(cmd.contains(&"--remove-source-files".to_string()));
    }

    #[test]
    fn test_delete_excluded_flag() {
        let mut opts = RsyncOptions::default();
        opts.delete_excluded = true;
        let cmd = build_command("/src", "/dest", &opts);

        assert!(cmd.contains(&"--delete-excluded".to_string()));
    }

    #[test]
    fn test_progress_per_file_flag() {
        let mut opts = RsyncOptions::default();
        opts.progress_per_file = true;
        let cmd = build_command("/src", "/dest", &opts);

        assert!(cmd.contains(&"--info=progress2".to_string()));
    }
}
