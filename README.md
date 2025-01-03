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

## 2022

Well another immense year on the personal front,
including a new house. Very busy at the start of the
month and some of the initial days will definately be
late.

Going to do rust again, with the same framework.
There have been some real improvements in the Visual Studio
Code Rust plugin which now more clearly displays the types
in e.g. iterator chains. Hopefully this will stop me getting
stuck with a missing reference or pointer like I have
in previous years.

Still in eastern Australia - so puzzles are released at 3PM.

My results (for days I was able to start at 3PM):
| Day | Part 1             | Part 2            | 3PM Start | Notes                                            |
|-----|--------------------|-------------------|-----------|--------------------------------------------------|
| 1   | 01:06:12 /  13639  | 01:10:43 /  12939 | No        | Late start due to moving stuff                   |
| 2   | 16:37:07 / 108073  | 16:49:04 / 101899 | No        | OMG - took me 35 minutes getting back into Rust  |
| 3   | 00:11:49 /   3662  | 00:34:51 /   7801 | Yes       | OMG - Rust itertools chunks is hard to use       |
| 4   | -                  | -                 | 9th       | Bit dull. But I guess the first parsing one.     |
| 5   | -                  | -                 | 9th       | Great puzzle. Stack management.                  |
| 6   | -                  | -                 | 9th       | OK. Not particularly fun.                        |
| 7   | -                  | -                 | 9th       | Interesting. Rust tricked me up.                 |
| 8   | -                  | -                 | 9th       | Fun little puzzle. Brute forced it.              |
| 9   | 00:35:09 /   4969  | 00:49:42 /   3314 | Yes       | ARGH! Slow part 1.                               |
| 10  | 20:02:02 /  50631  | 20:19:56 /  46677 | No        | Hung over after Xmas parties. Not interested.    |
| 11  | 02:17:41 /  10546  | 02:30:54 /   7163 | No        | Good puzzle. Happy I recognised % lcm quickly.   |
| 12  | 01:23:10 /   5610  | 01:32:08 /   5399 | No        | First search. Re-wrote part 2 after brute force. |
| 13  | 03:20:28 /   9814  | 03:27:30 /   9027 | No        | Interesting puzzle.                              |

Notes on each day:
1.  Late starting as I dropped off some flowers to some people
    who have been helping me out lately.
    Goot first puzzle. Very easy solution with the framework
    I have - including "input_to_groups".
    Easy to extend solution for both parts.
2.  What a disaster. Busy day at work, so not done
    until the next morning. I'm out of practise in Rust,
    and it took 35 minutes. Just converting a character to
    an integer took my 5 minutes to find.
    Anyway - done.
3.  Such an easy problem. I could not work out how to use
    Rust itertools "chunks" for part two to split into
    groups of 3. Lost a good 15 minutes here.
    Eventually gave up and wrote an iterative loop.
4.  Big few days. Only getting back to this on the 9th December.
    Really boring puzzle. But I guess it's the first one
    to test parsing skils.
5.  Really fun puzzle. I liked this one. Glad I got it correct
    on the first go with no errors.
6.  Yeah not really fun. Rust "windows" makes this easy.
7.  Interesting puzzle. I got tricked up a lot re-learning
    rust Rc and RefCell and where you need to clone the Rc
    and details like that.
8.  OK little puzzle. Just brute-forced it via two separate
    implementations to start with. Refactored into a shared
    algorithm afterwards - but don't think it's very readable.
9.  Interesting puzzle. I totally stuffed up the moving algorithm.
    Finally got there. Not impressed - but just goes to show how
    slow perhaps I am this week.
10. OK Not really fun. At this stage, I'm not 100% sure if this
    comms device CPU and CRT is going to be used again this year
    or not.
11. Good puzzle. Didn't start at 3PM as I was having beers in the
    pool. That's a good excuse. A lot of parsing - which I always
    do too well to be quick. I recognised that the maths could be
    done modulus the LCM of the divide tests (which all looked small
    and prime) - but I got tricked up with part 1. I tried adding 3
    into the LCM calculations - but then realized that the divide by
    three only happens when there's no overflow - so I only have to do
    the working MOD the LCM in part 2. Still took 15 minutes... when
    the winner only took 2 minutes to do this.
12. First A* search puzzle. First try solved part 1 in 0.2 seconds, so
    I just let it run for a minute or so to solve part 2. Then re-wrote
    to search from the top down.
13. Got tripped up on this one. Not doing well this year.

## 2023

Hmmm - not many notes here either.
Finished up to day 11. Started days 12 onwards in late November 2024
as practice.

## 2024

Ready to start - in Rust again.
Was thinking of moving to C# this year as I've been learning a lot
for work - but didn't get ready in time.

Comments from 2022 about the improvements to the VSCode plugins are
definately on point - I rely on these **A LOT** to fix e.g. filter closure takes
a reference and needing an extra deref operation.

My results (for days I was able to start at 3PM):
| Day | Part 1             | Part 2            | 3PM Start | Notes                                            |
|-----|--------------------|-------------------|-----------|--------------------------------------------------|
|  1  | 00:03:45 / 1183    | 00:05:30 / 941    | Yes       | OK. Obviously not 9 seconds!!!! OMG how????      |
|  2  |                    |                   | No        | Boring.                                          |
|  3  |                    |                   | No        | Didn't enjoy.                                    |
|  4  |                    |                   | No        | Meh.                                             |
|  5  |                    |                   | No        | OK - starting to get better...                   |
|  6  |                    | About 15 mins     | No        | Good puzzle.                                     |
|  7  | 6 mins             | 11 mins           | No        | Really enjoyed this one.                         |
|  8  | 00:14:05 / 1400    | 00:20:56 / 1372   | Yes       | Meh. Slow.                                       |
|  9  |                    |                   | No        | Intesting. Slow.                                 |
| 10  |                    |                   | No        | Meh. Solved part 2 first whoops!                 |
| 11  |                    |                   | No        | Fun! Silly mistake.                              |
| 12  |                    |                   | No        | Good puzzle. Found part 2 hard.                  |
| 13  |                    |                   | No        | Very good puzzle.                                |
| 14  |                    |                   | No        | Fun. Cheated a little.                           |
| 15  |                    |                   | No        | VERY FRUSTRATING - didn't enjoin this one.       |
| 16  |                    |                   | No        | Very good! Wish I started at 3PM.                |
| 17  |                    |                   | No        | Whoa!!! Got stuck. Interesting.                  |
| 18  |                    |                   | No        | Pretty easy. Good optimization chosen.           |
| 19  |                    |                   | No        | Pretty easy via memorization.                    |
| 20  | 00:19:00 / 559     | 01:58:47 / 2963   | Yes       | ARGH!!! Off-by-one errors!                       |
| 21  |                    |                   | No        | Very fun puzzle!                                 |
| 21  |                    |                   | No        | Did not enjoy this one.                          |
| 21  |                    |                   | No        | The big algorithm I didn't know.                 |
| 21  |                    |                   | No        | Whoa! Went in big for a generic solution.        |
| 21  |                    |                   | No        | Yaya! Done.                                      |

Notes:
1. OK. Sure they will get more interesting.
2. Boring.
3. Used regex. Don't know rust regex - not happy with this version but not doing it again.
4. Guessed the wrong part 1 - so my solution looks a bit weird. Can't be bothered doing it better.
5. Really enjoyed this one. Not the "cleanest" code - but again can't be bothered fixing it.
6. Missed the optimization here and tested every clear location on the map. Cheated to work out you only need to check
   locations on the original path. Parallel solution is fairly fast - about 0.6 seconds on my laptop.
7. Good puzzle. Happy with the solution. Optimizations include reserving space in HashSets, discarding large possible solutions
   (as all operators increase the size), and parallization - about 0.2 seconds on my laptop.
8. Hot. Tired after Christmas parties. Got messed up by grouping iterator operations. Got messed up by
   considering all '.' as a frequency and thus all 'empty' points were '.' transmitters.
9. Yeah - interesting puzzle. Still want to re-write using my set-of-ranges class, but probably won't get to it.
10. Bit mistake here solving number of paths instead of number of reachable summits - and oh look that was part 2!
    Part 2 was actually easier when using the Rust pathfinding crate.
11. Interesting puzzle. Got the right approach (recursive function). Proably got a bit slowed down by going to
    Memorization first up - turns out only about 50% of calculations are duplicates - so a straight-forward
    recursive function would possibly be quick enough.
12. Found this interesting. I found part 2 harder than I should have. Made a few silly mistakes.
    Happy with the solution (although I'm sure there's a better algorithm out there) - and have at least
    added it as a new common routine.
13. Really good puzzle. I got distracted thinking it should be a quadratic because it was "minimize" - but of
    course there's always a red-herring! It was linear and there was only a single possible combination of
    button presses. Had good diophantine routines to find integter solutions - so was easy once I got to this point.
14. Fun puzzle. Started going through the first 600 or so images to find part 2 - but then cheated.
    Should have guessed that unique locations would be the spot.
15. Very FRUSTRATING! I didn't like this one. Finally got an OK solution that's general to any box width scaling factor
    (although probably exponential slow-down in the wost case for larger values).
16. Good puzzle. Wish I was ready at 3PM for this one as I'm good at this. Only took a couple minutes.
    The ASTAR routine I use provides all least-cost solutions - so part 2 was simple.
17. Really interesting puzzle. It got sucked into part 1 and worked on the computer. Then WHOA!!!
    Eventually went back to de-compiling the code by hand, working out what it did, and trying a
    breath-first search executing the programme in reverse, to collect all solutions, then find the minimum
    input. This is OK as there is only about 12 possible starting A values for my input, and only about 1-2
    possible combinations for the lower bits lost in the A-register divide at each step - so the breath-first
    search is very narrow. Took me about 3 hours including a walk to solve. NOT A GENERAL SOLUTION!!!
18. Good short, easy puzzle. My first simple solution (using repeated A*) took about 20 seconds in debug build to solve.
    Then moved to a simpler breath-first algorithm which took 10 seconds. Visualized the result - and realized
    that the points are filled in in a random scattering (to eventually make a complex maze). So I find
    the shortest path, and keep fulling in points that are not on the shortest path. If a point on the current shortest path
    is 'corrupted' - I re-calculate a new shortest path (or see if it's fully blocked). This means most points are just
    filled in - and the A* algorithm only runs about 50 times. Solves in 0.2s even in debug build.
19. Good quick puzzle - via memorization worked out well.
20. ARGH!!!! Ready at 3PM. Got part 1 done pretty quickly. Part 2 I was plagued by off-by-one errors in the path
    length calculation and got really stuck. Went down bad routes (e.g. thinking you had to only count "good" cheats
    that didn't re-use any already existing paths becuase that would mean there is a "better" cheat).
    Eventually worked out my problems and realized it's a super simple problem iterating over (a well chosen subset)
    of the combinations of cheat start and end points from the original path. Rayon parallel iterator over the start
    point (with sub-iteration over the end point) gives a solution in 0.4s even in debug mode. Clearly learnt something
    from day 6 as I was on the right path from the beginning - just tired and made silly mistakes. It's been a very
    big week!
21. Ahahah! This was a good puzzle. I correctly guessed what part 2 would be. Made a couple of silly off-by-one errors. 
22. Meh. Didn't enjoy this puzzle today.
23. Didn't do that good a job here. Eventually cheated. Clearly the big algorithm that I didn't know
    for this year.
24. Whoa! This was epic. I think I went in too big in part 2 and wrote a generic algorithm that will handle
    (almost) any swapping of wires - as long as there is only one error in each 'digit' of the answer.
    I've checked in my first version I got working - but I will need to re-write it using a standard
    depth-first-search library as once I got the concepts organized it's not that hard and is not
    particularly expensive. Key points I found were that the zXX output digits are made from XOR-triplets,
    and the carry stage has an OR or AND as it's last element. So most swaps could be found by searching
    for the other "correct" equation - but with the XOR-triples the ordering of the operations were mixed
    up - so I matched correct branches and then grouped the remaining XOR terms into a new wire to
    perform the sub-search on. Took me too many hours!!
25. Done. Spent a lot of time of this this year - perhaps too much? Make sure you
    read this before you start next years ahahaha!