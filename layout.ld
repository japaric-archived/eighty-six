/* Magic number: multiboot 2 */
MAGIC_NUMBER = 0xe85250d6;

SECTIONS {
  /* Our program/kernel starts at address: `0x00100000` */
  . = 1M;

  /* Multiboot headers */
  /* See http://intermezzos.github.io/book/multiboot-headers.html#The%20Section */
  .boot :
  {
    header_start = .;

    /* Magic number */
    LONG(MAGIC_NUMBER)

    /* Architecture 0: Protected mode i386 */
    LONG(0)

    /* Header length */
    LONG(header_end - header_start)

    /* Checksum */
    LONG(0x100000000 - (MAGIC_NUMBER + (header_end - header_start)))

    /* Required end tag */
    /* Type */
    SHORT(0)
    /* Flags */
    SHORT(0)
    /* Size */
    LONG(8)

    header_end = .;
  }

  /* Global Descriptor Table */
  /* See http://intermezzos.github.io/book/setting-up-a-gdt.html */
  /* FIXME if we use .rodata here, the linker (?) will inject a .got section just after this one.
     This causes the boot fail, probably, because the processor executes the .got section instead of
     text.start */
  /* .rodata : */
  .gdt :
  {
    gdt64 = .;
    /* Zero entry */
    QUAD(0);

    /* NOTE This won't do we want! Linker script variables bind to *addresses*, not to arbitrary
       integers. So this will actually bind to the address of the code segment and not store the
       offset of the code segment with respect to the start of the GDT */
    /* gdt64.code = . - gdt64; */

    /* Code segment */
    QUAD(1 << 44 | 1 << 47 | 1 << 41 | 1 << 43 | 1 << 53);
    /* Meaning of the bits set */
    /* 44: descriptor type */
    /* 47: present */
    /* 41: read/write */
    /* 43: executable */
    /* 53: 64-bit */

    /* Data segment */
    gdt64.data = .;
    QUAD(1 << 44 | 1 << 47 | 1 << 41);

    gdt64.pointer = .;
    SHORT(gdt64.pointer - gdt64 - 1)
    QUAD(gdt64)

    /* We don't have an equivalent to the EQU instruction, which "gives a symbolic name to a numeric
       constant". Instead of that we'll store the GDT offsets in (read-only) memory and use that
       from our boot code */
    /* TODO Should we do this arithmetic in the boot routine instead? It would save us 32 bits of
       memory */
    gdt64.data.offset = .;
    SHORT(gdt64.data - gdt64);
  }

  .text :
  {
    /* NOTE we use KEEP here to prevent the linker from dropping these symbols */
    KEEP(*(.text.start))
    KEEP(*(.text.long_mode_start))
    *(.text.*)
  }

  /* Page tables */
  .bss ALIGN(4096) :
  {
    /* Page-Map Level-4 Table (PML4) */
    p4_table = .;
    . += 4096;
    /* Page-Directory Pointer Table (PDP) */
    p3_table = .;
    . += 4096;
    /* Page-Directory Table (PD) */
    p2_table = .;
    . += 4096;
  }

  /DISCARD/ :
  {
    *(.note.gnu.*)
  }
}
