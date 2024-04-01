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
    #!/bin/sh
    if echo {{lib}} | grep = >/dev/null; then
        name=$(echo {{lib}} | cut -d= -f1)
        version=$(echo {{lib}} | cut -d= -f2)
        pkg-config --exists ${name} --atleast-version ${version} || (
            has=$(pkg-config --modversion ${name})
            just print-error "expected version ${version}, found ${has}" 'library' ${name}; exit 1)
    else
        pkg-config --exists {{lib}} || (just print-error 'missing dependency' 'library' {{lib}}; exit 1)
    fi

# Display a formatted error for the user.
[private]
print-error msg key value:
    @echo '\e[0;31mERROR  {{msg}}, \e[1;31m{{key}}: \e[0;31m{{value}}\e[0m'