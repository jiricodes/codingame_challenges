# Codingame Spring Challenge 2022

## Notes and Ideas
- [ ] create new attact idle - for some unknown reason the offensive patrol times out... often
- [x] add timing, so I know why timing out - offensive patrol is the reason
- [ ] calculate time to kill per monster
- [ ] calculate time to arrive to base per monster
- [ ] calculate time to reach the monster by a hero
- [ ] create decision making based on above
- [ ] check whether monster disappears when pushed out of the map
- [ ] one attack, two defense
- [ ] try to gain wild mana if nothing else
- [ ] attacker cast shield on monsters that have eta < 12 + ttd
- [x] change Vec2 to be f32
- [ ] add simulate turn
	- [x] simulate monster
	- [ ] simulate hero
	- [ ] simulate gamestate