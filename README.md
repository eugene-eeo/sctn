# sctn

**Output interSeCTioN tool**, written in Rust and mostly
an excuse for myself to learn the nice features and work
with the new 1.x release.

```shell
$ sctn "$(cat file1)" "$(cat file2)" "$(cat file3)" ...
```

Returns the lines common to all files, but ordered
with resepect to the last file.

## todo

 - actually test this shit
