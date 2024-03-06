# Installation command
[no-cd, private]
install-cmd options src dest:
    install {{options}} {{src}} {{dest}}

[no-cd, private]
install-bin src dest: (install-cmd '-Dm0755' src dest)

[no-cd, private]
install-file src dest: (install-cmd '-Dm0644' src dest)

# Check if required dependencies are installed on the system
dep-check:
    #!/bin/sh
    echo "{{lib-depends}}" | while read -r lib; do
        if [ -n "${lib}" ]; then
            just dep-lib "${lib}" || exit 1
        fi
    done
    echo "{{cmd-depends}}" | while read -r cmd; do
        if [ -n "${cmd}" ]; then
            just dep-cmd "${cmd}" || exit 1
        fi
    done

# Errors if a command does not exist
[private]
dep-cmd cmd:
    @which {{cmd}} >/dev/null || (just print-error 'missing dependency' 'command' {{cmd}}; exit 1)

# Errors if a library does not exist
[private]
dep-lib lib:
    @pkg-config --exists {{lib}} || (just print-error 'missing dependency' 'library' {{lib}}; exit 1)

# Display a formatted error for the user.
[private]
print-error msg key value:
    @echo '\e[0;31mERROR  {{msg}}, \e[1;31m{{key}}: \e[0;31m{{value}}\e[0m'