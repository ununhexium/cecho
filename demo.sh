#!/usr/bin/bash

dir=$(dirname $0)

[[ -e "$dir/target/release/cecho" ]] || cd "$dir" && cargo build --release

cd "$dir/target/release/"

function pause {

    echo
    if [[ -z ${skip+x} ]]
    then
        echo -e "\e[2mPress the ANY key to continue ;P\e[0m"
        read -n 1 -s -r
        echo -e "\e[2J"
    fi

}

pr() {

    echo
    set -x
    ./cecho "$1" "${@:2}"
    { set +x; } 2>/dev/null;
        echo

        echo
        pause
        echo
    }

prnp() {

    echo
    set -x
    ./cecho "$1" "${@:2}"
    { set +x; } 2>/dev/null;
        echo

    }

# Clear the screen
echo -e "\e[2J"

base() {

    echo 'Basic features'
    echo

    echo 'Canonical cecho'
    pr '' 'Hi there!'

    echo 'Add a color specifier with a parameter {color=...}'
    pr '{color=red}' 'Look!'

    echo 'Add a color specifier with a symbol {#...}'
    pr '{#green}' 'Easy colors!'

    echo 'Arguments are optional if there is no specifier'
    pr 'Hello, world!'

    echo 'Positional arguments'
    pr '{} + {} = {}' 1 2 3
}

color_features() {

    echo 'Add colors everywhere'
    echo

    echo 'Long notation'
    pr 'The long style notation uses a word: {color=green}' 'color='

    echo 'Short notation'
    pr 'The short style notation uses a hash {color=blue}' '#'

    echo 'The colors can be specified using 3 notations'
    pr 'The ANSI code: {color=1}, a single letter: {color=r} and a word: {color=red}' '1' 'r' 'red'

    echo 'Background color'
    pr 'The background color can be selected using {#/blue}' '#/color'

    echo 'You can change the foreground and the background'
    pr '{#blue/white}' 'Blue over White'

    echo 'Or only the background'
    pr '{#/g}' 'Like this'

    echo 'Arbitrary colors'
    pr '{#w/54370f} and {#e8acca}' 'Poop' 'Toilet paper'

    echo 'Real rainbow'
    pr "$(sed 's/.*/{%1 #&} /;' <<EOF | tr -d '\n'  | cat
ff0000
ff4400
ff8800
ffc800
f2ff00
aeff00
6aff00
26ff00
00ff1a
00ff5e
00ffa2
00ffe6
00d9ff
0095ff
0051ff
000dff
3700ff
7700ff
bb00ff
ff00ff
EOF
)" 'X'

    pr '{#ff0000}'
}

col_row() {
    ./cecho "{#$1}\t{#$2}\t{#$3}\n" $1 $2 $3
}

color_detail() {

    pr 'Uses the {#r}{#g}{#b} and {#c}{#m}{#y}{#k/w}{#w} conventions' R G B C M Y K W
    pr 'Long form {#r} {#g} {#b}' Red Green Blue
    pr 'and {#c} {#m} {#y} {#k/w} {#w} conventions' Cyan Magenta Yellow Black White

    echo 'Here is a table of the available colors'
    echo -e 'Code\tLetter\tWord'

    ./cecho "{#0/white}\t{#k/white}\t{#black/white}\n" 0 k black
    col_row 1 r red
    col_row 2 g green
    col_row 3 y yellow
    col_row 4 b blue
    col_row 5 m magenta
    col_row 6 c cyan
    col_row 7 w white

    pause

    echo
    echo 'The bright bit changes the color intensity'
    echo 'To use it, use the upper case notation'
    echo -e 'Code\tLetter\tWord'
    ./cecho '{#k/w} {#K/w}\n' black BLACK
    col_row 9 R RED
    col_row 10 G GREEN
    col_row 11 Y YELLOW
    col_row 12 B BLUE
    col_row 13 M MAGENTA
    col_row 14 C CYAN
    col_row 15 W WHITE

    echo 'The color codes come from the ANSI specification'
    pr 'It has 4 components in 4 bits: 0b{#k/w}{#k/b}{#k/g}{#k/r}' L R G B
    pr '{#w} {#b} {#g} {#r}' Brightness Blue Green Red
    pr '0b{#b}{#k/g}{#k/r} = 3 is {#y}' 0 1 1 yellow
    pr '0b{#k/b}{#g}{#k/r} = 5 is {#m}' 1 0 1 magenta
    pr '0b{#k/b}{#k/g}{#r} = 6 is {#c}' 1 1 0 cyan
    pr '0b{#k/b}{#k/g}{#k/r} = 7 is {#w}' 1 1 1 white

    echo 'Comparison for the regular and bright color modes'
    ./cecho '{%1#1}{%1#g}{%1#blue}' '█'
    echo
    ./cecho '{%1#9}{%1#G}{%1#BLUE}' '█'
    echo
    pause
    echo
}

styles() {
    echo 'The ANSI spec supports 8 styles and can be combined with colors'

    echo Bold
    pr '{#y}, {#y !bold}' regular bold

    echo 'Dim or faint'
    pr '{}, {!dim} or the equivalent {!faint}' regular dark faint

    echo 'Italic'
    pr '{}, {!italic}' regular 'fancy'

    echo 'Underline'
    pr '{}, {!underline}' regular underlined

    echo 'Blink'
    pr '{}, {#R !blink} ; {#k/C !blinking}' regular 'look at me' 'no, me!'

    echo 'Inverted / reversed'
    pr '{} VS {!invert} {#r !inverted} {#g !inverse} {#b !reverse} {#c !reversed}' Normy can\'t do like the others

    echo 'Hidden'
    pr '-->{!hidden}, {!invisible}<--' "Cant see me XP" spooky

    echo 'Strike through'
    pr '{}, {#r !strike} and also {#M !strikethrough}' regular wrong wrong

}


special_chars() {

    echo 'Use the usual c-style escape codes are supported'

    echo 'Bell'
    pr '{#yellow}\a!' Ding

    echo 'Backspace'
    pr '{#green}\b{#r}' 'Whoo~' 'ps!'

    echo 'Tabulation'
    pr '\t{#k/m}\t{#k/c}\t{#k/y}' '⇥' '⇥' '⇥'

    echo 'New line'
    pr '{#g}\n{#b}' new line

    echo 'Vertical Tab'
    pr '1\v2\v3'

    echo 'Form feed'
    pr 'Page 1\fPage 2'

    echo 'Carriage return'
    pr '{#black/white}\r{#red}' 'I hate cecho' 'I love '

}

quality_of_life() {
    echo 'mix order'
    echo 'mix long short'
    echo 'mix word= symbol'
    echo 'Special cases'
}

#base
color_features
#color_detail
#styles
#special_chars


