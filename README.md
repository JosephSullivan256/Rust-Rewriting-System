# Rust-Rewriting-System
Rewriting System implemented in Rust

Let's say you're writing some well formed formulas in propositional logic, and you're not quite sure where you can take it.

Is "not(A and B)" the same as "not(A) or not(B)"? If only there was a neat program that could allow you to specify a list of rules, parse your formula (specified in prefix notation), and spit out all possible ways to rewrite your formula. Well fear no more, with Rust-Rewriting-System the solution is here.

## Examples

Just as a preface, T represents True/Tautology, and F represents False/Contradiction

```python
>>> rrs -h

Use: [OPTION] "<formula>" <rule>
Rules:
0: =(and(x,y),and(y,x))
1: =(and(x,and(y,z)),and(and(x,y),z))
2: =(or(x,y),or(y,x))
3: =(or(x,or(y,z)),or(or(x,y),z))
4: =(or(x,and(y,z)),and(or(x,y),or(x,z)))
5: =(and(x,or(y,z)),or(and(x,y),and(x,z)))
6: =(not(not(x)),x)
7: =(not(T),F)
8: =(and(not(x),x),F)
9: =(or(not(x),x),T)
10: =(or(x,x),x)
11: =(and(x,x),x)
12: =(and(x,T),x)
13: =(or(x,F),x)
14: =(not(and(x,y)),or(not(x),not(y)))
15: =(not(or(x,y)),and(not(x),not(y)))
16: =(imp(x,y),or(not(x),y))
17: =(bi(x,y),and(imp(x,y),imp(y,x)))

>>> rrs "not(and(not(and(x,y)),y))" 14

Inputted Formula: not(and(not(and(x,y)),y))
Outputted Formulas:
or(not(not(and(x,y))),not(y))
not(and(or(not(x),not(y)),y))
```

# Future

Why stop at propositional logic? I should be able to extend it rather easily to cover the rules of predicate logic. Why not capture the axioms of groups and rings? I hope to be able to extend this project to be able to deductions in all sorts of contexts. Furthermore, prefix notation is awfully annoying to read and type, but it was easy to parse. In the future I hope to support more specifications.

Beyond this, as a long term goal, is adding heuristics to the program to reach some sort of canonical form for each formula.
