# BlueBird

The functional BlueJ assambly language. _Now turing complete!_

If it is valid Java code, it _should_ compile to a BlueBird binary.  
Any BlueBird program is also a valid BlueJ project.

# Language guide

## Quick start

1. Create a BlueJ project
2. Create a class named `main` (this will be the entry point)
3. Add classes with static fields as instructions
4. Draw arrows from main to those classes
5. Compile with `bluebird_compile [FOLDER]` that points to the BlueJ project folder
6. Run the program with `bluebird_emulator [BINARY]` that points to the `.bb` binary from step 5

## Static field syntax

Field name is anything except whitespace. If multiple are present, they are parsed in the order shown [below](#field-names) with the jump operation beeing parsed first.

`static int [field_name] = [value]`  
`value := integer` (keywords like `Infinity` are not accepted)

`static String [field_name] = [value]`  
`value := "text"`

## Field names

- `call` = `int` - System call code to apply (takes in integer)
- `save` = `int` - Save previous output to register with id
- `load` = `int` - Load value from register with id
- `jump` = `String` - Jump to block with classname
- `add` = `int` - Add the value from the register with id `int`
- `addi` = `int` - Add a the immediate `int`

## Register ids

- `0` - Zero constant
- `1` - Usable register 1
- `2` - Usable register 2
- `3` - Usable register 3

## Notes

Jumps can only be performed one level, nested jumps are not allowed.
