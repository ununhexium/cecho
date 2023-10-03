# cecho

Crisp echo.

A sanitized echo and sprintf alternative that also prints colors.

## Motivation

Must be simple to start with and not trap people by default!

Must be powerfull enough to not require extra tooling if it's easier, be it
* bash's number formatting trick `$(([##7]v))` for format the variable v in base 7 for instance
* printf's decimal formatting
* coloring

I'm tired of looking up color codes and have unreadable color escape sequences.

I don't want to play with `tput setaf 3` and have an unreadable echo statement.

I'm tired of looking up printf's formats, I didn't touch C for years. I use modern languages with friendlier formatting options.

I don't want to build another layer of bash workarounds. It still contains all of echo's flaws regarding `-` `-n` `-e` and escape sequences.

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/cecho

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/cecho2

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/bases

I don't want to start a real scripting language, or even worse the JVM: to highlight a few lines of output.

It may as well be comprehensive so there's 1 tool that gives access to all of the shell's printing capabilities if the format string remains terse.
I'm likely engaging on a slippery slope but we'll see if it's possible.


### Limitation is current tools

Mix of bash's base formatting, sprintf's decimal formatting, echo workarounds, escape sequences or tput, 

## Implemented

Spec drafts

## Goals

echo splits the options with `--` what comes before are the options, what comes later are the items to be printed

no `-` `--` `-n` `-e` etc. ambiguity.

must support color

must have an optional formatting string

Fast! Like a native binary.

Format support a la printf but with modern format specifiers: 
 * positional `{}`
 * indexed `{1}`
 * named `{foo}`
 * number formatting `{1.5}`
 * simple colors `{@red}`
 * reset to default after printing? `{@red@}` or auto reset? 🤔
 * any color `{#A03472}`
 * backgroud colors `{@white/red}`
 * Styles: bold, italic, blink `{!bold}`
 * Position on screen

## Examples

The quoting and escaping in these examples is assuming that you write these command in a `sh`-like shell.

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

### Litteral brackets

```bash
cecho '\{{}\}' 'value'
```

`{value}`

### New lines

```bash
cecho 'a\nb\nc'
```

```
a
b
c
```

### Positional arguments

```bash
cecho '{3}-{2}={1}' a b c
```

`c-b=a`

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

```bash

```

### Extra opt-in features

These will break the features above and must be selected explicitly

#### Named arguments

```bash
cecho '{!named}{foo} and {bar}' --foo=A --bar=B
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
cecho '{!recurse}' '{!named}{foo}' --foo='{03.5@blue}' '3.14159265359'
```

Step by step this is equivalent to

```bash
cecho '{!recurse}{!named}{foo}' --foo='{03.5@blue}' '3.14159265359'
```

```bash
cecho '{!recurse}{!named}{03.5@blue}' '3.14159265359'
```

```bash
cecho '{!recurse}{!named}3.14159'
```

TODO: distinguish indexed argument from number formatting.

<span style="color:'blue'"> `3.14159` </span>


## Format specifiers brainstorming

All the format given below are assumed ot be enclosed in `{}`

It must be possible to easily combine the format speficifiers.

Format types:

* Name 1st position
* Number format `%`
* Color `#`
* Position `@`
* Options and style `!`



```bash
cecho '{!named}{foo@2,3#r%1.2!bold}' --foo=3.1415
```

Shows the content of foo, at the cursor row 2 column 3, in red, with 2 decimals, in bold.

<b style="color:red;">
`3.14`
</b>

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

### Options

`!option`

## TODOs

What formatting style to use? Python? Rust? C?
