section .data
str_0 db "hi", 0
str_1 db " in == ", 0
str_2 db " bla", 0
format db "%d", 10, 0
str_format db "%s", 10, 0

section .text
global _start
extern printf
testfnc:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov [rbp-8], rdi
    mov rax, [rbp-8]
    cmp rax, 1
    jge label_0
    mov rdi, str_format
    lea rsi, [str_0]
    xor rax, rax
    call printf
    mov rcx, [rbp-8]
    mov rdx, rcx
    add rdx, 1
    mov [rbp-8], rdx
    jmp label_1
label_0:
    mov rsi, [rbp-8]
    cmp rsi, 1
    jne label_2
    mov rdi, str_format
    lea rsi, [str_1]
    xor rax, rax
    call printf
    jmp label_3
label_2:
    mov rdi, str_format
    lea rsi, [str_2]
    xor rax, rax
    call printf
label_3:
label_1:
    mov r8, 0
    mov rax, r8
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov r9, 0
    mov rdi, r9
    call testfnc
    mov r10, rax
    mov [rbp-24], r10
    mov rax, 0
    leave
    ret
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall
