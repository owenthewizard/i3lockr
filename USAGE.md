# Usage

See the help:

```
i3lockr 1.0.0-final
Owen Walpole <owenthewizard@hotmail.com>
Distort a screenshot and run i3lock

USAGE:
    i3lockr [FLAGS] [OPTIONS] [-- <i3lock>...]

FLAGS:
    -v, --verbose    Print how long each step takes, among other things. Always enabled in debug builds.
    -h, --help       Prints help information
        --invert     Interpret the icon as a mask, inverting masked pixels on the screenshot. Try it to see an example.
    -V, --version    Prints version information

OPTIONS:
        --ignore-monitors <0,2>...    Don't overlay an icon on these monitors. Must be comma separated. Example: 0,2
                                      [aliases: ignore]
    -i, --icon <file.png>             Path to icon to overlay on screenshot.
    -u, --position <945,-20>...       Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no
                                      effect without --icon. Must be comma separated. Defaults to center if not
                                      specified. Example: "945,-20" [aliases: pos]
    -b, --blur <radius>               Blur strength. Example: 10

ARGS:
    <i3lock>...    Arguments to pass to i3lock. Example: "--nofork --ignore-empty-password"
```

Items marked `[NYI]` are `Not Yet Implemented` and may function partially or not at all!
