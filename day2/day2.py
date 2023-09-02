#!/usr/bin/python3
rounds = []
with open("ex.txt", "r") as f:
    for line in f:
        words = line.split()
        opponent = ord(words[0]) - ord('A')
        me = ord(words[1]) - ord('X')
        rounds.append([opponent, me])

#print(rounds)

score = 0
for (opponent, me) in rounds:
    diff = ((me + 3) - opponent) % 3
    if diff == 0:
        score += 3 # draw
    elif diff == 1:
        score += 6
    score += 1 + me

print(score)




