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