target remote :3333
set print asm-demangle on
set print pretty on
load
break application_main
continue