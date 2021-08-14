# jrn
_jrn_ (short for journal) is a command line application for journal writing and not keeping, with
emphasize being on simplicity. The barrier to creating new notes should always be as low as
possible; simply write `jrn` in a terminal, and start writing in your favourite editor.

```
USAGE:
    jrn [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -D, --debug      Print debug information
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --namespace <namespace>    Namespace [default: default]

SUBCOMMANDS:
    edit      Edit journal
    help      Prints this message or the help of the given subcommand(s)
    log       Show history
    search    Search journal entries
```