SECTIONS {
  /* Our program/kernel starts at address: `0x00100000` */
  . = 0x00100000;

  .boot :
  {
    KEEP(*(.boot))
  }

  .text :
  {
    KEEP(*(.text._start))
    *(.text.*)
  }

  /DISCARD/ :
  {
    *(.note.gnu.*)
  }
}
