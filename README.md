# adventofcode
Solutions to the https://adventofcode.com/ puzzles

## 2016
These are in C++11 and there's a Linux and G++ Makefile provided.
I've just copied these files in from an older repo from back in those years.
Looks like I got nearly all done except the day 25 part 2.

## 2017
These are in C++14 and there's a Linux and G++ Makefile provided.
I've just copied these files in from an older repo from back in those years.
Only got 10 days done.

## 2018
There are written in Rust stable 1.39.0 from 2019-11-04.
These I completed in the days leading up to 1 December 2019.
I wanted to get a Crate etc. setup so I'd be ready to start on the 2019 puzzles.
Also, I wanted to learn some of the Rust graph libraries so I'd have a handle on them
for the later 2019 puzzles that (I'm sure) will need graph algorithms.

## 2019
Hmmm.... didn't write many notes here...

## 2020
Well what a year hey!
I've learnt a bit of Rust, and been brushing up on my number theory
so I hope I don't get so stuck towards the end as I did last year.
I should still go back and fix up the 2019 answers...

For me, the puzzles are released at 3PM - so most weekdays I'll be
doing these at work.

I've also started re-doing some of the 2019 puzzles to try
and have some ready solutions to expected issues....

Notes about each day:

1. Finding one combination from a list of integers.
   * Was interrupted just before 3PM and didn't get to start this
     on time.
   * I first solved this with 2/3 nested loops.
   * However, I had Itertools ready as I knew part 2 could get more
     difficult - but it didn't.
   * Refactored to use Itertools after submitting. It's a little slower,
     but fine in release mode.
2. Parsing input then checking "passwords" based on parsed data.
   * 13m/17m - 2552th Bad.
   * I knew parsing in Rust is not straight forward, and
     didn't have this ready. Not really impressed with myself.
     Will be spending tonight getting some parsing stuff ready for future
     puzzles.
3. Simple looping through text input and couting.
   * 8m/10m - 944th
   * Again I was distracted at work and wasn't prepared and only
     started about 2 minutes past. Had a chance to do well here
     if I was available a bit earlier.
4. Input validation.
   * Too long. Unrated.
   * Again distrated at work. Didn't start until 3:08.
   * Then discovered it was a boring validation puzzle and gave up.
   * Input validation - super important - but also super boring as
     it's **real world** shit that professional people have to deal with.
     I've spend too many years dealing with that to find it fun.
   * I'm glad it's a puzzle - it shows of what you need to deal with - but
     that doesn't mean it's fun for a 39 year old with experience.
   * Has proved that my scan(...) utility is really well suited for
     these puzzles - separating delimitation (e.g. take_digits())
     from parsing/validation (e.g. ensure it's six digits long)
5. Binary number encoding, as letters, finding gaps
   * 7m/11m - 728th
   * Fun little puzzle.
   * I mapped FL/BR to 0/1, then folded (2*acc + new)
   * Should have mapped to 0/1 chars then just called from_string
     with a binary encoding - but I may have taken a while to find
     that in the rust std library.
  * Rust Vec::windows() made part 2 easy.
  * Forgot to sort which cost me a little while.
  * Also away and did it on my laptop which slows me down a little.
  * Happy with today's outcome.
6. Counting matching characters, in blank-line-separated groups
   * Was out and about on the weekend. Didn't start until a couple
     hours past the release. No score.
7. AT WORK - Not commited yet     
8. Hand-held Console - possible start of a re-useable computer...
   * Was at the salon getting my hair done today, then dinner.
   * Didn't start until late. Was tired.
   * Took me about 25 mins - not good.....
10. Recursive function, needing memorization
    * Again - busy at work. Waited until I got home.
    * Took about 20 mins, but I did search for then hand-roll a memoriztion
      solution.
    * Then I re-factored into a re-useable memorization function for
      future days.
    * Then I also implemented a bottom-up solution.
11. Game of life clone - iterate until stable config
    * 31 minutes - too slow!
    * I didn't have my character grid stuff prepared. I knew
      this would be coming - and I find this part of Rust very
      difficult because the isize/usize stuff really seems to get
      in the way.
    * Re-write with a better library after the fact.
12. 2D movement and rotation.
    * 27 minutes - too long - didn't have 2D rotations ready.
      Should have known this was coming!
13. Number theory
    * Was away for the weekend, so was late doing this one.
    * Knocked the first part over in 3 mins.
    * The second part was dreadful. Just tired from the weekend.
    * It took close to an hour - with some reading the news, chatting
      etc. in between times.
    * Eventually realized to only consider the first two busses, then
      add in the third bus....
14. Bit masking
    * 43 mins.
    * Meh - just a boring puzzle.
    * I had had the day off after a big social weekend, and to be honest
      the puzzle was just not interesting. I lost focus and mucked around.
15. Iterative sequence with memory
    * 15min/17min
    * I wrote part 1 using the dictionary rather than linear search through
      the history. Made part 2 easy.
    * Made some off-by-one errors - which is why it took so long.
    * Part 2 just worked - in about 2.5 seconds.
    * Improved to remove one HashMap lookup for previously spoken numbers -
      which improved performance 25% to about 2 seconds (on the i7-6770HQ
      I was using).