# Engineering Notebook for this project

Working with rotation. Running into the issue of what to rotate around. 
A few ideas:
- rotate around center, this allows for easy calculation and is probably the right way to do this
- rotate around x1,y1. easier calc
- only 90 (genomically) since pins aren't represented anyway, and since we assume all shapes to be rectangular, then we can just say you can only rotate 90 degrees
- Try genome only representation
- Checking overlap is hard too. 
- Current naive implementation doesn't check on move. 
- Want to make sure new loc is valid => get new loc and make sure its zeroed
- run overlap checks on all comps - there is a fast way to do this 


## ToDo
- real genome rep
- ~~Scoring?~~ 
- Actual algo still (the fun part. with a GA is this is pretty easy)


running into a problem of how to only allow valid moves, need to make sure a move
- doesnt overlap
- doesnt wrap around
- is in bounds
# Update (2-19-2025)

## Gave up on genomic representation (for now)
- Got most of it working, had a hard time with rotation (I was lazy and tried chat gpt code)
- Now something is causing the components to escape placement bounds 

(*Future Max Note:* I had my booleans backwards. It is probably good to go back and play with this again.)


# Latest Update (2-26-2025). 
I've spent the last week or so dealing with parsing modern kicad. I really, really, wish the kicad parse crate worked but it doesn't. 
I am well aware there are better ways to do this but I just want to make something that works well enough so I can run my algo on a real pcb

We are using the arduino uno as our test board since its simple, (mostly) single sided, and maybe we can get good results (especially if we lock the micro controller)

After I got this working, I couldn't figure out why my components liked going right to the edge. Turns out out I forgot to invert my out of bounds check so the only valid placements were ones that were out of bounds!

After fixing this we are actually getting really "good" results. it is slow but not horribly so and honestly even compared to SOTA layout tools not that bad. 

I still havent added the compaction mutator. I wish psu had a GA class since adding the compaction operator would shrink the search space so heavily I'm not sure it would be beneficial. 

What I dont understand about GA's is how much your mutators should drive towards a better score and how much your selection criteria should. In my limited understanding, it would make sense for mutation and crossover ops to result in maximal variablitiy while selection criteria drives quality, thus a compaction operator might not be good. 

It should also be noted we are SURELY being helped by starting from a "placed" board. 


# TODOs
- ~~change selection criteria~~
- ~~Parallel~~
- ~~SPEED UP~~
- ~~actually plot nets~~
- shape traits for placment, comp, pin , bbox
- ~~add from scratch fn~~
- ~~CLI~~

## Notes again (2-28-2025). 
- We calc score on every one on the sort. That is definetly inefficent to some degree. 
- added rough CLI using clap
## Fixes
- added fitness field to class. Now only calc when mutate (should add a check to only calc when successful)
- added rough ev selector, think I want to add my weighting function to the elitist plan too
- add net plot, need to change it to ordered by distance sort of, just so its actually visible. I also need to figure out whats going on with my pins. They are there but aren't getting plotted.

## Parallel Notes (3-6-2025)
- initially implemented parallel mutations, but almost no speed up - even when we have a large population size
- Most likely want to switch to parallel implementation with different "communities" and reusing our selection operator for a migration operator

## Parallel Notes (3-10-2025)
- switched to using communities/migrations
- since we have to clone ( I know I am doing this wrong but too lazy to fix atm)
- only merge communities every 4

went from 15.8 to 8.1 

### New Plan 
```pseudo code
pop[Pop_size * threads]
par iter into chunkmut
do work
selection algos

combine
selection algo


```

I implemented this but I think something is broken - getting WAY more overlaps when I used to have 0

Need to look at my selection/crossover. I am certain its in crossover

## Final Parallel Notes(3-11-2025)
so double parallel is slower, not sure why that is. 

Changing from ev to elitist selection makes a huge difference (not sure why)


# Project Status (3-11-2025)
Looking back at my goals, I 
## Minimum
- ~~Implement GA using operations from [GA_breakdown.pdf](GA_breakdown.pdf)~~
## Should
-  ~~Add concurrent/multi-threaded support~~
-  ~~Ability to parse Kicad input~~ *and output to Kicad* (Not sure if I'll get to output, but I feel okay with my plotting)
## Stretch
-  Utilizing mutations created above, implement PSA/MOSA 
-  Add concurrent/multi-threaded support to our SA 
## Reach
-  improve upon operators and try other simulated annealing based approaches
-  implement a better wire length calculation (maybe A*?)


I think if I had to submit this now I would feel *fine* about my progress. 

I think I'd like to add a really basic SA to play with, although sadly I don't know if Ill have time to get PSA/MOSA but I really would and probably will after the term ends.

I also really want to play with traits but there is not much benefit for the refactoring required




# Project Status (4-29-2025)
I added mst euclidean distance for net length, currently no slow down

need to add better testing for debugging where overlaps come from.
# Project Status (5-1-2025)
found overlap bug, was in swap, I wasn't moving A comp back when placement fails.

next step is adding support for fixed
- all ops cant pick fixed I think is the only thing needed. 
- Also maybe could add a speed up in crossover by skipping the fixed. 

after that I should be able to make run some better benchmarks. Ideally I want to generate the kicad equivalent of placement.txt 

# Project Status (5-1-2025)

added fixed components, pretty naive right now but good to have. Think next step is place back to kicad. 
