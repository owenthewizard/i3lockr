# Usage

See the help:

```
i3lockr 1.0.0
Owen Walpole <owenthewizard@hotmail.com>
Distort a screenshot and run i3lock

USAGE:
    i3lockr [FLAGS] [OPTIONS] [i3lock]...

FLAGS:
    -v, --verbose     Print how long each step takes, among other things. Always enabled in debug builds.
    -h, --help        Prints help information
        --invert      Interpret the icon as a mask, inverting masked pixels on the screenshot. Try it to see an example.
        --one-icon    Only place one icon. Default is to place an icon on each monitor. [NYI]
    -V, --version     Prints version information

OPTIONS:
    -i, --icon <file.png>             Path to icon to overlay on screenshot.
    -u, --position <coords|center>    Icon placement, "center" to center, "x, y" (from top-left), or "-x,-y" (from
                                      bottom-right). Has no effect without --icon. Example: "(945, -20)" [default:
                                      Center]
    -b, --blur <radius>               Blur strength. Example: 10

ARGS:
    <i3lock>...    Arguments to pass to i3lock. '--' must be used. Example: "-- --nofork --ignore-empty-password"
```

Items marked `[NYI]` are `Not Yet Implemented` and may function partially or not at all!
