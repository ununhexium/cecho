#!/usr/bin/bash

dir=$(dirname $0)

[[ -e "$dir/target/release/cecho" ]] || cd "$dir" && cargo build --release

cecho="$dir/target/release/cecho"

function pr {
    set -x

    $cecho "$1" "${@:2}"

    { set +x; } 2>/dev/null
    echo
    echo
    echo -e "\e[2mPress the ANY key to continue ;P\e[0m"
    [[ -z ${skip+x} ]] && read -n 1 -s -r
    echo -e "\e[2J"
    echo
}

# Clear the screen
echo -e "\e[2J"

echo 'Simple echo'
pr '' 'Simple echo'

echo 'Omit the arguments if there is no specifier'
pr 'Hello, world!'

echo 'Canonical echo'
pr 'Specifier: {color=magenta}' 'value'

echo 'Positional arguments'
pr '{}+{}={}' 1 2 3

echo 'ANSI RGB with long notation'
pr '{index=1 color=1}{index=1 color=g}{index=1 color=blue}' '█'

echo 'ANSI bright RGB with short notation'
pr '{%1#9}{%1#G}{%1#BLUE}' '█'

echo 'Comparison for the regular and bright color modes'
$cecho '{%1#1}{%1#g}{%1#blue}' '█'
echo
$cecho '{%1#9}{%1#G}{%1#BLUE}' '█'
echo
echo

echo 'Use the usual c-style escape codes'

echo 'Bell'
pr '{#yellow}\a!' Ding

echo 'Backspace'
pr '{#green}\bps' 'Whooo'

echo 'Tabulation'
pr '\t{#magenta}' 'tab'

echo 'New line'
pr '{}\n{}' new line

echo 'Vertival Tab'
pr '1\v2\v3' ''

echo 'Form feed'
pr 'Page 1\fPage 2' ''

echo 'Carriage return'
pr '{#black/white}\r{#red}' 'I hate cecho' 'I love'

echo 'Whitespace is allowed aroung the specifiers'
pr '{ %1 #yellow/magenta }' 'WEIRD'

echo "The specifier's parts order doesn't matter"
pr '{%1 #blue}' XXX
pr '{#blue %1}' XXX

