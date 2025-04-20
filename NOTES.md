# Project idea: Sourses

Create a small CLI tool that extracts various types of text items from your
current command-line session (normal shell or tmux) so you can fuzzy-find them.

The tools should extract:

- URLs: `https://example.com`
- File paths: `/path/to/file`
- Commands: `ls -la`
- Shell output: `foo bar baz`
- Tmux pane content: `another command output`
- Environment variables `$HOME`
- Process IDs: `1234`
- Error messages: `Error: something went wrong`
- Clipboard content: `Some copied text`

Then we can filter by those types.

The goal is to boost productivity by letting you quickly access these items
through fuzzy searching.

It should be possible to configure the tool to extract additional types via
patterns. Using regex or similar would be a good approach.

The tool should be really fast and have a minimal memory footprint. The max time
to extract all items should be less than 1ms if possible, at least for the text
in the current view.

Once the text item/s are selected we could have a few options:

- Copy to clipboard
- Open in browser
- Insert in shell

Extra:

- Open in editor
- Run command
- Search in browser

## Security

How the tool will handle sensitive data? It should be possible to disable the
extraction of some types of data, like environment variables or clipboard content.

## Shell compatibility

- Bash
- Zsh
- Fish (if possible)

Shell completion would be a nice addition.

## Use cases

- Quickly open a URL that was printed in the terminal
- Copy a command output to the clipboard
- Insert a file path in the current command
- Search for a specific error message

## Cli examples

Example with no filter:

```bash
$ sour
Fuzzy>
[sh]: ls -la
[path]: ~/clones/forks/tldr/README.md
[url]: https://example.com
[pid]: 1234
[tmux]: another command output
[env]: $HOME -> /home/user
```

Example with filter (only URLs):

```bash
$ sour -t url
Fuzzy> \.com
https://example.com
https://another.com
https://rust.com
```

Example with regex filter:

```bash
$ sour -r 'Error:.*'
Fuzzy>
Error: something went wrong
Error[404]: another error message
```

Example with multiple filters:

```bash
$ sour -t url -t path
Fuzzy> ls
[path]: ls -la
[url]: https://lols.com
[path]: /home/user/antils
```

Example with the modes footer:

```bash
$ sour -m
Fuzzy>
[sh]: ls -la
[path]: ~/clones/forks/tldr/README.md
[url]: https://example.com
[pid]: 1234
[tmux]: another command output
[env]: $HOME -> /home/user
(modes) | Refilter: C+f | Regex: C+r | Clip: C+c | Select: C+s | Insert: C+i | Open by type: C+o | Exit: Esc
```
