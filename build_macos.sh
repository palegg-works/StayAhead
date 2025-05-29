#!/bin/bash
set -e

VERSION=$(grep '^version' Cargo.toml | sed -E 's/version *= *"(.*)"/\1/')

# Run the command and capture output
dx clean
output=$(dx bundle --release --platform macos 2>&1)

# Print the full output for logging (optional)
echo "$output"

# Extract the line that contains "Bundled app at:"
bundle_line=$(echo "$output" | grep "Bundled app at:" | tail -n 1)

# Extract the path from the line
dmg_path=$(echo "$bundle_line" | sed -E 's/.*Bundled app at: (.*)/\1/')

base_dir=$(dirname "$dmg_path")
base_dir=${base_dir%/dmg} 
app_name=$(basename "$dmg_path" .dmg) 
app_name=${app_name%%_*}
app_path="$base_dir/macos/${app_name}.app"

dmg_dir=$(dirname "$dmg_path")
bundle_dmg_script="$dmg_dir/bundle_dmg.sh"

# Construct new path

# Signing
SIGNATURE="-"
codesign -s "${SIGNATURE}" "$app_path"
xattr -rd com.apple.quarantine $app_path
# xattr -cr  /path/to/YourApp.app

"$bundle_dmg_script" \
  --volname "StayAhead" \
  --window-size 500 300 \
  --icon-size 128 \
  --app-drop-link 400 150 \
  --codesign "$SIGNATURE" \
  --skip-jenkins \
  ~/Desktop/StayAhead_"$VERSION".dmg \
  $app_path

echo "✅ Created $dmg_path to ~/Desktop"

# Verify that the file exists
# if [[ -f "$dmg_path" ]]; then
#     cp "$dmg_path" ~/Desktop/
#     echo "✅ Copied $dmg_path to ~/Desktop"
# else
#     echo "❌ DMG file not found: $dmg_path"
#     exit 1
# fi

