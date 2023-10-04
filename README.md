# cecho

Crisp echo.

A sanitized echo and sprintf alternative that also prints colors.

## Motivation

Must be simple to start with and not trap people by default!

Must be powerfull enough to not require extra tooling if it's easier, be it
* bash's number formatting trick `$(([##7]v))` for format the variable v in base 7 for instance
* printf's decimal formatting
* escape sequences or tput coloring

I'm tired of looking up color codes and have unreadable color escape sequences.

I don't want to play with `tput setaf 3` and have an unreadable echo statement.

I'm tired of looking up printf's formats, I didn't touch C for years. I use modern languages with friendlier formatting options.

I don't want to build another layer of bash workarounds. It still contains all of echo's flaws regarding `-` `-n` `-e` and escape sequences.

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/cecho

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/cecho2

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/bases

I don't want to start a real scripting language, or even worse: the JVM, to highlight a few lines of output in a bash script.

It may as well be comprehensive so there's 1 tool that gives access to all of the shell's printing capabilities if the format string remains terse.
I'm likely engaging on a slippery slope but we'll see if it's possible.

I don't want to import and redeclare the colors each time I use them. This should be done once and for all.

https://stackoverflow.com/questions/5412761/using-colors-with-printf



### Limitation is current tools

Mix of bash's base formatting, sprintf's decimal formatting, echo workarounds, escape sequences or tput, alternative tool that auto-color based on various criteria

### Alternatives?

TODO: is there anything that comes close to this?

## Implemented

Spec draft v0.1

### Print any string by omitting the format

```bash
cecho '' 'Whatever you want here!'
```

`Whatever you want here!`

### Print multiple strings by omitting the format

```bash
cecho '' 'Whatever you want here,' ' and there,' ' and some more..'
```

`Whatever you want here, and there, and some more...`

### Specify a format

```bash
cecho '{}+{}={}' 1 2 3
```

`1+2=3`

### Print just brackets

```bash
cecho '{}' '{}'
```

`{}`

### Literal brackets

```bash
cecho '\{{}\}' 'value'
```

`{value}`

### Indexed arguments

```bash
cecho '{3}-{2}={1}' a b c
```

`c-b=a`


```bash
cecho '{3} {2} {1} {2} {3}' 1 2 3
```

`3 2 1 2 3`


## Goals

No `-` `--` `-n` `-e` etc. ambiguity as in `echo`

No `%x` and other random characters like in `printf`

No looking up for color code indexes nor figuring out which option ins corerct like in tput.

Use full english nouns but allow existing styles for easy migration form those tools.

Support color.

Optional formatting string. I must not need to guess what format I need if I only want to print.

Fast! Like a native binary.

Format support à la printf but with modern format specifiers: 
 * positional `{}`
 * indexed `{1}`
 * named `{foo}`
 * number formatting `{1.5}`
 * simple colors `{#red}`
 * reset to default after printing? `{#red!preserve}` or auto reset? 🤔
 * any color `{#A03472}`
 * backgroud colors `{#white/red}`
 * Styles: bold, italic, blinki, reset, ... `{!bold}`
 * Position on screen `{@5,10}`

## Examples

The quoting and escaping in these examples is assuming that you write these command in a `sh`-like shell.

### New lines

```bash
cecho 'a\nb\nc'
```

```
a
b
c
```

### Decimal formatting

TODO: is there another readily available program that could do the number formatting and let me not re-implement the wheel?

Leave this to `printf` ?

```bash
cecho '{02.02}' '9.8'
```

`09.80`

TODO: extends the spec to support generalized number formats

https://alvinalexander.com/programming/printf-format-cheat-sheet/

### Base-X

TODO

```bash
{}
```

### Extra opt-in features

These will break the features above and must be selected explicitly

#### Named arguments

```bash
cecho '{?named}{foo} and {bar}' --foo=A --bar=B
```

`A and B`

Regular behaviour:

```bash
cecho '{foo} and {bar}' --foo=A --bar=B
```

`--foo=A and --bar=B`

#### Recursivity

This one may become unmanageable but let's see if it's possible anyway.

It recursively evaluates the arguments 1 by 1, modifying the format at each evaluation.

```bash
cecho '{?recurse}' '{?named}{foo}' --foo='{03.5@blue}' '3.14159265359'
```

Step by step this is equivalent to

```bash
cecho '{!recurse}{?named}{foo}' --foo='{03.5@blue}' '3.14159265359'
```

```bash
cecho '{!recurse}{?named}{03.5@blue}' '3.14159265359'
```

```bash
cecho '{!recurse}{?named}3.14159'
```

TODO: distinguish indexed argument from number formatting.

<span style="color:'blue'"> `003.14159` </span>


## Format specifiers brainstorming

All the format given below are assumed ot be enclosed in `{}`

It must be possible to easily combine the format speficifiers.

Format types:

* Name 1st position
* Number format `%`
* Color `#`
* Position `@`
* Style `!`
* Option `?`



```bash
cecho '{?named}{foo@2,3#r%1.2!bold}' --foo=3.1415
```

Shows the content of foo, at the cursor row 2 column 3, in red, with 2 decimals, in bold.

<pre>
<b style="color:red;">3.14</b>
</pre>

### Name

Must be first to avoid ambiguities

### Number format

printf style?

In that case why not delegate the formatting to `printf`?

### Colors

Colors could be specified by index (tput's setaf code), single letter or full name, case insensitive.

```
#0
#k
#black

#1
#r
#red

#2
#g
#green

#3
#y
#yellow

#4
#b
#blue

#5
#m
#magenta

#6
$c
#cyan

#7
#w
#white

#abc
#abc123
```

The last 2 lines are for arbitrary hex color codes.


#### Forefround/background

Split with slash foreground over background.

`#foreground/background`

`#white/red` if a white font on a red background.

TODO: what if the terminal only supports 8 colors? Find the closest color that matches?

### Position

`@row,column`

Support relative movement?

### Styles

`!style`

### Options

Similar to regex options `(?=...)`

`?option`

## TODOs

What formatting style to use? Python? Rust? C?

