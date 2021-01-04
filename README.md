## Egg
An application that will take words and give you a list of animal names ranked based on how similar they are to the words.

## Motivation
I wanted to have a common theme for the names of my personal projects. My first project made a nice acronym with an animal name (COWS), so I decided to give all my projects animal names.

Rather than having the names be arbitrary, I decided to make a program that would let me pick names that were similar to a short description of the project.

## Build status

Initial working release completed. Further improvements to the algorithm may give slightly different rankings to the animal names, but i don't expect massive changes.

To Do List:

- [ ] Add flags to the cmd line
- [ ] Change the internal data structures
- [ ] Include the animal list in the binary
- [ ] Improve the list of animals
- [ ] Update tests

## Code Example

```
$ ./egg.exe 2 reverse shredder

Animals with rank: 23
SHREw

Animals with rank: 14
gRaSSHopper
kingfiSHeR
SHaRk
SquiRREl
```

## Installation
Currently requires a file called `animals.txt` in the same folder as `egg.exe` which contains the name of all the animals to search through. The `animals.txt` file is a list of names seperated by a newline. 

## Tests
They are rust tests, so just go something like:
```
$ cargo test
```

Apparently the tests fail currently
