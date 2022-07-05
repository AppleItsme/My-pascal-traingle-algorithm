# My-pascal-traingle-algorithm
## Description of the algortihm
**[Considering that the tip of the Pascal's triangle (`1`) is the 0th row]**
Take any row of the pascal's triangle, let's say 5. The row looks like the following:  
`1, 5, 10, 10 5 1`
What can we see? 
- 1 is always at the ends of the row; 
- The 2nd element is the row number. I. e. the number is exactly the same as the row position (5);
- The numbers are perfectly mirrored from the centre.
- The length of the row is n + 1, in this case, 5 + 1, which gives us 6.  

We can identify the same properties in every other row:
`1, 2, 1` - 2nd row
- it starts with 1;
- 2nd element is 2;
- The number is mirrored from the centre (2) giving us 1 on the end too;
- 2 + 1 = 3. 3 is the length of the row.  
`1, 9, 36, 84, 126, 126, 84, 36, 9, 1` - 9th row
and the rest.

This means: 
* If we need to generate the whole pascal's triangle again, there is no need to work with the full triangle. Half is sufficient.
* We have the first 2 elements given.

Therefore, the algorithm follows the method below:
1. Start from 2nd row (1st can also work though) and save it in `past_line`;
2. Start the loop, with `i` iterating from 3 (or 2nd, depends on which row you prefer to start from) to `n+1`;
3. Set the first 2 elements of the `current_line` to 1 and `i`;
4. Get the indices of the row that will not be repeated (I.e. first half). In case the number is odd, the first half should also contain the centre of the row.
5. For every remaining index inside `unique_indices` (from 2 to `unique_indices`):
	`j` is the current index;
	If `j` is less than the length of the `past_line`, we push `past_line[j] + past_line[j-1]` onto `current_line`;
	Else, we push `past_line[j-1] * 2`. This is because whenever this happens, it means that we surpassed the length of the past_line's half, and when it happens, the value we would normally expect would be the same as `past_line[j]`. Therefore, `past_line[j] * 2` works.
6. Set `past_line` to `current_line`
7. Loop again until we reach `n+1`
8. Make another variable, which will contain the same elements as `past_line` but in reverse.
9. Append the variable to `past_line`.
10. Done.
## State of the algorithm
I just developed it on my spare time, and it could very well be a worse variation compared to the rest out there, but I see that this algorithm works and gets the job done, so maybe checking it out might not be a bad idea.
## Space-time complexity
**DISCLAIMER!**  
The data might very well be wrong, so feel free to inform me about the issues and an explanation would be appreciated :)

Time: O(n)
Space: O(2n) 
