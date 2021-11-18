# BlueBird

Do you want BlueJ as an IDE for an assembly language? If not, screw you! I do!

BlueBird is the functional BlueJ assambly language that provides visual clarity with system calls.  
_Now turing complete!_

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

## Example program

This example program adds 15 to the number requested as input when run.

![Example program in BlueJ](https://i.imgur.com/grJxu4I.png)

```java
// main.java
public class main extends Class1
{
    static int call = 5; // get input
}
```

```java
// Class1.java
public class Class1 extends Class2
{
    static int addi = 15; // add 15 (immediate)
}
```

```java
// Class2.java
public class Class2 extends Class3
{
    static int call = 1; // print output
}
```

[Download the BlueJ project]()

## Output

The output of the previous command is automatically piped to the next command. This applies for functions (separate inheritance chains in BlueJ) too.

## Static field syntax

Field name is anything except whitespace. If multiple are present, the first valid one is parsed with precedence shown [below](#field-names) with the jump operation beeing parsed first. The static field can start with either `private`, `public` or nothing.

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

# System calls

- `1` - Print output
- `5` - Read input
- `10` - Exit program

## Register ids

- `0` - Zero constant
- `1` - Usable register 1
- `2` - Usable register 2
- `3` - Usable register 3

## Notes

Jumps can only be performed one level, nested jumps are not allowed.
