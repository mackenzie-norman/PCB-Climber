# Engineering Notebook for this project

Working with rotation. Running into the issue of what to rotate around. 
A few ideas:
- rotate around center, this allows for easy calculation and is probably the right way to do this
- rotate around x1,y1. easier calc
- only 90 (genomically) since pins aren't represented anyway, and since we assume all shapes to be rectangular, then we can just say you can only rotate 90 degrees
- Try genome only representation
Checking overlap is hard too. Current naive implementation doesn't check on move. 
- Want to make sure new loc is valid => get new loc and make sure its zeroed
- run overlap checks on all comps - there is a fast way to do this 

Okay I think were ready to implement this for real ish
## ToDo
- real genome rep
- Scoring? 
- Actual algo still (the fun part with a GA is this is pretty easy)

running into a problem of how to only allow valid moves, need to make sure a move
- doesnt overlap
- doesnt wrap around
- is in bounds

## Gave up on genomic representation (for now)
- Got most of it working, had a hard time with rotation (I was lazy and tried chat gpt code)
- Now something is causing the components to escape placement bounds 


# Latest Update. 
I've spent the last week or so dealing with parsing modern kicad. I really, really, wish the kicad parse crate worked but it doesn't. 
I am well aware there are better ways to do this but I just want to make something that works well enough so I can run my algo on a real pcb

We are using the arduino uno as our test board since its simple, (mostly) single sided, and maybe we can get good results (especially if we lock the micro controller)

After I got this working, I couldn't figure out why my components liked going right to the edge. Turns out out I forgot to invert my out of bounds check so the only valid placements were ones that were out of bounds!

After fixing this we are actually getting really "good" results. it is slow but not horribly so and honestly even compared to SOTA layout tools not that bad. 

I still havent added the compaction mutator. I wish psu had a GA class since adding the compaction operator would shrink the search space so heavily I'm not sure it would be beneficial. 

What I dont understand about GA's is how much your mutators should drive towards a better score and how much your selection criteria should. In my limited understanding, it would make sense for mutation and crossover ops to result in maximal variablitiy while selection criteria drives quality, thus a compaction operator might not be good. 

It should also be noted we are SURELY being helped by starting from a "placed" board. 


# TODOs
- add from scratch fn
- Parallel 
- shape traits for placment, comp, pin , bbox
- actually plot nets

