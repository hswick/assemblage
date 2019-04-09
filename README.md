# Assemblage

This repository contains a Ruby implementation of the ideas presented in Manuel DeLanda's book Assemblage Theory. DeLanda's book sought to add rigor and clarity to the concepts associated with assemblage theory. Namely the assemblage itself. This implementation is an attempt to extend that effort of clarity and rigor. Expressing ideas in code requires the thinker to be systematic and precise. Before now, these ideas have always been illustrated with human text. I would argue that text is not abstract enough or complex enough to describe assemblages in detail. Also, it is my assumption that the assemblage theory ideas do not require to be expressed in natural language. If anything, the theory stands to gain everything, by being expressed with a Turing machine.

Ruby was chosen because it is a very expressive and elegant language that approaches natural language. The entire introduction of Assemblage Theory can be expressed in <30 lines of code. Since Assemblages can be thought of as a mutable weighted, directed graph. This means Ruby's shared memory by default is very convenient. This is clearly not threadsafe. Threadsafe implementations of Assemblages can function as a separate field of study.

As a concept, the assemblage is another form of knowledge representation. It can be thought of as a weighted directed graph. However, it extends the ideas of graph theory by also storing the trajectory or a log of the change in its intensity value over time. This trajectory can be thought of as its history, and constitutes the individuality of the assemblage. Assemblages are singular. Singularity is not a concept that graph theory or mathematics is concerned with. They typically study the class of objects or their form.

Assemblage theory can be thought of as a knowledge representation capable of expressing an entire metaphysics. 

# Usage

```
ruby example.rb
```