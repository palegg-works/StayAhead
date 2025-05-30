#!/bin/bash
set -e

./build_android.sh

./build_macos.sh

./build_gh_pages.sh

echo "✅ All targets have been built!"

