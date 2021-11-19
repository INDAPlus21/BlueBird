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

Click `Inspect` on the main method in BlueJ shows the call tree for that execution chain.

## Factorial

Factorial of `n >= 2` implemented in BlueBird.

![Image of factorial implementation]()

Each "column" is roughly equivalent to each line in the following Python code:

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

## Output

The output of the previous command is automatically piped to the next command. This applies for functions (separate inheritance chains in BlueJ) too.

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

# System calls

- `0` - Do nothing
- `1` - Print output
- `5` - Read input
- `10` - Exit program

## Register ids

- `0` - Zero constant
- `1` - Temporary register 1
- `2` - Temporary register 2
- `3` - Temporary register 3
- `5` - "Persistent" register 1
- `6` - "Persistent" register 2
- `7` - "Persistent" register 3
- ...

> Note: all registers are initialized with value `0`  
> Note: Persistent registers are exactly the same as the temporary ones
