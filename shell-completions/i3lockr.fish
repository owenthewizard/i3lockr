complete -c i3lockr -l darken -l dark -d 'Darken the screenshot by [1, 255]. Example: 15' -r
complete -c i3lockr -l brighten -l bright -d 'Brighten the screenshot by [1, 255]. Example: 15' -r
complete -c i3lockr -s b -l blur -d 'Blur strength. Example: 10' -r
complete -c i3lockr -s p -l scale -d 'Scale factor. Increases blur strength by a factor of this. Example: 2' -r
complete -c i3lockr -l ignore-monitors -l ignore -d 'Don\'t overlay an icon on these monitors. Useful if you\'re mirroring displays. Must be comma separated. Example: 0,2' -r
complete -c i3lockr -s u -l position -l pos -d 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"' -r
complete -c i3lockr -s i -l icon -d 'Path to icon to overlay on screenshot' -r -F
complete -c i3lockr -s v -l verbose -d 'Print how long each step takes, among other things. Always enabled in debug builds'
complete -c i3lockr -l invert -d 'Interpret the icon as a mask, inverting masked pixels on the screenshot. Try it to see an example'
complete -c i3lockr -s h -l help -d 'Print help'
complete -c i3lockr -s V -l version -d 'Print version'
