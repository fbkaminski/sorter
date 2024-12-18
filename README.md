### About

This is an implementation of a sorter for positive integer numbers implementing
a bit vector as the underlying sort structure as described in the book "Programming Perls" by John Bentley.

The sort should read a file filled with integers from 0 to a 1000000 (you can define the limit as you wish) separated by a new line.

### Why

Its a nice sorting trick, possibly the fastest and one where you can sort with partial data
which can help not blowing the memory if you have millions of positive numbers to sort.

The caveat is that is only suitable for positive integers without duplication.

### Usage

To generate such a file you can use the command generate as in

```
sorter generate numbers.txt
```

and then sort then to a output file with the bitvec sorter with something like

```
sorter sort bit number.txt sorted.txt
```

which will create the file with the sorted numbers in the 'sorted.txt' file.

You can also check if any number is missing with

```
sorter check number.txt sorted.txt
```

For comparison i also implemented a sorter using the rust vector sorter and a radix sort

```
sorter sort rust number.txt sorted.txt
```
and
```
sorter sort radix number.txt sorted.txt
```
respectively


### Building

```
cargo build --release
```

### Notes

Theres no proper hardening in the processing of the input. It expects a well formatted input file
with a list of integers separated by a line end. Anything different than that and the program will break.
To facilitate the checking the generate creates a list of N integers creating some gaps and then shuffles them.

### License

None for now
