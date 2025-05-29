#!/bin/bash
set -e

# Run the command and capture output
dx clean
output=$(dx bundle --release --platform macos 2>&1)

# Print the full output for logging (optional)
echo "$output"

# Extract the line that contains "Bundled app at:"
bundle_line=$(echo "$output" | grep "Bundled app at:" | tail -n 1)

# Extract the path from the line
dmg_path=$(echo "$bundle_line" | sed -E 's/.*Bundled app at: (.*)/\1/')

# Verify that the file exists
if [[ -f "$dmg_path" ]]; then
    cp "$dmg_path" ~/Desktop/
    echo "✅ Copied $dmg_path to ~/Desktop"
else
    echo "❌ DMG file not found: $dmg_path"
    exit 1
fi

