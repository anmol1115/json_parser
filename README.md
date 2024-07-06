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

---
# Results
## Timing
I have timed this code using two tools
- [time](https://man7.org/linux/man-pages/man1/time.1.html): The time command runs the specified program command with the given arguments.  When command finishes, time writes a message to standard error giving timing statistics about this program run.
![time result](https://github.com/anmol1115/json_parser/blob/master/time_res.png)
- [hyperfine](https://github.com/sharkdp/hyperfine): A command line based benchmarking tool
![hyperfine result](https://github.com/anmol1115/json_parser/blob/master/hyperfine_res.png)

## Validation
Because of the file sizes it is impossible to compare each line manually, I used two command line tools to validate the result.
- grep: grep is used to search the file for occurances of the query string. We can see 4 lines in the input file where ';' is present and 0 in the output file.
![grep result](https://github.com/anmol1115/json_parser/blob/master/grep_res.png)
- wc: wc is a tool used to check the number of lines in a file. Here we can see both input and output have same number of lines.
![wc result](https://github.com/anmol1115/json_parser/blob/master/wc_res.png)
