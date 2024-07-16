# Embedded controller for coffee machine pressure control

My Pavoni Espresso Maker in their oldtimer form comes with limited and
inefficient steam pressure control
(litterally steam all you can, and vent the surplus away).
A power control via TRIAC and electronic pressure sensor can make a significant
difference on power intake and startup time.

Attention! This design involves dealing with high voltages that can be dealy of
lead to serious injury. Only qualified personel is allowed to assemble, test
and use these schematics. There's no warranity or responsibility from my
side, if things go wrong.

## documentation

all documentation is found in form of an [mdbook](https://rust-lang.github.io/mdBook/index.html).

## hardware

as this project is esp32 based, we need a pcb to mount the chip, the sensors
and the power stage. the pcb design and component simulation was made with
kicad 8.

## software

software is planned to be a rust embedded project.
