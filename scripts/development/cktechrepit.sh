#!/usr/bin/env zsh

# Prompt for repo name
echo -n "Enter new CKTech GitHub repo name (e.g. ghostwin): "
read repo

# Prompt for language (used to generate .gitignore)
echo -n "Project language (rust/zig/python/js/none): "
read lang

# Prompt for private or public visibility
echo -n "Make repository private? (y/n): "
read private_input
[[ "$private_input" =~ ^[Yy]$ ]] && visibility="--private" || visibility="--public"

# Optional: add GitHub topics
echo -n "Add GitHub topics? (space-separated, or leave blank): "
read topics

# Set working directory
cd /data/projects || exit 1
mkdir "$repo"
cd "$repo" || exit 1

# Base files
echo "# $repo" > README.md
cat <<EOF > LICENSE
MIT License

Copyright (c) 2025 CK Technology LLC

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
EOF

# .gitignore based on language
case "$lang" in
  rust)
    cat <<EOF > .gitignore
/target
Cargo.lock
**/*.rs.bk
EOF
    ;;
  zig)
    cat <<EOF > .gitignore
/zig-cache/
/zig-out/
*.o
*.exe
EOF
    ;;
  python)
    cat <<EOF > .gitignore
__pycache__/
*.pyc
*.pyo
.venv/
env/
venv/
EOF
    ;;
  js)
    cat <<EOF > .gitignore
node_modules/
dist/
npm-debug.log
yarn-error.log
EOF
    ;;
  *)
    touch .gitignore
    ;;
esac

# GitHub Actions stub
mkdir -p .github/workflows
touch .github/workflows/main.yml

# Initialize git and push
git init
git add .
git commit -m "Initial commit"

# Create the repository under CK-Technology org
gh repo create "CK-Technology/$repo" \
  $visibility \
  --source=. \
  --remote=origin \
  --push

# Set SSH remote explicitly
git remote set-url origin "git@github.com:CK-Technology/$repo.git"

# Apply topics if provided
if [[ -n "$topics" ]]; then
  gh repo edit "CK-Technology/$repo" --add-topic ${(s: :)topics}
fi

# Confirmation
echo "\n✅ Repo '$repo' created under CK-Technology and pushed."
git remote -v

