# *comfy* script
*comfy* is a cross-platform command script manager / runner tool, which allows you to run commands in the command line itself, but being these predefined in a portable and universal *.comfy* file.  
*.comfy* files are plain text, but with some arguments so *comfy parser* understands what do you want to do.

## System Clauses

Comfy allows you to write system independant scripts while minimizing code
reproduction by allowing you to arbitrarily specialize sections of the script
which are system dependant, while using cross platform code for sections that
are not.

```
// this is a comment
> linux
echo linux system!
> windows
echo windows system!
> always
echo this always runs!
@ sleep 2000
echo you waited 2000 ms!
```
As you are thinking, the above code only runs the commands, that should be run, depending on the environment in which the script is run. Also, keep in mind that everything after *always* clause will run on any operating system. *comfy* automatically detects the system, and, being programmed in Rust, allows the specification of the following systems:
- linux
- macos
- freebsd
- solaris
- android
- windows
- others

Not to mention that you can intertwine different system clauses, like this:
```
> linux
echo Linux user here!
> windows
echo Windows user here!
> linux
echo Linux user here! x2
> windows
echo Windows user here! x2
```

If you are on Linux, output should be:
```
Linux user here!
Linux user here! x2
```


## Universal Functions

*comfy* also has some *universal functions*, they work on any system regardless of the installed libraries. *comfy* comes with several packaged libraries, so far, we have the following *universal functions*:
- @ sleep [int] (ms)
- @ print [str] (text)

## Basic Usage

By defaut *comfy* will try to run a script named `run.comfy` in your current
working directory. If That file does not exist it will tell you so and exit.
*comfy* has the following command line arguments:

| Command                | Use                                                        |
|------------------------|------------------------------------------------------------|
| --help                 | Prints this message or the help of the given subcommand(s) |
| --helpf                | Scripting help                                             |
| run <run.comfy>        | Run a script                                               |
| run <run.comfy> --c    | Show comments from source while running                    |

If you are on a unix-like system also have the option, with `cargo install`, to
make the script itself runnable like so:

First you will need to make sure you have *comfy* installed:
```bash
cargo install --git https://github.com/dacousb/comfy

# and then take note of its installation location
which comfy
```

You can then write your script. Replace `comfy` in the example below (on the first
line) with the installation loation noted before.

```bash
#!comfy run
> linux
echo Linux Executable Script!
> macos
echo But also works on Mac
> solaris
echo or Solaris!
```

and then you simply need to

```bash
# Once for each script you create (to enable the shebang on the first line)
chmod +x run.comfy

# To run the script (the ./ is important)
./run.comfy
```
