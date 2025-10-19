#!/bin/bash
echo "Building..."
trunk build --release

echo "Deploying..."
cd dist
git init
git add -A
git commit -m "Deploy $(date)"
git branch -M gh-pages
git remote add origin git@github.com:hcdvall/crab-o-clock.git 2>/dev/null || true
git push -f origin gh-pages
cd ..

echo "✅ Deployed!"