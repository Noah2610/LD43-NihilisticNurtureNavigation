__So, we didn't finish in time...__

We still wanted to upload the game in its unfinished state, because we are proud of what we created.  
We really want to upload a post-jam version in the coming days, with the features properly implemented.

Most features are fully implemented, but lack the polish to fully work in conjunction.  
We all really need to take a break now.  
Updated README will follow soon...

---

## Gameplay
There isn't much gameplay, aside from the chaotic and un-polished level you see in the jam-version.  
The goal in this level is to get as many persons (player and children) into the isolated room,  
closed-off by the two yellow doors.  
You are able to bring __three persons__ to the "goal"; one person will need to be _sacrificed_.  
In the final version children will reward with more points than the player when saved.

To command the monster children, click the left/right arrow buttons at the bottom of the window.  
The target child will then move to that direction until they hit a wall, at which point they can accept new input.  
When a child is moving, they cannot be commanded.

The released jam-version is more of a prototype, as it isn't really a game in its state.  
We will (hopefully) be releasing a playable post-jam version with multiple levels this coming weekend.

## Controls
| Action           | Input(s)                          |
| ---------------- | --------------------------------- |
| Player Movement  | A, D                              |
| Player Jump      | Space                             |
| Minion Movement  | Mouse click on arrow buttons      |
| Camera Movement  | Arrow keys                        |
| Quit             | Escape                            |

---

## Concept
Most of what is listed in this "__Concept__" section is not implemented in the jam-version of the game.  
These were our main ideas, which we are working on implementing for a post-jam version.

The game should be a __puzzle__ game, in which you dictate your children (weird monster children) what to do.  
You could tell them to move right and fall into a gap from which they can not escape;  
you (and your other children) would then be able to walk on top of the sacrificed child to cross the gap.  
The goal would be to get to some food at the end of the level (hence the name).

Some children would have different abilities:  
one would be able to jump (_Long Leg Larry_, the one with the long legs);  
one would be able to fall through _one-way platforms*_ (_Bloat_, the fat one);  
the last one would not be able to do anything special, but would reward with extra points when saved (_The Thing_).

Every saved person (player and children) would give points when saved, some more than others:  
the player would reward the least amount of points, as they have the most freedom of movement;  
the children would reward more points, the one without an ability rewarding the most.

We planned the setting to be post-apocalyptic and the goal would be to gather food by beating levels.  
We wanted to implement some minor story/lore which would unfold via a conversation/monologue-system,  
explaining/hinting at the situation (who are these _monster children_ and why are they following this ominous-looking dude?).

__*one-way platforms__:  
A person (player or child) can jump through these platforms from below, but cannot fall through them from above.  
They are implemented and exist in the jam-version, but they are invisible / have no texture(s).

## Missing Features
- __A goal__  
  Each level should have a _goal_;  
  an area where you would bring as many persons as possible to gain as many points as possible  
  and to move on to the next level.  
- __Level Manager / Level Selection__  
  Obviously, there should be more than one level in the final product.  
  After you beat one level, you would move on to the next one.  
  Additionally, a __level selection__ menu would be nice.
- __Audio Manager__  
  The jam-version only loops a single song, which is sad because  
  our sound artist (@williwiderstand) has created __5 songs__ in total.  
  You can still check them all out in the `resources/audio` directory if you downloaded the zip file.
- __Sound Effects__
- __Background Image(s)__  
  @hoichael has created a temporary background image, but I was not able to implement it in time.  
  The final game should definitely have some background images, which may change per level.
- __Background Decorations__  
  @williwiderstand has created a neat, animated eye that was planned to be put in the background as decoration.
- __Child abilities__  
  As mentioned above, children were planned to have special abilities.
- __Pivoter__  
  An "Interactable" object, which when touched by a child will invert their direction of movement.
- __Conversation/Monologue system__  
  A system to convey or hint at some lore regarding the monster children and the setting of the game.
- __Text Popups__  
  Text that will pop-up above a child when they are commanded to move.  
- __Level Reset__ button
- __Pause__  
  Being able to pause the running game would be nice and shouldn't be too hard to implement.  
  (Although probably unnecessary)
- __MacOS Version__  
  A MacOS version shouldn't be difficult to build and distribute (I think), I just need access to an updated mac.

---

## Programs Used
|                  |                                   |
| ---------------- | --------------------------------- |
| Language         | __Rust__                          |
| Engine           | [__ggez__][ggez]                  |
| Framework        | [__noframe__][noframe]            |
| Graphics         | __Gimp__                          |
| Audio            | [__BoscaCeoil__][boscaceoil]      |
| Editor           | __vim__                           |

## Credits
|                  |                                   |
| ---------------- | --------------------------------- |
| @hoichael        | Concept, Design, Graphics         |
| @williwiderstand | Audio, Graphics                   |
| @noahro          | Programming                       |

---

Thank you for your time!  
Although we weren't able to release a proper game, we still had a ton of fun and learned a lot during this jam.  
This was my first proper Rust project and I am very happy about how I managed to go at it.  
In case anybody is interested in the code, it is available on [github][source].  
The framework I wrote for the game is also available on [github][noframe].  
It doesn't have many features, but it did help provide base code, which I would have had to write during the jam.

[ggez]:       https://ggez.rs
[boscaceoil]: https://boscaceoil.net
[noframe]:    https://github.com/Noah2610/noframe
[source]:     https://github.com/Noah2610/LD43
