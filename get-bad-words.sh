
current_dir=$(pwd)
rm -rf google-profanity-words

git clone https://github.com/coffee-and-fun/google-profanity-words/

cd google-profanity-words/data

# get all the english and french words but wrap them in "quotes" and separate them with a comma

cat list.txt | sed 's/^/"/' | sed 's/$/",/' > "$current_dir/bad_words.txt"

num_lines=$(wc -l < "$current_dir/bad_words.txt")

array_start="pub static BAD_WORDS: [&str; $num_lines] = ["
array_end="];"

cd "$current_dir" || exit 1

echo "$array_start" > src/bad_words.rs
cat bad_words.txt >> src/bad_words.rs
echo "$array_end" >> src/bad_words.rs

# format the file
rustfmt src/bad_words.rs

rm -rf google-profanity-words
rm bad_words.txt