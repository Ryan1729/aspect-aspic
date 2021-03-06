### Goal 
##### Elevator pitch
> Recombine aspects to create allies

In addition to the below mechanics, if you combine all N colours you create an entity identical to yourself and instantly win the game.

### Mechanics

* Dead orb : ![shine-orb.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/shine-orb.png) if left alone slowly recharges turning into...
* Charged Orb : ![zigzag-orb.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/zigzag-orb.png) if this moves into another Charged Orb creates a ...
* Blob : ![pair-circle-blob.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/pair-circle-blob.png) If bumped into by the player, split into component aspects.
* Created Orb: ![concentric-orb.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/concentric-orb.png) Created when the player throws a Charged Orb at a Dead Orb.  When the player bumps into it it is activated, and it's aspects are thrown outwards in the process

___

### Trying out an idea

Taking inspiration from the Magic the Gathering colour pie, Let's try the following aspects:

* Red   : Empathy
* White : Rules
* Black : Self-intrest

_ | ![shine-orb.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/shine-orb.png) | ![zigzag-orb.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/zigzag-orb.png) | ![pair-circle-blob.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/pair-circle-blob.png)  | ![concentric-orb.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/concentric-orb.png) 
  --- | --- | --- | --- | --- 
Red  | Moves towards nearest other thing | Shoots healing ray towards most injured other | Moves towards most injured other and then heals it | Heals everyone
White | Stays in the center of it's cell | Moves in coordination with other white creatures to make a symmetrical pattern | pushes / attacks other creatures/orbs to make ***the*** pattern | rearranges everyone into ***the*** pattern
Black | Moves to the safest place it can | Moves towards where it can be healed | Evaluates White creature situation and moves in accordance with their pattern if White is "in power" | Creates more black orbs/creatures


### Problems with this idea

There does not seem to be anything to damage things. Besides the player I suppose? What does that say about the player? Minecraft has a similar thing going on where a forset has rested in possibility space undisturbed until you walk by and the lava lights the whole thing on fire. Similarly you walk by causing a wolf to spawn and attack an also freshly spawned sheep. You the player are a source of suffering. In this hypothetical game though, at least you only would be actively doing the damaging, rather than merely causing suffering by forcing the world to exist. 


### Try Again

The goal and mechanics seem alright so far but as mentioned above that attempt had some problems. So your goal as the player is to make something using all the different aspects at once. What are some interesting ways to make that non-trivial? Making some n-1 combinations completly impossible seems to be wasting the possibility space. But maybe that's a price worth paying? If we're going to make some of the n choose n-1 possibilities impossible then shoulod there be a nice pattern to which one are impossible? Or would it be interesting to embrace "dumb" reasons for things to not work? Real life is complicated in ways that don't obviously fit into nice patterns. 

For example:
* ~~Someone has an idea and starts feverishly working then goes to the bathroom and realizes that the idea they had won't work but another one will!~~ This isn't a great example because it happens at a particular time and place, rather than being a set of rules/laws. 

* [Arrow function syntax in javascript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions#Syntax) particularly when returning an object literal. This is a better example because it arises within a set of rules, in particular one created by humans. Can this somehow be made to fit in a nice pattern?

* Another example is non-consecutive numbering due to delays (other reasons may deserve their own section.) For example see ECMAScript 3.1 and 4 which eventually essentially became 5 and 6 respectively where 4 was never released to avoid confusion.


___

I'm not really feeling this "dumb" reasons for things as a "feature" idea. But I would be willing to pass through "dumb" reasons for things as a part of iterative development. So I think I might try just implmenting the goal above with randomly moving blobs and then iterate (possibly using generation in some way) through different possibilities.

___

### Blob/Orb movement Idea

The live orbs could move in a 3 by 3 grid while the player can throw orbs in a 2 by 2 grid. 

![2by2and3by3.png](https://github.com/Ryan1729/aspect-aspic/raw/master/design/2by2and3by3.png)

It seems reasonable for the orbs to "connect" if, after the orbs move, the thrown to area overlaps at least half of the orb.
This implies that if the orbs are completely inside the 2 by 2 grid cell then a throw to that cell is guaranteed to hit (assuming orthogonal movement.) 
However, if the orb is on the edge between two 2 by 2 grid cells, then there is only a 1/3 chance of a hit if either of the two 2 by 2 cells is throw to.
Also, an orb in the center of the 3 by 3 grid has a 1/2 chance of being hit if any of the 2 by 2 cells is thrown to.
These probabilities seem at least somewhat interesting assuming you can push them in your favour somehow. Perhaps placing obstacles? maybe the obstacles could simply be the dead orbs themselves?

This grid aspect of this is already more or less implemented.