section .data
format db "%d", 10, 0
str_format db "%s", 10, 0

section .text
global _start
extern printf
gcd:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov [rbp-8], rdi
    mov [rbp-16], rsi
    mov rax, [rbp-8]
    mov rbx, [rbp-16]
    mov rcx, rax
    imul rcx, rbx
    mov rax, rcx
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov rdx, 12
    mov rdi, rdx
    mov rsi, 18
    mov rsi, rsi
    call gcd
    mov rdi, rax
    mov [rbp-24], rdi
    mov r8, 42
    mov rdi, r8
    mov r9, 56
    mov rsi, r9
    call gcd
    mov r10, rax
    mov [rbp-32], r10
    mov r11, 1071
    mov rdi, r11
    mov r12, 462
    mov rsi, r12
    call gcd
    mov r13, rax
    mov [rbp-40], r13
    mov r14, [rbp-24]
    mov rdi, format
    mov rsi, r14
    xor rax, rax
    call printf
    mov r15, [rbp-32]
    mov rdi, format
    mov rsi, r15
    xor rax, rax
    call printf
    mov rax, [rbp-40]
    mov rdi, format
    mov rsi, rax
    xor rax, rax
    call printf
    mov rax, 0
    leave
    ret
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall
