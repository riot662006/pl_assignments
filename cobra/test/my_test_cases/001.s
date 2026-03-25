section .text
extern snek_error
global our_code_starts_here
our_code_starts_here:
  mov rax, 3
  ret

error:
  mov rdi, 1
  sub rsp, 8
  call snek_error
