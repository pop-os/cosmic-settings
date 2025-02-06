use users::os::unix::GroupExt;

/// Check if the user is an administrator on this system.
pub fn is_admin() -> bool {
    users::get_current_username().map_or(false, |username| {
        username == "root" || in_admin_group(&username)
    })
}

/// Check if a user is in an administrative group, such as `adm` or `sudo`.
fn in_admin_group(user: &std::ffi::OsStr) -> bool {
    let in_group = |name| {
        users::get_group_by_name(name).map_or(false, |group| {
            group
                .members()
                .iter()
                .any(|member| member.as_os_str() == user)
        })
    };

    ["adm", "sudo"]
        .into_iter()
        .filter(|g| !g.is_empty())
        .any(in_group)
}
