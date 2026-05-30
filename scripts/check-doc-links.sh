#!/usr/bin/env sh
set -eu

check_target() {
    source_file="$1"
    raw_target="$2"

    case "$raw_target" in
        http://*|https://*|mailto:*|\#*|"")
            return 0
            ;;
    esac

    target="${raw_target%%#*}"
    case "$target" in
        /*)
            resolved=".$target"
            ;;
        *)
            source_dir="$(dirname "$source_file")"
            resolved="$source_dir/$target"
            ;;
    esac

    if [ ! -e "$resolved" ]; then
        echo "broken markdown link: $source_file -> $raw_target" >&2
        return 1
    fi
}

for file in README.md SECURITY.md CHANGELOG.md docs/*.md release-notes/*.md .github/*.md; do
    [ -f "$file" ] || continue
    sed -n 's/.*](\([^)]*\)).*/\1/p' "$file" | while IFS= read -r target; do
        check_target "$file" "$target"
    done
done
