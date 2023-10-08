# Kochiyama's Minigrep

Minigrep is a command line tool that mimics the feature of filtering lines of a file through a search query, provided by the tool grep.

This tool implementation was taugh on "The Rust Programming Language Book".

Throughout the process of working on this project I could put in practice plenty of concepts of Rust. And was very fun to put Rust in practice on a project that I can, and will, use on my daily life.

## What is grep

Grep, the tool that inspired this project, is a tool to search through files, and display the lines that matches your query.

## How to use It

You should provide a query, that is a search param, and a file path. The minigrep will read the file on the path provided, and print to the console all lines that have the query provided.

By default the tool will do a case sensitive search, but you can configure the behaviour by providing an environment variable "IGNORE_CASE" with a value 1 to do a case insensitive search.

Not providing or providing 0 to the "IGNORE_CASE" environment variable will do a case sensitive search.

## Example

Target File (target.txt):

```
Hello World!
This is the phrase you should console log,
in all your first projects,
to say hello to your new technology.
```

query: `hello`

### Case sensitive use:

`minigrep hello target.txt`

Output:

```
to say hello to your new technology.
```

### Case insensitive use:

`IGNORE_CASE=1 minigrep hello target.txt`

Output:

```
Hello World!
to say hello to your new technology.
```

## Permanent Environment

You can set this IGNORE_CASE variable on your .zshrc or .bashrc, that way your search will always be case insensitive.
