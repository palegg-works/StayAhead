#!/bin/bash
set -e

./build_macos.sh

./build_android.sh

./build_gh_pages.sh

echo "✅ All targets have been built!"

