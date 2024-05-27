# Usage

See the help:

```
Distort a screenshot and run i3lock

Usage: i3lockr [OPTIONS] [-- <i3lock>...]

Arguments:
  [i3lock]...  Arguments to pass to i3lock. Example: "--nofork --ignore-empty-password"

Options:
  -v, --verbose                Print how long each step takes, among other things. Always enabled in debug builds
      --darken <DARK>          Darken the screenshot by [1, 255]. Example: 15 [aliases: dark]
      --brighten <BRIGHT>      Brighten the screenshot by [1, 255]. Example: 15 [aliases: bright]
  -b, --blur <RADIUS>          Blur strength. Example: 10
  -p, --scale <FACTOR>         Scale factor. Increases blur strength by a factor of this. Example: 2
      --ignore-monitors <0,2>  Don't overlay an icon on these monitors. Useful if you're mirroring displays. Must be comma separated. Example: 0,2 [aliases: ignore]
      --invert                 Interpret the icon as a mask, inverting masked pixels on the screenshot. Try it to see an example
  -u, --position <x,y>         Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20" [aliases: pos]
  -i, --icon <lock.png>        Path to icon to overlay on screenshot
  -h, --help                   Print help
  -V, --version                Print version
```

Items marked `[NYI]` are `Not Yet Implemented` and may function partially or not at all!
