# Rust-Rewriting-System
Rewriting System implemented in Rust

Let's say you're writing some well formed formulas in propositional logic, and you're not quite sure where you can take it.

Is "not(A and B)" the same as "not(A) or not(B)"? If only there was a neat program that could allow you to specify a list of rules, parse your formula (specified in prefix notation), and spit out all possible ways to rewrite your formula. Well fear no more, with Rust-Rewriting-System the solution is here.

Examples:

# Future

Why stop at propositional logic? I should be able to extend it rather easily to cover the rules of predicate logic. Why not capture the axioms of groups and rings? I hope to be able to extend this project to be able to deductions in all sorts of contexts.

Beyond this, as a long term goal, is adding heuristics to the program to reach some sort of canonical form for each formula.
