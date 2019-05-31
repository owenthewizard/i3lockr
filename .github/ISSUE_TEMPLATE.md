# Sanity Checks

- [ ] Using a recent enough `i3lock`? (TODO i3/i3lock#231)
- [ ] All libraries installed? `i3lockr` requires `libxcb` including the RandR and SHM extensions. Many distros require a `-dev` or `-devel` package to be installed to compile software. Check `ldd $(which i3lockr)` for required runtime libraries.

# Important Info

Output of `i3lock --version`, for example:
> i3lock: version 2.11.1-20-gaa7984f (2019-05-23, branch "makepkg") © 2010 Michael Stapelberg

Output of `i3lockr --version`, for example:
> i3lockr v1.0.0 compiled for 'x86_64-unknown-linux-gnu' at 1558728285 (v2@9d61d55)

Your distro (and version, if applicable), for example:
> Arch Linux

# Issue

Type your issue here...
