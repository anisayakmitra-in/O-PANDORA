use pandora_types::permissions_manifest::{FilesystemScope, PermissionManifest, ShellPermissions};

fn main() {
    let perm = PermissionManifest {
        filesystem: vec![
            FilesystemScope {
                path: "/tmp".into(),
                read: true,
                write: true,
            },
            FilesystemScope {
                path: ".".into(),
                read: true,
                write: true,
            },
        ],
        shell: ShellPermissions {
            enabled: true,
            blocked: vec!["rm -rf *".into(), "sudo *".into()],
            auto_approved: vec!["ls *".into(), "git status".into()],
            ..Default::default()
        },
        ..Default::default()
    };

    for cmd in &["ls -la", "git status", "sudo rm -rf /"] {
        println!("  {:<30} -> {:?}", cmd, perm.is_shell_allowed(cmd));
    }
    for (path, write) in &[
        ("/tmp/file", true),
        ("/etc/passwd", false),
        ("/root/key", false),
    ] {
        println!(
            "  {:<20} write={} -> {:?}",
            path,
            write,
            perm.is_path_allowed(path, *write)
        );
    }
}
