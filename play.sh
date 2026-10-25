#!/usr/bin/env bash
# alexandre raduly <alexander14k28@gmail.com>
# update 25 oct 2026
# AGPL-3.0

RELEASE="--release"
LINUX_TARGET="x86_64-unknown-linux-musl"
WINDOWS_TARGET="x86_64-pc-windows-gnu"

main_function() {
    echo "b -- build"
    echo "c -- clean"
    read -rp "> " choice

    case "$choice" in
        b)
            echo "1 -- linux"
            echo "2 -- windows"
            read -rp "> " platform

            case "$platform" in
                1)
                    TARGET="$LINUX_TARGET"
                    EXT=""
                    ;;
                2)
                    TARGET="$WINDOWS_TARGET"
                    EXT=".exe"
                    ;;
                *)
                    echo "invalid platform. exiting."
                    exit 1
                    ;;
            esac

            if cargo build $RELEASE --target "$TARGET"; then
                mkdir -p bin

                BIN_NAME=$(cargo metadata --no-deps --format-version 1 |
                    sed -n 's/.*"name":"\([^"]*\)".*/\1/p' | head -n 1 |
                    tr '-' '_')

                SRC="target/$TARGET/release/$BIN_NAME$EXT"

                if [[ -f "$SRC" ]]; then
                    cp "$SRC" "bin/$BIN_NAME$EXT"
                    echo "copied to bin/$BIN_NAME$EXT"
                else
                    echo "build succeeded, but no bin found: $SRC"
                    exit 1
                fi
            else
                echo "build failed"
                exit 1
            fi
            ;;
        c)
            cargo clean
            ;;
        *)
            exit 0
            ;;
    esac
}

if [ -t 0 ]; then
    main_function
else
    title="$BIN"
    xfce4-terminal\
        --title="$title"\
        -e "bash -c '$0 $@; exec bash'"
fi
