# BlueBird

Do you want [BlueJ](https://www.bluej.org/) as an IDE for an assembly language? If not, screw you! I do!

BlueBird is the functional BlueJ assembly language that combines visual clarity with system calls.  
_Now turing complete!_

The language is expressed in java class files and uses static fields as instructions. They are then linked to gether with arrows (they extend each other) and are executed in a chain starting from `main`.

Specifications for the language and binary executables can be found in [`specification.md`](specification.md).

Factorial example and running instructions for it can be found [below](factorial).

**Usage (after cloning):**  
Compile: `cargo run --bin bluebird_compiler [path_to_BlueJ_folder]`  
Run: `cargo run --bin bluebird_emulator [binary_name].bb`

# Examples

## Add immediate (constant)

This example program adds 15 to the number requested as input when run.

![Example program in BlueJ](https://i.imgur.com/grJxu4I.png)

Click `Inspect` on the main method in BlueJ shows the call tree for that execution chain.

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
public class Class2
{
    static int call = 1; // print output
}
```

## Factorial

Factorial of `n >= 2` implemented in BlueBird. It works for numbers up to 12 (output is within 32 bit integer).

**All nodes:**  
![Image of factorial implementation](https://i.imgur.com/Y7JCj2O.png)

**Call tree:**  
![Call tree of factorial implementation](https://i.imgur.com/VuRcWeo.png)

Each "column" in the first image is roughly equivalent to each line in the following Python code:

```python
def factorialWithoutMul(inp5):
    ans6 = inp5 # variable to store the final factorial
    outer1 = 1 # outer loop counter
    while True:
        sum3 = 0
        inner2 = 0 # inner loop counter
        while True:
            sum3 += ans6
            inner2 += 1
            if inner2 == outer1:
                break
        ans6 = sum3
        outer1 += 1
        if outer1 == inp5:
            break
    return ans6
```

#### How to compile and run:

> The following instructions require you to have `cargo` installed

1. Clone this repository (then cd into that folder)
2. Extract the [`factorial.zip`](factorial.zip) file to a folder called `factorial`
3. Execute `cargo run --bin bluebird_compiler factorial` to compile
4. Execute `cargo run --bin bluebird_emulator factorial.bb` to run the program
5. Type an integer, press enter
6. PROFIT!!! (the factorial is printed)

The project [`factorial.zip`](factorial.zip) can of course be opened in BlueJ to edit and display the program.

# Language guide

## Quick start - create a program

1. Create a BlueJ project
2. Create a class named `main` (this will be the entry point)
3. Add classes with static fields as instructions
4. Draw arrows from main to those classes
5. Compile with `bluebird_compile [FOLDER]` that points to the BlueJ project folder
6. Run the program with `bluebird_emulator [BINARY]` that points to the `.bb` binary from step 5

## Output

The output of the previous command is automatically piped to the next command, following the arrows in BlueJ. This applies for functions in BlueBird too.

## Static field syntax

Field name is anything except whitespace. If multiple are present, the first valid one is parsed with precedence shown [below](#field-names) with the jump operation beeing parsed first. The static field can start with either `private`, `public` or nothing.

`static int [field_name] = [value]`  
`value := integer` (keywords like `Infinity` are not accepted)

`static String [field_name] = [value]`  
`value := "text"`

> Note: each block/class needs a static field for the whole program to work. Use syscall 0 to do notning as a label to jump to.

## Field names

- `call` = `int` - System call code to apply (takes in integer)
- `save` = `int` - Save previous output to register with id
- `load` = `int` - Load value from register with id
- `jump` = `String` - Jump to block with classname (cannot be done from detached chain)
- `add` = `int` - Add the value from the register with id `int`
- `addi` = `int` - Add a the immediate `int`
- `skipeq` = `int` Skip next instruction if output equals value in register with id

# System calls

- `0` - Do nothing (use this as a jump label)
- `1` - Print output
- `5` - Read input
- `10` - Exit program

## Register ids

- `0` - Zero constant
- `1` - Temporary register 1
- `2` - Temporary register 2
- `3` - Temporary register 3
- `4` - Reserved by assembler
- `5` - "Persistent" register 1
- `6` - "Persistent" register 2
- `7` - "Persistent" register 3
- `8` - "Persistent" register 4
- `...`

> Note: all registers are initialized with value `0`  
> Note: persistent registers are exactly the same as the temporary ones
