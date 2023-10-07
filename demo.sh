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
    [[ -z ${skip+x} ]] && read -n 1 -s -r -p "Press the ANY key to continue ;P"
}

echo 'Simple echo'
pr '' 'Simple echo'

echo 'Positional arguments'
pr '{}+{}={}' 1 2 3

echo 'ANSI RGB'
pr '{%1#1}{%1#g}{%1#blue}' '█'

echo 'ANSI bright RGB'
pr '{%1#9}{%1#G}{%1#BLUE}' '█'

echo 'Comparison for the regular and bright color modes'
$cecho '{%1#1}{%1#g}{%1#blue}' '█'
echo
$cecho '{%1#9}{%1#G}{%1#BLUE}' '█'
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



