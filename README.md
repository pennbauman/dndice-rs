# DnDice
DnD format command line dice roller and statistics generator



## Usage

	dndice [command] [dice] [options]

### Commands
`dice [dice]` Roll provided dice. This command is used if not command is provided

`stats [method]` Generates a set of six statistics with the provided method.

- `std` or `standard` Use the standard 5th edition statistics array.
- `1d20` Roll 1d20 for each score.
- `4d6` Roll 4d6 and sum the largest 3 for each score.


### Dice Format
Dice expressions are formatted as an equation using dice sets, numbers, and the `+`, `-`, and `*` operators. Each dice set is written `#d#`, where the first number is the quantity of dice rolled and the second number is the maximum number on each die, and its result is the sum of the values rolled on each die. When the dice expression is rolled, the value of the equation is calculated using the results of the dice sets and the standard order of operations.

#### Examples
`2d6 + 3` means two 6 sided dice will be rolled and 3 will be added to the sum of their results.

`1d4*10` means a 4 sides die will be rolled and its result multiplied by 10.


### Options
`--help`, `-h` Print a help menu.

`--version` Print the version number.

`--number [number]`, `-n [number]` Repeat command the provided number of times.

`--quiet`, `-q` Print only essential information from command.
