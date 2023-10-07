# cecho

Color echo.

A sanitized `printf` alternative that also prints colors.

Actually works in `printf` style but with easy color and style specifiers.

It's named after echo for "marketing" purposes because that's what people will 
probably look for when the first want to print colored text.

## Examples

The quoting and escaping in these examples is assuming that you write these command in a `sh`-like shell.

For more examples, compile and check the `./demo.bash` examples.

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

### Specify a format to use colors

```bash
cecho '{#r}+{#g}={#b}' 1 2 3
```

<pre><span style="color:red;">1</span>+<span style="color:green;">2</span>=<span style="color:blue;">3</span></pre>

### ANSI rainbow

Use either codes, single letters or words to describe a color.

`cecho '{%1#1}{%1#3}{%1#g}{%1#c}{%1#blue}{%1#magenta}' '█'`

<pre><span style="color:red;">█</span><span style="color:yellow;">█</span><span style="color:green;">█</span><span style="color:cyan;">█</span><span style="color:blue;">█</span><span style="color:magenta;">█</span></pre>

### ANSI bright RGB

Shows the color in a brighter variant.

`cecho '{%1#9}{%1#G}{%1#BLUE}' '█'`

<pre><span style="color:indianred;">█</span><span style="color:chartreuse;">█</span><span style="color:deepskyblue;">█</span></pre>

### Literal brackets

```bash
cecho '\{{#cyan}\}' 'value'
```

<pre><span style="color:cyan;">{value}</span></pre>

### Foreground and background colors

```bash
cecho '{#RED/YELLOW}' 'Warning!'
```

<pre><span style="color:red; background-color: yellow;">Warning!</span></pre>

### Indexed arguments

Change the order of display or re-use a value.

```bash
cecho '{%3}-{%2}={%1}' a b c
```

`c-b=a`

```bash
cecho '{%3} {%2} {%1} {%2} {%3}' 1 2 3
```

`3 2 1 2 3`

### New lines and other c-style escape sequences

```bash
cecho 'a\nb\nc'
```

```
a
b
c
```

### Print brackets

```bash
cecho '{}' '{}'
```

`{}`

```bash
cecho '\{}'
```

`{}`

### Unordered

The specifiers' order is not important

```bash
cecho '{#red%1}' 'RED'
```
and

```bash
cecho '{%1#red}' 'RED'
```

Are the same

<pre><span style="color:red;">RED</span></pre>

### Unclutter

If the specifier for an item is getting cluttered, for instance

```bash
cecho '{#yellow/magenta%1}' 'foo'
```

Take a deep breath, relax, and add whitespace for the same result.
Space and tabs are accepted.

```bash
cecho '{ # yellow / magenta   % 1 }' 'foo'
```

<pre><span style="color:yellow; background-color: magenta;">foo</span></pre>

# Specification

`cecho` takes at least 1 argument: a format and as many arguments as necessary.

It works in the style of `printf`.

## Format

The format is a string of characters made of regular strings and placeholders.

Example: `Some text followed by a specifier {}`

## Specifiers

Specified in `{}`, rust/python style.

The specifier may contain the following information.

### A color

The color is indicated with `#`.

The color may be regular or bright.

The color is specified with different formats: code (number), single letter or word.

The color short names are based on the
RGB: `r`ed, `g`reen, `b`lue,
and CMYKW: `c`yan, `m`agenta, `y`ellow, blac`k` + `w`hite conventions.

Lower case is the normal variant, upper case is the bright variant.

#### Named colors reference table

| Name    | code | short |  long   | code bright | short bright | long bright |
|---------|:----:|:-----:|:-------:|:-----------:|:------------:|:-----------:|
| Black   |  0   |   b   |  black  |      8      |      B       |    BLACK    |
| Red     |  1   |   r   |   red   |      9      |      R       |     RED     |
| Green   |  2   |   g   |  green  |     10      |      G       |    GREEN    |
| Yellow  |  3   |   y   | yellow  |     11      |      Y       |   YELLOW    |
| Blue    |  4   |   b   |  blue   |     12      |      B       |    BLUE     |
| Magenta |  5   |   m   | magenta |     13      |      M       |   MAGENTA   |
| Cyan    |  6   |   c   |  cyan   |     14      |      C       |    CYAN     |
| White   |  7   |   w   |  white  |     15      |      W       |    WHITE    |

[//]: # (TODO: Html hex codes)

#### Foreground/background

The color may be applied to either the foreground, the background or both.

Split with slash foreground over background.

`#foreground/background`

Either side is optional.

`#red` is a red font, regular background.
`#/green` is a regular font, green background.
`#red/green` is a red font, green background.
`#/` is no color, same as not specifying a color.

#### A reference

Unlike `printf` where the order of arguments is forced,
`cecho` may use arguments in any order with `%x` 
where `x` is an integer referring to the arguments passed to `cecho`.

To use it like `printf` do

```bash
cecho '{} {} {}' a b c
``` 

`a b c`

To use it like a python format do

```bash
cecho '{3} {2} {1}' a b c
``` 

`c b a`

## Goals

No `-` `--` `-n` `-e` etc. ambiguity as in `echo`

No `%x` and other random characters like in `printf`

No looking up for color code indexes nor figuring out which option is correct like in tput.

Use full english nouns but allow existing styles for easy migration form those tools.

Support color.

Fast! Native binary.

Format support à la printf but with modern format specifiers:

* selectors
    * positional `{}`
    * indexed `{%1}`
* styles
    * simple colors `{#red}`
    * reset to default after printing? `{#red!preserve}` or auto reset? 🤔
    * any color `{#A03472}`
    * backgroud colors `{#white/red}`
* Styles: bold, italic, blink, reset, ... `{!bold}`
* Position on screen `{@5,10}`

## Motivation

Must be simple to start with and not trap people by default!

Must be powerfull enough to not require extra tooling if it's easier, be it

* bash's number formatting trick `$(([##7]v))` for format the variable v in base 7 for instance
* printf's decimal formatting
* escape sequences or tput coloring

I'm tired of looking up color codes and have unreadable color escape sequences.

I don't want to play with `tput setaf 3` and have an unreadable echo statement.

I'm tired of looking up printf's formats, I didn't touch C for years. I use modern languages with friendlier formatting
options.

I don't want to build another layer of bash workarounds. It still contains all of echo's flaws regarding `-` `-n` `-e`
and escape sequences.

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/cecho

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/cecho2

https://github.com/ununhexium/configfiles/blob/master/.local/scripts/bases

I don't want to start a real scripting language, or even worse: the JVM, to highlight a few lines of output in a bash
script.

It may as well be comprehensive so there's 1 tool that gives access to all of the shell's printing capabilities if the
format string remains terse.
I'm likely engaging on a slippery slope but we'll see if it's possible.

I don't want to import and redeclare the colors each time I use them. This should be done once and for all.

https://stackoverflow.com/questions/5412761/using-colors-with-printf

### Limitation in current tools

Mix of bash's base formatting, sprintf's decimal formatting, echo workarounds, escape sequences or tput, alternative
tool that auto-color based on various criteria

### Alternatives?

TODO: is there anything that comes close to this?

If yes list here.

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

### Position

`@row,column`

Support relative movement?

### Styles

`!style`

TODO: local style (within the {})

TODO: continuous style (from the marker until cancelled)

### Options

Similar to regex options `(?=...)`

`?option`

## TODOs

Index is 0-based or 1-based?

Error message improvements: position hint

Make the arguments to be formatted optional when there are no format specs


### Decimal formatting

TODO: is there another readily available program that could do the number formatting and let me not re-implement the
wheel?

Leave this to `printf` ?

```bash
cecho '{02.02}' '9.8'
```

`09.80`

TODO: extends the spec to support generalized number formats

https://alvinalexander.com/programming/printf-format-cheat-sheet/

### In-place formatting

```bash
cecho '{this is blue#blue}'
```

<span style="color:blue;"> `this is blue` </span>

```bash
cecho '{this is not\#blue}'
```

`this is not#blue`

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

<span style="color:blue;"> `003.14159` </span>

TODO: what if the terminal only supports 8 colors? Find the closest color that matches?

## Documents and references

### ANSI escape codes gist

https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

