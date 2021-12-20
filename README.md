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

## 2021

Another year down! It's been a big one. A lot of personal changes.
As such there has not been much personal programming this year.

Still going to try in rust, using the same framework as 2020.

Still in eastern Australia - so puzzles are released at 3PM.

My results (for days I was able to start at 3PM):
| Day | Part 1        | Part 2         |
|-----|---------------|----------------|
| 1   | 2:24 / 933    | 9:00 / 2049    |
| 5   | 13:26 / 1243  | 18:59 / 834    |
| 10  | 10:57 / 1814  | 21:40 / 2087   |
| 11  | 13:30 / 510   | 16:05 / 521    |
| 13  | 14:47 / 1006  | 29:50 / 2031   |
| 14  | 19:56 / 3278  | 44:08 / 1834   |
| 15  | 13:10 / 629   | 26:23 / 419    |
| 17  | 21:03 / 1055  | 25:43 / 773    |

Puzzles more than 24 hours late:
| Day | Puzzle(s) |
|-----|-----------|
| 3   | Both      |
| 8   | 2nd       |

Notes on each day:
1.  Came home from work early and got ready.
    Pretty happy with 2:24 for part 1. Got stuck with
    Rust iterators and references and slices vs slices of
    references vs references to slices of references :(
    this must be one of the worst parts of rust for simple
    projects like this. Feel like I need to write "collect_vec"
    and "windows_copied" to make this all much easier. Perhaps
    that's a job for tonight. Got to go as I need a lesson for
    using the hedge trimmer, and have an appointment at the barber
    at 6 PM.
2.  Was ready and waiting, but got a call at work at 2:59.
    Didn't do anything until I got home.
3.  Christmas party. No work done. Found this puzzle very boring.
4.  Busy Saturday doing things. Totally missed 3PM.
5.  Busy Sunday doing lots of wood-working making Christmas presents -
    see https://github.com/kezenator/pico-lamp. Made it upstairs in
    time for 3PM but pretty tired and hungry.
    Got really tripped up about not considering diagonals in part 1.
    Annoyed my Line class didn't already have "points_on_line()" method.
    Pretty slow. Not particularly impressed with this result - but stil
    under 1000th for part 2. I thought this puzzle was a little bit of fun.
6.  Was at work and IT changes mean I don't have access to a VM with rust
    on it at the moment. Then found an online Rust RPEL loop and decided to
    use that. Could have got part 1 done quickly if I was ready. Got stuck on
    part 2. I was going to use memorization to calculate 256 as 8 halves combined.
    Then I realized the real solution. So a bit slow. Wrote a real version
    and checked it in when I got home. Done in 9:14 / 2648th and 25:55 / 2924th,
    so perhaps no bad considering the slow start. I'm not putting this in the table above.
7.  Busy working. Fun little puzzle. I got the triangle number equation,
    but missed the median/mean part.
8.  Bad day at work, and was out. Was pretty tired when I got home and
    really struggled to get anything. I went down totally the wrong track.
    I think the final solution is pretty easy to read.
    It was an enjoyable puzzle to do on day 9 - but still took me a good hour.
9.  Working at 3. Fun puzzle. First use of graph algorithms.
    I got stuck with the meaning of 9 and edges in part 1.
    Part 2 I got stuck using "strongly connected" rather than "breadth-first search reach".
    Good refresher into the rust pathfinding library that will hopefully
    help me on future days.
10. Fun little puzzle. Rust enums made it pretty readable.
    Didn't to particularly well in the speed department - but it was a Friday
    afternoon.
11. Big day woodworking today. I read the puzzle and my heart sank. I don't
    enjoy these ones. I sat around for a few minutes not having the energy to
    start. But then thought lets get it done. The funny thing I got the best result
    yet this year! Perhaps that's just because it's Friday night in the US?
    Anyway - done.
12. What a shit show! I got way to involved in AStar algorithm to find shortest
    paths, when actually we only want to count *all* paths - which of course is
    in the first line of the description. Took an hour and didn't get far.
    Also hung over from another Christmas party - so that didn't help.
    Eventually gave up, looked at redit, and was like "oh of course".
    Basically copied a solution from there.
13. Pretty slow. Mixed up with my point co-ordinates having inverse Z direction.
    Missed that the input had multiple folds. Fun puzzle.
14. Was at work. Started, but got distracted by a someone asking me questions
    about work :). Thought I was doing **really** badly, but seems I picked up
    a lot of time in part 2 - maybe would have done OK if I didn't get
    distracted. Immediately realized the number of pairs was the key in
    part 2. Got slowed a bit because I wrote the example counts wrong on my
    notepad and got confused. Counting pairs, then dividing by 2 and rouding up
    (because each letter is counted in two pairs - except the start and end
    letter) seems to work - but I haven't proved that it's correct for all
    inputs.
15. Just got home in time. Standard ASTAR algorithm. Got tripped up with the
    Rust borrow checker again. Basically the neighbours function is much easier
    to write if it returns a vector of neighbours rather than an iterator
    that returns neighbours. Lost quite a few minutes on this. Other trick
    was the addition is not mod 10 - but (((a + b - 1) % 9) + 1).
    Best score yet this year!
16. Great Day! Great Puzzle! It's been one of the best days of the year.
    Not the most fun, but one of the most productive. And a puzzle that's right
    up my street. Unfortunately I'm an engineer - and network packet decoding
    means I look for accuracy not development speed. It's got some error handling.
    Also - two wines before I wrote this one. Look it's Christmas - and this
    is meant to be fun?!?!?!? Will this be used again this year? It's hard to
    say. It seems like a building block - there are plenty of unused
    op-codes. Perhaps even recursive functions via a map of "function definition"
    and then "function call" opcodes. We'll see how my predictions play out....
    Merry Christmas!
17. Intresting puzzle. Implemented imperatively, then refactored into
    rust iterators - which is actually slightly faster! Did part 2 really quick
    as I had already iterated across all possible initial velocities.
    Main problems were off-by-one errors using rust ranges.
18. Busy day with Christmas stuff. Only got to it the next morning.
    Just found this one long winded. Done.
19. Out and about enjoying myself swiming on a great summer day.
    I don't want to do this puzzle. I have too much stuff to organise
    before Christmas. Can't guarantee I will do puzzles from here on in.
20. Easy puzzle. Going to do it. Messed up the inifinite and needed to
    look at the hints. Don't care. Need to get other Christmas stuff done.