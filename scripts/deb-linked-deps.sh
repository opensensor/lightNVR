#!/bin/sh
# Print the Debian package dependencies for the FFmpeg and llhttp libraries a
# binary links against, so a .deb can't declare versions it wasn't built with.
#
# Usage: deb-linked-deps.sh <binary>
set -eu

if [ "$#" -ne 1 ] || [ ! -f "$1" ]; then
    echo "Usage: $0 <binary>" >&2
    exit 2
fi

needed=$(readelf -d "$1" | sed -n 's/.*Shared library: \[\(.*\)\]$/\1/p')

# Debian names these packages <library><soname version>.
package_for() {
    printf '%s\n' "$needed" | sed -n "s/^$1\.so\.\([0-9.]*\)$/$1\1/p" | head -n 1
}

deps=""
for lib in libavcodec libavformat libavutil libswscale libswresample libavfilter libllhttp; do
    package=$(package_for "$lib")
    if [ -z "$package" ]; then
        # libavfilter is optional at build time; the rest are required.
        [ "$lib" = libavfilter ] && continue
        echo "ERROR: $1 does not link against $lib" >&2
        exit 1
    fi
    [ "$lib" = libavcodec ] && package="$package | libavcodec-extra"
    deps="${deps:+$deps, }$package"
done

printf '%s\n' "$deps"
