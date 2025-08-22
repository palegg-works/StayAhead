#! /bin/sh
set -e

VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f2)

dx clean

mv Dioxus.toml Dioxus.toml.bak
cp Dioxus.ghpages.toml Dioxus.toml
dx bundle --platform web
mv Dioxus.toml.bak Dioxus.toml

git checkout gh-pages
rm -rf assets index.html wasm
cp -r target/dx/stay-ahead/release/web/public/* .

# Add Simple Analytics
file="index.html"
snippet='<!-- 100% privacy-first analytics --><script data-collect-dnt="true" async src="https://scripts.simpleanalyticscdn.com/latest.js"></script><noscript><img src="https://queue.simpleanalyticscdn.com/noscript.gif?collect-dnt=true" alt="" referrerpolicy="no-referrer-when-downgrade"/></noscript>'
if sed --version >/dev/null 2>&1; then
  # GNU sed
  sed -i "/<\/body>/i $snippet" "$file"
else
  # BSD/macOS sed
  sed -i '' "/<\/body>/i\\
$snippet
" "$file"
fi

cp index.html 404.html
git add -A
git commit -m "web release for version $VERSION"
git push

git checkout main
[ -d wasm ] && rm -rf wasm
[ -d 404.html ] && rm -rf 404.html

echo gh-pages have been updated to $VERSION!
