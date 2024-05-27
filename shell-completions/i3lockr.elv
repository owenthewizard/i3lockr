
use builtin;
use str;

set edit:completion:arg-completer[i3lockr] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'i3lockr'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'i3lockr'= {
            cand --darken 'Darken the screenshot by [1, 255]. Example: 15'
            cand --dark 'Darken the screenshot by [1, 255]. Example: 15'
            cand --brighten 'Brighten the screenshot by [1, 255]. Example: 15'
            cand --bright 'Brighten the screenshot by [1, 255]. Example: 15'
            cand -b 'Blur strength. Example: 10'
            cand --blur 'Blur strength. Example: 10'
            cand -p 'Scale factor. Increases blur strength by a factor of this. Example: 2'
            cand --scale 'Scale factor. Increases blur strength by a factor of this. Example: 2'
            cand --ignore-monitors 'Don''t overlay an icon on these monitors. Useful if you''re mirroring displays. Must be comma separated. Example: 0,2'
            cand --ignore 'Don''t overlay an icon on these monitors. Useful if you''re mirroring displays. Must be comma separated. Example: 0,2'
            cand -u 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"'
            cand --position 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"'
            cand --pos 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"'
            cand -i 'Path to icon to overlay on screenshot'
            cand --icon 'Path to icon to overlay on screenshot'
            cand -v 'Print how long each step takes, among other things. Always enabled in debug builds'
            cand --verbose 'Print how long each step takes, among other things. Always enabled in debug builds'
            cand --invert 'Interpret the icon as a mask, inverting masked pixels on the screenshot. Try it to see an example'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
