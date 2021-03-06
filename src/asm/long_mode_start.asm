; I put the 64-bit code into a new file to separate it from the 32-bit code, 
; thereby we can’t call the (now invalid) 32-bit code accidentally.

global long_mode_start

section .text
bits 64

long_mode_start:
    ; load 0/null into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ;call kmain
    extern kmain
    call kmain

    ; print okay to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt