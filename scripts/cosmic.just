rootdir := ''
prefix := '/usr'

# Installation command
[private]
install-cmd options src dest:
    install {{options}} {{src}} {{dest}}

[private]
install-bin src dest: (install-cmd '-Dm0755' src dest)

[private]
install-file src dest: (install-cmd '-Dm0644' src dest)

# Errors if a command does not exist
[private]
dep-cmd cmd:
    @which {{cmd}} >/dev/null || (just print-error 'missing dependency' 'command' {{cmd}}; exit 1)

# Errors if a library does not exist
[private]
dep-lib lib:
    @pkg-config --libs {{lib}} >/dev/null 2>&1 || (just print-error 'missing dependency' 'library' {{lib}}; exit 1)

# Display a formatted error for the user.
[private]
print-error msg key value:
    @echo '\e[0;31mERROR  {{msg}}, \e[1;31m{{key}}: \e[0;31m{{value}}\e[0m'