# nu_plugin_skim

This is a [Nushell](https://nushell.sh/) plugin that adds integrates the [skim](https://github.com/lotabout/skim) fuzzy finder.

The regular `sk` executable filters lines of text, but the `sk` command added by this plugin can filter Nushell's structured data.

## Installing

Install the crate using:

```nushell
> cargo install --path .
```

Then register the plugin using (this must be done inside Nushell):

```nushell
> plugin add ~/.cargo/bin/nu_plugin_skim
```

## Usage

Pipe the input of any Nushell command into `sk`:

```nushell
> ps | sk --format {get name} --preview {}
```

This will open the skim TUI, allowing you to select an item from the stream:

```
                    │╭─────────┬─────────────╮            1/9
                    ││ pid     │ 4223        │
                    ││ ppid    │ 3944        │
> /usr/bin/nu       ││ name    │ /usr/bin/nu │
  nu                ││ status  │ Running     │
  nu                ││ cpu     │ 28.78       │
  nu                ││ mem     │ 37.4 MiB    │
  nu                ││ virtual │ 1.4 GiB     │
  nu                │╰─────────┴─────────────╯
  6/258         5/0 │
> 'nu               │
```

The item will be returned as the same structured Nushell type that was fed into `sk`:

```nushell
> ps | sk --format {get name} --preview {}
╭─────────┬─────────────╮
│ pid     │ 4223        │
│ ppid    │ 3944        │
│ name    │ /usr/bin/nu │
│ status  │ Running     │
│ cpu     │ 28.57       │
│ mem     │ 37.8 MiB    │
│ virtual │ 1.4 GiB     │
╰─────────┴─────────────╯
```

Of course, the result of the command can be piped into another command:

```nushell
> ps | sk --format {get name} --preview {} | kill $in.pid
```

## Notable flags

nu_plugin_skim aims to repliacte skim's sytnax, but there are some differences to better integrate with Nushell:

* `--format` - this is a flag that the regular skim does not have. It receives a Nushell closure, and pipes the items through that closure before showing them as user selectable rows.

  If the closure returns a complex Nushell data type, it'll be formatted in a notation similar to [Nushell's `debug` command](http://www.nushell.sh/commands/docs/debug.html)

  Note that in skim one would use `--with-nth` for a similar purpose - but the syntax and usage are different enough to warren a different name.
* `--preview` - unlike the regular skim, where `--preview` accepts a string, here `--preview` accepts a Nushell closure. The item under the cursor will get piped into the closure and the result will be displayed inside the preview window.

  If the closure returns a complex Nushell data type, it'll be formatted into a table.

  To display the item as is, use the empty closure `--preview {}`.

* `--bind` - unlike regular `sk` that recieves bindings as a comma-separated list of colon-seperated key-values (e.g. `sk --bind alt-s:down,alt-w:up`), here the bindings are given as a record (e.g. `sk --bind {alt-s: down, alt-w: up}`)

* `--multi` / `-m` - this flag works exactly the same as in the regular skim, but unlike the regular skim that always returns text - here `sk` returns structured Nushell data, and this flag changes the type of that data. Without it, the chosen item is returned as is. With it, it gets returned as a list (even if the user only chooses a single item)

  ```nushell
  > seq 1 10 | sk | describe
  int
  > seq 1 10 | sk -m | describe
  list<int> (stream)
  ```
