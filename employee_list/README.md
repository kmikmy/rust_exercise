# description

```
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
```
from [rust tutorial](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#summary)

# usage

(ex)
```
$ cargo run
   Compiling list_of_integers v0.1.0 (/foo/var/rust_exercise/pig_latin)
    Finished dev [unoptimized + debuginfo] target(s) in 1.64s
     Running `target/debug/list_of_integers`
> Add Emi to Engineering
> Add Ami to Engineering
> Add Tom to Sales
> Add Yuki to Sales
> Get Sales
[Sales]
- Tom
- Yuki

> Get
[Engineering]
- Ami
- Emi

[Sales]
- Tom
- Yuki

> exit
$
```

