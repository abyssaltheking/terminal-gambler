# *TERMINAL GAMBLER*

> ***it's like gambling but in the terminal***<br>

*- bob the dragon*

<hr>

Terminal Gambler is the ultimate casino experience that you can experience right at home, in your terminal! Play slots, blackjack, and coinflip, all in the comfort of your own terminal. No window managers needed for this. As a bonus, it was made in Rust to make sure that even the slowest systems have no slowdown when it comes to committing to your gambling addiction.

## Games
- `coinflip` `bet, max 1k` `heads/tails`<br>
Simple coinflip, make a bet on which side it'll be and if you're right you get 2x your bet back.
- `blackjack` `bet, max 10k`<br>
Slightly modified for my coding convenience. Cards are just numbers 1-10, so there's no aces. To account for this, the number at which you bust is 20 instead of 21. Besides that, it's the same game.
- `slots` `bet, max 100k`<br>
Numbers 2-7 are rolled for 4 seconds. Once finished, if a row ends up having all of the same number, the number it lands on is added to a multiplier, which is applied to your bet.
<details>
<summary>See an example</summary>

You run <code>slots</code> <code>100</code>

Results end up being:<br> 
<code>3 3 3<br></code>
<code>5 4 7<br></code>
<code>6 6 6</code>

This equation ends up being applied, since both the top and bottom row had one of the same numbers per row:<br>
<code>winnings = bet * (3 + 6)</code>

So you end up winning $1800 from a single $100 spin. Lucky you!
</details>