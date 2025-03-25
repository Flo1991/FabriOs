# FabriOs
THE PROJECT IS WORK IN PROGRESS.

Simple Rust Os using rust core features only. It uses no unstable features and no external crates for minimum external dependencies and maximum stability.
In fact this project is not a "ready to use" Os - instead you can find all the necessary functionality to build / adapt to a custom Rust Os on this code.

So it mainly provides <br>
(1) very simple cooperative scheduler using timeslots with a function pointer array (--> tasks) <br>
(2) simple and easy to understand scheduler for processes that take to long for a single task slot (--> very basic context switch, adapt the inline assembler to match your target mcu) <br>
(3) simple software timer implementation for slow tasks to do <br>

The implementation completely interrupt free. This maximizes determinism and makes it easy to adapt to different other mcu target plattforms.


