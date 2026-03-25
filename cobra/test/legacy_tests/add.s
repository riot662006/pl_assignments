section .text
extern snek_error
global our_code_starts_here
our_code_starts_here:
  mov rax, 5
sal rax, 1
mov rcx, rax
  and rcx, 1
  cmp rcx, 0
  jne error
add rax, 2
mov rcx, rax
  and rcx, 1
  cmp rcx, 0
  jne error
add rax, 2
  ret

error:
  mov rdi, 1
  sub rsp, 8
  call snek_error
