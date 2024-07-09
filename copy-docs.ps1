rm ./target/doc -r -fo
cargo doc --no-deps
echo '<meta http-equiv=\"refresh\" content=\"0; url=simplebuffers_core\">' > target/doc/index.html
rm ./docs -r -fo
cp -r target/doc ./docs