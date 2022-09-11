# description

```
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
```
from [rust tutorial](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#summary)

# usage


(ex)
```
$ cargo run
   Compiling list_of_integers v0.1.0 (/foo/var/rust_exercise/pig_latin)
    Finished dev [unoptimized + debuginfo] target(s) in 1.64s
     Running `target/debug/list_of_integers`
Please enter some words.
first apple
pig latin: irst-fay apple-hay 
$
```
