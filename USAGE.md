# Usage

See the help:

```
i3lockr v0.1.0 (unknown-branch@unknown-commit)
Owen Walpole <owenthewizard@hotmail.com>
Distort a screenshot and run i3lock

USAGE:
    i3lockr [FLAGS] [OPTIONS] [i3lock]...

FLAGS:
    -h, --help       Prints help information
        --invert     Draw the lock image as an invert mask on the background, i.e. invert every pixel on the background
                     where the same pixel on the lock image has >50% alpha
    -V, --version    Prints version information

OPTIONS:
    -d, --dark <dark>            Darkens image by an amount [default: -36]
    -g, --gravity <gravity>      Text position [default: south]  [possible values: north, east, south, west]
    -i, --iter <iter>            Number of blur iterations [default: 1]
    -l, --lock <lock>            Path to lock image
    -f, --scale <scale>          Scale factor for faux-blur. Divisor of 1, so 5 == 20% [default: 2]
    -s, --strength <strength>    Blur strength [default: 3]
    -t, --text <text>            Text to draw on the screen [UNINPLEMENTED] [default: ]

ARGS:
    <i3lock>...    Args to pass to i3lock
```

[default.png](default.png) is embedded into the executable at build-time as the default lock image.
