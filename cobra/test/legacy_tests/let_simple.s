section .text
extern snek_error
global our_code_starts_here
our_code_starts_here:
  mov rax, 5
sal rax, 1
  mov [rsp - 8], rax
  mov rax, [rsp - 8]
  ret

error:
  mov rdi, 1
  sub rsp, 8
  call snek_error
