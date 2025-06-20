#!/usr/bin/env zsh

# Prompt for repo name
echo -n "Enter new GitHub repo name (e.g. dotshot): "
read repo

# Set working path
cd /data/projects || exit
mkdir "$repo"
cd "$repo" || exit

# Create base files
echo "# $repo" > README.md
cat <<EOF > LICENSE
MIT License

Copyright (c) $(date +%Y) GhostKellz

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions...

EOF

# Initialize git
git init
git add .
git commit -m "Initial commit: bootstrap"

# Create GitHub repo (SSH, public, push it)
gh repo create "ghostkellz/$repo" \
  --public \
  --source=. \
  --remote=origin \
  --push

# Ensure SSH is used for future pushes
git remote set-url origin "git@github.com:ghostkellz/$repo.git"

# Confirmation
echo "\n✅ Repo '$repo' created at /data/projects/$repo and pushed via SSH"
git remote -v
