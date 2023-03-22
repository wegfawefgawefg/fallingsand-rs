# fallingsand-rs

heavy ai use falling sand game. aiming basic functionality, then highly paralelized, then wasm port

## note

this is unraveling because i cant decide on how to seperate the particles side effects from the effects of the nearby ones in
the same frame.
order of processing seems to make a difference
prevent instant propogation of particles around the grid. like fire instantly in one frame going all the way across a burnable substrate
if the fire spreads from left to right bc the particles were made in that order, then the fire will spread all the way across the substrate.
but that effect goes away if they are unordered/

## LATER VERSION

try fixed number of particles, where we cycle in offset usize for the id?
then we can have a fixed size array of particles.
