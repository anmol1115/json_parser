# Problem
**This is work in progress.** Since all there are at a time 4 threads processing a chunk each, it could happen that these chunks are received at the receiver in a random manner hence corrupting the data.

---
# Usage
- Build the release version of code using
```console
foo@bar:~$ cargo build -r
```
- This will generate a binary *target/release/json_parser*
- Use the binary the following way to process the json file
```console
foo@bar:~$ target/release/json_parser <INPUT_FILE_PATH> <OUTPUT_FILE_PATH>
```
- This will generate the processed json file at the *<OUTPUT_FILE_PATH>*.
