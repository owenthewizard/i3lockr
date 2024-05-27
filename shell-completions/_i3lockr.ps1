
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'i3lockr' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'i3lockr'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'i3lockr' {
            [CompletionResult]::new('--darken', 'darken', [CompletionResultType]::ParameterName, 'Darken the screenshot by [1, 255]. Example: 15')
            [CompletionResult]::new('--dark', 'dark', [CompletionResultType]::ParameterName, 'Darken the screenshot by [1, 255]. Example: 15')
            [CompletionResult]::new('--brighten', 'brighten', [CompletionResultType]::ParameterName, 'Brighten the screenshot by [1, 255]. Example: 15')
            [CompletionResult]::new('--bright', 'bright', [CompletionResultType]::ParameterName, 'Brighten the screenshot by [1, 255]. Example: 15')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Blur strength. Example: 10')
            [CompletionResult]::new('--blur', 'blur', [CompletionResultType]::ParameterName, 'Blur strength. Example: 10')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Scale factor. Increases blur strength by a factor of this. Example: 2')
            [CompletionResult]::new('--scale', 'scale', [CompletionResultType]::ParameterName, 'Scale factor. Increases blur strength by a factor of this. Example: 2')
            [CompletionResult]::new('--ignore-monitors', 'ignore-monitors', [CompletionResultType]::ParameterName, 'Don''t overlay an icon on these monitors. Useful if you''re mirroring displays. Must be comma separated. Example: 0,2')
            [CompletionResult]::new('--ignore', 'ignore', [CompletionResultType]::ParameterName, 'Don''t overlay an icon on these monitors. Useful if you''re mirroring displays. Must be comma separated. Example: 0,2')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"')
            [CompletionResult]::new('--position', 'position', [CompletionResultType]::ParameterName, 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"')
            [CompletionResult]::new('--pos', 'pos', [CompletionResultType]::ParameterName, 'Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right). Has no effect without --icon. Must be comma separated. Defaults to center if not specified. Example: "945,-20"')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'Path to icon to overlay on screenshot')
            [CompletionResult]::new('--icon', 'icon', [CompletionResultType]::ParameterName, 'Path to icon to overlay on screenshot')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Print how long each step takes, among other things. Always enabled in debug builds')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Print how long each step takes, among other things. Always enabled in debug builds')
            [CompletionResult]::new('--invert', 'invert', [CompletionResultType]::ParameterName, 'Interpret the icon as a mask, inverting masked pixels on the screenshot. Try it to see an example')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
