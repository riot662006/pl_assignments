section .text
extern snek_error
global our_code_starts_here
our_code_starts_here:
  mov rax, 5
sal rax, 1
  mov [rsp - 8], rax
  mov rax, 10
sal rax, 1
  mov [rsp - 16], rax
  mov rax, [rsp - 8]
  mov rcx, rax
  and rcx, 1
  cmp rcx, 0
  jne error
  sar rax, 1
  mov [rsp - 24], rax
  mov rax, [rsp - 16]
  mov rcx, rax
  and rcx, 1
  cmp rcx, 0
  jne error
  sar rax, 1
  add rax, [rsp - 24]
  sal rax, 1
  ret

error:
  mov rdi, 1
  sub rsp, 8
  call snek_error
