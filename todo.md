# ToDo

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

* [ ] There are 11 tracks
* [ ] Each player has max. 10 base camps
* [ ] Base camps can be built in the same field
* [ ] When a player has a camp at the top of the track, the track is closed
  * [ ] All other base camps are removed
  * [ ] When the 10th base camp of a player is removed, she can reuse it
* [ ] When a player has won three tracks he has won the game
* [x] A player can give 2 dice pairs per turn
  * [!] The given pair indices are checked to contain exactly each die
* [ ] A player has 3 climbers, when 3 climbers are already climbing,
      and a pair points to an open track without a climber, then the
      pair is ignored
* [ ] When the pair points to a closed track then it is ignored
* [.] When a pair points to a free track, where no climber is yet,
      a climber enters the track after the last base camp of the same player
* [ ] When a pair points to a track where a climber is already climbing,
      then the climber advances 1 step (this can happen twice per turn)
* [ ] When a player sets base camps, it's the next player's turn
* [ ] A player can only set camps, when he used all three climbers
* [ ] When a player cannot move any climber, it's the next player's turn
* [ ] A turn goes as follows:
  * [ ] The player rolls 4 dice
  * [ ] The player pairs up the dice
  * [ ] The player inserts or moves 1 to 2 climbers, or if impossible ends the turn
  * [ ] The player decides if she wants to go on or to set camp
