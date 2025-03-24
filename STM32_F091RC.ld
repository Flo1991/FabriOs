/*
   ______    _          _    ____      
  |  ____|  | |        (_)  / __ \     
  | |__ __ _| |__  _ __ _  | |  | |___ 
  |  __/ _` | '_ \| '__| | | |  | / __|
  | | | (_| | |_) | |  | | | |__| \__ \
  |_|  \__,_|_.__/|_|  |_|  \____/|___/
                                                                         
  Copyright (c) 2025, Flo1991
  
  BSD 3-Clause License - see LICENSE file for details
  
  Author : Florian Wank
  Creation Date : 21.03.2025
  File : STM32_F091RC.ld
  
  # Short description
  Linker script file for STM32_F091RC Mcu
  
  # Detailed description
  This linker script file is responsible for correct data mapping.
  The implementation is for static data only - heap is currently not supported!
  This linker script file can also be used with a bare metal C application.

*/  

ENTRY(application_reset_handler)

/* define physical memory map here */
MEMORY
{
   FLASH(rx)       : ORIGIN = 0x08000000, LENGTH = 256K
   SYSMEM(rw)      : ORIGIN = 0x1FFFD800, LENGTH = 8K
   OPTIONBYTES(rw) : ORIGIN = 0x1FFFF800, LENGTH = 2K
   SRAM(rwx)       : ORIGIN = 0x20000000, LENGTH = 32K
}

SECTIONS
{
   /* the text section is read only, so the load memory address (from flash, short LMA) is the same as the 
      virtual memory address (VMA), which is used at runtime to access data (also read from flash) */
   .text : 
   {   
      /* get location counter of start of flash memory */  
      _start_of_flash = .;  
      _stext = .;
      /* get current location counter "." and store to _isr_start; */	  
      _isr_start = .;
      /* * = wild-card */
      /* locate the isr vector table directly at memory start; never remove this data! */
      KEEP(*(.isr_vector))
      
      /* Align the location counter, so that the next memory area is also aligned to 4 bytes = 32 bit */
      . = ALIGN(4);
      
      /* Text section */
      /* this expression matches all .text sections in all input files */
      *(.text)
      /* this expression matches all .text* sections in all input files, so also match .text.mysection (e.g. need for GCC -ffunction-sections) */
      *(.text*)
      
      /* Read only data section */
      *(.rodata)
      
      /* Align the location counter, so that the next memory area is also aligned to 4 bytes = 32 bit */
      . = ALIGN(4);  
       
      /* Location counter has here the end of the text region; create symbol here */
      _etext = .;
   }> FLASH
   
   /* the data section has a load memory address (LMA) in flash (here the const data is stored), but a
      virtual memory address (VMA) in SRAM, from which the data is accessed at runtime; therefore we 
      use ">SRAM AT>FLASH"; the _sdata and _edata are refered to VMA */
   .data :
   {
      /* Start of Data section on SRAM; create symbol here -> VMA */
      _sdata = .;
      *(.data)
      *(.data*)
      *(.ramtext)
      /* Align to next 32bit multiple at the End of this section */
      . = ALIGN(4);
      /* locate ram functions aligned to 32 bit */
      *(.code_ram)
      /* Align to next 32bit multiple at the End of this section */
      . = ALIGN(4);
      /* End of Data section on SRAM; create symbol here -> VMA */
      _edata = .;
   } > SRAM AT>FLASH
   
   /* get the size of the .data section */
   _data_size = _edata - _sdata;
   /* get the load address (LMA) of the .data section */
   _data_loadaddr = LOADADDR(.data);
	
   .bss :
   {
      /* Start of Uninitialized Data section on SRAM; create symbol here -> VMA */
      _sbss = .;
      *(.bss)
      *(.bss*)
      *(COMMON)
      /* Align to next 32bit multiple at the End of this section */
      . = ALIGN(4);
      /* End of Uninitialized Data section on SRAM; create symbol here -> VMA */
      _ebss = .;
   } > SRAM AT>FLASH   
   
   /* get the size of the .bss section */
   _bss_size = _ebss - _sbss;
   
   /* use all unused SRAM as stack */
   .stack :
   {
     /* one should always align the stackpointer to 8 bytes */
     . = ALIGN(8);
	 /* end of stack; stack is predecrementing */
     _estack = .;
     *(.stack)     
   } > SRAM AT>FLASH
   
   /* End of SRAM is start of stack, because stackpointer of Mcu is implemented by predecrement */
   _sstack = 0x20008000;
   /* get the size of the .stack section */
   _stack_size = _sstack - _estack;
   
   /* This section is used for Mcu configuration; it contains the optionbytes */
   /* that configure the Mcu (e.g. which start address should be used) */
   .MCU_CONFIG_DATA 0x1FFFF800 :
   {
   	 KEEP(*(.mcu_cfg_data*))     
   } > OPTIONBYTES
   
}