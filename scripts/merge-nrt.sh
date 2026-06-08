#!/usr/bin/env bash
# merge-nrt.sh — merge all track WAVs from an NRT run into a single file.
#
# Reads nrt_output_dir from ~/.config/jdw.toml [pycompose] section.
# Falls back to ~/jdw_output.
# Output: <output_dir>/nrt_merged.wav

set -euo pipefail

CONFIG="${JDW_CONFIG:-$HOME/.config/jdw.toml}"

# Parse nrt_output_dir from [pycompose] section, default to ~/jdw_output
output_dir=""
in_pycompose=false
if [[ -f "$CONFIG" ]]; then
    while IFS= read -r line; do
        case "$line" in
            '['*']') in_pycompose=false; [[ "$line" == "[pycompose]" ]] && in_pycompose=true ;;
            *=*)
                if $in_pycompose; then
                    key="${line%%=*}"
                    key="${key// /}"
                    val="${line#*=}"
                    val="${val# }"
                    val="${val% }"
                    val="${val//\"/}"
                    if [[ "$key" == "nrt_output_dir" ]]; then
                        output_dir="${val/#\~/$HOME}"
                    fi
                fi
                ;;
        esac
    done < "$CONFIG"
fi

output_dir="${output_dir:-$HOME/jdw_output}"
output_dir="${output_dir%/}"
merged="/tmp/nrt_merged.wav"

echo "Source dir: $output_dir"
echo "Merged:      $merged"

cd "$output_dir"
wavs=(*.wav)

if [[ ${#wavs[@]} -eq 0 ]]; then
    echo "No WAV files found in $output_dir" >&2
    exit 1
fi

echo "Merging ${#wavs[@]} tracks..."
sox -m "${wavs[@]}" "$merged"
echo "Done. Playing..."
mpv "$merged"
