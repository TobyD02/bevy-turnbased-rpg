# TurnBased RPG

## TODO

- [ ] Turn Groupsy
    - In the turn order struct, give each entity a "turn group". Then add a new method that allows you to "
      get_active_in_turn_group", which returns an vec of all entities in that turn group which take a turn before the
      next intiative in a different turn group
    - i.e. imagine 3 turn groups: player, friendly, enemy; in the following order:
        - Friendly 1: 20
        - Friendly 2: 19
        - Player: 17
        - Friendly 3: 16
        - Enemy 1: 15
        - Enemy 2: 15
        - Enemy 3: 10
    - When processing the characters (frinedly and enemy), if i "get_active_in_turn_group" then i can process both
      friendly 1 and friendly 2 in the same frame, since they both take turns before the player (who is in a different
      group). Then after the players turn it would contain only friendly 3. Then all the enemies would take turns on the
      same frame.
    - The ordering wouldn't matter either, so long as the method returns the entities in initiative order, the map
      resource would be updated in the correct order, so there would be no race conditions.