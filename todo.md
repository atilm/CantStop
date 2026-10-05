# ToDo

## Start ssh agent and load key

```
eval $(ssh-agent -s)
ssh-add ~/.ssh/id_ed25519
```

## Pseudocode

```
let mut game = GameState::new(4);

while !game.has_winner {
    let player = game.get_player()
    game.roll_dice()
    let pair_a, pair_b, set_camps = player.select_move(game)
    game.move(pair_a, pair_b, set_camps)
}
```

## Software Design Principles

* The game should validate each player action

## Rules

* [x] There are 11 tracks
* [x] Each player has max. 10 base camps
* [x] A player can give 2 dice pairs per turn
  * [x] The given pair indices are checked to contain exactly each die
* WINNING
  * [ ] When a player has won three tracks he has won the game 
* MOVING
  * [x] A player can insert 2 climbers at the beginning
  * [x] A player can insert 1 climber at step 2 at the beginning
  * [x] A player can insert the 3rd climber
  * [x] A player cannot insert a 4th climber
  * [x] A player has 3 climbers, when 3 climbers are already climbing,
        and a pair points to an open track without a climber, then the
        pair is ignored
  * [x] When a pair points to a track where a climber is already climbing,
        then the climber advances 1 step (this can happen twice per turn)
  * [x] When the player has arrived at the top of a track, then further moves are ignored
  * [ ] When the pair points to a closed track then it is ignored
  * [x] When a pair points to a free track, where no climber is yet,
        a climber enters the track after the last base camp of the same player
* SETTING CAMPS
  * [x] A player can set base camps when the third climber has been placed
  * [x] When a player sets base camps, it's the next player's turn
    * [x] and the the player's climbers are removed
  * [x] A player can only set camps when all three climbers have been placed
  * [ ] Base camps can be built in the same field
  * [ ] When a player has a camp at the top of the track, the track is closed
    * [ ] All other base camps are removed
    * [ ] When the 10th base camp of a player is removed, she can reuse it
* TAKING TURNS
  * [x] When a player cannot move any climber, it's the next player's turn
  * [x] When the last player has played, its the first player's turn again
  * [ ] A turn goes as follows:
    * [ ] The player rolls 4 dice
    * [ ] The player pairs up the dice
    * [ ] The player inserts or moves 1 to 2 climbers, or if impossible ends the turn
    * [ ] The player decides if she wants to go on or to set camp
