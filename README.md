# sctn

**Output interSeCTioN tool**, written in Rust and mostly
an excuse for myself to learn the nice features and work
with the new 1.x release. Most of the time you'd want to
call it like:

```shell
$ sctn "$(cat file1)" "$(cat file2)" "$(cat file3)" ...
```

Returns the lines common to all files, but ordered with
respect to the last file. Useful for stuff like figuring
out dependencies.

## todo

 - actually test this shit
 - write the help page
