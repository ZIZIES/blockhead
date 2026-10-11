# blockhead

render text as big block-letter art in the terminal. no deps, just rust + unicode half-blocks.

```
$ blockhead "hey twin"
█   █ █▀▀▀▀ ▀▄ ▄▀     ▀▀█▀▀ █   █ ▀█▀ █▄  █
█▀▀▀█ █▀▀▀    █         █   █▄▀▄█  █  █ ▀▄█
▀   ▀ ▀▀▀▀▀   ▀         ▀   ▀   ▀ ▀▀▀ ▀   ▀
```

or chill mode:

```
$ blockhead --lowercase "hey twin"
█▄▄▄   ▄▄▄  ▄   ▄     ▄█▄ ▄   ▄  ▀  ▄▄▄▄
█   █ █▄▄▄█ ▀▄▄▄█      █  █ ▄ █  █  █   █
▀   ▀  ▀▀▀      ▀      ▀▀  ▀ ▀  ▀▀▀ ▀   ▀
```

## build

```
cargo build --release
```

binary ends up in `target/release/blockhead`.

## usage

```
blockhead [options] <text...>
```

words get joined with a single space. text is uppercased by default (or lowercased with `--lowercase`), so casing of input doesn't matter.

### options

| flag | what it does |
| --- | --- |
| `--color <hex\|name>` | truecolor output. takes `#rrggbb` or a named color. |
| `--spacing <n>` | columns of space between letters (default `1`). |
| `--shade` | vertical fade — top row bright, dims each row down. |
| `--lowercase`, `--lower`, `-l` | render lowercase letters instead of caps. |
| `-h`, `--help` | show help. |

`--gradient` is an alias for `--shade`. `--color` and `--spacing` also work as `--flag=value`.

### named colors

`black`, `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `white`, `gray`/`grey`, `orange`, `purple`.

```
blockhead --color orange "hot"
blockhead --color '#ff8800' --shade "fire"
blockhead -l --spacing 3 "yo"
```

### shading

`--shade` fades the output: top row full brightness, dimming each row down to dark at the bottom. it's truecolor, so it only shows in a truecolor terminal — here's the same render with each line's color labeled so you can see the fade:

```
rgb(255,136,  0)   █▀▀▀▀ ▀█▀ █▀▀▀▄ █▀▀▀▀
rgb(169, 90,  0)   █▀▀▀   █  █▀▀█  █▀▀▀
rgb( 84, 45,  0)   ▀     ▀▀▀ ▀   ▀ ▀▀▀▀▀
```

it works with `--lowercase` too, and defaults to white if you don't pass `--color`:

```
blockhead --shade "glow"
blockhead -l --shade --color '#44aaff' "cold"
```

## how it works

each glyph is a 5-row bitmap, one `u8` per row (bits = pixels). the renderer walks two rows at a time and maps the top/bottom pixel pair onto unicode block chars:

- both on → `█` (full block)
- top only → `▀` (upper half)
- bottom only → `▄` (lower half)
- neither → space

so a 5-row font packs into 3 terminal lines.

## supported chars

`a-z`, `A-Z`, `0-9`, space, and `. , ! ? - _ : / + = * # ( )`. anything else renders as blank space.
