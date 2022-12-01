# Advent of Code

After a few years of playing AoC this is my attempt to move all my code into a
single repo.

My goals with this setup:

* Support multiple years.
* Support multiple languages.
* No need to support multiple solutions for the same day/part in the same
  language.
* Store both actual inputs and test inputs.
* Optionally store solution for inputs.
* Nice tables showing how each language performs on the inputs.
* Be able to run a single input in a single language (for development).

## Project structure

### The `input` directory

This directory stores all the input and solution files. File names must
follow the following convention:

* Input: `YYYY_DD_0_TITLE.txt`
* Solution: `YYYY_DD_P_TITLE.txt`

The file name begins with the year, day and (for solutions) the part, which
together identify the solver to use. The file names end with the URL-encoded
title of the day for actual challenges, or a test identifier for test cases.
Test identifiers start with a `T` and are followed by numbers, e.g. `T1`, `T2`
or similar. (Even when test cases come from the AoC website, they don't have an
official name, so pick any test identifier you please.)

### The language directories

Each language has its own subdirectory. The layout of these directories depend
on the language, but all of them shall contain an executable in the top level
called `aoc` (which may be just a shell script to invoke the actual solvers).

The `aoc` executable takes the inputs to solve as command line
arguments. Each input is described in 4 consecutive arguments:

* Year
* Day
* Part
* Path to input file

For each input the program shall print a line on `stdout` with two columns
separated by a space. The columns are:

* Solution: the solution calculated for the input or the string `N/A` if there
  is no solver implemented for the given input.

* Runtime: measured in nanoseconds. If there is no solver, the runtime is
  suggested to be reported as 0.
