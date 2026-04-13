# TurnBased RPG

## TODO 13/04/26

- [ ] Game Events
    - Core Idea, entity actions on their turns may trigger subsequent events etc... I need a way of triggering/handling
      these.
    - IDEA - GameEventQueueResource (and CurrentGameEventResource).
        - On a turn, the entity pushes "Intents" (GameEvents) to the event queue. Once the entity has finished declaring
          its intents, it sets some "commit_turn()" method (just a boolean).
        - Important - once a turn is commited, that entity cannot take any more action. However, since the turn has not
          been ended, neither can any other entity. This puts the world in a freeze, allowing for events to be processed
        - When turn is commited, perform a simple check:
            ```rust
            pub fn process_game_event_queue(
                mut turn_order: ResMut<TurnOrderResource>,
                mut current_game_event: ResMut<CurrentGameEventResource>,
                mut game_event_queue: ResMut<GameEventQueueResource>
              ) {
                if !turn_order.is_turn_commited() {
                    return;
                }
              
                if let Some(event) = game_event_queue.pop_front() {
                    current_game_event = event
                } else {
                    turn_order.end_turn() // sets turn_commited to false, and sets turn to next entity
                }
            }
            ```
        - Each frame/cycle of the ecs, a new event will be pulled out of queue and set as the current.
        - IMPORTANT - this system will need to run before any event handling systems. This way, once the current event
          is set, event handling systems can check if they handle the current game event, and if they do perform their
          logic within this tick of the ecs (publishing events, moving entities, etc...).
        - When the full game event queue has been processed (i.e. pop_front() returns None), the turn is ended, which
          sets the next entity in the turn order as the currently active turn, as well as also setting turn_commited to
          false.
        - As a slight pitfall, turn_groups may not work any longer.
            - There is potential to have some `GameStateResource`, with values of `GameStateCombat` and
              `GameStateExploration`. If the player is exploring, then all decisions can be processed in groups, as
              there (likely) wont be any world changes taking place. Mostly movement etc... However if in combat, turn
              groups are not processed, instead entities are processed individually so that when they intend to
              end_turn, the events can be processed, and the world updated before the next entity takes its actions.
            - Worst case - I shelve turn groups for now.

## TODO 11/04/26

- [ ] Turn Groups
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