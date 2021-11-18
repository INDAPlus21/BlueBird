# Instructions

| **operation** | **immediate (only positive)** |
| :-----------: | :---------------------------: |
|     `000`     |            `00000`            |

Operation range: 0-7  
Immediate range: 0-31

# Operations

| Operation                       | Symbol   | Argument                                                         | Binary | Decimal |
| ------------------------------- | -------- | ---------------------------------------------------------------- | :----: | :-----: |
| Reserved                        |          |                                                                  | `000`  |   `0`   |
| Call syscall                    | `call`   | immediate                                                        | `001`  |   `1`   |
| Save to register                | `save`   | register id                                                      | `010`  |   `2`   |
| Load to register                | `load`   | register id                                                      | `011`  |   `3`   |
| Jump to address                 | `jump`   | offset (-15 to 15, leading bit is sign, _not_ two's complement ) | `100`  |   `4`   |
| Add from register               | `add`    | register id                                                      | `101`  |   `5`   |
| Add immediate                   | `addi`   | immediate                                                        | `110`  |   `6`   |
| Skip next instruction if equals | `skipeq` | register id                                                      | `111`  |   `7`   |

# Register ids

- `0` - Zero constant
- `1` - Usable register 1
- `2` - Usable register 2
- `3` - Usable register 3
- `4` - Reserved for assembler

# System calls

- `0` - Reserved for assembler
- `1` - Print output
- `5` - Read input
- `10` - Exit program
- `16` - Run function
- `20` - Start function (only called by the assembler)
- `21` - Return function (only called by the assembler)
