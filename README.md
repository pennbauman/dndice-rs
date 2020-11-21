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
TO DO

### Options
`--help`, `-h` Print a help menu.

`--version` Print the version number.

`--number [number]`, `-n [number]` Repeat command the provided number of times.

`--quiet`, `-q` Print only essential information from command.
