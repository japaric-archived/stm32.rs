cd target/doc

git init
git config user.name "Jorge Aparicio"
git config user.email "japaricious@gmail.com"
git remote add origin "https://$GH_TOKEN@github.com/japaric/stm32.rs"

git checkout -b gh-pages
git add -A
git commit -m 'docs!'
git push -fq origin gh-pages
