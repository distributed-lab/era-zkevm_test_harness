    .text
    .file	"Test_26"
    .rodata.cst32
    .p2align	5
    .text
    .globl	__entry
__entry:
.main:
        add 32, r0, r2
        add 64, r0, r3
        ; execute each possible context opcode
        context.set_ergs_per_pubdata r2
        context.set_context_u128 r3
        context.inc_tx_num
        context.meta r5
        context.sp r6
        context.ergs_left r7
        context.this r8
        context.caller r9
        context.code_source r10
        ret.ok r0