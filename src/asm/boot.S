.section .text.init

.global _start
_start:
    csrw satp, zero
    csrr	t0, mhartid
	bnez	t0, idle
    la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, set_stack_pointer
zero_bss:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, zero_bss
set_stack_pointer:
	# The stack grows from bottom to top, so we put the stack pointer
	# to the very end of the stack range.
	la		sp, _stack_end
    li		t0, (0b11 << 11) | (1 << 13)
    csrr    mstatus, t0
    csrw	mie, zero
    la		t1, kmain
    csrw	mepc, t1
    la		ra, idle
    mret
idle:
    wfi
    j idle
    