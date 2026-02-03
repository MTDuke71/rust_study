	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
@feat.00 = 0
	.intel_syntax noprefix
	.file	"8fkt43vh06dzz96dgkmk541q1"
	.def	_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E,unique,0
	.p2align	4
_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E:
.seh_proc _ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 128
	.seh_stackalloc 128
	.seh_endprologue
	cmp	byte ptr [rcx + 121], 0
	je	.LBB0_2
.LBB0_1:
	xor	eax, eax
	jmp	.LBB0_67
.LBB0_2:
	mov	rsi, qword ptr [rcx + 72]
	mov	r9, qword ptr [rcx + 80]
	cmp	dword ptr [rcx], 1
	jne	.LBB0_5
	lea	rdx, [rcx + 8]
	mov	rax, qword ptr [rcx + 88]
	cmp	qword ptr [rcx + 56], -1
	mov	rdi, rcx
	mov	rcx, qword ptr [rcx + 96]
	je	.LBB0_11
	mov	qword ptr [rsp + 40], rcx
	mov	qword ptr [rsp + 32], rax
	mov	byte ptr [rsp + 48], 0
	jmp	.LBB0_12
.LBB0_5:
	cmp	byte ptr [rcx + 26], 0
	je	.LBB0_7
.LBB0_6:
	xor	eax, eax
	mov	qword ptr [rsp + 56], rax
	cmp	dword ptr [rsp + 56], 1
	je	.LBB0_13
.LBB0_41:
	cmp	byte ptr [rcx + 121], 0
	jne	.LBB0_1
	mov	byte ptr [rcx + 121], 1
	cmp	byte ptr [rcx + 120], 1
	jne	.LBB0_44
	mov	rsi, qword ptr [rcx + 104]
	mov	rax, qword ptr [rcx + 112]
	jmp	.LBB0_45
.LBB0_7:
	mov	rax, qword ptr [rcx + 8]
	movzx	edx, byte ptr [rcx + 24]
	test	rax, rax
	je	.LBB0_15
	cmp	rax, r9
	jae	.LBB0_14
	cmp	byte ptr [rsi + rax], -64
	jge	.LBB0_15
.LBB0_68:
	mov	r8, rax
.LBB0_69:
	xor	dl, 1
	mov	byte ptr [rcx + 24], dl
	lea	rax, [rip + alloc_8af13d31a0ec5c8f7fb83c7ff891ca76]
	mov	qword ptr [rsp + 32], rax
	mov	rcx, rsi
	mov	rdx, r9
	call	_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE
.LBB0_11:
	mov	qword ptr [rsp + 40], rcx
	mov	qword ptr [rsp + 32], rax
	mov	byte ptr [rsp + 48], 1
.LBB0_12:
	lea	rcx, [rsp + 56]
	mov	r8, rsi
	call	_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE
	mov	rcx, rdi
	cmp	dword ptr [rsp + 56], 1
	jne	.LBB0_41
.LBB0_13:
	mov	rax, qword ptr [rsp + 64]
	mov	rdx, qword ptr [rsp + 72]
	mov	r8, qword ptr [rcx + 104]
	sub	rax, r8
	add	rsi, r8
	mov	qword ptr [rcx + 104], rdx
	jmp	.LBB0_46
.LBB0_44:
	mov	rsi, qword ptr [rcx + 104]
	mov	rax, qword ptr [rcx + 112]
	cmp	rax, rsi
	je	.LBB0_1
.LBB0_45:
	sub	rax, rsi
	add	rsi, qword ptr [rcx + 72]
.LBB0_46:
	mov	qword ptr [rsp + 56], 0
	mov	qword ptr [rsp + 64], rax
	mov	qword ptr [rsp + 72], rsi
	mov	qword ptr [rsp + 80], rax
	mov	qword ptr [rsp + 88], 0
	mov	qword ptr [rsp + 96], rax
	movabs	rax, 42949672970
	mov	qword ptr [rsp + 104], rax
	mov	byte ptr [rsp + 112], 1
	mov	word ptr [rsp + 120], 0
	lea	rcx, [rsp + 56]
	call	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
	xor	esi, esi
	test	rax, rax
	je	.LBB0_66
	lea	rdi, [rsp + 56]
	mov	ebx, 10
	jmp	.LBB0_51
.LBB0_48:
	xor	eax, eax
.LBB0_49:
	add	rsi, rax
.LBB0_50:
	mov	rcx, rdi
	call	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
	test	rax, rax
	je	.LBB0_66
.LBB0_51:
	mov	rcx, rax
	call	_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E
	test	rdx, rdx
	je	.LBB0_50
	mov	rcx, rax
	mov	r8, rdx
	cmp	rdx, 1
	jne	.LBB0_55
	movzx	eax, byte ptr [rcx]
	cmp	eax, 43
	je	.LBB0_50
	cmp	eax, 45
	je	.LBB0_50
	jmp	.LBB0_56
	.p2align	4
.LBB0_55:
	movzx	eax, byte ptr [rcx]
.LBB0_56:
	xor	r9d, r9d
	cmp	al, 43
	sete	r9b
	mov	rdx, r8
	sub	rdx, r9
	add	rcx, r9
	mov	rax, r9
	neg	rax
	cmp	rdx, 17
	jae	.LBB0_61
	test	rdx, rdx
	je	.LBB0_48
	add	r8, rax
	neg	r8
	xor	edx, edx
	xor	eax, eax
	.p2align	4
.LBB0_59:
	movzx	r9d, byte ptr [rcx + rdx]
	add	r9d, -48
	cmp	r9d, 9
	ja	.LBB0_50
	lea	rax, [rax + 4*rax]
	mov	r9d, r9d
	lea	rax, [r9 + 2*rax]
	inc	rdx
	mov	r9, r8
	add	r9, rdx
	jne	.LBB0_59
	jmp	.LBB0_49
	.p2align	4
.LBB0_61:
	add	r8, rax
	neg	r8
	xor	r9d, r9d
	xor	eax, eax
	.p2align	4
.LBB0_62:
	mov	rdx, r8
	add	rdx, r9
	je	.LBB0_49
	movzx	r10d, byte ptr [rcx + r9]
	add	r10d, -48
	cmp	r10d, 9
	ja	.LBB0_50
	mul	rbx
	mov	rdx, rax
	mov	eax, r10d
	seto	r10b
	add	rax, rdx
	setb	dl
	test	r10b, r10b
	jne	.LBB0_50
	inc	r9
	test	dl, dl
	je	.LBB0_62
	jmp	.LBB0_50
.LBB0_66:
	mov	eax, 1
.LBB0_67:
	mov	rdx, rsi
	.seh_startepilogue
	add	rsp, 128
	pop	rbx
	pop	rdi
	pop	rsi
	.seh_endepilogue
	ret
.LBB0_14:
	jne	.LBB0_68
.LBB0_15:
	cmp	rax, r9
	jne	.LBB0_18
	mov	eax, edx
	xor	al, 1
	mov	byte ptr [rcx + 24], al
	test	dl, 1
	jne	.LBB0_40
	mov	byte ptr [rcx + 26], 1
	jmp	.LBB0_6
.LBB0_18:
	movzx	r11d, byte ptr [rsi + rax]
	movzx	r10d, r11b
	test	r10b, r10b
	jns	.LBB0_24
	mov	r8d, r10d
	and	r8d, 31
	movzx	edi, byte ptr [rsi + rax + 1]
	and	edi, 63
	cmp	r10b, -32
	jb	.LBB0_22
	movzx	r10d, byte ptr [rsi + rax + 2]
	shl	edi, 6
	and	r10d, 63
	or	r10d, edi
	cmp	r11b, -16
	jb	.LBB0_23
	movzx	r11d, byte ptr [rsi + rax + 3]
	and	r8d, 7
	shl	r8d, 18
	shl	r10d, 6
	and	r11d, 63
	or	r11d, r10d
	or	r11d, r8d
	mov	r10d, r11d
	jmp	.LBB0_24
.LBB0_22:
	shl	r8d, 6
	or	r8d, edi
	mov	r10d, r8d
	jmp	.LBB0_24
.LBB0_23:
	shl	r8d, 12
	or	r10d, r8d
.LBB0_24:
	test	dl, 1
	je	.LBB0_26
	mov	r9, rax
	jmp	.LBB0_39
.LBB0_26:
	mov	r8d, 1
	cmp	r10d, 128
	jb	.LBB0_29
	mov	r8d, 2
	cmp	r10d, 2048
	jb	.LBB0_29
	cmp	r10d, 65536
	mov	r8d, 4
	sbb	r8, 0
.LBB0_29:
	add	r8, rax
	mov	qword ptr [rcx + 8], r8
	je	.LBB0_34
	cmp	r8, r9
	jae	.LBB0_33
	cmp	byte ptr [rsi + r8], -64
	jge	.LBB0_34
.LBB0_32:
	mov	dl, 1
	jmp	.LBB0_69
.LBB0_33:
	jne	.LBB0_32
.LBB0_34:
	cmp	r8, r9
	je	.LBB0_39
	movzx	eax, byte ptr [rsi + r8]
	test	al, al
	jns	.LBB0_38
	cmp	al, -32
.LBB0_38:
	mov	r9, r8
.LBB0_39:
	mov	byte ptr [rcx + 24], 0
.LBB0_40:
	mov	qword ptr [rsp + 64], r9
	mov	qword ptr [rsp + 72], r9
	mov	eax, 1
	mov	qword ptr [rsp + 56], rax
	cmp	dword ptr [rsp + 56], 1
	je	.LBB0_13
	jmp	.LBB0_41
	.seh_endproc

	.def	_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E,unique,1
	.p2align	4
_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E:
.Lfunc_begin0:
.seh_proc _ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 48
	.seh_stackalloc 48
	lea	rbp, [rsp + 48]
	.seh_setframe rbp, 48
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	qword ptr [rbp - 16], rcx
	add	rcx, 8
.Ltmp0:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp1:
	mov	rax, qword ptr [rbp - 16]
	mov	rdx, qword ptr [rax + 56]
	test	rdx, rdx
	je	.LBB1_6
	mov	rcx, qword ptr [rax + 64]
	mov	r8d, 1
	.seh_startepilogue
	add	rsp, 48
	pop	rbp
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB1_6:
	nop
	.seh_startepilogue
	add	rsp, 48
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.long	$cppxdata$_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@IMGREL
	.section	.text,"xr",one_only,_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E,unique,1
	.seh_endproc
	.def	"?dtor$3@?0?_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@4HA":
.seh_proc "?dtor$3@?0?_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@4HA"
.LBB1_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	rax, qword ptr [rbp - 16]
	mov	rdx, qword ptr [rax + 56]
	test	rdx, rdx
	je	.LBB1_5
	mov	rax, qword ptr [rbp - 16]
	mov	rcx, qword ptr [rax + 64]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB1_5:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end0:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E,unique,1
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E,unique,0
	.p2align	2, 0x0
$cppxdata$_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@IMGREL
	.long	40
	.long	0
	.long	1
$stateUnwindMap$_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E:
	.long	-1
	.long	"?dtor$3@?0?_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E@4HA"@IMGREL
$ip2state$_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E,unique,1

	.def	_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E,unique,2
	.p2align	4
_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E:
.seh_proc _ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rdi, rcx
	mov	rsi, qword ptr [rcx + 8]
	mov	rbx, qword ptr [rcx + 16]
	test	rbx, rbx
	je	.LBB2_5
	lea	r14, [rsi + 8]
	jmp	.LBB2_2
	.p2align	4
.LBB2_4:
	add	r14, 24
	dec	rbx
	je	.LBB2_5
.LBB2_2:
	mov	rdx, qword ptr [r14 - 8]
	test	rdx, rdx
	je	.LBB2_4
	mov	rcx, qword ptr [r14]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB2_4
.LBB2_5:
	mov	rax, qword ptr [rdi]
	test	rax, rax
	je	.LBB2_6
	shl	rax, 3
	lea	rdx, [rax + 2*rax]
	mov	r8d, 8
	mov	rcx, rsi
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB2_6:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E,unique,3
	.p2align	4
_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E:
.seh_proc _ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rdi, rcx
	mov	rsi, qword ptr [rcx + 8]
	mov	rbx, qword ptr [rcx + 16]
	test	rbx, rbx
	je	.LBB3_5
	lea	r14, [rsi + 8]
	jmp	.LBB3_2
	.p2align	4
.LBB3_4:
	add	r14, 24
	dec	rbx
	je	.LBB3_5
.LBB3_2:
	mov	rdx, qword ptr [r14 - 8]
	test	rdx, rdx
	je	.LBB3_4
	mov	rcx, qword ptr [r14]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB3_4
.LBB3_5:
	mov	rax, qword ptr [rdi]
	test	rax, rax
	je	.LBB3_6
	shl	rax, 3
	lea	rdx, [rax + 2*rax]
	mov	r8d, 8
	mov	rcx, rsi
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB3_6:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E,unique,4
	.p2align	4
_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E:
	test	rdx, rdx
	je	.LBB4_2
	lea	rax, [4*rdx + 19]
	and	rax, -16
	add	rdx, rax
	add	rdx, 17
	je	.LBB4_2
	sub	rcx, rax
	mov	r8d, 16
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB4_2:
	ret

	.def	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE,unique,5
	.p2align	4
_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE:
.seh_proc _ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rax, qword ptr [rcx]
	cmp	rax, 3
	je	.LBB5_23
	cmp	eax, 2
	jb	.LBB5_23
	mov	rsi, rcx
	mov	eax, dword ptr [rcx + 40]
	test	eax, eax
	je	.LBB5_5
	cmp	eax, 2
	je	.LBB5_23
	cmp	eax, 3
	jne	.LBB5_24
.LBB5_5:
	mov	rdi, qword ptr [rsi + 16]
	mov	r14, qword ptr [rsi + 24]
	test	r14, r14
	je	.LBB5_22
	xor	r15d, r15d
	jmp	.LBB5_7
	.p2align	4
.LBB5_21:
	inc	r15
	cmp	r15, r14
	je	.LBB5_22
.LBB5_7:
	imul	r12, r15, 56
	mov	rbx, qword ptr [rdi + r12 + 8]
	mov	r13, qword ptr [rdi + r12 + 16]
	test	r13, r13
	je	.LBB5_19
	lea	rbp, [rbx + 40]
	jmp	.LBB5_9
	.p2align	4
.LBB5_17:
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB5_18:
	add	rbp, 72
	dec	r13
	je	.LBB5_19
.LBB5_9:
	mov	rdx, qword ptr [rbp - 8]
	lea	rax, [2*rdx]
	test	rax, rax
	jne	.LBB5_10
	mov	rax, qword ptr [rbp - 40]
	cmp	rax, 2
	je	.LBB5_18
	jmp	.LBB5_12
	.p2align	4
.LBB5_10:
	mov	rcx, qword ptr [rbp]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	mov	rax, qword ptr [rbp - 40]
	cmp	rax, 2
	je	.LBB5_18
.LBB5_12:
	mov	rdx, qword ptr [rbp - 32]
	test	rax, rax
	je	.LBB5_13
	test	rdx, rdx
	je	.LBB5_18
	mov	rcx, qword ptr [rbp - 24]
	add	rdx, rdx
	mov	r8d, 2
	jmp	.LBB5_17
.LBB5_13:
	test	rdx, rdx
	je	.LBB5_18
	mov	rcx, qword ptr [rbp - 24]
	mov	r8d, 1
	jmp	.LBB5_17
	.p2align	4
.LBB5_19:
	add	r12, rdi
	mov	rax, qword ptr [r12]
	test	rax, rax
	je	.LBB5_21
	shl	rax, 3
	lea	rdx, [rax + 8*rax]
	mov	r8d, 8
	mov	rcx, rbx
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB5_21
.LBB5_22:
	mov	rax, qword ptr [rsi + 8]
	test	rax, rax
	je	.LBB5_23
	imul	rdx, rax, 56
	mov	r8d, 8
	mov	rcx, rdi
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB5_23:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
.LBB5_24:
	lea	rcx, [rip + alloc_a931397211c33a1c8fe0d17838460834]
	lea	r8, [rip + alloc_6a6fc231b3cb64280fdbf03fad4b13a2]
	mov	edx, 121
	call	_ZN4core9panicking9panic_fmt17hdddacd639c98ccdaE
	int3
	.seh_endproc

	.def	_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E,unique,6
	.p2align	4
_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E:
	mov	rdx, qword ptr [rcx]
	test	rdx, rdx
	je	.LBB6_1
	mov	rcx, qword ptr [rcx + 8]
	mov	r8d, 1
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB6_1:
	ret

	.def	_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE,unique,7
	.p2align	4
_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE:
	add	rcx, 8
	jmp	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE

	.def	_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E,unique,8
	.p2align	4
_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E:
.seh_proc _ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	.seh_endprologue
	mov	rax, rcx
	lea	r8, [rcx + rdx]
	test	rdx, rdx
	je	.LBB8_1
	mov	r10, qword ptr [rip + __imp__ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE]
	xor	edx, edx
	mov	r9, rax
	jmp	.LBB8_7
.LBB8_25:
	movzx	r11d, r11b
	movzx	r11d, byte ptr [r10 + r11]
	shr	r11b
.LBB8_26:
	test	r11b, 1
	je	.LBB8_2
	.p2align	4
.LBB8_27:
	cmp	r9, r8
	je	.LBB8_28
.LBB8_7:
	mov	rsi, r9
	mov	rcx, rdx
	movzx	r9d, byte ptr [r9]
	movzx	r11d, r9b
	test	r11b, r11b
	js	.LBB8_9
	lea	r9, [rsi + 1]
	jmp	.LBB8_14
	.p2align	4
.LBB8_9:
	mov	edx, r11d
	and	edx, 31
	movzx	edi, byte ptr [rsi + 1]
	and	edi, 63
	cmp	r11b, -33
	jbe	.LBB8_10
	movzx	r11d, byte ptr [rsi + 2]
	shl	edi, 6
	and	r11d, 63
	or	r11d, edi
	cmp	r9b, -16
	jb	.LBB8_12
	lea	r9, [rsi + 4]
	movzx	edi, byte ptr [rsi + 3]
	and	edx, 7
	shl	edx, 18
	shl	r11d, 6
	and	edi, 63
	or	edi, r11d
	or	edi, edx
	mov	r11d, edi
	jmp	.LBB8_14
.LBB8_10:
	lea	r9, [rsi + 2]
	shl	edx, 6
	or	edx, edi
	mov	r11d, edx
	jmp	.LBB8_14
.LBB8_12:
	lea	r9, [rsi + 3]
	shl	edx, 12
	or	r11d, edx
	.p2align	4
.LBB8_14:
	mov	rdx, r9
	sub	rdx, rsi
	add	rdx, rcx
	lea	esi, [r11 - 9]
	cmp	esi, 5
	jb	.LBB8_27
	cmp	r11d, 32
	je	.LBB8_27
	cmp	r11d, 128
	jb	.LBB8_2
	mov	esi, r11d
	shr	esi, 8
	cmp	esi, 31
	jg	.LBB8_21
	test	esi, esi
	je	.LBB8_24
	cmp	esi, 22
	jne	.LBB8_2
	cmp	r11d, 5760
	sete	r11b
	jmp	.LBB8_26
.LBB8_21:
	cmp	esi, 32
	je	.LBB8_25
	cmp	esi, 48
	jne	.LBB8_2
	cmp	r11d, 12288
	sete	r11b
	jmp	.LBB8_26
.LBB8_24:
	movzx	r11d, r11b
	movzx	r11d, byte ptr [r10 + r11]
	jmp	.LBB8_26
.LBB8_28:
	xor	ecx, ecx
	xor	edx, edx
	jmp	.LBB8_51
.LBB8_1:
	xor	edx, edx
	mov	r9, rax
	xor	ecx, ecx
.LBB8_2:
	cmp	r9, r8
	je	.LBB8_51
	mov	r10, qword ptr [rip + __imp__ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE]
	jmp	.LBB8_4
.LBB8_47:
	movzx	esi, sil
	movzx	ebx, byte ptr [r10 + rsi]
	shr	bl
.LBB8_48:
	test	bl, 1
	je	.LBB8_50
	.p2align	4
.LBB8_49:
	cmp	r9, r8
	je	.LBB8_51
.LBB8_4:
	mov	r11, r8
	movsx	esi, byte ptr [r8 - 1]
	test	esi, esi
	js	.LBB8_29
	lea	r8, [r11 - 1]
	lea	edi, [rsi - 9]
	cmp	edi, 5
	jb	.LBB8_49
	jmp	.LBB8_37
	.p2align	4
.LBB8_29:
	movzx	edi, byte ptr [r11 - 2]
	cmp	dil, -64
	jge	.LBB8_30
	movzx	ebx, byte ptr [r11 - 3]
	cmp	bl, -64
	jge	.LBB8_32
	lea	r8, [r11 - 4]
	movzx	ebp, byte ptr [r11 - 4]
	and	ebp, 7
	shl	ebp, 6
	and	ebx, 63
	or	ebx, ebp
	jmp	.LBB8_34
.LBB8_30:
	lea	r8, [r11 - 2]
	and	edi, 31
	jmp	.LBB8_35
.LBB8_32:
	lea	r8, [r11 - 3]
	and	ebx, 15
.LBB8_34:
	shl	ebx, 6
	and	edi, 63
	or	edi, ebx
.LBB8_35:
	shl	edi, 6
	and	sil, 63
	movzx	esi, sil
	or	esi, edi
	lea	edi, [rsi - 9]
	cmp	edi, 5
	jb	.LBB8_49
.LBB8_37:
	cmp	esi, 32
	je	.LBB8_49
	cmp	esi, 128
	jb	.LBB8_50
	mov	edi, esi
	shr	edi, 8
	cmp	edi, 31
	jg	.LBB8_43
	test	edi, edi
	je	.LBB8_46
	cmp	edi, 22
	jne	.LBB8_50
	cmp	esi, 5760
	sete	bl
	jmp	.LBB8_48
.LBB8_43:
	cmp	edi, 32
	je	.LBB8_47
	cmp	edi, 48
	jne	.LBB8_50
	cmp	esi, 12288
	sete	bl
	jmp	.LBB8_48
.LBB8_46:
	movzx	esi, sil
	movzx	ebx, byte ptr [r10 + rsi]
	jmp	.LBB8_48
.LBB8_50:
	sub	rdx, r9
	add	rdx, r11
.LBB8_51:
	sub	rdx, rcx
	add	rax, rcx
	.seh_startepilogue
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE,unique,9
	.p2align	4
_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE:
.seh_proc _ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	.seh_endprologue
	mov	rax, rdx
	mov	qword ptr [rsp + 32], rcx
	mov	rdx, qword ptr [rsp + 184]
	mov	rsi, qword ptr [rax + 32]
	lea	r10, [rdx + rsi]
	dec	r10
	cmp	r10, r9
	jae	.LBB9_8
	movzx	ebx, byte ptr [rsp + 192]
	mov	rdi, qword ptr [rsp + 176]
	lea	rcx, [rdx - 1]
	mov	qword ptr [rsp + 64], rcx
	mov	r15, qword ptr [rax + 24]
	mov	r12, qword ptr [rax]
	mov	rcx, qword ptr [rax + 16]
	mov	r11, rdx
	mov	qword ptr [rsp + 48], rcx
	sub	r11, rcx
	mov	qword ptr [rsp + 40], r11
	mov	r13, qword ptr [rax + 48]
	mov	rcx, r12
	neg	rcx
	mov	qword ptr [rsp + 56], rcx
	jmp	.LBB9_2
	.p2align	4
.LBB9_6:
	mov	qword ptr [rax + 48], r10
	mov	r13, r10
.LBB9_7:
	mov	rcx, qword ptr [rsp + 64]
	lea	r10, [rcx + rsi]
	cmp	r10, r9
	jae	.LBB9_8
.LBB9_2:
	movzx	ecx, byte ptr [r8 + r10]
	bt	r15, rcx
	jae	.LBB9_3
	cmp	r13, r12
	mov	r10, r12
	cmova	r10, r13
	test	bl, bl
	cmovne	r10, r12
	lea	r11, [r8 + rsi]
	mov	rbp, r10
	.p2align	4
.LBB9_10:
	cmp	rbp, rdx
	jae	.LBB9_11
	lea	rcx, [rsi + rbp]
	cmp	rcx, r9
	jae	.LBB9_25
	lea	rcx, [rbp + 1]
	movzx	r14d, byte ptr [rdi + rbp]
	cmp	r14b, byte ptr [r11 + rbp]
	mov	rbp, rcx
	je	.LBB9_10
	add	rsi, qword ptr [rsp + 56]
	add	rsi, rcx
	mov	qword ptr [rax + 32], rsi
	test	bl, bl
	jne	.LBB9_7
	jmp	.LBB9_5
	.p2align	4
.LBB9_3:
	add	rsi, rdx
	mov	qword ptr [rax + 32], rsi
	test	bl, bl
	jne	.LBB9_7
.LBB9_5:
	xor	r10d, r10d
	jmp	.LBB9_6
	.p2align	4
.LBB9_11:
	test	bl, bl
	mov	rbp, r13
	mov	ecx, 0
	cmovne	rbp, rcx
	mov	r10, r12
	.p2align	4
.LBB9_12:
	cmp	rbp, r10
	jae	.LBB9_13
	dec	r10
	cmp	r10, rdx
	jae	.LBB9_26
	lea	r11, [r10 + rsi]
	cmp	r11, r9
	jae	.LBB9_24
	movzx	ecx, byte ptr [rdi + r10]
	cmp	cl, byte ptr [r8 + r11]
	je	.LBB9_12
	add	rsi, qword ptr [rsp + 48]
	mov	qword ptr [rax + 32], rsi
	mov	r10, qword ptr [rsp + 40]
	test	bl, bl
	je	.LBB9_6
	jmp	.LBB9_7
.LBB9_8:
	mov	qword ptr [rax + 32], r9
	xor	eax, eax
	mov	rcx, qword ptr [rsp + 32]
	jmp	.LBB9_16
.LBB9_13:
	add	rdx, rsi
	mov	qword ptr [rax + 32], rdx
	test	bl, bl
	jne	.LBB9_15
	mov	qword ptr [rax + 48], 0
.LBB9_15:
	mov	rcx, qword ptr [rsp + 32]
	mov	qword ptr [rcx + 8], rsi
	mov	qword ptr [rcx + 16], rdx
	mov	eax, 1
.LBB9_16:
	mov	qword ptr [rcx], rax
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
.LBB9_25:
	add	r10, rsi
	cmp	r9, r10
	cmova	r10, r9
	lea	r8, [rip + alloc_aeae60839ee01e593e8491fb61f2dda8]
	mov	rcx, r10
	mov	rdx, r9
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
.LBB9_24:
	lea	r8, [rip + alloc_3c328170803c8011e86f11240ac4582e]
	mov	rcx, r11
	mov	rdx, r9
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
.LBB9_26:
	lea	r8, [rip + alloc_960bc1bf861ba41d9b8231bbd37f66f6]
	mov	rcx, r10
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
	int3
	.seh_endproc

	.def	_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E,unique,10
	.p2align	4
_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E:
.Lfunc_begin1:
.seh_proc _ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 96
	.seh_stackalloc 96
	lea	rbp, [rsp + 96]
	.seh_setframe rbp, 96
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	rdi, r8
	mov	rbx, rdx
	mov	rsi, rcx
	call	_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$51__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$17hed5e461344c1f9f9E
	cmp	byte ptr [rax + 16], 1
	jne	.LBB10_2
	mov	rcx, qword ptr [rax]
	mov	rdx, qword ptr [rax + 8]
.LBB10_3:
	lea	r8, [rcx + 1]
	mov	qword ptr [rax], r8
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.0+16]
	movaps	xmmword ptr [rbp - 48], xmm0
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.0]
	movaps	xmmword ptr [rbp - 64], xmm0
	mov	qword ptr [rbp - 32], rcx
	mov	qword ptr [rbp - 24], rdx
	mov	rax, rdi
	sub	rax, rbx
	jne	.LBB10_4
.LBB10_16:
	movaps	xmm0, xmmword ptr [rbp - 64]
	movaps	xmm1, xmmword ptr [rbp - 48]
	mov	rax, qword ptr [rbp - 32]
	mov	qword ptr [rsi + 32], rax
	mov	rax, qword ptr [rbp - 24]
	mov	qword ptr [rsi + 40], rax
	movups	xmmword ptr [rsi + 16], xmm1
	movups	xmmword ptr [rsi], xmm0
	.seh_startepilogue
	add	rsp, 96
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
.LBB10_2:
	mov	r14, rax
	call	_ZN3std3sys6random19hashmap_random_keys17hc3f03c6d163b2da2E
	mov	rcx, rax
	mov	rax, r14
	mov	qword ptr [r14 + 8], rdx
	mov	byte ptr [r14 + 16], 1
	jmp	.LBB10_3
.LBB10_4:
	lea	r8, [rbp - 32]
	mov	rdx, rax
	shr	rdx, 2
	and	eax, 3
	cmp	rax, 1
	sbb	rdx, -1
.Ltmp2:
	lea	rcx, [rbp - 64]
	mov	r9b, 1
	call	_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E
.Ltmp3:
	lea	r14, [rbp - 64]
.LBB10_6:
	movzx	r8d, byte ptr [rbx]
	movzx	edx, r8b
	test	dl, dl
	js	.LBB10_8
	inc	rbx
	jmp	.LBB10_14
.LBB10_8:
	mov	eax, edx
	and	eax, 31
	movzx	r9d, byte ptr [rbx + 1]
	and	r9d, 63
	cmp	dl, -33
	jbe	.LBB10_9
	movzx	ecx, byte ptr [rbx + 2]
	shl	r9d, 6
	and	ecx, 63
	or	ecx, r9d
	cmp	r8b, -16
	jb	.LBB10_11
	movzx	edx, byte ptr [rbx + 3]
	and	eax, 7
	shl	eax, 18
	shl	ecx, 6
	and	edx, 63
	or	edx, ecx
	or	edx, eax
	cmp	edx, 1114112
	je	.LBB10_16
	add	rbx, 4
	jmp	.LBB10_14
.LBB10_9:
	add	rbx, 2
	shl	eax, 6
	or	eax, r9d
	mov	edx, eax
	jmp	.LBB10_14
.LBB10_11:
	add	rbx, 3
	shl	eax, 12
	or	ecx, eax
	mov	edx, ecx
.LBB10_14:
.Ltmp4:
	mov	rcx, r14
	call	_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE
.Ltmp5:
	cmp	rbx, rdi
	jne	.LBB10_6
	jmp	.LBB10_16
	.seh_handlerdata
	.long	$cppxdata$_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@IMGREL
	.section	.text,"xr",one_only,_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E,unique,10
	.seh_endproc
	.def	"?dtor$17@?0?_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$17@?0?_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@4HA":
.seh_proc "?dtor$17@?0?_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@4HA"
.LBB10_17:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 96]
	.seh_endprologue
	mov	rcx, qword ptr [rbp - 64]
	mov	rdx, qword ptr [rbp - 56]
	call	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end1:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E,unique,10
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E,unique,1
	.p2align	2, 0x0
$cppxdata$_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@IMGREL
	.long	88
	.long	0
	.long	1
$stateUnwindMap$_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E:
	.long	-1
	.long	"?dtor$17@?0?_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E@4HA"@IMGREL
$ip2state$_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E:
	.long	.Lfunc_begin1@IMGREL
	.long	-1
	.long	.Ltmp2@IMGREL+1
	.long	0
	.long	.Ltmp5@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E,unique,10

	.def	_ZN4core5error5Error11description17h1649966410d8c91fE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error11description17h1649966410d8c91fE,unique,11
	.p2align	4
_ZN4core5error5Error11description17h1649966410d8c91fE:
	lea	rax, [rip + alloc_04d7ce44d7c86a9a02b346ab945bf155]
	mov	edx, 40
	ret

	.def	_ZN4core5error5Error11description17h6d684e2ae41137b1E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error11description17h6d684e2ae41137b1E,unique,12
	.p2align	4
_ZN4core5error5Error11description17h6d684e2ae41137b1E:
	lea	rax, [rip + alloc_04d7ce44d7c86a9a02b346ab945bf155]
	mov	edx, 40
	ret

	.def	_ZN4core5error5Error5cause17h3780c7e2d97e9501E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error5cause17h3780c7e2d97e9501E,unique,13
	.p2align	4
_ZN4core5error5Error5cause17h3780c7e2d97e9501E:
.seh_proc _ZN4core5error5Error5cause17h3780c7e2d97e9501E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	call	_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E
	mov	rcx, rax
	.seh_startepilogue
	add	rsp, 40
	.seh_endepilogue
	rex64 jmp	qword ptr [rdx + 48]
	.seh_endproc

	.def	_ZN4core5error5Error5cause17h7a349d6c962996eeE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error5cause17h7a349d6c962996eeE,unique,14
	.p2align	4
_ZN4core5error5Error5cause17h7a349d6c962996eeE:
	xor	eax, eax
	ret

	.def	_ZN4core5error5Error7provide17h05d6e66acb486045E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error7provide17h05d6e66acb486045E,unique,15
	.p2align	4
_ZN4core5error5Error7provide17h05d6e66acb486045E:
	ret

	.def	_ZN4core5error5Error7provide17h9df6819c0298c953E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error7provide17h9df6819c0298c953E,unique,16
	.p2align	4
_ZN4core5error5Error7provide17h9df6819c0298c953E:
	ret

	.def	_ZN4core5error5Error7type_id17h01cfa94765d144f3E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error7type_id17h01cfa94765d144f3E,unique,17
	.p2align	4
_ZN4core5error5Error7type_id17h01cfa94765d144f3E:
	mov	rax, rcx
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.1]
	movups	xmmword ptr [rcx], xmm0
	ret

	.def	_ZN4core5error5Error7type_id17h515f3f39467e6574E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error7type_id17h515f3f39467e6574E,unique,18
	.p2align	4
_ZN4core5error5Error7type_id17h515f3f39467e6574E:
	mov	rax, rcx
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.2]
	movups	xmmword ptr [rcx], xmm0
	ret

	.def	_ZN4core5error5Error7type_id17hc7a2819971514648E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error7type_id17hc7a2819971514648E,unique,19
	.p2align	4
_ZN4core5error5Error7type_id17hc7a2819971514648E:
	mov	rax, rcx
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.3]
	movups	xmmword ptr [rcx], xmm0
	ret

	.def	_ZN4core5error5Error7type_id17hdc7adb8570ca242eE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5error5Error7type_id17hdc7adb8570ca242eE,unique,20
	.p2align	4
_ZN4core5error5Error7type_id17hdc7adb8570ca242eE:
	mov	rax, rcx
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.4]
	movups	xmmword ptr [rcx], xmm0
	ret

	.def	_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE,unique,21
	.p2align	4
_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE:
.seh_proc _ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rdi, r8
	mov	rsi, rdx
	cmp	r9, 8
	jb	.LBB21_2
	shr	r9, 3
	mov	rbx, r9
	shl	rbx, 5
	lea	rdx, [rcx + rbx]
	imul	r14, r9, 56
	lea	r8, [rcx + r14]
	mov	r15, r9
	call	_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE
	mov	r12, rax
	lea	rdx, [rsi + rbx]
	lea	r8, [rsi + r14]
	mov	rcx, rsi
	mov	r9, r15
	call	_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE
	mov	rsi, rax
	add	rbx, rdi
	add	r14, rdi
	mov	rcx, rdi
	mov	rdx, rbx
	mov	r8, r14
	mov	r9, r15
	call	_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE
	mov	rcx, r12
	mov	rdi, rax
.LBB21_2:
	mov	rax, qword ptr [rcx]
	mov	rdx, qword ptr [rsi]
	cmp	rdx, rax
	setb	r8b
	mov	r9, qword ptr [rdi]
	cmp	r9, rax
	setb	al
	xor	al, r8b
	cmp	r9, rdx
	setb	dl
	xor	dl, r8b
	cmovne	rsi, rdi
	test	al, al
	cmovne	rsi, rcx
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E,unique,22
	.p2align	4
_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E:
	cmp	r8, rdx
	ja	.LBB22_12
	jne	.LBB22_2
.LBB22_11:
	ret
.LBB22_2:
	lea	rax, [rcx + 8*rdx]
	lea	rdx, [rcx + 8*r8]
	shl	r8, 3
	jmp	.LBB22_3
	.p2align	4
.LBB22_6:
	mov	r10, rcx
.LBB22_9:
	mov	qword ptr [r10], r9
.LBB22_10:
	add	rdx, 8
	add	r8, 8
	cmp	rdx, rax
	je	.LBB22_11
.LBB22_3:
	mov	r11, qword ptr [rdx - 8]
	mov	r9, qword ptr [rdx]
	cmp	r11, r9
	jae	.LBB22_10
	mov	r10, r8
	.p2align	4
.LBB22_5:
	mov	qword ptr [rcx + r10], r11
	cmp	r10, 8
	je	.LBB22_6
	mov	r11, qword ptr [rcx + r10 - 16]
	add	r10, -8
	cmp	r11, r9
	jb	.LBB22_5
	add	r10, rcx
	jmp	.LBB22_9
.LBB22_12:
	ud2

	.def	_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE,unique,23
	.globl	_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE
	.p2align	4
_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE:
.seh_proc _ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 48
	.seh_stackalloc 48
	.seh_endprologue
	cmp	rdx, 2
	jb	.LBB23_20
	mov	rax, qword ptr [rcx]
	mov	r9, qword ptr [rcx + 8]
	mov	r10d, 2
	cmp	rax, r9
	jae	.LBB23_6
	cmp	rdx, 2
	je	.LBB23_7
	mov	r11, r9
	.p2align	4
.LBB23_4:
	mov	rsi, r11
	mov	r11, qword ptr [rcx + 8*r10]
	cmp	rsi, r11
	jae	.LBB23_7
	inc	r10
	cmp	rdx, r10
	jne	.LBB23_4
	jmp	.LBB23_12
.LBB23_6:
	cmp	rdx, 2
	jne	.LBB23_9
.LBB23_7:
	cmp	r10, rdx
	je	.LBB23_12
	mov	rax, rdx
	or	rax, 1
	bsr	r9, rax
	xor	r9d, 63
	add	r9d, r9d
	xor	r9d, 126
	mov	qword ptr [rsp + 32], r8
	xor	r8d, r8d
	call	_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E
	nop
	.seh_startepilogue
	add	rsp, 48
	pop	rsi
	.seh_endepilogue
	ret
.LBB23_9:
	mov	r11, r9
	.p2align	4
.LBB23_10:
	mov	rsi, r11
	mov	r11, qword ptr [rcx + 8*r10]
	cmp	rsi, r11
	jb	.LBB23_7
	inc	r10
	cmp	rdx, r10
	jne	.LBB23_10
.LBB23_12:
	cmp	rax, r9
	jae	.LBB23_20
	mov	rax, rdx
	shr	rax
	cmp	rdx, 8
	jae	.LBB23_15
	xor	r8d, r8d
	jmp	.LBB23_18
.LBB23_15:
	movabs	r8, 576460752303423484
	and	r8, rax
	mov	r9, rax
	and	r9, -4
	neg	r9
	lea	r10, [rcx + 16]
	lea	r11, [rcx + 8*rdx]
	add	r11, -16
	xor	esi, esi
	.p2align	4
.LBB23_16:
	movdqu	xmm0, xmmword ptr [r10 - 16]
	movdqu	xmm1, xmmword ptr [r10]
	movdqu	xmm2, xmmword ptr [r11 + 8*rsi - 16]
	movdqu	xmm3, xmmword ptr [r11 + 8*rsi]
	pshufd	xmm3, xmm3, 78
	pshufd	xmm2, xmm2, 78
	movdqu	xmmword ptr [r10 - 16], xmm3
	movdqu	xmmword ptr [r10], xmm2
	pshufd	xmm0, xmm0, 78
	movdqu	xmmword ptr [r11 + 8*rsi], xmm0
	pshufd	xmm0, xmm1, 78
	movdqu	xmmword ptr [r11 + 8*rsi - 16], xmm0
	add	rsi, -4
	add	r10, 32
	cmp	r9, rsi
	jne	.LBB23_16
	cmp	rax, r8
	je	.LBB23_20
.LBB23_18:
	neg	rax
	lea	r9, [rcx + 8*r8]
	neg	r8
	lea	rcx, [rcx + 8*rdx]
	add	rcx, -8
	.p2align	4
.LBB23_19:
	mov	rdx, qword ptr [r9]
	mov	r10, qword ptr [rcx + 8*r8]
	mov	qword ptr [r9], r10
	mov	qword ptr [rcx + 8*r8], rdx
	dec	r8
	add	r9, 8
	cmp	rax, r8
	jne	.LBB23_19
.LBB23_20:
	.seh_startepilogue
	add	rsp, 48
	pop	rsi
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE,unique,24
	.globl	_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE
	.p2align	4
_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE:
.seh_proc _ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE
	push	rsi
	.seh_pushreg rsi
	.seh_endprologue
	mov	rax, rdx
	shr	rax
	add	rax, rdx
	jne	.LBB24_1
.LBB24_10:
	.seh_startepilogue
	pop	rsi
	.seh_endepilogue
	ret
	.p2align	4
.LBB24_9:
	test	rax, rax
	je	.LBB24_10
.LBB24_1:
	dec	rax
	mov	r8, rax
	sub	r8, rdx
	jae	.LBB24_3
	mov	r8, qword ptr [rcx]
	mov	r9, qword ptr [rcx + 8*rax]
	mov	qword ptr [rcx], r9
	mov	qword ptr [rcx + 8*rax], r8
	xor	r8d, r8d
.LBB24_3:
	cmp	rdx, rax
	mov	r9, rax
	cmovb	r9, rdx
	lea	r10, [2*r8 + 1]
	cmp	r10, r9
	jae	.LBB24_9
	lea	r11, [r8 + r8]
	.p2align	4
.LBB24_5:
	lea	rsi, [r11 + 2]
	cmp	rsi, r9
	jae	.LBB24_7
	mov	r11, qword ptr [rcx + 8*r11 + 16]
	cmp	r11, qword ptr [rcx + 8*r10]
	adc	r10, 0
.LBB24_7:
	mov	rsi, r10
	mov	r10, qword ptr [rcx + 8*r8]
	mov	r11, qword ptr [rcx + 8*rsi]
	cmp	r11, r10
	jae	.LBB24_9
	mov	qword ptr [rcx + 8*r8], r11
	mov	qword ptr [rcx + 8*rsi], r10
	lea	r11, [rsi + rsi]
	lea	r10, [2*rsi + 1]
	mov	r8, rsi
	cmp	r10, r9
	jb	.LBB24_5
	jmp	.LBB24_9
	.seh_endproc

	.def	_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E,unique,25
	.p2align	4
_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E:
.seh_proc _ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 376
	.seh_stackalloc 376
	.seh_endprologue
	mov	r13, rdx
	mov	rbp, rcx
	cmp	rdx, 33
	jae	.LBB25_1
.LBB25_5:
	cmp	r13, 2
	jb	.LBB25_30
	mov	r12, r13
	shr	r12
	cmp	r13, 18
	mov	rdx, r12
	cmovb	rdx, r13
	lea	r15, [8*r12]
	add	r15, rbp
	mov	rax, r13
	sub	rax, r12
	mov	qword ptr [rsp + 112], rax
	mov	r8, rbp
	mov	qword ptr [rsp + 88], r13
	mov	qword ptr [rsp + 80], rbp
	mov	qword ptr [rsp + 72], r15
	mov	qword ptr [rsp + 64], r12
	.p2align	4
.LBB25_7:
	cmp	rdx, 12
	jbe	.LBB25_8
	mov	r12, qword ptr [r8 + 96]
	mov	rax, qword ptr [r8]
	mov	rcx, qword ptr [r8 + 8]
	cmp	rax, r12
	mov	r9, r12
	cmovb	r9, rax
	mov	qword ptr [rsp + 48], r9
	cmova	r12, rax
	mov	rax, qword ptr [r8 + 80]
	cmp	rcx, rax
	mov	r15, rax
	cmovb	r15, rcx
	cmova	rax, rcx
	mov	r11, qword ptr [r8 + 72]
	mov	qword ptr [rsp + 56], rdx
	mov	rdx, qword ptr [r8 + 16]
	cmp	rdx, r11
	mov	rcx, r11
	cmova	r11, rdx
	cmovb	rcx, rdx
	mov	rbx, qword ptr [r8 + 56]
	mov	rdx, qword ptr [r8 + 24]
	cmp	rdx, rbx
	mov	r10, rbx
	cmovb	r10, rdx
	cmova	rbx, rdx
	mov	rdx, qword ptr [r8 + 88]
	mov	r9, qword ptr [r8 + 40]
	cmp	r9, rdx
	mov	rbp, rdx
	cmova	rdx, r9
	mov	qword ptr [rsp + 96], rdx
	cmovb	rbp, r9
	mov	r14, qword ptr [r8 + 64]
	mov	r9, qword ptr [r8 + 48]
	cmp	r9, r14
	mov	r13, r14
	cmovb	r13, r9
	cmova	r14, r9
	cmp	rax, r14
	mov	rdi, r14
	cmovb	rdi, rax
	cmova	r14, rax
	cmp	r11, rbx
	mov	rsi, rbx
	cmovb	rsi, r11
	cmova	rbx, r11
	mov	r9, qword ptr [r8 + 32]
	cmp	r9, rbp
	mov	rax, rbp
	cmovb	rax, r9
	cmova	rbp, r9
	cmp	r10, rcx
	mov	r9, rcx
	cmovb	r9, r10
	cmovbe	r10, rcx
	cmp	r13, r15
	mov	r11, r15
	cmovb	r11, r13
	cmovbe	r13, r15
	cmp	r12, rbp
	mov	rdx, rbp
	cmovb	rdx, r12
	cmova	rbp, r12
	cmp	r14, rbx
	mov	rcx, rbx
	cmovb	rcx, r14
	mov	qword ptr [rsp + 104], rcx
	cmova	rbx, r14
	cmp	rsi, rdi
	mov	r15, rdi
	cmovb	r15, rsi
	cmovbe	rsi, rdi
	cmp	r10, r13
	mov	r12, r13
	cmovb	r12, r10
	cmova	r13, r10
	cmp	r9, r11
	mov	rcx, r11
	cmovb	rcx, r9
	cmova	r11, r9
	mov	r9, qword ptr [rsp + 48]
	cmp	rax, r9
	mov	r10, r9
	cmovb	r10, rax
	cmovbe	rax, r9
	cmp	rdx, r15
	mov	r9, r15
	cmovb	r9, rdx
	cmova	r15, rdx
	mov	rdx, qword ptr [rsp + 96]
	cmp	rdx, r11
	mov	rdi, r11
	cmovb	rdi, rdx
	cmova	r11, rdx
	cmp	r12, rax
	mov	r14, rax
	cmovb	r14, r12
	cmova	rax, r12
	cmp	rcx, r10
	mov	rdx, r10
	cmovb	rdx, rcx
	mov	qword ptr [rsp + 48], rdx
	cmova	r10, rcx
	cmp	rbp, r11
	mov	r12, r11
	cmovb	r12, rbp
	cmova	r11, rbp
	cmp	rsi, rax
	mov	rdx, rax
	cmovb	rdx, rsi
	cmova	rax, rsi
	cmp	r15, r13
	mov	rsi, r13
	cmovb	rsi, r15
	cmovbe	r15, r13
	cmp	r9, r14
	mov	rcx, r14
	cmovb	rcx, r9
	cmova	r14, r9
	cmp	rdi, r10
	mov	r13, r10
	cmovb	r13, rdi
	cmova	r10, rdi
	cmp	r11, rbx
	mov	rbp, rbx
	cmovb	rbp, r11
	cmovbe	r11, rbx
	mov	rdi, qword ptr [rsp + 104]
	cmp	rdi, r12
	mov	r9, r12
	cmovb	r9, rdi
	cmova	r12, rdi
	cmp	r14, r10
	mov	rdi, r10
	cmovb	rdi, r14
	cmova	r10, r14
	cmp	rsi, rdx
	mov	rbx, rdx
	cmovb	rbx, rsi
	cmovbe	rsi, rdx
	cmp	r13, rcx
	mov	r14, rcx
	cmovb	r14, r13
	cmovbe	r13, rcx
	cmp	rbp, rax
	mov	rdx, rax
	cmovb	rdx, rbp
	cmovbe	rbp, rax
	cmp	r12, r15
	mov	rax, r15
	cmovb	rax, r12
	cmovbe	r12, r15
	cmp	r9, r10
	mov	rcx, r10
	cmovb	rcx, r9
	cmova	r10, r9
	cmp	rdi, r13
	mov	r9, r13
	cmovb	r9, rdi
	cmova	r13, rdi
	cmp	rbp, r12
	mov	rdi, r12
	cmovb	rdi, rbp
	mov	r15, qword ptr [rsp + 48]
	mov	qword ptr [r8 + 96], r15
	cmova	r12, rbp
	mov	rbp, qword ptr [rsp + 80]
	cmp	rdx, rax
	mov	r15, rax
	cmovb	r15, rdx
	mov	qword ptr [r8], r11
	cmova	rax, rdx
	cmp	r10, rsi
	mov	r11, rsi
	cmovb	r11, r10
	mov	qword ptr [r8 + 88], r14
	cmovbe	r10, rsi
	cmp	rcx, rbx
	mov	rsi, rbx
	cmovb	rsi, rcx
	mov	qword ptr [r8 + 80], r9
	cmovbe	rcx, rbx
	cmp	rdi, rax
	mov	r9, rax
	cmovb	r9, rdi
	mov	qword ptr [r8 + 8], r12
	mov	r12, qword ptr [rsp + 64]
	cmova	rax, rdi
	cmp	r15, r10
	mov	rdx, r10
	cmovb	rdx, r15
	mov	qword ptr [r8 + 16], rax
	cmova	r10, r15
	mov	r15, qword ptr [rsp + 72]
	cmp	rcx, r11
	mov	rax, r11
	cmovb	rax, rcx
	cmovbe	rcx, r11
	mov	qword ptr [r8 + 56], rax
	cmp	rsi, r13
	mov	rax, r13
	cmovb	rax, rsi
	cmovbe	rsi, r13
	mov	r13, qword ptr [rsp + 88]
	mov	qword ptr [r8 + 64], rsi
	cmp	r9, r10
	mov	r11, r10
	cmovb	r11, r9
	mov	qword ptr [r8 + 72], rax
	cmova	r10, r9
	cmp	rdx, rcx
	mov	rax, rcx
	cmovb	rax, rdx
	mov	qword ptr [r8 + 24], r10
	cmova	rcx, rdx
	mov	rdx, qword ptr [rsp + 56]
	mov	qword ptr [r8 + 32], r11
	mov	qword ptr [r8 + 40], rcx
	mov	qword ptr [r8 + 48], rax
	mov	ecx, 13
	jmp	.LBB25_10
	.p2align	4
.LBB25_8:
	mov	ecx, 1
	cmp	rdx, 8
	jbe	.LBB25_10
	mov	r9, qword ptr [r8 + 24]
	mov	rax, qword ptr [r8]
	mov	qword ptr [rsp + 56], rdx
	mov	rdx, qword ptr [r8 + 8]
	cmp	rax, r9
	mov	rbx, r9
	cmovb	rbx, rax
	cmova	r9, rax
	mov	rcx, qword ptr [r8 + 56]
	cmp	rdx, rcx
	mov	rsi, rcx
	cmovb	rsi, rdx
	cmova	rcx, rdx
	mov	r11, qword ptr [r8 + 40]
	mov	rax, qword ptr [r8 + 16]
	cmp	rax, r11
	mov	r10, r11
	cmova	r11, rax
	cmovb	r10, rax
	mov	rdx, qword ptr [r8 + 64]
	mov	rdi, qword ptr [r8 + 32]
	cmp	rdi, rdx
	mov	rax, rdx
	cmovb	rax, rdi
	cmova	rdx, rdi
	cmp	r9, rsi
	mov	r14, rsi
	cmovb	r14, r9
	cmova	rsi, r9
	cmp	r11, rdx
	mov	rdi, rdx
	cmovb	rdi, r11
	cmova	rdx, r11
	cmp	rbx, rax
	mov	r11, rax
	cmovb	r11, rbx
	cmova	rax, rbx
	mov	rbx, qword ptr [r8 + 48]
	cmp	r10, rbx
	mov	r9, rbx
	cmovb	r9, r10
	cmovbe	r10, rbx
	cmp	rsi, rdx
	mov	rbx, rdx
	cmovb	rbx, rsi
	cmova	rdx, rsi
	cmp	rcx, rax
	mov	rsi, rax
	cmovb	rsi, rcx
	cmova	rax, rcx
	cmp	rdi, r10
	mov	rbp, r10
	cmovb	rbp, rdi
	cmova	r10, rdi
	cmp	r14, r11
	mov	r13, r11
	cmovb	r13, r14
	cmova	r11, r14
	cmp	rax, r10
	mov	r12, r10
	cmovb	r12, rax
	cmova	r10, rax
	cmp	rsi, r9
	mov	rcx, r9
	cmovb	rcx, rsi
	cmovbe	rsi, r9
	cmp	rbp, r11
	mov	r15, r11
	cmovb	r15, rbp
	cmova	r11, rbp
	mov	rbp, qword ptr [rsp + 80]
	cmp	rdx, r10
	mov	r14, r10
	cmovb	r14, rdx
	cmova	r10, rdx
	cmp	rbx, r12
	mov	r9, r12
	cmovb	r9, rbx
	cmova	r12, rbx
	cmp	rsi, r11
	mov	rax, r11
	cmovb	rax, rsi
	cmova	r11, rsi
	cmp	rcx, r13
	mov	rsi, r13
	cmovb	rsi, rcx
	cmovbe	rcx, r13
	mov	r13, qword ptr [rsp + 88]
	cmp	r12, r11
	mov	rdi, r11
	cmovb	rdi, r12
	mov	qword ptr [r8], r10
	cmova	r11, r12
	mov	r12, qword ptr [rsp + 64]
	cmp	r9, rax
	mov	rdx, rax
	cmovb	rdx, r9
	mov	qword ptr [r8 + 64], rsi
	cmova	rax, r9
	cmp	rcx, r15
	mov	r9, r15
	cmovb	r9, rcx
	cmovbe	rcx, r15
	mov	r15, qword ptr [rsp + 72]
	mov	qword ptr [r8 + 56], r9
	cmp	r14, r11
	mov	r9, r11
	cmovb	r9, r14
	cmova	r11, r14
	mov	qword ptr [r8 + 8], r11
	cmp	rdi, rax
	mov	r10, rax
	cmovb	r10, rdi
	mov	qword ptr [r8 + 16], r9
	cmova	rax, rdi
	cmp	rdx, rcx
	mov	r9, rcx
	cmovb	r9, rdx
	mov	qword ptr [r8 + 24], rax
	cmova	rcx, rdx
	mov	rdx, qword ptr [rsp + 56]
	mov	qword ptr [r8 + 32], r10
	mov	qword ptr [r8 + 40], rcx
	mov	qword ptr [r8 + 48], r9
	mov	ecx, 9
.LBB25_10:
	cmp	rcx, rdx
	ja	.LBB25_57
	jne	.LBB25_12
.LBB25_21:
	cmp	r13, 18
	jb	.LBB25_30
	mov	rdx, qword ptr [rsp + 112]
	cmp	r8, rbp
	mov	r8, r15
	je	.LBB25_7
	jmp	.LBB25_23
	.p2align	4
.LBB25_12:
	lea	rdx, [r8 + 8*rdx]
	shl	ecx, 3
	lea	rax, [r8 + rcx]
	jmp	.LBB25_13
	.p2align	4
.LBB25_16:
	mov	r10, r8
.LBB25_19:
	mov	qword ptr [r10], r9
.LBB25_20:
	add	rax, 8
	add	rcx, 8
	cmp	rax, rdx
	je	.LBB25_21
.LBB25_13:
	mov	r11, qword ptr [rax - 8]
	mov	r9, qword ptr [rax]
	cmp	r11, r9
	jae	.LBB25_20
	mov	r10, rcx
	.p2align	4
.LBB25_15:
	mov	qword ptr [r8 + r10], r11
	cmp	r10, 8
	je	.LBB25_16
	mov	r11, qword ptr [r8 + r10 - 16]
	add	r10, -8
	cmp	r11, r9
	jb	.LBB25_15
	add	r10, r8
	jmp	.LBB25_19
.LBB25_1:
	mov	ebx, r9d
	mov	r14, r8
	mov	r15, qword ptr [rsp + 480]
	jmp	.LBB25_2
	.p2align	4
.LBB25_53:
	cmp	r13, 33
	jb	.LBB25_5
.LBB25_2:
	sub	ebx, 1
	jb	.LBB25_59
	mov	r9, r13
	shr	r9, 3
	mov	rax, r9
	shl	rax, 5
	add	rax, rbp
	imul	r8, r9, 56
	add	r8, rbp
	cmp	r13, 64
	jae	.LBB25_4
	mov	rcx, qword ptr [rbp]
	mov	rdx, qword ptr [rax]
	cmp	rdx, rcx
	setb	r9b
	mov	r10, qword ptr [r8]
	cmp	r10, rcx
	setb	cl
	xor	cl, r9b
	cmp	r10, rdx
	setb	dl
	xor	dl, r9b
	cmovne	rax, r8
	test	cl, cl
	cmovne	rax, rbp
	sub	rax, rbp
	test	r14, r14
	je	.LBB25_33
.LBB25_44:
	mov	rdx, qword ptr [rbp + rax]
	mov	rcx, qword ptr [rbp]
	cmp	rdx, qword ptr [r14]
	jb	.LBB25_34
	mov	qword ptr [rbp], rdx
	mov	qword ptr [rbp + rax], rcx
	lea	rcx, [rbp + 8]
	mov	r8, qword ptr [rbp]
	mov	rdx, qword ptr [rbp + 8]
	lea	r10, [8*r13 - 8]
	add	r10, rbp
	lea	r9, [rbp + 16]
	cmp	r9, r10
	jae	.LBB25_46
	xor	eax, eax
	.p2align	4
.LBB25_55:
	mov	r11, qword ptr [r9]
	cmp	r11, r8
	mov	rsi, qword ptr [rcx + 8*rax]
	mov	qword ptr [r9 - 8], rsi
	mov	qword ptr [rcx + 8*rax], r11
	sbb	rax, -1
	mov	r11, qword ptr [r9 + 8]
	cmp	r11, r8
	mov	rsi, qword ptr [rcx + 8*rax]
	mov	qword ptr [r9], rsi
	mov	qword ptr [rcx + 8*rax], r11
	sbb	rax, -1
	add	r9, 16
	cmp	r9, r10
	jb	.LBB25_55
	lea	r10, [r9 - 8]
	lea	r11, [8*r13]
	add	r11, rbp
	cmp	r9, r11
	jne	.LBB25_49
	jmp	.LBB25_51
	.p2align	4
.LBB25_4:
	mov	rcx, rbp
	mov	rdx, rax
	call	_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE
	sub	rax, rbp
	test	r14, r14
	jne	.LBB25_44
.LBB25_33:
	mov	rcx, qword ptr [rbp]
	mov	rdx, qword ptr [rbp + rax]
.LBB25_34:
	mov	qword ptr [rbp], rdx
	mov	qword ptr [rbp + rax], rcx
	lea	rax, [rbp + 8]
	mov	r8, qword ptr [rbp]
	mov	rcx, qword ptr [rbp + 8]
	lea	r10, [8*r13 - 8]
	add	r10, rbp
	lea	r9, [rbp + 16]
	cmp	r9, r10
	jae	.LBB25_35
	xor	edx, edx
	.p2align	4
.LBB25_43:
	mov	r11, qword ptr [r9]
	xor	esi, esi
	cmp	r8, r11
	setb	sil
	mov	rdi, qword ptr [rax + 8*rdx]
	mov	qword ptr [r9 - 8], rdi
	mov	qword ptr [rax + 8*rdx], r11
	lea	r11, [rdx + rsi]
	mov	rdi, qword ptr [r9 + 8]
	cmp	r8, rdi
	mov	r12, qword ptr [rax + 8*r11]
	mov	qword ptr [r9], r12
	mov	qword ptr [rax + 8*r11], rdi
	adc	rdx, rsi
	add	r9, 16
	cmp	r9, r10
	jb	.LBB25_43
	lea	r10, [r9 - 8]
	lea	r11, [8*r13]
	add	r11, rbp
	cmp	r9, r11
	jne	.LBB25_38
	jmp	.LBB25_40
	.p2align	4
.LBB25_35:
	mov	r10, rax
	xor	edx, edx
	lea	r11, [8*r13]
	add	r11, rbp
	cmp	r9, r11
	je	.LBB25_40
	.p2align	4
.LBB25_38:
	mov	rsi, qword ptr [r9]
	cmp	r8, rsi
	mov	rdi, qword ptr [rax + 8*rdx]
	mov	qword ptr [r10], rdi
	mov	qword ptr [rax + 8*rdx], rsi
	adc	rdx, 0
	mov	r10, r9
	add	r9, 8
	cmp	r9, r11
	jne	.LBB25_38
	add	r9, -8
	mov	r10, r9
.LBB25_40:
	cmp	r8, rcx
	mov	r8, qword ptr [rax + 8*rdx]
	mov	qword ptr [r10], r8
	mov	qword ptr [rax + 8*rdx], rcx
	adc	rdx, 0
	cmp	rdx, r13
	jae	.LBB25_57
	lea	rsi, [8*rdx]
	add	rsi, rbp
	mov	rax, qword ptr [rbp]
	mov	rcx, qword ptr [rbp + 8*rdx]
	mov	qword ptr [rbp], rcx
	mov	qword ptr [rbp + 8*rdx], rax
	lea	rdi, [8*rdx + 8]
	add	rdi, rbp
	mov	rax, rdx
	not	rax
	add	r13, rax
	mov	qword ptr [rsp + 32], r15
	mov	rcx, rbp
	mov	r8, r14
	mov	r9d, ebx
	call	_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E
	mov	r14, rsi
	mov	rbp, rdi
	jmp	.LBB25_53
.LBB25_46:
	mov	r10, rcx
	xor	eax, eax
	lea	r11, [8*r13]
	add	r11, rbp
	cmp	r9, r11
	je	.LBB25_51
	.p2align	4
.LBB25_49:
	mov	rsi, qword ptr [r9]
	cmp	rsi, r8
	mov	rdi, qword ptr [rcx + 8*rax]
	mov	qword ptr [r10], rdi
	mov	qword ptr [rcx + 8*rax], rsi
	sbb	rax, -1
	mov	r10, r9
	add	r9, 8
	cmp	r9, r11
	jne	.LBB25_49
	add	r9, -8
	mov	r10, r9
.LBB25_51:
	cmp	rdx, r8
	mov	r8, qword ptr [rcx + 8*rax]
	mov	qword ptr [r10], r8
	mov	qword ptr [rcx + 8*rax], rdx
	sbb	rax, -1
	cmp	rax, r13
	jae	.LBB25_57
	mov	rcx, qword ptr [rbp]
	mov	rdx, qword ptr [rbp + 8*rax]
	mov	qword ptr [rbp], rdx
	mov	qword ptr [rbp + 8*rax], rcx
	lea	rcx, [rax + 1]
	sub	r13, rcx
	lea	rbp, [rbp + 8*rax + 8]
	xor	r14d, r14d
	jmp	.LBB25_53
.LBB25_59:
	mov	rcx, rbp
	mov	rdx, r13
	.seh_startepilogue
	add	rsp, 376
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	jmp	_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE
.LBB25_23:
	lea	r10, [r13 - 1]
	lea	rdx, [8*r13 - 8]
	add	rdx, rbp
	lea	r8, [r15 - 8]
	xor	ecx, ecx
	mov	r9, rbp
	.p2align	4
.LBB25_24:
	mov	r11, qword ptr [r15]
	mov	rsi, qword ptr [r9]
	xor	eax, eax
	xor	edi, edi
	cmp	rsi, r11
	cmova	r11, rsi
	setae	al
	setb	dil
	mov	rsi, qword ptr [rdx]
	mov	rbx, qword ptr [r8]
	cmp	rbx, rsi
	mov	r14d, 0
	adc	r14, -1
	cmp	rbx, rsi
	cmovb	rsi, rbx
	mov	qword ptr [rsp + 8*rcx + 120], r11
	mov	r11d, 0
	sbb	r11, r11
	inc	rcx
	lea	r15, [r15 + 8*rdi]
	lea	r9, [r9 + 8*rax]
	mov	qword ptr [rsp + 8*r10 + 120], rsi
	lea	rdx, [rdx + 8*r14]
	lea	r8, [r8 + 8*r11]
	dec	r10
	cmp	r12, rcx
	jne	.LBB25_24
	add	r8, 8
	test	r13b, 1
	je	.LBB25_27
	xor	eax, eax
	xor	r10d, r10d
	cmp	r9, r8
	setae	al
	setb	r10b
	mov	r11, r15
	cmovb	r11, r9
	mov	r11, qword ptr [r11]
	mov	qword ptr [rsp + 8*rcx + 120], r11
	lea	r9, [r9 + 8*r10]
	lea	r15, [r15 + 8*rax]
.LBB25_27:
	cmp	r9, r8
	jne	.LBB25_58
	add	rdx, 8
	cmp	r15, rdx
	jne	.LBB25_58
	shl	r13, 3
	lea	rdx, [rsp + 120]
	mov	rcx, rbp
	mov	r8, r13
	call	memcpy
.LBB25_30:
	nop
	.seh_startepilogue
	add	rsp, 376
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
.LBB25_58:
	call	_ZN4core5slice4sort6shared9smallsort22panic_on_ord_violation17ha8ac69acadf1c3c7E
.LBB25_57:
	ud2
	.seh_endproc

	.def	_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E,unique,26
	.p2align	4
_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E:
.seh_proc _ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	r11, rdx
	mov	rsi, rcx
	mov	rbx, qword ptr [rsp + 112]
	mov	r10, qword ptr [rsp + 120]
	lea	rcx, [rbx + r10]
	dec	rcx
	mov	rax, rbx
	neg	rax
	and	rax, rcx
	mul	r9
	mov	rdi, rax
	seto	al
	movabs	rcx, -9223372036854775808
	sub	rcx, rbx
	cmp	rdi, rcx
	seta	cl
	or	cl, al
	mov	r14d, 1
	je	.LBB26_2
	mov	eax, 8
	xor	edi, edi
	jmp	.LBB26_10
.LBB26_2:
	test	r11, r11
	je	.LBB26_4
	imul	r10, r11
	mov	rcx, r8
	mov	rdx, r10
	mov	r8, rbx
	mov	r9, rdi
	call	_RNvCshXwFllX56pT_7___rustc14___rust_realloc
	test	rax, rax
	jne	.LBB26_6
	jmp	.LBB26_9
.LBB26_4:
	test	rdi, rdi
	je	.LBB26_5
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	rcx, rdi
	mov	rdx, rbx
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	jne	.LBB26_6
.LBB26_9:
	mov	qword ptr [rsi + 8], rbx
	mov	eax, 16
	jmp	.LBB26_10
.LBB26_5:
	mov	rax, rbx
.LBB26_6:
	mov	qword ptr [rsi + 8], rax
	mov	eax, 16
	xor	r14d, r14d
.LBB26_10:
	mov	qword ptr [rsi + rax], rdi
	mov	qword ptr [rsi], r14
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E,unique,27
	.p2align	4
_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E:
.seh_proc _ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	sub	rsp, 72
	.seh_stackalloc 72
	.seh_endprologue
	add	rdx, r8
	jb	.LBB27_1
	mov	rsi, rcx
	mov	rcx, qword ptr [rsp + 128]
	mov	rax, qword ptr [rsi]
	lea	r8, [rax + rax]
	cmp	rdx, r8
	cmova	r8, rdx
	cmp	r8, 5
	mov	edi, 4
	cmovae	rdi, r8
	mov	r8, qword ptr [rsi + 8]
	mov	qword ptr [rsp + 40], rcx
	mov	qword ptr [rsp + 32], r9
	lea	rcx, [rsp + 48]
	mov	rdx, rax
	mov	r9, rdi
	call	_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E
	cmp	dword ptr [rsp + 48], 1
	je	.LBB27_3
	mov	rax, qword ptr [rsp + 56]
	mov	qword ptr [rsi + 8], rax
	mov	qword ptr [rsi], rdi
	.seh_startepilogue
	add	rsp, 72
	pop	rdi
	pop	rsi
	.seh_endepilogue
	ret
.LBB27_1:
	xor	ecx, ecx
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.LBB27_3:
	mov	rcx, qword ptr [rsp + 56]
	mov	rdx, qword ptr [rsp + 64]
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
	int3
	.seh_endproc

	.def	_ZN6anyhow5error10object_ref17h0ce20ab8fd8e770aE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error10object_ref17h0ce20ab8fd8e770aE,unique,28
	.p2align	4
_ZN6anyhow5error10object_ref17h0ce20ab8fd8e770aE:
	lea	rax, [rcx + 56]
	lea	rdx, [rip + vtable.1]
	ret

	.def	_ZN6anyhow5error10object_ref17h3950046690ebc90bE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error10object_ref17h3950046690ebc90bE,unique,29
	.p2align	4
_ZN6anyhow5error10object_ref17h3950046690ebc90bE:
	lea	rax, [rcx + 56]
	lea	rdx, [rip + vtable.2]
	ret

	.def	_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE,unique,30
	.p2align	4
_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE:
.Lfunc_begin2:
.seh_proc _ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 56
	.seh_stackalloc 56
	lea	rbp, [rsp + 48]
	.seh_setframe rbp, 48
	.seh_endprologue
	mov	qword ptr [rbp], -2
	mov	qword ptr [rbp - 8], rcx
	add	rcx, 8
.Ltmp6:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp7:
	mov	rsi, qword ptr [rbp - 8]
	mov	rdx, qword ptr [rsi + 56]
	test	rdx, rdx
	je	.LBB30_3
	mov	rcx, qword ptr [rsi + 64]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB30_3:
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, rsi
	.seh_startepilogue
	add	rsp, 56
	pop	rsi
	pop	rbp
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@IMGREL
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE,unique,30
	.seh_endproc
	.def	"?dtor$4@?0?_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$4@?0?_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@4HA":
.seh_proc "?dtor$4@?0?_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@4HA"
.LBB30_4:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	rax, qword ptr [rbp - 8]
	mov	rdx, qword ptr [rax + 56]
	test	rdx, rdx
	je	.LBB30_6
	mov	rax, qword ptr [rbp - 8]
	mov	rcx, qword ptr [rax + 64]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB30_6:
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 8]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end2:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE,unique,30
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE,unique,2
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@IMGREL
	.long	48
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE:
	.long	-1
	.long	"?dtor$4@?0?_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE@4HA"@IMGREL
$ip2state$_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE:
	.long	.Lfunc_begin2@IMGREL
	.long	-1
	.long	.Ltmp6@IMGREL+1
	.long	0
	.long	.Ltmp7@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE,unique,30

	.def	_ZN6anyhow5error11object_drop17h9334d5975c0625abE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h9334d5975c0625abE,unique,31
	.p2align	4
_ZN6anyhow5error11object_drop17h9334d5975c0625abE:
.Lfunc_begin3:
.seh_proc _ZN6anyhow5error11object_drop17h9334d5975c0625abE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 48
	.seh_stackalloc 48
	lea	rbp, [rsp + 48]
	.seh_setframe rbp, 48
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	qword ptr [rbp - 16], rcx
	add	rcx, 8
.Ltmp8:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp9:
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	.seh_startepilogue
	add	rsp, 48
	pop	rbp
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error11object_drop17h9334d5975c0625abE@IMGREL
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h9334d5975c0625abE,unique,31
	.seh_endproc
	.def	"?dtor$2@?0?_ZN6anyhow5error11object_drop17h9334d5975c0625abE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$2@?0?_ZN6anyhow5error11object_drop17h9334d5975c0625abE@4HA":
.seh_proc "?dtor$2@?0?_ZN6anyhow5error11object_drop17h9334d5975c0625abE@4HA"
.LBB31_2:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end3:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h9334d5975c0625abE,unique,31
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error11object_drop17h9334d5975c0625abE,unique,3
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error11object_drop17h9334d5975c0625abE:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error11object_drop17h9334d5975c0625abE@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error11object_drop17h9334d5975c0625abE@IMGREL
	.long	40
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error11object_drop17h9334d5975c0625abE:
	.long	-1
	.long	"?dtor$2@?0?_ZN6anyhow5error11object_drop17h9334d5975c0625abE@4HA"@IMGREL
$ip2state$_ZN6anyhow5error11object_drop17h9334d5975c0625abE:
	.long	.Lfunc_begin3@IMGREL
	.long	-1
	.long	.Ltmp8@IMGREL+1
	.long	0
	.long	.Ltmp9@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN6anyhow5error11object_drop17h9334d5975c0625abE,unique,31

	.def	_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE,unique,32
	.p2align	4
_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE:
	xor	eax, eax
	ret

	.def	_ZN6anyhow5error12object_boxed17hdd48d83f6f5fca94E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error12object_boxed17hdd48d83f6f5fca94E,unique,33
	.p2align	4
_ZN6anyhow5error12object_boxed17hdd48d83f6f5fca94E:
	mov	rax, rcx
	lea	rdx, [rip + vtable.3]
	ret

	.def	_ZN6anyhow5error12object_boxed17hde21296211696ba2E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error12object_boxed17hde21296211696ba2E,unique,34
	.p2align	4
_ZN6anyhow5error12object_boxed17hde21296211696ba2E:
	mov	rax, rcx
	lea	rdx, [rip + vtable.4]
	ret

	.def	_ZN6anyhow5error15object_downcast17h9f35c6fdf8d2f673E;
	.scl	3;
	.type	32;
	.endef
	.globl	__xmm@b98b1b7157a6417863eb502cd6cb5d6d
	.section	.rdata,"dr",discard,__xmm@b98b1b7157a6417863eb502cd6cb5d6d
	.p2align	4, 0x0
__xmm@b98b1b7157a6417863eb502cd6cb5d6d:
	.byte	109
	.byte	93
	.byte	203
	.byte	214
	.byte	44
	.byte	80
	.byte	235
	.byte	99
	.byte	120
	.byte	65
	.byte	166
	.byte	87
	.byte	113
	.byte	27
	.byte	139
	.byte	185
	.section	.text,"xr",one_only,_ZN6anyhow5error15object_downcast17h9f35c6fdf8d2f673E,unique,35
	.p2align	4
_ZN6anyhow5error15object_downcast17h9f35c6fdf8d2f673E:
	movdqu	xmm0, xmmword ptr [rdx]
	pcmpeqb	xmm0, xmmword ptr [rip + __xmm@b98b1b7157a6417863eb502cd6cb5d6d]
	pmovmskb	edx, xmm0
	add	rcx, 56
	xor	eax, eax
	cmp	edx, 65535
	cmove	rax, rcx
	ret

	.def	_ZN6anyhow5error15object_downcast17hefd3fb1cb0d21f49E;
	.scl	3;
	.type	32;
	.endef
	.globl	__xmm@8a943b009ca9aba2c8f6b21b6615e9e8
	.section	.rdata,"dr",discard,__xmm@8a943b009ca9aba2c8f6b21b6615e9e8
	.p2align	4, 0x0
__xmm@8a943b009ca9aba2c8f6b21b6615e9e8:
	.byte	232
	.byte	233
	.byte	21
	.byte	102
	.byte	27
	.byte	178
	.byte	246
	.byte	200
	.byte	162
	.byte	171
	.byte	169
	.byte	156
	.byte	0
	.byte	59
	.byte	148
	.byte	138
	.section	.text,"xr",one_only,_ZN6anyhow5error15object_downcast17hefd3fb1cb0d21f49E,unique,36
	.p2align	4
_ZN6anyhow5error15object_downcast17hefd3fb1cb0d21f49E:
	movdqu	xmm0, xmmword ptr [rdx]
	pcmpeqb	xmm0, xmmword ptr [rip + __xmm@8a943b009ca9aba2c8f6b21b6615e9e8]
	pmovmskb	edx, xmm0
	add	rcx, 56
	xor	eax, eax
	cmp	edx, 65535
	cmove	rax, rcx
	ret

	.def	_ZN6anyhow5error17object_drop_front17h468434592611aba3E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17h468434592611aba3E,unique,37
	.p2align	4
_ZN6anyhow5error17object_drop_front17h468434592611aba3E:
.Lfunc_begin4:
.seh_proc _ZN6anyhow5error17object_drop_front17h468434592611aba3E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 48
	.seh_stackalloc 48
	lea	rbp, [rsp + 48]
	.seh_setframe rbp, 48
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	qword ptr [rbp - 16], rcx
	add	rcx, 8
.Ltmp10:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp11:
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	.seh_startepilogue
	add	rsp, 48
	pop	rbp
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error17object_drop_front17h468434592611aba3E@IMGREL
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17h468434592611aba3E,unique,37
	.seh_endproc
	.def	"?dtor$2@?0?_ZN6anyhow5error17object_drop_front17h468434592611aba3E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$2@?0?_ZN6anyhow5error17object_drop_front17h468434592611aba3E@4HA":
.seh_proc "?dtor$2@?0?_ZN6anyhow5error17object_drop_front17h468434592611aba3E@4HA"
.LBB37_2:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end4:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17h468434592611aba3E,unique,37
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error17object_drop_front17h468434592611aba3E,unique,4
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error17object_drop_front17h468434592611aba3E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error17object_drop_front17h468434592611aba3E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error17object_drop_front17h468434592611aba3E@IMGREL
	.long	40
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error17object_drop_front17h468434592611aba3E:
	.long	-1
	.long	"?dtor$2@?0?_ZN6anyhow5error17object_drop_front17h468434592611aba3E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error17object_drop_front17h468434592611aba3E:
	.long	.Lfunc_begin4@IMGREL
	.long	-1
	.long	.Ltmp10@IMGREL+1
	.long	0
	.long	.Ltmp11@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17h468434592611aba3E,unique,37

	.def	_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E,unique,38
	.p2align	4
_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E:
.Lfunc_begin5:
.seh_proc _ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 48
	.seh_stackalloc 48
	lea	rbp, [rsp + 48]
	.seh_setframe rbp, 48
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	qword ptr [rbp - 16], rcx
	add	rcx, 8
.Ltmp12:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp13:
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	.seh_startepilogue
	add	rsp, 48
	pop	rbp
	.seh_endepilogue
	jmp	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@IMGREL
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E,unique,38
	.seh_endproc
	.def	"?dtor$2@?0?_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$2@?0?_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@4HA":
.seh_proc "?dtor$2@?0?_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@4HA"
.LBB38_2:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end5:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E,unique,38
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E,unique,5
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@IMGREL
	.long	40
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E:
	.long	-1
	.long	"?dtor$2@?0?_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E:
	.long	.Lfunc_begin5@IMGREL
	.long	-1
	.long	.Ltmp12@IMGREL+1
	.long	0
	.long	.Ltmp13@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E,unique,38

	.def	_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E,unique,39
	.p2align	4
_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E:
.Lfunc_begin6:
.seh_proc _ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 48
	.seh_stackalloc 48
	lea	rbp, [rsp + 48]
	.seh_setframe rbp, 48
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	r14, rcx
	mov	rdi, qword ptr [rcx + 56]
	mov	rbx, qword ptr [rcx + 64]
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	ecx, 16
	mov	edx, 8
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	mov	qword ptr [rbp - 16], r14
	je	.LBB39_1
	mov	rsi, rax
	mov	qword ptr [rax], rdi
	mov	qword ptr [rax + 8], rbx
	lea	rcx, [r14 + 8]
.Ltmp14:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp15:
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	lea	rdx, [rip + vtable.2]
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 48
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
.LBB39_1:
.Ltmp16:
	mov	ecx, 8
	mov	edx, 16
	call	_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E
.Ltmp17:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@IMGREL
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E,unique,39
	.seh_endproc
	.def	"?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA":
.seh_proc "?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA"
.LBB39_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	rsi, qword ptr [rbp - 16]
	lea	rcx, [rsi + 8]
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, rsi
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E,unique,39
	.seh_endproc
	.def	"?dtor$4@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$4@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA":
.seh_proc "?dtor$4@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA"
.LBB39_4:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 48]
	.seh_endprologue
	mov	edx, 72
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end6:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E,unique,39
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E,unique,6
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E:
	.long	429065506
	.long	2
	.long	$stateUnwindMap$_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@IMGREL
	.long	0
	.long	0
	.long	4
	.long	$ip2state$_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@IMGREL
	.long	40
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E:
	.long	-1
	.long	"?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA"@IMGREL
	.long	-1
	.long	"?dtor$4@?0?_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E:
	.long	.Lfunc_begin6@IMGREL
	.long	-1
	.long	.Ltmp14@IMGREL+1
	.long	1
	.long	.Ltmp16@IMGREL+1
	.long	0
	.long	.Ltmp17@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E,unique,39

	.def	_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E,unique,40
	.p2align	4
_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E:
.Lfunc_begin7:
.seh_proc _ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rsp + 64]
	.seh_setframe rbp, 64
	.seh_endprologue
	mov	qword ptr [rbp], -2
	mov	rbx, rcx
	mov	r14, qword ptr [rcx + 56]
	mov	r15, qword ptr [rcx + 64]
	mov	rdi, qword ptr [rcx + 72]
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	ecx, 24
	mov	edx, 8
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	mov	qword ptr [rbp - 8], rbx
	je	.LBB40_1
	mov	rsi, rax
	mov	qword ptr [rax], r14
	mov	qword ptr [rax + 8], r15
	mov	qword ptr [rax + 16], rdi
	lea	rcx, [rbx + 8]
.Ltmp18:
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
.Ltmp19:
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 8]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	lea	rdx, [rip + vtable.1]
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.LBB40_1:
.Ltmp20:
	mov	qword ptr [rbp - 16], r15
	mov	qword ptr [rbp - 24], r14
	mov	ecx, 8
	mov	edx, 24
	call	_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E
.Ltmp21:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@IMGREL
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E,unique,40
	.seh_endproc
	.def	"?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA":
.seh_proc "?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA"
.LBB40_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 64]
	.seh_endprologue
	mov	rdx, qword ptr [rbp - 24]
	test	rdx, rdx
	je	.LBB40_5
	mov	r8d, 1
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB40_5:
	mov	rsi, qword ptr [rbp - 8]
	lea	rcx, [rsi + 8]
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, rsi
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E,unique,40
	.seh_endproc
	.def	"?dtor$6@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$6@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA":
.seh_proc "?dtor$6@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA"
.LBB40_6:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 64]
	.seh_endprologue
	mov	edx, 80
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 8]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end7:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E,unique,40
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E,unique,7
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E:
	.long	429065506
	.long	2
	.long	$stateUnwindMap$_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@IMGREL
	.long	0
	.long	0
	.long	4
	.long	$ip2state$_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@IMGREL
	.long	64
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E:
	.long	-1
	.long	"?dtor$3@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA"@IMGREL
	.long	-1
	.long	"?dtor$6@?0?_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E:
	.long	.Lfunc_begin7@IMGREL
	.long	-1
	.long	.Ltmp18@IMGREL+1
	.long	1
	.long	.Ltmp20@IMGREL+1
	.long	0
	.long	.Ltmp21@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E,unique,40

	.def	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E,unique,41
	.p2align	4
_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E:
.seh_proc _ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	sub	rsp, 88
	.seh_stackalloc 88
	.seh_endprologue
	mov	rsi, rcx
	lea	rdi, [rsp + 40]
	mov	rcx, rdi
	call	_ZN3std9backtrace9Backtrace7capture17hf7fc842bd7b2c58bE
	mov	rcx, rsi
	mov	rdx, rdi
	call	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E
	nop
	.seh_startepilogue
	add	rsp, 88
	pop	rdi
	pop	rsi
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E,unique,42
	.p2align	4
_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E:
.Lfunc_begin8:
.seh_proc _ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 160
	.seh_stackalloc 160
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	.seh_endprologue
	mov	qword ptr [rbp + 24], -2
	mov	qword ptr [rbp + 8], rcx
	mov	byte ptr [rbp + 23], 1
.Ltmp22:
	lea	rcx, [rbp - 96]
	call	_ZN3std9backtrace9Backtrace7capture17hf7fc842bd7b2c58bE
.Ltmp23:
	movups	xmm0, xmmword ptr [rbp - 96]
	movups	xmm1, xmmword ptr [rbp - 80]
	movups	xmm2, xmmword ptr [rbp - 64]
	movaps	xmmword ptr [rbp - 16], xmm2
	movaps	xmmword ptr [rbp - 32], xmm1
	movaps	xmmword ptr [rbp - 48], xmm0
	mov	byte ptr [rbp + 23], 0
.Ltmp24:
	lea	rdx, [rbp - 48]
	mov	rcx, qword ptr [rbp + 8]
	call	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E
.Ltmp25:
	nop
	.seh_startepilogue
	add	rsp, 160
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@IMGREL
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E,unique,42
	.seh_endproc
	.def	"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@4HA":
.seh_proc "?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@4HA"
.LBB42_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 128]
	.seh_endprologue
	mov	rax, qword ptr [rbp + 8]
	cmp	byte ptr [rbp + 23], 0
	je	.LBB42_6
	mov	rdx, qword ptr [rax]
	test	rdx, rdx
	je	.LBB42_6
	mov	rcx, qword ptr [rax + 8]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB42_6:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end8:
	.seh_handlerdata
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E,unique,42
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E,unique,8
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@IMGREL
	.long	152
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E:
	.long	-1
	.long	"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E:
	.long	.Lfunc_begin8@IMGREL
	.long	-1
	.long	.Ltmp22@IMGREL+1
	.long	0
	.long	.Ltmp25@IMGREL+1
	.long	-1
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E,unique,42

	.def	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E,unique,43
	.p2align	4
_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E:
.Lfunc_begin9:
.seh_proc _ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 112
	.seh_stackalloc 112
	lea	rbp, [rsp + 112]
	.seh_setframe rbp, 112
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	lea	rax, [rip + alloc_a04e47d083146d15ce3892a825ec94b0]
	mov	qword ptr [rbp - 80], rax
	movups	xmm0, xmmword ptr [rdx]
	movups	xmm1, xmmword ptr [rdx + 16]
	movups	xmm2, xmmword ptr [rdx + 32]
	movups	xmmword ptr [rbp - 72], xmm0
	movups	xmmword ptr [rbp - 56], xmm1
	movups	xmmword ptr [rbp - 40], xmm2
	lea	rax, [rip + alloc_cb2aea7e2fdb2fba562edabf1f950868]
	mov	qword ptr [rbp - 24], rax
	mov	qword ptr [rbp - 16], rcx
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	ecx, 72
	mov	edx, 8
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB43_1
	mov	rcx, qword ptr [rbp - 16]
	mov	qword ptr [rax + 64], rcx
	movups	xmm0, xmmword ptr [rbp - 80]
	movups	xmm1, xmmword ptr [rbp - 64]
	movups	xmm2, xmmword ptr [rbp - 48]
	movups	xmm3, xmmword ptr [rbp - 32]
	movups	xmmword ptr [rax + 48], xmm3
	movups	xmmword ptr [rax + 32], xmm2
	movups	xmmword ptr [rax + 16], xmm1
	movups	xmmword ptr [rax], xmm0
	.seh_startepilogue
	add	rsp, 112
	pop	rbp
	.seh_endepilogue
	ret
.LBB43_1:
.Ltmp26:
	mov	ecx, 8
	mov	edx, 72
	call	_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E
.Ltmp27:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@IMGREL
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E,unique,43
	.seh_endproc
	.def	"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@4HA":
.seh_proc "?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@4HA"
.LBB43_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 112]
	.seh_endprologue
	lea	rcx, [rbp - 72]
	call	_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end9:
	.seh_handlerdata
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E,unique,43
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E,unique,9
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@IMGREL
	.long	104
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E:
	.long	-1
	.long	"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E:
	.long	.Lfunc_begin9@IMGREL
	.long	-1
	.long	.Ltmp26@IMGREL+1
	.long	0
	.long	.Ltmp27@IMGREL+1
	.long	-1
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E,unique,43

	.def	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E,unique,44
	.p2align	4
_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E:
.Lfunc_begin10:
.seh_proc _ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 128
	.seh_stackalloc 128
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	lea	rax, [rip + alloc_00e51742134d344daa7116ffd2ad9e35]
	mov	qword ptr [rbp - 88], rax
	movups	xmm0, xmmword ptr [rdx]
	movups	xmm1, xmmword ptr [rdx + 16]
	movups	xmm2, xmmword ptr [rdx + 32]
	movups	xmmword ptr [rbp - 80], xmm0
	movups	xmmword ptr [rbp - 64], xmm1
	movups	xmmword ptr [rbp - 48], xmm2
	movups	xmm0, xmmword ptr [rcx]
	movups	xmmword ptr [rbp - 32], xmm0
	mov	rax, qword ptr [rcx + 16]
	mov	qword ptr [rbp - 16], rax
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	ecx, 80
	mov	edx, 8
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB44_1
	movups	xmm0, xmmword ptr [rbp - 24]
	movups	xmmword ptr [rax + 64], xmm0
	movups	xmm0, xmmword ptr [rbp - 88]
	movups	xmm1, xmmword ptr [rbp - 72]
	movups	xmm2, xmmword ptr [rbp - 56]
	movups	xmm3, xmmword ptr [rbp - 40]
	movups	xmmword ptr [rax + 48], xmm3
	movups	xmmword ptr [rax + 32], xmm2
	movups	xmmword ptr [rax + 16], xmm1
	movups	xmmword ptr [rax], xmm0
	.seh_startepilogue
	add	rsp, 128
	pop	rbp
	.seh_endepilogue
	ret
.LBB44_1:
.Ltmp28:
	mov	ecx, 8
	mov	edx, 80
	call	_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E
.Ltmp29:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@IMGREL
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E,unique,44
	.seh_endproc
	.def	"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@4HA":
.seh_proc "?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@4HA"
.LBB44_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 128]
	.seh_endprologue
	lea	rcx, [rbp - 88]
	call	_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end10:
	.seh_handlerdata
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E,unique,44
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E,unique,10
	.p2align	2, 0x0
$cppxdata$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@IMGREL
	.long	120
	.long	0
	.long	1
$stateUnwindMap$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E:
	.long	-1
	.long	"?dtor$3@?0?_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E@4HA"@IMGREL
$ip2state$_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E:
	.long	.Lfunc_begin10@IMGREL
	.long	-1
	.long	.Ltmp28@IMGREL+1
	.long	0
	.long	.Ltmp29@IMGREL+1
	.long	-1
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E,unique,44

	.def	_ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE;
	.scl	3;
	.type	32;
	.endef
	.section	.text$unlikely,"xr",one_only,_ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE,unique,45
	.p2align	4
_ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE:
.seh_proc _ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 64
	.seh_stackalloc 64
	.seh_endprologue
	mov	r8, rcx
	test	r8b, 1
	jne	.LBB45_2
	lea	rdx, [rip + alloc_cb2aea7e2fdb2fba562edabf1f950868]
	lea	rsi, [rsp + 40]
	mov	rcx, rsi
	call	_ZN5alloc3fmt6format12format_inner17hbb70ff8f9f00ea6cE
	mov	rcx, rsi
	call	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E
	nop
	.seh_startepilogue
	add	rsp, 64
	pop	rsi
	.seh_endepilogue
	ret
.LBB45_2:
	shr	r8
	mov	rcx, r8
	.seh_startepilogue
	add	rsp, 64
	pop	rsi
	.seh_endepilogue
	jmp	_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E
	.seh_endproc

	.def	_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E,unique,46
	.p2align	4
_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E:
	jmp	_ZN6anyhow3fmt42_$LT$impl$u20$anyhow..error..ErrorImpl$GT$5debug17h7876388f4297a68bE

	.def	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E,unique,47
	.p2align	4
_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E:
.seh_proc _ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	call	_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E
	mov	rcx, rax
	.seh_startepilogue
	add	rsp, 40
	.seh_endepilogue
	rex64 jmp	qword ptr [rdx + 48]
	.seh_endproc

	.def	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE,unique,48
	.p2align	4
_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE:
.seh_proc _ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 32
	.seh_stackalloc 32
	.seh_endprologue
	mov	rsi, rdx
	call	_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E
	mov	r8, qword ptr [rdx + 32]
	mov	rcx, rax
	mov	rdx, rsi
	.seh_startepilogue
	add	rsp, 32
	pop	rsi
	.seh_endepilogue
	rex64 jmp	r8
	.seh_endproc

	.def	_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h0607a4d5b05d13c1E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h0607a4d5b05d13c1E,unique,49
	.p2align	4
_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h0607a4d5b05d13c1E:
	mov	r8, rdx
	mov	rax, qword ptr [rcx + 8]
	mov	rdx, qword ptr [rcx + 16]
	mov	rcx, rax
	jmp	_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hdd75f4eba36c23acE

	.def	_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h663cff1b8df227b7E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h663cff1b8df227b7E,unique,50
	.p2align	4
_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h663cff1b8df227b7E:
	mov	r8, rdx
	mov	rax, qword ptr [rcx]
	mov	rdx, qword ptr [rcx + 8]
	mov	rcx, rax
	jmp	_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hdd75f4eba36c23acE

	.def	_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E,unique,51
	.p2align	4
_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E:
	mov	r8, rdx
	mov	rax, qword ptr [rcx + 8]
	mov	rdx, qword ptr [rcx + 16]
	mov	rcx, rax
	jmp	_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h2e02e0ff298d12e0E

	.def	_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE,unique,52
	.p2align	4
_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE:
	mov	r8, rdx
	mov	rax, qword ptr [rcx]
	mov	rdx, qword ptr [rcx + 8]
	mov	rcx, rax
	jmp	_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h2e02e0ff298d12e0E

	.def	_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E,unique,53
	.globl	_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E
	.p2align	4
_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E:
.Lfunc_begin11:
.seh_proc _ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 336
	.seh_stackalloc 336
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	.seh_endprologue
	mov	qword ptr [rbp + 200], -2
	mov	rdi, r8
	mov	rsi, rcx
	mov	qword ptr [rsp + 32], 2
	lea	r9, [rip + alloc_3f62f09340ec4217b72fe8840b861b6c]
	lea	rcx, [rbp + 48]
	call	_ZN4core3str7pattern11StrSearcher3new17h068a94d23c181adaE
	mov	qword ptr [rbp + 152], 0
	mov	qword ptr [rbp + 160], rdi
	mov	word ptr [rbp + 168], 1
	mov	rax, qword ptr [rbp + 160]
	mov	qword ptr [rbp + 32], rax
	movzx	eax, word ptr [rbp + 168]
	mov	word ptr [rbp + 40], ax
	mov	eax, dword ptr [rbp + 170]
	mov	dword ptr [rbp + 42], eax
	movzx	eax, word ptr [rbp + 174]
	mov	word ptr [rbp + 46], ax
	mov	rax, qword ptr [rbp + 144]
	mov	qword ptr [rbp + 16], rax
	mov	rax, qword ptr [rbp + 152]
	mov	qword ptr [rbp + 24], rax
	movups	xmm0, xmmword ptr [rbp + 128]
	movaps	xmmword ptr [rbp], xmm0
	movups	xmm0, xmmword ptr [rbp + 112]
	movaps	xmmword ptr [rbp - 16], xmm0
	movups	xmm0, xmmword ptr [rbp + 48]
	movups	xmm1, xmmword ptr [rbp + 64]
	movups	xmm2, xmmword ptr [rbp + 80]
	movups	xmm3, xmmword ptr [rbp + 96]
	movaps	xmmword ptr [rbp - 32], xmm3
	movaps	xmmword ptr [rbp - 48], xmm2
	movaps	xmmword ptr [rbp - 64], xmm1
	movaps	xmmword ptr [rbp - 80], xmm0
	lea	rcx, [rbp - 80]
	call	_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E
	test	al, 1
	je	.LBB53_1
	mov	rbx, rdx
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	ecx, 32
	mov	edx, 8
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB53_17
	mov	rdi, rax
	mov	qword ptr [rax], rbx
	mov	qword ptr [rbp + 176], 4
	mov	qword ptr [rbp + 184], rax
	mov	qword ptr [rbp + 192], 1
	movaps	xmm0, xmmword ptr [rbp + 32]
	movaps	xmmword ptr [rbp + 160], xmm0
	movaps	xmm0, xmmword ptr [rbp + 16]
	movaps	xmmword ptr [rbp + 144], xmm0
	movaps	xmm0, xmmword ptr [rbp]
	movaps	xmmword ptr [rbp + 128], xmm0
	movaps	xmm0, xmmword ptr [rbp - 16]
	movaps	xmmword ptr [rbp + 112], xmm0
	movaps	xmm0, xmmword ptr [rbp - 80]
	movaps	xmm1, xmmword ptr [rbp - 64]
	movaps	xmm2, xmmword ptr [rbp - 48]
	movaps	xmm3, xmmword ptr [rbp - 32]
	movaps	xmmword ptr [rbp + 96], xmm3
	movaps	xmmword ptr [rbp + 80], xmm2
	movaps	xmmword ptr [rbp + 64], xmm1
	movaps	xmmword ptr [rbp + 48], xmm0
.Ltmp30:
	lea	rcx, [rbp + 48]
	call	_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E
.Ltmp31:
	test	al, 1
	je	.LBB53_6
	mov	r14, rdx
	mov	ebx, 1
	lea	r15, [rbp + 176]
	lea	r12, [rbp + 48]
	.p2align	4
.LBB53_8:
	cmp	rbx, qword ptr [rbp + 176]
	jne	.LBB53_11
.Ltmp32:
	mov	qword ptr [rsp + 32], 8
	mov	r8d, 1
	mov	r9d, 8
	mov	rcx, r15
	mov	rdx, rbx
	call	_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E
.Ltmp33:
	mov	rdi, qword ptr [rbp + 184]
.LBB53_11:
	mov	qword ptr [rdi + 8*rbx], r14
	inc	rbx
	mov	qword ptr [rbp + 192], rbx
.Ltmp34:
	mov	rcx, r12
	call	_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E
.Ltmp35:
	mov	r14, rdx
	test	al, 1
	jne	.LBB53_8
	mov	rax, qword ptr [rbp + 176]
	mov	rdi, qword ptr [rbp + 184]
	jmp	.LBB53_2
.LBB53_1:
	mov	edi, 8
	xor	ebx, ebx
	xor	eax, eax
	jmp	.LBB53_2
.LBB53_6:
	mov	eax, 4
	mov	ebx, 1
.LBB53_2:
	mov	qword ptr [rsi], rax
	mov	qword ptr [rsi + 8], rdi
	mov	qword ptr [rsi + 16], rbx
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 336
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.LBB53_17:
	mov	ecx, 8
	mov	edx, 32
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
	int3
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E,unique,53
	.seh_endproc
	.def	"?dtor$14@?0?_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$14@?0?_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@4HA":
.seh_proc "?dtor$14@?0?_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@4HA"
.LBB53_14:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 48
	.seh_stackalloc 48
	lea	rbp, [rdx + 128]
	.seh_endprologue
	mov	rdx, qword ptr [rbp + 176]
	test	rdx, rdx
	je	.LBB53_16
	mov	rcx, qword ptr [rbp + 184]
	shl	rdx, 3
	mov	r8d, 8
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB53_16:
	nop
	.seh_startepilogue
	add	rsp, 48
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end11:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E,unique,53
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E,unique,11
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@IMGREL
	.long	328
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E:
	.long	-1
	.long	"?dtor$14@?0?_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E:
	.long	.Lfunc_begin11@IMGREL
	.long	-1
	.long	.Ltmp30@IMGREL+1
	.long	0
	.long	.Ltmp35@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E,unique,53

	.def	_ZN7aoc20226solver5day0111solve_part117hdf4b875787c5cc5dE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111solve_part117hdf4b875787c5cc5dE,unique,54
	.globl	_ZN7aoc20226solver5day0111solve_part117hdf4b875787c5cc5dE
	.p2align	4
_ZN7aoc20226solver5day0111solve_part117hdf4b875787c5cc5dE:
.seh_proc _ZN7aoc20226solver5day0111solve_part117hdf4b875787c5cc5dE
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 64
	.seh_stackalloc 64
	.seh_endprologue
	mov	r8, rdx
	mov	rdx, rcx
	lea	rcx, [rsp + 40]
	call	_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E
	mov	rcx, qword ptr [rsp + 48]
	mov	r9, qword ptr [rsp + 56]
	test	r9, r9
	je	.LBB54_1
	mov	r8, rcx
	cmp	r9, 1
	je	.LBB54_18
	movabs	r8, 2305843009213693951
	add	r9, r8
	and	r8, r9
	mov	rdx, qword ptr [rcx]
	dec	r8
	mov	eax, r9d
	and	eax, 3
	cmp	r8, 3
	jae	.LBB54_5
	xor	r10d, r10d
	mov	r8, rcx
.LBB54_15:
	test	rax, rax
	je	.LBB54_18
	lea	r9, [rcx + 8*r10]
	add	r9, 8
	.p2align	4
.LBB54_17:
	mov	r10, qword ptr [r9]
	cmp	rdx, r10
	cmovbe	rdx, r10
	cmovbe	r8, r9
	add	r9, 8
	dec	rax
	jne	.LBB54_17
	jmp	.LBB54_18
.LBB54_1:
	xor	r8d, r8d
.LBB54_18:
	test	r8, r8
	lea	rax, [rip + alloc_53973d2fe29b4adba8bb7390b5678745]
	cmovne	rax, r8
	mov	rsi, qword ptr [rax]
	mov	rdx, qword ptr [rsp + 40]
	test	rdx, rdx
	je	.LBB54_20
	shl	rdx, 3
	mov	r8d, 8
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB54_20:
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 64
	pop	rsi
	.seh_endepilogue
	ret
.LBB54_5:
	lea	r11, [rcx + 8]
	movabs	r8, 2305843009213693948
	and	r9, r8
	add	r11, 24
	xor	r10d, r10d
	mov	r8, rcx
	jmp	.LBB54_6
	.p2align	4
.LBB54_14:
	add	r10, 4
	add	r11, 32
	cmp	r9, r10
	je	.LBB54_15
.LBB54_6:
	mov	rsi, qword ptr [r11 - 24]
	cmp	rdx, rsi
	jbe	.LBB54_7
	mov	rsi, qword ptr [r11 - 16]
	cmp	rdx, rsi
	jbe	.LBB54_9
.LBB54_10:
	mov	rsi, qword ptr [r11 - 8]
	cmp	rdx, rsi
	jbe	.LBB54_11
.LBB54_12:
	mov	rsi, qword ptr [r11]
	cmp	rdx, rsi
	ja	.LBB54_14
	jmp	.LBB54_13
	.p2align	4
.LBB54_7:
	lea	r8, [r11 - 24]
	mov	rdx, rsi
	mov	rsi, qword ptr [r11 - 16]
	cmp	rdx, rsi
	ja	.LBB54_10
.LBB54_9:
	lea	r8, [r11 - 16]
	mov	rdx, rsi
	mov	rsi, qword ptr [r11 - 8]
	cmp	rdx, rsi
	ja	.LBB54_12
.LBB54_11:
	lea	r8, [r11 - 8]
	mov	rdx, rsi
	mov	rsi, qword ptr [r11]
	cmp	rdx, rsi
	ja	.LBB54_14
.LBB54_13:
	mov	rdx, rsi
	mov	r8, r11
	jmp	.LBB54_14
	.seh_endproc

	.def	_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E,unique,55
	.globl	_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E
	.p2align	4
_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E:
.Lfunc_begin12:
.seh_proc _ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rsp + 64]
	.seh_setframe rbp, 64
	.seh_endprologue
	mov	qword ptr [rbp], -2
	mov	r8, rdx
	mov	rdx, rcx
	lea	rcx, [rbp - 32]
	call	_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E
	mov	rcx, qword ptr [rbp - 24]
	mov	rdx, qword ptr [rbp - 16]
.Ltmp36:
	mov	qword ptr [rbp - 8], rcx
	call	_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE
.Ltmp37:
	mov	rdx, qword ptr [rbp - 32]
	test	rdx, rdx
	je	.LBB55_3
	shl	rdx, 3
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 8]
	mov	rsi, rax
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	mov	rax, rsi
.LBB55_3:
	.seh_startepilogue
	add	rsp, 72
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E,unique,55
	.seh_endproc
	.def	"?dtor$4@?0?_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$4@?0?_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@4HA":
.seh_proc "?dtor$4@?0?_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@4HA"
.LBB55_4:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 64]
	.seh_endprologue
	mov	rdx, qword ptr [rbp - 32]
	test	rdx, rdx
	je	.LBB55_6
	shl	rdx, 3
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 8]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB55_6:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end12:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E,unique,55
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E,unique,12
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@IMGREL
	.long	64
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E:
	.long	-1
	.long	"?dtor$4@?0?_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E:
	.long	.Lfunc_begin12@IMGREL
	.long	-1
	.long	.Ltmp36@IMGREL+1
	.long	0
	.long	.Ltmp37@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E,unique,55

	.def	_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE,unique,56
	.p2align	4
_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE:
.Lfunc_begin13:
.seh_proc _ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rsp + 64]
	.seh_setframe rbp, 64
	.seh_endprologue
	mov	qword ptr [rbp], -2
	mov	rdi, rcx
	lea	rbx, [8*rdx]
	test	rdx, rdx
	je	.LBB56_7
	mov	rsi, rdx
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 8
	mov	rcx, rbx
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB56_2
	mov	qword ptr [rbp - 8], rax
	mov	rcx, rax
	mov	rdx, rdi
	mov	qword ptr [rbp - 16], rbx
	mov	r8, rbx
	call	memcpy
	cmp	rsi, 1
	jne	.LBB56_5
.LBB56_9:
	cmp	rsi, 3
	mov	eax, 3
	cmovb	rax, rsi
	xor	esi, esi
	xor	edx, edx
	mov	rcx, qword ptr [rbp - 8]
	.p2align	4
.LBB56_10:
	add	rsi, qword ptr [rcx + 8*rdx]
	inc	rdx
	cmp	rax, rdx
	jne	.LBB56_10
	mov	r8d, 8
	mov	rdx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB56_12
.LBB56_7:
	mov	ecx, 8
	mov	rdx, rdi
	mov	r8, rbx
	call	memcpy
	xor	esi, esi
.LBB56_12:
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.LBB56_5:
	cmp	rsi, 21
	jae	.LBB56_6
	mov	r8d, 1
	mov	rcx, qword ptr [rbp - 8]
	mov	rdx, rsi
	call	_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E
	jmp	.LBB56_9
.LBB56_2:
	mov	ecx, 8
	mov	rdx, rbx
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.LBB56_6:
.Ltmp38:
	lea	r8, [rbp - 24]
	mov	rcx, qword ptr [rbp - 8]
	mov	rdx, rsi
	call	_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE
.Ltmp39:
	jmp	.LBB56_9
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE,unique,56
	.seh_endproc
	.def	"?dtor$3@?0?_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@4HA":
.seh_proc "?dtor$3@?0?_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@4HA"
.LBB56_3:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 64]
	.seh_endprologue
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 8]
	mov	rdx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end13:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE,unique,56
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE,unique,13
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@IMGREL
	.long	64
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE:
	.long	-1
	.long	"?dtor$3@?0?_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE:
	.long	.Lfunc_begin13@IMGREL
	.long	-1
	.long	.Ltmp38@IMGREL+1
	.long	0
	.long	.Ltmp39@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE,unique,56

	.def	_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E,unique,57
	.globl	_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E
	.p2align	4
_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E:
.Lfunc_begin14:
.seh_proc _ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	sub	rsp, 80
	.seh_stackalloc 80
	lea	rbp, [rsp + 80]
	.seh_setframe rbp, 80
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	r8, rdx
	mov	rdx, rcx
	lea	rcx, [rbp - 40]
	call	_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E
	mov	rcx, qword ptr [rbp - 32]
	mov	rdx, qword ptr [rbp - 24]
	test	rdx, rdx
	je	.LBB57_1
	mov	r8, rcx
	cmp	rdx, 1
	je	.LBB57_18
	movabs	r8, 2305843009213693951
	lea	r9, [rdx + r8]
	and	r8, r9
	mov	rdi, qword ptr [rcx]
	dec	r8
	mov	eax, r9d
	and	eax, 3
	cmp	r8, 3
	jae	.LBB57_5
	xor	r10d, r10d
	mov	r8, rcx
.LBB57_15:
	test	rax, rax
	je	.LBB57_18
	lea	r9, [rcx + 8*r10]
	add	r9, 8
	.p2align	4
.LBB57_17:
	mov	r10, qword ptr [r9]
	cmp	rdi, r10
	cmovbe	rdi, r10
	cmovbe	r8, r9
	add	r9, 8
	dec	rax
	jne	.LBB57_17
	jmp	.LBB57_18
.LBB57_1:
	xor	r8d, r8d
.LBB57_18:
	test	r8, r8
	lea	rax, [rip + alloc_53973d2fe29b4adba8bb7390b5678745]
	cmovne	rax, r8
	mov	rsi, qword ptr [rax]
.Ltmp40:
	mov	qword ptr [rbp - 16], rcx
	call	_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE
.Ltmp41:
	mov	rdi, rax
	mov	rdx, qword ptr [rbp - 40]
	test	rdx, rdx
	je	.LBB57_21
	shl	rdx, 3
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB57_21:
	mov	rax, rsi
	mov	rdx, rdi
	.seh_startepilogue
	add	rsp, 80
	pop	rdi
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.LBB57_5:
	lea	r11, [rcx + 8]
	movabs	r8, 2305843009213693948
	and	r9, r8
	add	r11, 24
	xor	r10d, r10d
	mov	r8, rcx
	jmp	.LBB57_6
	.p2align	4
.LBB57_14:
	add	r10, 4
	add	r11, 32
	cmp	r9, r10
	je	.LBB57_15
.LBB57_6:
	mov	rsi, qword ptr [r11 - 24]
	cmp	rdi, rsi
	jbe	.LBB57_7
	mov	rsi, qword ptr [r11 - 16]
	cmp	rdi, rsi
	jbe	.LBB57_9
.LBB57_10:
	mov	rsi, qword ptr [r11 - 8]
	cmp	rdi, rsi
	jbe	.LBB57_11
.LBB57_12:
	mov	rsi, qword ptr [r11]
	cmp	rdi, rsi
	ja	.LBB57_14
	jmp	.LBB57_13
	.p2align	4
.LBB57_7:
	lea	r8, [r11 - 24]
	mov	rdi, rsi
	mov	rsi, qword ptr [r11 - 16]
	cmp	rdi, rsi
	ja	.LBB57_10
.LBB57_9:
	lea	r8, [r11 - 16]
	mov	rdi, rsi
	mov	rsi, qword ptr [r11 - 8]
	cmp	rdi, rsi
	ja	.LBB57_12
.LBB57_11:
	lea	r8, [r11 - 8]
	mov	rdi, rsi
	mov	rsi, qword ptr [r11]
	cmp	rdi, rsi
	ja	.LBB57_14
.LBB57_13:
	mov	rdi, rsi
	mov	r8, r11
	jmp	.LBB57_14
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E,unique,57
	.seh_endproc
	.def	"?dtor$22@?0?_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$22@?0?_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@4HA":
.seh_proc "?dtor$22@?0?_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@4HA"
.LBB57_22:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 80]
	.seh_endprologue
	mov	rdx, qword ptr [rbp - 40]
	test	rdx, rdx
	je	.LBB57_24
	shl	rdx, 3
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB57_24:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rdi
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end14:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E,unique,57
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E,unique,14
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@IMGREL
	.long	72
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E:
	.long	-1
	.long	"?dtor$22@?0?_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E:
	.long	.Lfunc_begin14@IMGREL
	.long	-1
	.long	.Ltmp40@IMGREL+1
	.long	0
	.long	.Ltmp41@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E,unique,57

	.def	_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E,unique,58
	.globl	_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E
	.p2align	4
_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E:
.Lfunc_begin15:
.seh_proc _ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 232
	.seh_stackalloc 232
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	.seh_endprologue
	mov	qword ptr [rbp + 96], -2
	mov	rsi, rcx
	mov	qword ptr [rbp], 0
	mov	qword ptr [rbp + 8], r8
	mov	qword ptr [rbp + 16], rdx
	mov	qword ptr [rbp + 24], r8
	mov	qword ptr [rbp + 32], 0
	mov	qword ptr [rbp + 40], r8
	movabs	rax, 42949672970
	mov	qword ptr [rbp + 48], rax
	mov	byte ptr [rbp + 56], 1
	mov	word ptr [rbp + 64], 0
	mov	rbx, rbp
	xor	edi, edi
	.p2align	4
.LBB58_1:
	mov	rcx, rbx
	call	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
	test	rax, rax
	je	.LBB58_2
	mov	r14, rax
	mov	r15, rdx
	mov	rcx, rax
	call	_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E
	cmp	r15, 3
	jb	.LBB58_1
	test	rdx, rdx
	je	.LBB58_1
	movzx	r15d, byte ptr [r14]
	movzx	r14d, byte ptr [r14 + 2]
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edi, 1
	mov	ecx, 8
	mov	edx, 1
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB58_20
	mov	rbx, rax
	add	r15b, -65
	add	r14b, -88
	mov	byte ptr [rax], r15b
	mov	byte ptr [rax + 1], r14b
	mov	qword ptr [rbp + 72], 4
	mov	qword ptr [rbp + 80], rax
	mov	qword ptr [rbp + 88], 1
	mov	rax, qword ptr [rbp + 64]
	mov	qword ptr [rbp - 16], rax
	movups	xmm0, xmmword ptr [rbp]
	movups	xmm1, xmmword ptr [rbp + 16]
	movups	xmm2, xmmword ptr [rbp + 32]
	movups	xmm3, xmmword ptr [rbp + 48]
	movaps	xmmword ptr [rbp - 32], xmm3
	movaps	xmmword ptr [rbp - 48], xmm2
	movaps	xmmword ptr [rbp - 64], xmm1
	movaps	xmmword ptr [rbp - 80], xmm0
	lea	r14, [rbp - 80]
	lea	r15, [rbp + 72]
	.p2align	4
.LBB58_7:
.Ltmp42:
	mov	rcx, r14
	call	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
.Ltmp43:
	mov	r12, rax
	test	rax, rax
	je	.LBB58_18
	mov	r13, rdx
	mov	rcx, r12
	call	_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E
	cmp	r13, 3
	jb	.LBB58_7
	test	rdx, rdx
	je	.LBB58_7
	movzx	r13d, byte ptr [r12]
	movzx	r12d, byte ptr [r12 + 2]
	cmp	rdi, qword ptr [rbp + 72]
	jne	.LBB58_14
.Ltmp44:
	mov	qword ptr [rsp + 32], 2
	mov	r8d, 1
	mov	r9d, 1
	mov	rcx, r15
	mov	rdx, rdi
	call	_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E
.Ltmp45:
	mov	rbx, qword ptr [rbp + 80]
.LBB58_14:
	add	r13b, -65
	add	r12b, -88
	mov	byte ptr [rbx + 2*rdi], r13b
	mov	byte ptr [rbx + 2*rdi + 1], r12b
	inc	rdi
	mov	qword ptr [rbp + 88], rdi
	jmp	.LBB58_7
.LBB58_2:
	mov	eax, 1
	xor	ecx, ecx
	jmp	.LBB58_19
.LBB58_18:
	mov	rcx, qword ptr [rbp + 72]
	mov	rax, qword ptr [rbp + 80]
.LBB58_19:
	mov	qword ptr [rsi], rcx
	mov	qword ptr [rsi + 8], rax
	mov	qword ptr [rsi + 16], rdi
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 232
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.LBB58_20:
	mov	ecx, 1
	mov	edx, 8
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
	int3
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E,unique,58
	.seh_endproc
	.def	"?dtor$15@?0?_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$15@?0?_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@4HA":
.seh_proc "?dtor$15@?0?_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@4HA"
.LBB58_15:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 128]
	.seh_endprologue
	mov	rdx, qword ptr [rbp + 72]
	test	rdx, rdx
	je	.LBB58_17
	mov	rcx, qword ptr [rbp + 80]
	add	rdx, rdx
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB58_17:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end15:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E,unique,58
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E,unique,15
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@IMGREL
	.long	224
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E:
	.long	-1
	.long	"?dtor$15@?0?_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E:
	.long	.Lfunc_begin15@IMGREL
	.long	-1
	.long	.Ltmp42@IMGREL+1
	.long	0
	.long	.Ltmp45@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E,unique,58

	.def	_ZN7aoc20226solver5day025solve17h15f04a137efee39bE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day025solve17h15f04a137efee39bE,unique,59
	.globl	_ZN7aoc20226solver5day025solve17h15f04a137efee39bE
	.p2align	4
_ZN7aoc20226solver5day025solve17h15f04a137efee39bE:
.Lfunc_begin16:
.seh_proc _ZN7aoc20226solver5day025solve17h15f04a137efee39bE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	sub	rsp, 80
	.seh_stackalloc 80
	lea	rbp, [rsp + 80]
	.seh_setframe rbp, 80
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	r8, rdx
	mov	rdx, rcx
	lea	rcx, [rbp - 40]
	call	_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E
	mov	r10, qword ptr [rbp - 32]
	mov	rdx, qword ptr [rbp - 24]
	test	rdx, rdx
	je	.LBB59_1
	xor	r8d, r8d
	lea	r9, [rip + anon.44ffa63e8e95c400711a21744c5ea708.5]
	xor	esi, esi
	.p2align	4
.LBB59_3:
	movzx	ecx, byte ptr [r10 + 2*r8]
	cmp	rcx, 2
	ja	.LBB59_6
	movzx	eax, byte ptr [r10 + 2*r8 + 1]
	cmp	rax, 3
	jae	.LBB59_5
	lea	rcx, [rcx + 2*rcx]
	add	rcx, r9
	movzx	eax, byte ptr [rax + rcx]
	add	rsi, rax
	inc	r8
	cmp	rdx, r8
	jne	.LBB59_3
	xor	r8d, r8d
	lea	r9, [rip + anon.44ffa63e8e95c400711a21744c5ea708.6]
	xor	edi, edi
	.p2align	4
.LBB59_10:
	movzx	ecx, byte ptr [r10 + 2*r8]
	cmp	rcx, 2
	ja	.LBB59_16
	movzx	eax, byte ptr [r10 + 2*r8 + 1]
	cmp	rax, 3
	jae	.LBB59_12
	lea	rcx, [rcx + 2*rcx]
	add	rcx, r9
	movzx	eax, byte ptr [rax + rcx]
	add	rdi, rax
	inc	r8
	cmp	rdx, r8
	jne	.LBB59_10
	mov	rdx, qword ptr [rbp - 40]
	test	rdx, rdx
	je	.LBB59_20
.LBB59_19:
	add	rdx, rdx
	mov	r8d, 1
	mov	rcx, r10
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB59_20:
	mov	rax, rsi
	mov	rdx, rdi
	.seh_startepilogue
	add	rsp, 80
	pop	rdi
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.LBB59_1:
	xor	esi, esi
	xor	edi, edi
	mov	rdx, qword ptr [rbp - 40]
	test	rdx, rdx
	jne	.LBB59_19
	jmp	.LBB59_20
.LBB59_5:
.Ltmp48:
	mov	qword ptr [rbp - 16], r10
	lea	r8, [rip + alloc_04ec0caaeb79a0ce9c7e48871a54c01a]
	mov	edx, 3
	mov	rcx, rax
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
.Ltmp49:
	jmp	.LBB59_7
.LBB59_6:
.Ltmp46:
	mov	qword ptr [rbp - 16], r10
	lea	r8, [rip + alloc_04ec0caaeb79a0ce9c7e48871a54c01a]
	mov	edx, 3
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
.Ltmp47:
	jmp	.LBB59_7
.LBB59_12:
.Ltmp52:
	mov	qword ptr [rbp - 16], r10
	lea	r8, [rip + alloc_a1b3d90f42852e8e0faf60831e20d8b1]
	mov	edx, 3
	mov	rcx, rax
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
.Ltmp53:
	jmp	.LBB59_7
.LBB59_16:
.Ltmp50:
	mov	qword ptr [rbp - 16], r10
	lea	r8, [rip + alloc_a1b3d90f42852e8e0faf60831e20d8b1]
	mov	edx, 3
	call	_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE
.Ltmp51:
.LBB59_7:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day025solve17h15f04a137efee39bE,unique,59
	.seh_endproc
	.def	"?dtor$13@?0?_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$13@?0?_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@4HA":
.seh_proc "?dtor$13@?0?_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@4HA"
.LBB59_13:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 80]
	.seh_endprologue
	mov	rdx, qword ptr [rbp - 40]
	test	rdx, rdx
	je	.LBB59_15
	add	rdx, rdx
	mov	r8d, 1
	mov	rcx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB59_15:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rdi
	pop	rsi
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end16:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day025solve17h15f04a137efee39bE,unique,59
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day025solve17h15f04a137efee39bE,unique,16
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day025solve17h15f04a137efee39bE:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@IMGREL
	.long	72
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day025solve17h15f04a137efee39bE:
	.long	-1
	.long	"?dtor$13@?0?_ZN7aoc20226solver5day025solve17h15f04a137efee39bE@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day025solve17h15f04a137efee39bE:
	.long	.Lfunc_begin16@IMGREL
	.long	-1
	.long	.Ltmp48@IMGREL+1
	.long	0
	.long	.Ltmp51@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day025solve17h15f04a137efee39bE,unique,59

	.def	_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,60
	.globl	_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E
	.p2align	4
_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E:
.Lfunc_begin17:
.seh_proc _ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 248
	.seh_stackalloc 248
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	.seh_endprologue
	mov	qword ptr [rbp + 112], -2
	mov	rsi, rcx
	mov	qword ptr [rbp], 0
	mov	qword ptr [rbp + 8], r8
	mov	qword ptr [rbp + 16], rdx
	mov	qword ptr [rbp + 24], r8
	mov	qword ptr [rbp + 32], 0
	mov	qword ptr [rbp + 40], r8
	movabs	rax, 42949672970
	mov	qword ptr [rbp + 48], rax
	mov	byte ptr [rbp + 56], 1
	mov	word ptr [rbp + 64], 0
	mov	r14, rbp
	xor	edi, edi
	mov	r15d, 8
	.p2align	4
.LBB60_1:
	mov	rcx, r14
	call	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
	test	rax, rax
	je	.LBB60_7
	mov	rbx, rax
	mov	r12, rdx
	mov	rcx, rax
	call	_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E
	test	rdx, rdx
	je	.LBB60_1
	test	r12, r12
	je	.LBB60_8
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, r12
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB60_25
	mov	rdi, rax
	mov	rcx, rax
	mov	rdx, rbx
	mov	r8, r12
	call	memcpy
	mov	rax, r12
	neg	rax
	jno	.LBB60_9
	xor	edi, edi
.LBB60_7:
	xor	eax, eax
	jmp	.LBB60_23
.LBB60_8:
	mov	edi, 1
.LBB60_9:
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	ecx, 96
	mov	edx, 8
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB60_24
	mov	rbx, rax
	mov	qword ptr [rax], r12
	mov	qword ptr [rax + 8], rdi
	mov	qword ptr [rax + 16], r12
	mov	qword ptr [rbp + 72], 4
	mov	qword ptr [rbp + 80], rax
	mov	qword ptr [rbp + 88], 1
	mov	rax, qword ptr [rbp + 64]
	mov	qword ptr [rbp - 16], rax
	movups	xmm0, xmmword ptr [rbp]
	movups	xmm1, xmmword ptr [rbp + 16]
	movups	xmm2, xmmword ptr [rbp + 32]
	movups	xmm3, xmmword ptr [rbp + 48]
	movaps	xmmword ptr [rbp - 32], xmm3
	movaps	xmmword ptr [rbp - 48], xmm2
	movaps	xmmword ptr [rbp - 64], xmm1
	movaps	xmmword ptr [rbp - 80], xmm0
	mov	edi, 1
	lea	r14, [rbp - 80]
	lea	r15, [rbp + 72]
	.p2align	4
.LBB60_11:
.Ltmp54:
	mov	rcx, r14
	call	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
.Ltmp55:
	mov	r12, rax
	test	rax, rax
	je	.LBB60_22
	mov	r13, rdx
	mov	rcx, r12
	call	_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E
	test	rdx, rdx
	je	.LBB60_11
	test	r13, r13
	je	.LBB60_18
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, r13
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB60_26
	mov	qword ptr [rbp + 104], rax
	mov	rcx, rax
	mov	rdx, r12
	mov	r8, r13
	call	memcpy
	mov	rax, r13
	neg	rax
	jo	.LBB60_22
	cmp	rdi, qword ptr [rbp + 72]
	je	.LBB60_19
	jmp	.LBB60_21
.LBB60_18:
	mov	eax, 1
	mov	qword ptr [rbp + 104], rax
	cmp	rdi, qword ptr [rbp + 72]
	jne	.LBB60_21
.LBB60_19:
.Ltmp58:
	mov	qword ptr [rbp + 96], r13
	mov	qword ptr [rsp + 32], 24
	mov	r8d, 1
	mov	r9d, 8
	mov	rcx, r15
	mov	rdx, rdi
	call	_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E
.Ltmp59:
	mov	rbx, qword ptr [rbp + 80]
	mov	r13, qword ptr [rbp + 96]
.LBB60_21:
	lea	rax, [rdi + 2*rdi]
	mov	qword ptr [rbx + 8*rax], r13
	mov	rcx, qword ptr [rbp + 104]
	mov	qword ptr [rbx + 8*rax + 8], rcx
	mov	qword ptr [rbx + 8*rax + 16], r13
	inc	rdi
	mov	qword ptr [rbp + 88], rdi
	jmp	.LBB60_11
.LBB60_22:
	mov	rax, qword ptr [rbp + 72]
	mov	r15, qword ptr [rbp + 80]
.LBB60_23:
	mov	qword ptr [rsi], rax
	mov	qword ptr [rsi + 8], r15
	mov	qword ptr [rsi + 16], rdi
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 248
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.LBB60_24:
.Ltmp60:
	mov	qword ptr [rbp + 96], rdi
	mov	qword ptr [rbp + 104], r12
	mov	ecx, 8
	mov	edx, 96
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.Ltmp61:
	jmp	.LBB60_27
.LBB60_25:
	mov	ecx, 1
	mov	rdx, r12
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.LBB60_26:
.Ltmp56:
	mov	ecx, 1
	mov	rdx, r13
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.Ltmp57:
.LBB60_27:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,60
	.seh_endproc
	.def	"?dtor$28@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$28@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA":
.seh_proc "?dtor$28@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA"
.LBB60_28:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 128]
	.seh_endprologue
	cmp	qword ptr [rbp + 96], 0
	je	.LBB60_30
	mov	r8d, 1
	mov	rcx, qword ptr [rbp + 104]
	mov	rdx, qword ptr [rbp + 96]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB60_30:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,60
	.seh_endproc
	.def	"?dtor$31@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$31@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA":
.seh_proc "?dtor$31@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA"
.LBB60_31:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 128]
	.seh_endprologue
	cmp	qword ptr [rbp + 104], 0
	je	.LBB60_33
	mov	r8d, 1
	mov	rcx, qword ptr [rbp + 96]
	mov	rdx, qword ptr [rbp + 104]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB60_33:
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,60
	.seh_endproc
	.def	"?dtor$34@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$34@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA":
.seh_proc "?dtor$34@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA"
.LBB60_34:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 128]
	.seh_endprologue
	lea	rcx, [rbp + 72]
	call	_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end17:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,60
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,17
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E:
	.long	429065506
	.long	3
	.long	$stateUnwindMap$_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@IMGREL
	.long	0
	.long	0
	.long	8
	.long	$ip2state$_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@IMGREL
	.long	240
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E:
	.long	-1
	.long	"?dtor$31@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA"@IMGREL
	.long	-1
	.long	"?dtor$34@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA"@IMGREL
	.long	1
	.long	"?dtor$28@?0?_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E:
	.long	.Lfunc_begin17@IMGREL
	.long	-1
	.long	.Ltmp54@IMGREL+1
	.long	1
	.long	.Ltmp55@IMGREL+1
	.long	-1
	.long	.Ltmp58@IMGREL+1
	.long	2
	.long	.Ltmp60@IMGREL+1
	.long	0
	.long	.Ltmp61@IMGREL+1
	.long	-1
	.long	.Ltmp56@IMGREL+1
	.long	1
	.long	.Ltmp57@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E,unique,60

	.def	_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E,unique,61
	.globl	_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E
	.p2align	4
_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E:
.Lfunc_begin18:
.seh_proc _ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 136
	.seh_stackalloc 136
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	.seh_endprologue
	mov	qword ptr [rbp], -2
	mov	r8, rdx
	mov	rdx, rcx
	lea	rcx, [rbp - 80]
	call	_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E
	mov	rax, qword ptr [rbp - 72]
	mov	qword ptr [rbp - 40], rax
	mov	r14, qword ptr [rbp - 64]
	test	r14, r14
	je	.LBB61_1
	mov	r11d, 1
	xor	r15d, r15d
	mov	dword ptr [rbp - 20], 0
	mov	qword ptr [rbp - 32], r14
	jmp	.LBB61_3
	.p2align	4
.LBB61_27:
	mov	r11d, 1
	inc	r15
	cmp	r15, r14
	je	.LBB61_33
.LBB61_3:
	lea	rcx, [r15 + 2*r15]
	mov	r8, qword ptr [rbp - 40]
	mov	rax, qword ptr [r8 + 8*rcx + 8]
	mov	rdx, qword ptr [r8 + 8*rcx + 16]
	cmp	rdx, 2
	jae	.LBB61_5
	xor	r13d, r13d
	xor	r12d, r12d
	test	rdx, rdx
	je	.LBB61_17
.LBB61_18:
	cmp	rdx, 4
	jae	.LBB61_20
	xor	esi, esi
	xor	r9d, r9d
	and	edx, 3
	mov	r11d, 0
	mov	ebx, 1
	jne	.LBB61_24
	jmp	.LBB61_26
	.p2align	4
.LBB61_5:
	mov	r9, rdx
	shr	r9
	cmp	byte ptr [rax + r9], -65
	jle	.LBB61_6
	mov	qword ptr [rbp - 8], rdx
	lea	rcx, [r9 - 1]
	mov	r13d, 0
	mov	r12d, 0
	mov	rsi, rax
	cmp	rcx, 3
	jb	.LBB61_12
	mov	qword ptr [rbp - 16], r15
	mov	rdi, r9
	movabs	rcx, 4611686018427387900
	and	rdi, rcx
	xor	r13d, r13d
	xor	r12d, r12d
	mov	rsi, rax
	xor	edx, edx
	mov	r11d, 1
	.p2align	4
.LBB61_10:
	movzx	ecx, byte ptr [rsi]
	xor	ebx, ebx
	shld	rbx, r11, cl
	mov	r14d, 1
	shl	r14, cl
	movzx	r8d, byte ptr [rsi + 1]
	test	cl, 64
	cmovne	rbx, r14
	cmovne	r14, rdx
	xor	r10d, r10d
	mov	ecx, r8d
	shld	r10, r11, cl
	or	rbx, r12
	or	r14, r13
	mov	r12d, 1
	shl	r12, cl
	test	r8b, 64
	cmovne	r10, r12
	cmovne	r12, rdx
	movzx	ecx, byte ptr [rsi + 2]
	xor	r8d, r8d
	shld	r8, r11, cl
	mov	r15d, 1
	shl	r15, cl
	test	cl, 64
	cmovne	r8, r15
	cmovne	r15, rdx
	or	r8, r10
	or	r8, rbx
	or	r15, r12
	movzx	ecx, byte ptr [rsi + 3]
	xor	r12d, r12d
	shld	r12, r11, cl
	or	r15, r14
	add	rsi, 4
	mov	r13d, 1
	shl	r13, cl
	test	cl, 64
	cmovne	r12, r13
	cmovne	r13, rdx
	or	r12, r8
	or	r13, r15
	add	rdi, -4
	jne	.LBB61_10
	mov	r14, qword ptr [rbp - 32]
	mov	r15, qword ptr [rbp - 16]
.LBB61_12:
	mov	r8d, r9d
	and	r8d, 3
	mov	r11d, 0
	mov	edx, 1
	je	.LBB61_15
	xor	edi, edi
	.p2align	4
.LBB61_14:
	movzx	ecx, byte ptr [rsi + rdi]
	xor	r10d, r10d
	shld	r10, rdx, cl
	mov	ebx, 1
	shl	rbx, cl
	test	cl, 64
	cmovne	r10, rbx
	cmovne	rbx, r11
	or	r12, r10
	or	r13, rbx
	inc	rdi
	cmp	r8, rdi
	jne	.LBB61_14
.LBB61_15:
	add	rax, r9
	mov	rdx, qword ptr [rbp - 8]
	sub	rdx, r9
	mov	r11d, 1
	test	rdx, rdx
	jne	.LBB61_18
.LBB61_17:
	xor	esi, esi
	xor	r9d, r9d
	jmp	.LBB61_26
	.p2align	4
.LBB61_20:
	mov	qword ptr [rbp - 16], r15
	mov	qword ptr [rbp - 8], rdx
	mov	rdi, rdx
	and	rdi, -4
	xor	esi, esi
	xor	r9d, r9d
	xor	edx, edx
	.p2align	4
.LBB61_21:
	movzx	ecx, byte ptr [rax]
	xor	ebx, ebx
	shld	rbx, r11, cl
	mov	r14d, 1
	shl	r14, cl
	movzx	r8d, byte ptr [rax + 1]
	test	cl, 64
	cmovne	rbx, r14
	cmovne	r14, rdx
	xor	r10d, r10d
	mov	ecx, r8d
	shld	r10, r11, cl
	or	rbx, r9
	or	r14, rsi
	mov	r9d, 1
	shl	r9, cl
	test	r8b, 64
	cmovne	r10, r9
	cmovne	r9, rdx
	movzx	ecx, byte ptr [rax + 2]
	xor	r8d, r8d
	shld	r8, r11, cl
	mov	r15d, 1
	shl	r15, cl
	test	cl, 64
	cmovne	r8, r15
	cmovne	r15, rdx
	or	r8, r10
	or	r8, rbx
	or	r15, r9
	movzx	ecx, byte ptr [rax + 3]
	xor	r9d, r9d
	shld	r9, r11, cl
	or	r15, r14
	add	rax, 4
	mov	esi, 1
	shl	rsi, cl
	test	cl, 64
	cmovne	r9, rsi
	cmovne	rsi, rdx
	or	r9, r8
	or	rsi, r15
	add	rdi, -4
	jne	.LBB61_21
	mov	r14, qword ptr [rbp - 32]
	mov	r15, qword ptr [rbp - 16]
	mov	rdx, qword ptr [rbp - 8]
	and	edx, 3
	mov	r11d, 0
	mov	ebx, 1
	je	.LBB61_26
.LBB61_24:
	xor	r8d, r8d
	.p2align	4
.LBB61_25:
	movzx	ecx, byte ptr [rax + r8]
	xor	r10d, r10d
	shld	r10, rbx, cl
	mov	edi, 1
	shl	rdi, cl
	test	cl, 64
	cmovne	r10, rdi
	cmovne	rdi, r11
	or	r9, r10
	or	rsi, rdi
	inc	r8
	cmp	rdx, r8
	jne	.LBB61_25
.LBB61_26:
	and	rsi, r13
	and	r9, r12
	rep		bsf	rcx, rsi
	rep		bsf	rax, r9
	add	eax, 64
	test	rsi, rsi
	cmovne	eax, ecx
	or	r9, rsi
	je	.LBB61_27
	lea	ecx, [rax - 97]
	cmp	ecx, 26
	mov	edx, 0
	mov	r11d, 1
	jae	.LBB61_29
	add	eax, -96
	jmp	.LBB61_31
	.p2align	4
.LBB61_29:
	lea	ecx, [rax - 65]
	add	eax, -38
	cmp	ecx, 26
	cmovae	eax, edx
.LBB61_31:
	add	dword ptr [rbp - 20], eax
	inc	r15
	cmp	r15, r14
	jne	.LBB61_3
.LBB61_33:
	mov	r9d, 1
	mov	dword ptr [rbp - 24], 0
	mov	rcx, r14
	mov	r10, qword ptr [rbp - 40]
	jmp	.LBB61_34
.LBB61_66:
	mov	r14, qword ptr [rbp - 32]
	mov	r9d, 1
	mov	rcx, qword ptr [rbp - 16]
	mov	r10, qword ptr [rbp - 8]
	mov	rdx, qword ptr [rbp - 56]
	.p2align	4
.LBB61_71:
	lea	rax, [rdx + 2*rdx]
	mov	eax, eax
	lea	r10, [r10 + 8*rax]
	sub	rcx, rdx
	je	.LBB61_72
.LBB61_34:
	cmp	rcx, 3
	mov	edx, 3
	cmovb	rdx, rcx
	jb	.LBB61_71
	mov	rbx, qword ptr [r10 + 16]
	test	rbx, rbx
	mov	qword ptr [rbp - 16], rcx
	mov	qword ptr [rbp - 8], r10
	mov	qword ptr [rbp - 56], rdx
	je	.LBB61_36
	mov	rdi, qword ptr [r10 + 8]
	cmp	rbx, 4
	mov	r14d, 0
	jae	.LBB61_39
	xor	r12d, r12d
	xor	r15d, r15d
	and	ebx, 3
	jne	.LBB61_43
	jmp	.LBB61_45
.LBB61_36:
	xor	r12d, r12d
	xor	r15d, r15d
	xor	r14d, r14d
	jmp	.LBB61_45
.LBB61_39:
	mov	r13d, 1
	mov	r11, rbx
	and	r11, -4
	xor	r12d, r12d
	xor	r15d, r15d
	.p2align	4
.LBB61_40:
	movzx	ecx, byte ptr [rdi]
	xor	r10d, r10d
	shld	r10, r13, cl
	mov	r9d, 1
	shl	r9, cl
	movzx	eax, byte ptr [rdi + 1]
	test	cl, 64
	cmovne	r10, r9
	cmovne	r9, r14
	xor	edx, edx
	mov	ecx, eax
	shld	rdx, r13, cl
	or	r10, r15
	or	r9, r12
	mov	r8d, 1
	shl	r8, cl
	test	al, 64
	cmovne	rdx, r8
	cmovne	r8, r14
	movzx	ecx, byte ptr [rdi + 2]
	xor	eax, eax
	shld	rax, r13, cl
	mov	esi, 1
	shl	rsi, cl
	test	cl, 64
	cmovne	rax, rsi
	cmovne	rsi, r14
	or	rax, rdx
	or	rax, r10
	or	rsi, r8
	movzx	ecx, byte ptr [rdi + 3]
	xor	r15d, r15d
	shld	r15, r13, cl
	or	rsi, r9
	add	rdi, 4
	mov	r12d, 1
	shl	r12, cl
	test	cl, 64
	cmovne	r15, r12
	cmovne	r12, r14
	or	r15, rax
	or	r12, rsi
	add	r11, -4
	jne	.LBB61_40
	mov	r9d, 1
	mov	r10, qword ptr [rbp - 8]
	and	ebx, 3
	je	.LBB61_45
.LBB61_43:
	xor	eax, eax
	.p2align	4
.LBB61_44:
	movzx	ecx, byte ptr [rdi + rax]
	xor	edx, edx
	shld	rdx, r9, cl
	mov	r8d, 1
	shl	r8, cl
	test	cl, 64
	cmovne	rdx, r8
	cmovne	r8, r14
	or	r15, rdx
	or	r12, r8
	inc	rax
	cmp	rbx, rax
	jne	.LBB61_44
.LBB61_45:
	mov	r11, qword ptr [r10 + 40]
	test	r11, r11
	je	.LBB61_46
	mov	rsi, qword ptr [r10 + 32]
	cmp	r11, 4
	jae	.LBB61_49
	xor	ebx, ebx
	xor	r13d, r13d
	and	r11d, 3
	jne	.LBB61_53
	jmp	.LBB61_55
.LBB61_46:
	xor	ebx, ebx
	xor	r13d, r13d
	jmp	.LBB61_55
.LBB61_49:
	mov	qword ptr [rbp - 48], r11
	and	r11, -4
	xor	ebx, ebx
	xor	r13d, r13d
	.p2align	4
.LBB61_50:
	movzx	ecx, byte ptr [rsi]
	xor	r10d, r10d
	mov	edi, 1
	shld	r10, rdi, cl
	mov	r9d, 1
	shl	r9, cl
	movzx	eax, byte ptr [rsi + 1]
	test	cl, 64
	cmovne	r10, r9
	cmovne	r9, r14
	xor	edx, edx
	mov	ecx, eax
	shld	rdx, rdi, cl
	or	r10, r13
	or	r9, rbx
	mov	r8d, 1
	shl	r8, cl
	test	al, 64
	cmovne	rdx, r8
	cmovne	r8, r14
	movzx	ecx, byte ptr [rsi + 2]
	xor	eax, eax
	shld	rax, rdi, cl
	mov	r14d, 1
	shl	r14, cl
	test	cl, 64
	cmovne	rax, r14
	mov	ecx, 0
	cmovne	r14, rcx
	or	rax, rdx
	or	rax, r10
	or	r14, r8
	movzx	ecx, byte ptr [rsi + 3]
	xor	r13d, r13d
	shld	r13, rdi, cl
	or	r14, r9
	add	rsi, 4
	mov	ebx, 1
	shl	rbx, cl
	test	cl, 64
	cmovne	r13, rbx
	mov	ecx, 0
	cmovne	rbx, rcx
	or	r13, rax
	or	rbx, r14
	xor	r14d, r14d
	add	r11, -4
	jne	.LBB61_50
	mov	r9d, 1
	mov	r10, qword ptr [rbp - 8]
	mov	r11, qword ptr [rbp - 48]
	and	r11d, 3
	je	.LBB61_55
.LBB61_53:
	xor	eax, eax
	.p2align	4
.LBB61_54:
	movzx	ecx, byte ptr [rsi + rax]
	xor	edx, edx
	shld	rdx, r9, cl
	mov	r8d, 1
	shl	r8, cl
	test	cl, 64
	cmovne	rdx, r8
	cmovne	r8, r14
	or	r13, rdx
	or	rbx, r8
	inc	rax
	cmp	r11, rax
	jne	.LBB61_54
.LBB61_55:
	mov	rdi, qword ptr [r10 + 64]
	test	rdi, rdi
	je	.LBB61_56
	mov	rsi, qword ptr [r10 + 56]
	cmp	rdi, 4
	jae	.LBB61_59
	xor	r11d, r11d
	xor	r14d, r14d
	and	edi, 3
	mov	r9d, 0
	mov	r10d, 1
	jne	.LBB61_63
	jmp	.LBB61_65
.LBB61_56:
	xor	r11d, r11d
	xor	r14d, r14d
	jmp	.LBB61_65
.LBB61_59:
	mov	qword ptr [rbp - 48], rdi
	and	rdi, -4
	xor	r11d, r11d
	xor	r14d, r14d
	.p2align	4
.LBB61_60:
	movzx	ecx, byte ptr [rsi]
	xor	r9d, r9d
	mov	eax, 1
	shld	r9, rax, cl
	mov	edx, 1
	mov	r10d, 1
	shl	r10, cl
	movzx	eax, byte ptr [rsi + 1]
	test	cl, 64
	cmovne	r9, r10
	mov	ecx, 0
	cmovne	r10, rcx
	xor	r8d, r8d
	mov	ecx, eax
	shld	r8, rdx, cl
	or	r9, r14
	or	r10, r11
	mov	r11d, 1
	shl	r11, cl
	test	al, 64
	cmovne	r8, r11
	mov	r14d, 0
	cmovne	r11, r14
	movzx	ecx, byte ptr [rsi + 2]
	xor	eax, eax
	shld	rax, rdx, cl
	shl	rdx, cl
	test	cl, 64
	cmovne	rax, rdx
	cmovne	rdx, r14
	or	rax, r8
	or	rax, r9
	or	rdx, r11
	movzx	ecx, byte ptr [rsi + 3]
	xor	r14d, r14d
	mov	r8d, 1
	shld	r14, r8, cl
	or	rdx, r10
	add	rsi, 4
	mov	r11d, 1
	shl	r11, cl
	test	cl, 64
	cmovne	r14, r11
	mov	ecx, 0
	cmovne	r11, rcx
	or	r14, rax
	or	r11, rdx
	add	rdi, -4
	jne	.LBB61_60
	mov	rdi, qword ptr [rbp - 48]
	and	edi, 3
	mov	r9d, 0
	mov	r10d, 1
	je	.LBB61_65
.LBB61_63:
	xor	eax, eax
	.p2align	4
.LBB61_64:
	movzx	ecx, byte ptr [rsi + rax]
	xor	edx, edx
	shld	rdx, r10, cl
	mov	r8d, 1
	shl	r8, cl
	test	cl, 64
	cmovne	rdx, r8
	cmovne	r8, r9
	or	r14, rdx
	or	r11, r8
	inc	rax
	cmp	rdi, rax
	jne	.LBB61_64
.LBB61_65:
	xor	edx, edx
	and	rbx, r12
	and	rbx, r11
	and	r13, r15
	and	r13, r14
	rep		bsf	rcx, rbx
	rep		bsf	rax, r13
	add	eax, 64
	test	rbx, rbx
	cmovne	eax, ecx
	or	r13, rbx
	je	.LBB61_66
	lea	ecx, [rax - 97]
	cmp	ecx, 26
	mov	r14, qword ptr [rbp - 32]
	mov	r9d, 1
	mov	r10, qword ptr [rbp - 8]
	jae	.LBB61_68
	add	eax, -96
	jmp	.LBB61_70
.LBB61_68:
	lea	ecx, [rax - 65]
	add	eax, -38
	cmp	ecx, 26
	cmovae	eax, edx
.LBB61_70:
	mov	rcx, qword ptr [rbp - 16]
	mov	rdx, qword ptr [rbp - 56]
	add	dword ptr [rbp - 24], eax
	jmp	.LBB61_71
.LBB61_72:
	mov	rax, qword ptr [rbp - 40]
	lea	rsi, [rax + 8]
	jmp	.LBB61_73
	.p2align	4
.LBB61_75:
	add	rsi, 24
	dec	r14
	je	.LBB61_76
.LBB61_73:
	mov	rdx, qword ptr [rsi - 8]
	test	rdx, rdx
	je	.LBB61_75
	mov	rcx, qword ptr [rsi]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB61_75
.LBB61_76:
	mov	esi, dword ptr [rbp - 24]
	mov	rax, qword ptr [rbp - 80]
	test	rax, rax
	je	.LBB61_79
.LBB61_78:
	shl	rax, 3
	lea	rdx, [rax + 2*rax]
	mov	r8d, 8
	mov	rcx, qword ptr [rbp - 40]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB61_79:
	mov	eax, dword ptr [rbp - 20]
	mov	edx, esi
	.seh_startepilogue
	add	rsp, 136
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.LBB61_1:
	xor	esi, esi
	mov	dword ptr [rbp - 20], 0
	mov	rax, qword ptr [rbp - 80]
	test	rax, rax
	jne	.LBB61_78
	jmp	.LBB61_79
.LBB61_6:
.Ltmp62:
	lea	rcx, [rip + alloc_90427cacc85724e4d3b32dbfd394b367]
	mov	qword ptr [rsp + 32], rcx
	mov	rcx, rax
	xor	r8d, r8d
	call	_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE
.Ltmp63:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E,unique,61
	.seh_endproc
	.def	"?dtor$80@?0?_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$80@?0?_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@4HA":
.seh_proc "?dtor$80@?0?_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@4HA"
.LBB61_80:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	lea	rbp, [rdx + 128]
	.seh_endprologue
	lea	rcx, [rbp - 80]
	call	_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E
	nop
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end18:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E,unique,61
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E,unique,18
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E:
	.long	429065506
	.long	1
	.long	$stateUnwindMap$_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@IMGREL
	.long	0
	.long	0
	.long	3
	.long	$ip2state$_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@IMGREL
	.long	128
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E:
	.long	-1
	.long	"?dtor$80@?0?_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E:
	.long	.Lfunc_begin18@IMGREL
	.long	-1
	.long	.Ltmp62@IMGREL+1
	.long	0
	.long	.Ltmp63@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E,unique,61

	.def	_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.globl	_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE
	.p2align	4
_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE:
.Lfunc_begin19:
.seh_proc _ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 456
	.seh_stackalloc 456
	lea	rbp, [rsp + 128]
	.seh_setframe rbp, 128
	movdqa	xmmword ptr [rbp + 304], xmm7
	.seh_savexmm xmm7, 432
	movdqa	xmmword ptr [rbp + 288], xmm6
	.seh_savexmm xmm6, 416
	.seh_endprologue
	mov	qword ptr [rbp + 280], -2
	mov	r8, rdx
	mov	rdx, rcx
	lea	rcx, [rbp + 16]
	call	_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E
	mov	rax, qword ptr [rbp + 24]
	mov	qword ptr [rbp + 216], rax
	mov	rax, qword ptr [rbp + 32]
	mov	dword ptr [rbp + 252], 0
	mov	qword ptr [rbp + 192], rax
	test	rax, rax
	je	.LBB62_41
	lea	rdi, [rbp + 136]
	lea	r12, [rbp + 80]
	pcmpeqd	xmm6, xmm6
	xor	r13d, r13d
	mov	dword ptr [rbp + 252], 0
	jmp	.LBB62_2
	.p2align	4
.LBB62_38:
	add	r14d, -96
.LBB62_39:
	add	dword ptr [rbp + 252], r14d
.LBB62_40:
	inc	r13
	cmp	r13, qword ptr [rbp + 192]
	je	.LBB62_41
.LBB62_2:
	lea	rax, [2*r13]
	add	rax, r13
	mov	rcx, qword ptr [rbp + 216]
	mov	rdx, qword ptr [rcx + 8*rax + 8]
	mov	r14, qword ptr [rcx + 8*rax + 16]
	mov	r9, r14
	shr	r9
	cmp	r14, 2
	jae	.LBB62_4
	mov	r15, rdx
	jmp	.LBB62_6
	.p2align	4
.LBB62_4:
	cmp	byte ptr [rdx + r9], -64
	jl	.LBB62_18
	lea	r15, [rdx + r9]
	sub	r14, r9
.LBB62_6:
	add	r9, rdx
.Ltmp66:
	mov	rcx, rdi
	mov	r8, r9
	call	_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E
.Ltmp67:
	add	r14, r15
.Ltmp68:
	mov	rcx, r12
	mov	rdx, r15
	mov	r8, r14
	call	_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E
.Ltmp69:
	mov	r15, qword ptr [rbp + 136]
	mov	r10, qword ptr [rbp + 160]
	mov	r11, qword ptr [rbp + 104]
	cmp	r10, r11
	mov	rax, r11
	cmovb	rax, r10
	mov	rcx, qword ptr [rbp + 80]
	mov	rdx, qword ptr [rbp + 88]
	mov	r14d, 1114112
	test	rax, rax
	je	.LBB62_29
	cmp	r10, r11
	mov	r9, r15
	cmova	r9, rcx
	mov	rsi, r12
	cmova	rsi, rdi
	mov	r8, qword ptr [rbp + 144]
	cmovbe	r8, rdx
	mov	qword ptr [rbp + 224], r8
	mov	r8, rcx
	cmova	r8, r15
	mov	qword ptr [rbp + 264], r8
	lea	r8, [r9 + 16]
	movdqa	xmm0, xmmword ptr [r9]
	pmovmskb	ebx, xmm0
	or	r11, r10
	je	.LBB62_21
	mov	qword ptr [rbp + 256], rcx
	mov	rdi, qword ptr [rsi + 32]
	mov	r10, qword ptr [rsi + 40]
	mov	rsi, rdi
	movabs	rcx, 8317987319222330741
	xor	rsi, rcx
	mov	r12, r10
	movabs	rcx, 7237128888997146477
	xor	r12, rcx
	movabs	rcx, 7816392313619706465
	xor	rdi, rcx
	add	rsi, r12
	rol	r12, 13
	xor	r12, rsi
	rol	rsi, 32
	lea	rcx, [r12 + rdi]
	mov	qword ptr [rbp + 200], rcx
	rol	r12, 17
	not	ebx
	movabs	rcx, 8098989879002948979
	xor	r10, rcx
	mov	qword ptr [rbp + 232], r10
	mov	qword ptr [rbp + 240], r15
	mov	qword ptr [rbp + 184], rdi
.LBB62_11:
	test	bx, bx
	jne	.LBB62_14
	.p2align	4
.LBB62_12:
	movdqa	xmm0, xmmword ptr [r8]
	pmovmskb	ebx, xmm0
	add	r9, -64
	add	r8, 16
	cmp	ebx, 65535
	je	.LBB62_12
	not	ebx
.LBB62_14:
	rep		bsf	r10d, ebx
	shl	r10d, 2
	mov	r11, r9
	sub	r11, r10
	mov	r15d, dword ptr [r11 - 4]
	movabs	rcx, 288230376151711744
	lea	r10, [r15 + rcx]
	mov	qword ptr [rbp + 208], r15
	xor	r15, qword ptr [rbp + 232]
	add	rdi, r15
	mov	rcx, qword ptr [rbp + 200]
	lea	r11, [rcx + r15]
	rol	r15, 16
	xor	r15, rdi
	lea	rdi, [r15 + rsi]
	rol	r15, 21
	xor	r15, rdi
	xor	r10, rdi
	mov	rdi, r11
	rol	r11, 32
	xor	rdi, r12
	xor	r11, 255
	add	r10, rdi
	add	r11, r15
	rol	rdi, 13
	rol	r15, 16
	xor	rdi, r10
	xor	r15, r11
	rol	r10, 32
	add	r11, rdi
	add	r10, r15
	rol	rdi, 17
	xor	rdi, r11
	rol	r15, 21
	xor	r15, r10
	rol	r11, 32
	add	r10, rdi
	add	r11, r15
	rol	rdi, 13
	rol	r15, 16
	xor	rdi, r10
	xor	r15, r11
	rol	r10, 32
	add	r11, rdi
	add	r10, r15
	rol	rdi, 17
	xor	rdi, r11
	rol	r15, 21
	xor	r15, r10
	add	r10, rdi
	rol	rdi, 13
	xor	rdi, r10
	lea	ecx, [rbx - 1]
	rol	r11, 32
	add	r11, r15
	rol	r15, 16
	xor	r15, r11
	add	r11, rdi
	rol	rdi, 17
	rol	r15, 21
	and	ecx, ebx
	mov	r10, r11
	rol	r10, 32
	xor	r10, rdi
	xor	r10, r15
	mov	ebx, ecx
	dec	rax
	xor	r10, r11
	mov	rcx, r10
	shr	rcx, 57
	movd	xmm0, ecx
	punpcklbw	xmm0, xmm0
	pshuflw	xmm0, xmm0, 0
	pshufd	xmm0, xmm0, 68
	xor	r14d, r14d
.LBB62_15:
	mov	rcx, qword ptr [rbp + 264]
	mov	r15, qword ptr [rbp + 224]
	and	r10, r15
	movdqu	xmm1, xmmword ptr [rcx + r10]
	movdqa	xmm2, xmm1
	pcmpeqb	xmm2, xmm0
	pmovmskb	edi, xmm2
	test	edi, edi
	je	.LBB62_24
.LBB62_16:
	rep		bsf	ecx, edi
	add	rcx, r10
	and	rcx, r15
	shl	rcx, 2
	mov	r11, r15
	mov	r15, qword ptr [rbp + 264]
	sub	r15, rcx
	mov	rcx, qword ptr [rbp + 208]
	cmp	ecx, dword ptr [r15 - 4]
	je	.LBB62_17
	lea	ecx, [rdi - 1]
	and	cx, di
	mov	edi, ecx
	mov	r15, r11
	jne	.LBB62_16
	.p2align	4
.LBB62_24:
	pcmpeqb	xmm1, xmm6
	pmovmskb	ecx, xmm1
	test	ecx, ecx
	mov	r15, qword ptr [rbp + 240]
	jne	.LBB62_26
	add	r10, r14
	add	r10, 16
	add	r14, 16
	jmp	.LBB62_15
	.p2align	4
.LBB62_26:
	test	rax, rax
	mov	r14d, 1114112
	mov	rdi, qword ptr [rbp + 184]
	jne	.LBB62_11
	lea	rdi, [rbp + 136]
	lea	r12, [rbp + 80]
	jmp	.LBB62_28
	.p2align	4
.LBB62_17:
	mov	r14d, ecx
	lea	rdi, [rbp + 136]
	lea	r12, [rbp + 80]
	mov	r15, qword ptr [rbp + 240]
.LBB62_28:
	mov	rcx, qword ptr [rbp + 256]
	jmp	.LBB62_29
	.p2align	4
.LBB62_21:
	cmp	bx, -1
	jne	.LBB62_29
	.p2align	4
.LBB62_22:
	movdqa	xmm0, xmmword ptr [r8]
	add	r8, 16
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	je	.LBB62_22
	.p2align	4
.LBB62_29:
	test	rdx, rdx
	je	.LBB62_32
	lea	rax, [4*rdx + 19]
	and	rax, -16
	add	rdx, rax
	add	rdx, 17
	je	.LBB62_32
	sub	rcx, rax
	mov	r8d, 16
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB62_32:
	mov	rdx, qword ptr [rbp + 144]
	test	rdx, rdx
	je	.LBB62_35
	lea	rax, [4*rdx + 19]
	and	rax, -16
	add	rdx, rax
	add	rdx, 17
	je	.LBB62_35
	sub	r15, rax
	mov	r8d, 16
	mov	rcx, r15
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB62_35:
	cmp	r14d, 1114112
	je	.LBB62_40
	lea	eax, [r14 - 97]
	cmp	eax, 26
	jb	.LBB62_38
	lea	eax, [r14 - 65]
	add	r14d, -38
	cmp	eax, 26
	mov	eax, 0
	cmovae	r14d, eax
	jmp	.LBB62_39
.LBB62_41:
	pcmpeqd	xmm6, xmm6
	mov	r9, qword ptr [rbp + 216]
	mov	rdx, qword ptr [rbp + 192]
	mov	dword ptr [rbp + 276], 0
.LBB62_42:
	mov	rcx, rdx
	jmp	.LBB62_43
	.p2align	4
.LBB62_113:
	mov	rdx, qword ptr [rbp + 184]
	mov	rcx, rdx
	cmp	edi, 1114112
	mov	r9, qword ptr [rbp + 40]
	jne	.LBB62_114
.LBB62_43:
	test	rcx, rcx
	je	.LBB62_118
	mov	rax, r9
	cmp	rcx, 3
	mov	edx, 3
	cmovb	rdx, rcx
	mov	r8, rcx
	sub	r8, rdx
	lea	rdx, [rdx + 2*rdx]
	mov	edx, edx
	lea	r9, [r9 + 8*rdx]
	cmp	rcx, 3
	mov	rcx, r8
	jb	.LBB62_43
	mov	qword ptr [rbp + 40], r9
	mov	qword ptr [rbp + 184], r8
	mov	rdx, qword ptr [rax + 8]
	mov	r14, qword ptr [rax + 32]
	mov	r12, qword ptr [rax + 40]
	mov	rdi, qword ptr [rax + 56]
	mov	rbx, qword ptr [rax + 64]
	mov	r8, qword ptr [rax + 16]
	add	r8, rdx
.Ltmp70:
	lea	rcx, [rbp - 32]
	call	_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E
.Ltmp71:
	add	r12, r14
.Ltmp72:
	lea	rcx, [rbp - 80]
	mov	rdx, r14
	mov	r8, r12
	call	_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E
.Ltmp73:
	add	rbx, rdi
.Ltmp74:
	lea	rcx, [rbp + 136]
	mov	rdx, rdi
	mov	r8, rbx
	call	_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E
.Ltmp75:
	mov	r12, qword ptr [rbp - 32]
	mov	rsi, qword ptr [rbp - 8]
	mov	rax, qword ptr [rbp - 80]
	mov	r13, qword ptr [rbp - 56]
	cmp	rsi, r13
	mov	rbx, r13
	cmovb	rbx, rsi
	mov	qword ptr [rbp + 200], r12
	mov	qword ptr [rbp + 208], rax
	cmova	r12, rax
	movdqa	xmm7, xmmword ptr [r12]
.Ltmp76:
	call	_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$51__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$17hed5e461344c1f9f9E
.Ltmp77:
	mov	rdi, rax
	cmp	byte ptr [rax + 16], 1
	jne	.LBB62_54
	mov	rax, qword ptr [rdi]
	mov	r11, qword ptr [rdi + 8]
.LBB62_56:
	cmp	rsi, r13
	lea	rcx, [rbp - 80]
	lea	rdx, [rbp - 32]
	cmova	rcx, rdx
	lea	r8, [rax + 1]
	mov	qword ptr [rdi], r8
	mov	rdx, qword ptr [rbp - 24]
	mov	r14, qword ptr [rbp - 72]
	mov	qword ptr [rbp + 48], r14
	mov	qword ptr [rbp + 256], rdx
	cmova	r14, rdx
	mov	r15, qword ptr [rbp + 208]
	cmova	r15, qword ptr [rbp + 200]
	or	r13, rsi
	pmovmskb	r13d, xmm7
	not	r13d
	lea	rdi, [r12 + 16]
	movups	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.0+16]
	movaps	xmmword ptr [rbp + 96], xmm0
	movdqu	xmm0, xmmword ptr [rip + anon.44ffa63e8e95c400711a21744c5ea708.0]
	movdqa	xmmword ptr [rbp + 80], xmm0
	mov	qword ptr [rbp + 112], rax
	mov	qword ptr [rbp + 120], r11
	je	.LBB62_57
	mov	rdx, qword ptr [rcx + 32]
	mov	rcx, qword ptr [rcx + 40]
	mov	r8, rdx
	movabs	rax, 8317987319222330741
	xor	r8, rax
	mov	rsi, rcx
	movabs	rax, 7237128888997146477
	xor	rsi, rax
	add	r8, rsi
	rol	rsi, 13
	movabs	rax, 7816392313619706465
	xor	rdx, rax
	xor	rsi, r8
	rol	r8, 32
	mov	qword ptr [rbp + 240], r8
	mov	qword ptr [rbp + 264], rdx
	lea	rax, [rsi + rdx]
	mov	qword ptr [rbp + 232], rax
	rol	rsi, 17
	movabs	rax, 8098989879002948979
	xor	rcx, rax
	mov	qword ptr [rbp + 224], rcx
	jmp	.LBB62_65
	.p2align	4
.LBB62_70:
	and	r9, r14
	movdqu	xmm1, xmmword ptr [r15 + r9]
	movdqa	xmm2, xmm1
	pcmpeqb	xmm2, xmm0
	pmovmskb	ecx, xmm2
	test	ecx, ecx
	je	.LBB62_73
.LBB62_71:
	rep		bsf	r8d, ecx
	add	r8, r9
	and	r8, r14
	shl	r8, 2
	mov	r10, r15
	sub	r10, r8
	cmp	edx, dword ptr [r10 - 4]
	je	.LBB62_75
	lea	r8d, [rcx - 1]
	and	r8w, cx
	mov	ecx, r8d
	jne	.LBB62_71
	.p2align	4
.LBB62_73:
	pcmpeqb	xmm1, xmm6
	pmovmskb	ecx, xmm1
	test	ecx, ecx
	jne	.LBB62_76
	add	r9, rax
	add	r9, 16
	add	rax, 16
	jmp	.LBB62_70
	.p2align	4
.LBB62_75:
.Ltmp80:
	lea	rcx, [rbp + 80]
	call	_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE
.Ltmp81:
.LBB62_76:
	dec	rbx
.LBB62_65:
	test	r13w, r13w
	jne	.LBB62_69
	test	rbx, rbx
	je	.LBB62_78
	.p2align	4
.LBB62_67:
	movdqa	xmm0, xmmword ptr [rdi]
	pmovmskb	r13d, xmm0
	add	r12, -64
	add	rdi, 16
	cmp	r13d, 65535
	je	.LBB62_67
	not	r13d
.LBB62_69:
	lea	eax, [r13 - 1]
	rep		bsf	ecx, r13d
	and	eax, r13d
	shl	ecx, 2
	mov	rdx, r12
	sub	rdx, rcx
	mov	edx, dword ptr [rdx - 4]
	movabs	rcx, 288230376151711744
	lea	r9, [rdx + rcx]
	mov	rcx, rdx
	xor	rcx, qword ptr [rbp + 224]
	mov	r8, qword ptr [rbp + 264]
	lea	r10, [rcx + r8]
	mov	r8, qword ptr [rbp + 232]
	add	r8, rcx
	rol	rcx, 16
	xor	rcx, r10
	mov	r10, qword ptr [rbp + 240]
	add	r10, rcx
	rol	rcx, 21
	xor	rcx, r10
	xor	r9, r10
	mov	r10, r8
	xor	r10, rsi
	rol	r8, 32
	xor	r8, 255
	add	r9, r10
	rol	r10, 13
	add	r8, rcx
	xor	r10, r9
	rol	rcx, 16
	xor	rcx, r8
	rol	r9, 32
	add	r8, r10
	add	r9, rcx
	rol	r10, 17
	xor	r10, r8
	rol	rcx, 21
	rol	r8, 32
	xor	rcx, r9
	add	r9, r10
	rol	r10, 13
	add	r8, rcx
	xor	r10, r9
	rol	rcx, 16
	xor	rcx, r8
	rol	r9, 32
	add	r8, r10
	add	r9, rcx
	rol	r10, 17
	xor	r10, r8
	rol	rcx, 21
	rol	r8, 32
	xor	rcx, r9
	add	r9, r10
	rol	r10, 13
	add	r8, rcx
	xor	r10, r9
	rol	rcx, 16
	xor	rcx, r8
	add	r8, r10
	rol	r10, 17
	mov	r9, r8
	rol	r9, 32
	xor	r9, r10
	rol	rcx, 21
	xor	r9, rcx
	mov	r13d, eax
	xor	r9, r8
	mov	rax, r9
	shr	rax, 57
	movd	xmm0, eax
	punpcklbw	xmm0, xmm0
	pshuflw	xmm0, xmm0, 0
	pshufd	xmm0, xmm0, 68
	xor	eax, eax
	jmp	.LBB62_70
	.p2align	4
.LBB62_78:
	mov	rcx, qword ptr [rbp + 80]
	mov	r9, qword ptr [rbp + 88]
	mov	r14, qword ptr [rbp + 104]
	mov	r8, qword ptr [rbp + 112]
	mov	r11, qword ptr [rbp + 120]
	mov	rsi, qword ptr [rbp + 256]
	jmp	.LBB62_79
.LBB62_57:
	mov	r8, rax
	mov	rsi, qword ptr [rbp + 256]
	jmp	.LBB62_58
	.p2align	4
.LBB62_63:
	lea	ecx, [r13 - 1]
	and	ecx, r13d
	dec	rbx
	mov	r13d, ecx
.LBB62_58:
	test	r13w, r13w
	jne	.LBB62_63
	test	rbx, rbx
	je	.LBB62_60
	.p2align	4
.LBB62_61:
	movdqa	xmm0, xmmword ptr [rdi]
	pmovmskb	r13d, xmm0
	add	rdi, 16
	cmp	r13d, 65535
	je	.LBB62_61
	not	r13d
	jmp	.LBB62_63
.LBB62_60:
	xor	r14d, r14d
	xor	r9d, r9d
	lea	rcx, [rip + alloc_d0776666182ad032bd1011cf266e2f3a]
.LBB62_79:
	mov	r10, qword ptr [rbp + 160]
	cmp	r14, r10
	mov	rax, r10
	cmovb	rax, r14
	mov	edi, 1114112
	test	rax, rax
	je	.LBB62_101
	mov	r13, qword ptr [rbp + 136]
	cmp	r14, r10
	ja	.LBB62_81
	mov	r8, qword ptr [rbp + 168]
	mov	rdx, qword ptr [rbp + 176]
	mov	r11, qword ptr [rbp + 144]
	mov	qword ptr [rbp + 264], r11
	mov	r11, rcx
	jmp	.LBB62_83
.LBB62_81:
	mov	rdx, r11
	mov	r11, r13
	mov	qword ptr [rbp + 264], r9
	mov	r13, rcx
.LBB62_83:
	lea	rbx, [r11 + 16]
	movdqa	xmm0, xmmword ptr [r11]
	pmovmskb	r15d, xmm0
	or	r10, r14
	je	.LBB62_84
	mov	qword ptr [rbp + 72], rcx
	mov	rsi, r8
	movabs	rcx, 8317987319222330741
	xor	rsi, rcx
	mov	r14, rdx
	movabs	rcx, 7237128888997146477
	xor	r14, rcx
	movabs	rcx, 7816392313619706465
	xor	r8, rcx
	add	rsi, r14
	rol	r14, 13
	xor	r14, rsi
	rol	rsi, 32
	lea	rdi, [r14 + r8]
	rol	r14, 17
	not	r15d
	movabs	rcx, 8098989879002948979
	xor	rdx, rcx
	mov	qword ptr [rbp + 224], rdx
	mov	qword ptr [rbp + 240], r13
	mov	qword ptr [rbp + 56], rdi
.LBB62_87:
	test	r15w, r15w
	jne	.LBB62_90
	.p2align	4
.LBB62_88:
	movdqa	xmm0, xmmword ptr [rbx]
	pmovmskb	r15d, xmm0
	add	r11, -64
	add	rbx, 16
	cmp	r15d, 65535
	je	.LBB62_88
	not	r15d
.LBB62_90:
	rep		bsf	ecx, r15d
	shl	ecx, 2
	mov	r10, r11
	sub	r10, rcx
	mov	r13d, dword ptr [r10 - 4]
	movabs	rcx, 288230376151711744
	lea	r10, [rcx + r13]
	mov	qword ptr [rbp + 232], r13
	xor	r13, qword ptr [rbp + 224]
	lea	r12, [r8 + r13]
	lea	rcx, [rdi + r13]
	rol	r13, 16
	xor	r13, r12
	lea	r12, [rsi + r13]
	rol	r13, 21
	xor	r13, r12
	xor	r10, r12
	mov	r12, rcx
	rol	rcx, 32
	xor	r12, r14
	xor	rcx, 255
	add	r10, r12
	add	rcx, r13
	rol	r12, 13
	rol	r13, 16
	xor	r12, r10
	xor	r13, rcx
	rol	r10, 32
	add	rcx, r12
	add	r10, r13
	rol	r12, 17
	xor	r12, rcx
	rol	r13, 21
	xor	r13, r10
	rol	rcx, 32
	add	r10, r12
	add	rcx, r13
	rol	r12, 13
	rol	r13, 16
	xor	r12, r10
	xor	r13, rcx
	rol	r10, 32
	add	rcx, r12
	add	r10, r13
	rol	r12, 17
	xor	r12, rcx
	rol	r13, 21
	xor	r13, r10
	add	r10, r12
	rol	r12, 13
	xor	r12, r10
	lea	edi, [r15 - 1]
	rol	rcx, 32
	add	rcx, r13
	rol	r13, 16
	xor	r13, rcx
	add	rcx, r12
	rol	r12, 17
	rol	r13, 21
	and	edi, r15d
	mov	r10, rcx
	rol	r10, 32
	xor	r10, r12
	xor	r10, r13
	mov	r15d, edi
	dec	rax
	xor	r10, rcx
	mov	rcx, r10
	shr	rcx, 57
	movd	xmm0, ecx
	punpcklbw	xmm0, xmm0
	pshuflw	xmm0, xmm0, 0
	pshufd	xmm0, xmm0, 68
	xor	edx, edx
	mov	r13, qword ptr [rbp + 240]
	mov	rcx, qword ptr [rbp + 264]
.LBB62_91:
	and	r10, rcx
	movdqu	xmm1, xmmword ptr [r13 + r10]
	movdqa	xmm2, xmm1
	pcmpeqb	xmm2, xmm0
	pmovmskb	r12d, xmm2
	test	r12d, r12d
	je	.LBB62_96
	mov	qword ptr [rbp + 64], r14
.LBB62_93:
	mov	r14, rax
	mov	rax, r8
	mov	r8, rsi
	mov	rsi, rdx
	rep		bsf	edi, r12d
	add	rdi, r10
	and	rdi, rcx
	shl	rdi, 2
	mov	rdx, rcx
	mov	rcx, r13
	sub	r13, rdi
	mov	rdi, qword ptr [rbp + 232]
	cmp	edi, dword ptr [r13 - 4]
	je	.LBB62_94
	lea	edi, [r12 - 1]
	and	di, r12w
	mov	r12d, edi
	mov	r13, rcx
	mov	rcx, rdx
	mov	rdx, rsi
	mov	rsi, r8
	mov	r8, rax
	mov	rax, r14
	mov	r14, qword ptr [rbp + 64]
	jne	.LBB62_93
	.p2align	4
.LBB62_96:
	pcmpeqb	xmm1, xmm6
	pmovmskb	edi, xmm1
	test	edi, edi
	jne	.LBB62_98
	add	r10, rdx
	add	r10, 16
	add	rdx, 16
	jmp	.LBB62_91
	.p2align	4
.LBB62_98:
	test	rax, rax
	mov	rdi, qword ptr [rbp + 56]
	jne	.LBB62_87
	mov	rsi, qword ptr [rbp + 256]
	mov	edi, 1114112
	jmp	.LBB62_100
.LBB62_94:
	mov	rsi, qword ptr [rbp + 256]
.LBB62_100:
	mov	rcx, qword ptr [rbp + 72]
	jmp	.LBB62_101
.LBB62_84:
	cmp	r15w, -1
	jne	.LBB62_101
	.p2align	4
.LBB62_85:
	movdqa	xmm0, xmmword ptr [rbx]
	add	rbx, 16
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	je	.LBB62_85
	.p2align	4
.LBB62_101:
	test	r9, r9
	je	.LBB62_104
	lea	rax, [4*r9 + 19]
	and	rax, -16
	add	r9, rax
	add	r9, 17
	je	.LBB62_104
	sub	rcx, rax
	mov	r8d, 16
	mov	rdx, r9
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB62_104:
	mov	rdx, qword ptr [rbp + 144]
	test	rdx, rdx
	je	.LBB62_107
	lea	rax, [4*rdx + 19]
	and	rax, -16
	add	rdx, rax
	add	rdx, 17
	je	.LBB62_107
	mov	rcx, qword ptr [rbp + 136]
	sub	rcx, rax
	mov	r8d, 16
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB62_107:
	mov	rdx, qword ptr [rbp + 48]
	test	rdx, rdx
	je	.LBB62_110
	lea	rax, [4*rdx + 19]
	and	rax, -16
	add	rdx, rax
	add	rdx, 17
	je	.LBB62_110
	mov	rcx, qword ptr [rbp + 208]
	sub	rcx, rax
	mov	r8d, 16
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB62_110:
	test	rsi, rsi
	je	.LBB62_113
	lea	rax, [4*rsi + 19]
	and	rax, -16
	add	rsi, rax
	add	rsi, 17
	je	.LBB62_113
	mov	rcx, qword ptr [rbp + 200]
	sub	rcx, rax
	mov	r8d, 16
	mov	rdx, rsi
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB62_113
.LBB62_54:
.Ltmp78:
	call	_ZN3std3sys6random19hashmap_random_keys17hc3f03c6d163b2da2E
.Ltmp79:
	mov	r11, rdx
	mov	qword ptr [rdi + 8], rdx
	mov	byte ptr [rdi + 16], 1
	jmp	.LBB62_56
	.p2align	4
.LBB62_114:
	lea	eax, [rdi - 97]
	cmp	eax, 26
	jae	.LBB62_115
	add	edi, -96
	jmp	.LBB62_117
.LBB62_115:
	lea	eax, [rdi - 65]
	add	edi, -38
	cmp	eax, 26
	mov	eax, 0
	cmovae	edi, eax
.LBB62_117:
	add	edi, dword ptr [rbp + 276]
	mov	dword ptr [rbp + 276], edi
	jmp	.LBB62_42
.LBB62_118:
	cmp	qword ptr [rbp + 192], 0
	je	.LBB62_123
	mov	rax, qword ptr [rbp + 216]
	lea	rsi, [rax + 8]
	jmp	.LBB62_120
	.p2align	4
.LBB62_122:
	add	rsi, 24
	dec	qword ptr [rbp + 192]
	je	.LBB62_123
.LBB62_120:
	mov	rdx, qword ptr [rsi - 8]
	test	rdx, rdx
	je	.LBB62_122
	mov	rcx, qword ptr [rsi]
	mov	r8d, 1
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	jmp	.LBB62_122
.LBB62_123:
	mov	rax, qword ptr [rbp + 16]
	test	rax, rax
	je	.LBB62_125
	shl	rax, 3
	lea	rdx, [rax + 2*rax]
	mov	r8d, 8
	mov	rcx, qword ptr [rbp + 216]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB62_125:
	mov	eax, dword ptr [rbp + 252]
	mov	edx, dword ptr [rbp + 276]
	movaps	xmm6, xmmword ptr [rbp + 288]
	movaps	xmm7, xmmword ptr [rbp + 304]
	.seh_startepilogue
	add	rsp, 456
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.LBB62_18:
.Ltmp64:
	lea	rax, [rip + alloc_90427cacc85724e4d3b32dbfd394b367]
	mov	qword ptr [rsp + 32], rax
	mov	rcx, rdx
	mov	rdx, r14
	xor	r8d, r8d
	call	_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE
.Ltmp65:
	ud2
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.def	"?dtor$20@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$20@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA":
.seh_proc "?dtor$20@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"
.LBB62_20:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rdx + 128]
	movdqa	xmmword ptr [rsp + 32], xmm7
	.seh_savexmm xmm7, 32
	movdqa	xmmword ptr [rsp + 48], xmm6
	.seh_savexmm xmm6, 48
	.seh_endprologue
	mov	rcx, qword ptr [rbp + 136]
	mov	rdx, qword ptr [rbp + 144]
	call	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E
	movaps	xmm6, xmmword ptr [rsp + 48]
	movaps	xmm7, xmmword ptr [rsp + 32]
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.def	"?dtor$51@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$51@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA":
.seh_proc "?dtor$51@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"
.LBB62_51:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rdx + 128]
	movdqa	xmmword ptr [rsp + 32], xmm7
	.seh_savexmm xmm7, 32
	movdqa	xmmword ptr [rsp + 48], xmm6
	.seh_savexmm xmm6, 48
	.seh_endprologue
	mov	rcx, qword ptr [rbp - 32]
	mov	rdx, qword ptr [rbp - 24]
	call	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E
	movdqa	xmm6, xmmword ptr [rsp + 48]
	movdqa	xmm7, xmmword ptr [rsp + 32]
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.def	"?dtor$52@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$52@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA":
.seh_proc "?dtor$52@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"
.LBB62_52:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rdx + 128]
	movdqa	xmmword ptr [rsp + 32], xmm7
	.seh_savexmm xmm7, 32
	movdqa	xmmword ptr [rsp + 48], xmm6
	.seh_savexmm xmm6, 48
	.seh_endprologue
	mov	rcx, qword ptr [rbp - 80]
	mov	rdx, qword ptr [rbp - 72]
	call	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E
	movdqa	xmm6, xmmword ptr [rsp + 48]
	movdqa	xmm7, xmmword ptr [rsp + 32]
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.def	"?dtor$53@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$53@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA":
.seh_proc "?dtor$53@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"
.LBB62_53:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rdx + 128]
	movdqa	xmmword ptr [rsp + 32], xmm7
	.seh_savexmm xmm7, 32
	movdqa	xmmword ptr [rsp + 48], xmm6
	.seh_savexmm xmm6, 48
	.seh_endprologue
	mov	rcx, qword ptr [rbp + 136]
	mov	rdx, qword ptr [rbp + 144]
	call	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E
	movdqa	xmm6, xmmword ptr [rsp + 48]
	movdqa	xmm7, xmmword ptr [rsp + 32]
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.def	"?dtor$77@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$77@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA":
.seh_proc "?dtor$77@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"
.LBB62_77:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rdx + 128]
	movdqa	xmmword ptr [rsp + 32], xmm7
	.seh_savexmm xmm7, 32
	movdqa	xmmword ptr [rsp + 48], xmm6
	.seh_savexmm xmm6, 48
	.seh_endprologue
	mov	rcx, qword ptr [rbp + 80]
	mov	rdx, qword ptr [rbp + 88]
	call	_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E
	movdqa	xmm6, xmmword ptr [rsp + 48]
	movdqa	xmm7, xmmword ptr [rsp + 32]
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.def	"?dtor$126@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$126@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA":
.seh_proc "?dtor$126@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"
.LBB62_126:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 72
	.seh_stackalloc 72
	lea	rbp, [rdx + 128]
	movdqa	xmmword ptr [rsp + 32], xmm7
	.seh_savexmm xmm7, 32
	movdqa	xmmword ptr [rsp + 48], xmm6
	.seh_savexmm xmm6, 48
	.seh_endprologue
	lea	rcx, [rbp + 16]
	call	_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E
	movaps	xmm6, xmmword ptr [rsp + 48]
	movaps	xmm7, xmmword ptr [rsp + 32]
	.seh_startepilogue
	add	rsp, 72
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end19:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,19
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE:
	.long	429065506
	.long	6
	.long	$stateUnwindMap$_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@IMGREL
	.long	0
	.long	0
	.long	11
	.long	$ip2state$_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@IMGREL
	.long	408
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE:
	.long	-1
	.long	"?dtor$126@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"@IMGREL
	.long	0
	.long	"?dtor$51@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"@IMGREL
	.long	1
	.long	"?dtor$52@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"@IMGREL
	.long	2
	.long	"?dtor$53@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"@IMGREL
	.long	3
	.long	"?dtor$77@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"@IMGREL
	.long	0
	.long	"?dtor$20@?0?_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE:
	.long	.Lfunc_begin19@IMGREL
	.long	-1
	.long	.Ltmp66@IMGREL+1
	.long	0
	.long	.Ltmp68@IMGREL+1
	.long	5
	.long	.Ltmp70@IMGREL+1
	.long	0
	.long	.Ltmp72@IMGREL+1
	.long	1
	.long	.Ltmp74@IMGREL+1
	.long	2
	.long	.Ltmp76@IMGREL+1
	.long	3
	.long	.Ltmp80@IMGREL+1
	.long	4
	.long	.Ltmp78@IMGREL+1
	.long	3
	.long	.Ltmp64@IMGREL+1
	.long	0
	.long	.Ltmp65@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE,unique,62

	.def	_ZN7aoc20226solver7run_day17h0d314465522a10edE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,63
	.globl	_ZN7aoc20226solver7run_day17h0d314465522a10edE
	.p2align	4
_ZN7aoc20226solver7run_day17h0d314465522a10edE:
.Lfunc_begin20:
.seh_proc _ZN7aoc20226solver7run_day17h0d314465522a10edE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 96
	.seh_stackalloc 96
	lea	rbp, [rsp + 96]
	.seh_setframe rbp, 96
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	mov	rsi, rcx
	mov	qword ptr [rbp - 56], rdx
	cmp	rdx, 1
	je	.LBB63_4
	cmp	rdx, 2
	je	.LBB63_15
	cmp	rdx, 3
	jne	.LBB63_3
	mov	rcx, r8
	mov	rdx, r9
	call	_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE
	mov	edi, edx
	lea	rdx, [rbp - 48]
	mov	r8d, 10
	mov	ecx, eax
	call	_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h68bd8f419e61f018E
	mov	rbx, rax
	mov	r14, rdx
	test	rdx, rdx
	je	.LBB63_23
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, r14
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	jne	.LBB63_25
	jmp	.LBB63_38
.LBB63_15:
	mov	rcx, r8
	mov	rdx, r9
	call	_ZN7aoc20226solver5day025solve17h15f04a137efee39bE
	mov	rdi, rdx
	lea	rdx, [rbp - 48]
	mov	r8d, 20
	mov	rcx, rax
	call	_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE
	mov	rbx, rax
	mov	r14, rdx
	test	rdx, rdx
	je	.LBB63_16
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, r14
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	jne	.LBB63_18
	jmp	.LBB63_38
.LBB63_4:
	mov	rcx, r8
	mov	rdx, r9
	call	_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E
	mov	rdi, rdx
	lea	rdx, [rbp - 48]
	mov	r8d, 20
	mov	rcx, rax
	call	_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE
	mov	rbx, rax
	mov	r14, rdx
	test	rdx, rdx
	je	.LBB63_5
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, r14
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	jne	.LBB63_7
.LBB63_38:
	mov	ecx, 1
	mov	rdx, r14
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.LBB63_16:
	mov	eax, 1
.LBB63_18:
	mov	qword ptr [rbp - 24], rax
	mov	rcx, rax
	mov	rdx, rbx
	mov	qword ptr [rbp - 16], r14
	mov	r8, r14
	call	memcpy
.Ltmp86:
	lea	rdx, [rbp - 48]
	mov	r8d, 20
	mov	rcx, rdi
	call	_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE
.Ltmp87:
	mov	rbx, rax
	mov	rdi, rdx
	test	rdx, rdx
	je	.LBB63_9
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, rdi
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	mov	r14, rax
	test	rax, rax
	jne	.LBB63_10
.Ltmp88:
	mov	ecx, 1
	mov	rdx, rdi
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.Ltmp89:
	jmp	.LBB63_14
.LBB63_23:
	mov	eax, 1
.LBB63_25:
	mov	qword ptr [rbp - 24], rax
	mov	rcx, rax
	mov	rdx, rbx
	mov	qword ptr [rbp - 16], r14
	mov	r8, r14
	call	memcpy
.Ltmp82:
	lea	rdx, [rbp - 48]
	mov	r8d, 10
	mov	ecx, edi
	call	_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h68bd8f419e61f018E
.Ltmp83:
	mov	rbx, rax
	mov	rdi, rdx
	test	rdx, rdx
	je	.LBB63_9
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, rdi
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	mov	r14, rax
	test	rax, rax
	jne	.LBB63_10
.Ltmp84:
	mov	ecx, 1
	mov	rdx, rdi
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.Ltmp85:
	jmp	.LBB63_14
.LBB63_5:
	mov	eax, 1
.LBB63_7:
	mov	qword ptr [rbp - 24], rax
	mov	rcx, rax
	mov	rdx, rbx
	mov	qword ptr [rbp - 16], r14
	mov	r8, r14
	call	memcpy
.Ltmp90:
	lea	rdx, [rbp - 48]
	mov	r8d, 20
	mov	rcx, rdi
	call	_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE
.Ltmp91:
	mov	rbx, rax
	mov	rdi, rdx
	test	rdx, rdx
	je	.LBB63_9
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 1
	mov	rcx, rdi
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	mov	r14, rax
	test	rax, rax
	jne	.LBB63_10
.Ltmp92:
	mov	ecx, 1
	mov	rdx, rdi
	call	_ZN5alloc7raw_vec12handle_error17h8738464738de9066E
.Ltmp93:
.LBB63_14:
	ud2
.LBB63_9:
	mov	r14d, 1
.LBB63_10:
	mov	rcx, r14
	mov	rdx, rbx
	mov	r8, rdi
	call	memcpy
	mov	rax, qword ptr [rbp - 16]
	mov	qword ptr [rsi], rax
	mov	rcx, qword ptr [rbp - 24]
	mov	qword ptr [rsi + 8], rcx
	mov	qword ptr [rsi + 16], rax
	mov	qword ptr [rsi + 24], rdi
	mov	qword ptr [rsi + 32], r14
	mov	qword ptr [rsi + 40], rdi
.LBB63_11:
	mov	rax, rsi
	.seh_startepilogue
	add	rsp, 96
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
.LBB63_3:
	lea	rax, [rbp - 56]
	mov	qword ptr [rbp - 48], rax
	lea	rax, [rip + _ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hbcf79f68ff2d61d8E]
	mov	qword ptr [rbp - 40], rax
	lea	rcx, [rbp - 48]
	call	_ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE
	mov	qword ptr [rsi + 8], rax
	movabs	rax, -9223372036854775808
	mov	qword ptr [rsi], rax
	jmp	.LBB63_11
	.seh_handlerdata
	.long	$cppxdata$_ZN7aoc20226solver7run_day17h0d314465522a10edE@IMGREL
	.section	.text,"xr",one_only,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,63
	.seh_endproc
	.def	"?dtor$29@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$29@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA":
.seh_proc "?dtor$29@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA"
.LBB63_29:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 96]
	.seh_endprologue
	cmp	qword ptr [rbp - 16], 0
	je	.LBB63_31
	mov	r8d, 1
	mov	rcx, qword ptr [rbp - 24]
	mov	rdx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB63_31:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,63
	.seh_endproc
	.def	"?dtor$32@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$32@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA":
.seh_proc "?dtor$32@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA"
.LBB63_32:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 96]
	.seh_endprologue
	cmp	qword ptr [rbp - 16], 0
	je	.LBB63_34
	mov	r8d, 1
	mov	rcx, qword ptr [rbp - 24]
	mov	rdx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB63_34:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,63
	.seh_endproc
	.def	"?dtor$35@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$35@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA":
.seh_proc "?dtor$35@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA"
.LBB63_35:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 96]
	.seh_endprologue
	cmp	qword ptr [rbp - 16], 0
	je	.LBB63_37
	mov	r8d, 1
	mov	rcx, qword ptr [rbp - 24]
	mov	rdx, qword ptr [rbp - 16]
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
.LBB63_37:
	nop
	.seh_startepilogue
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	pop	r14
	pop	rbp
	.seh_endepilogue
	ret
.Lfunc_end20:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,63
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,20
	.p2align	2, 0x0
$cppxdata$_ZN7aoc20226solver7run_day17h0d314465522a10edE:
	.long	429065506
	.long	3
	.long	$stateUnwindMap$_ZN7aoc20226solver7run_day17h0d314465522a10edE@IMGREL
	.long	0
	.long	0
	.long	7
	.long	$ip2state$_ZN7aoc20226solver7run_day17h0d314465522a10edE@IMGREL
	.long	88
	.long	0
	.long	1
$stateUnwindMap$_ZN7aoc20226solver7run_day17h0d314465522a10edE:
	.long	-1
	.long	"?dtor$29@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA"@IMGREL
	.long	-1
	.long	"?dtor$32@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA"@IMGREL
	.long	-1
	.long	"?dtor$35@?0?_ZN7aoc20226solver7run_day17h0d314465522a10edE@4HA"@IMGREL
$ip2state$_ZN7aoc20226solver7run_day17h0d314465522a10edE:
	.long	.Lfunc_begin20@IMGREL
	.long	-1
	.long	.Ltmp86@IMGREL+1
	.long	1
	.long	.Ltmp89@IMGREL+1
	.long	-1
	.long	.Ltmp82@IMGREL+1
	.long	2
	.long	.Ltmp85@IMGREL+1
	.long	-1
	.long	.Ltmp90@IMGREL+1
	.long	0
	.long	.Ltmp93@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN7aoc20226solver7run_day17h0d314465522a10edE,unique,63

	.def	_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E,unique,64
	.p2align	4
_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E:
.seh_proc _ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	cmp	byte ptr [rcx + 65], 0
	jne	.LBB64_1
	mov	rbx, rcx
	mov	rsi, qword ptr [rcx + 16]
	mov	r13, qword ptr [rcx + 24]
	mov	r12, qword ptr [rcx + 40]
	cmp	r12, r13
	seta	al
	mov	rdi, qword ptr [rcx + 32]
	cmp	r12, rdi
	setb	cl
	or	cl, al
	jne	.LBB64_30
	movzx	r14d, byte ptr [rbx + 56]
	movzx	ebp, byte ptr [rbx + r14 + 47]
	cmp	r14, 5
	jae	.LBB64_17
	lea	r15, [rbx + 48]
	jmp	.LBB64_5
	.p2align	4
.LBB64_16:
	cmp	r12, rdi
	jb	.LBB64_30
.LBB64_5:
	mov	r8, r12
	sub	r8, rdi
	lea	rax, [rsi + rdi]
	cmp	r8, 16
	jae	.LBB64_6
	xor	edx, edx
	test	r8, r8
	je	.LBB64_12
	.p2align	4
.LBB64_8:
	cmp	byte ptr [rax + rdx], bpl
	je	.LBB64_9
	inc	rdx
	cmp	r8, rdx
	jne	.LBB64_8
	mov	rdx, r8
.LBB64_12:
	xor	eax, eax
	test	al, 1
	jne	.LBB64_14
	jmp	.LBB64_29
	.p2align	4
.LBB64_6:
	mov	ecx, ebp
	mov	rdx, rax
	call	_ZN4core5slice6memchr14memchr_aligned17he27ef990a57e50bcE
	test	al, 1
	jne	.LBB64_14
	jmp	.LBB64_29
.LBB64_9:
	mov	eax, 1
	test	al, 1
	je	.LBB64_29
	.p2align	4
.LBB64_14:
	add	rdi, rdx
	inc	rdi
	mov	qword ptr [rbx + 32], rdi
	mov	rcx, rdi
	sub	rcx, r14
	setb	al
	cmp	rdi, r13
	seta	dl
	or	dl, al
	jne	.LBB64_16
	add	rcx, rsi
	mov	rdx, r15
	mov	r8, r14
	call	memcmp
	test	eax, eax
	jne	.LBB64_16
	mov	rax, qword ptr [rbx]
	mov	qword ptr [rbx], rdi
	sub	rdi, rax
	jmp	.LBB64_32
.LBB64_26:
	add	rdi, rdx
	inc	rdi
	mov	qword ptr [rbx + 32], rdi
	cmp	rdi, r14
	setb	al
	cmp	rdi, r13
	seta	cl
	or	cl, al
	je	.LBB64_28
	cmp	r12, rdi
	jb	.LBB64_30
.LBB64_17:
	mov	r8, r12
	sub	r8, rdi
	lea	rax, [rsi + rdi]
	cmp	r8, 15
	ja	.LBB64_24
	xor	edx, edx
	test	r8, r8
	je	.LBB64_19
.LBB64_20:
	cmp	byte ptr [rax + rdx], bpl
	je	.LBB64_21
	inc	rdx
	cmp	r8, rdx
	jne	.LBB64_20
	mov	rdx, r8
	xor	eax, eax
	jmp	.LBB64_25
.LBB64_24:
	mov	ecx, ebp
	mov	rdx, rax
	call	_ZN4core5slice6memchr14memchr_aligned17he27ef990a57e50bcE
	jmp	.LBB64_25
.LBB64_19:
	xor	eax, eax
	jmp	.LBB64_25
.LBB64_21:
	mov	eax, 1
.LBB64_25:
	test	al, 1
	jne	.LBB64_26
.LBB64_29:
	mov	qword ptr [rbx + 32], r12
.LBB64_30:
	mov	byte ptr [rbx + 65], 1
	mov	rax, qword ptr [rbx]
	mov	rdi, qword ptr [rbx + 8]
	sub	rdi, rax
	setne	cl
	or	cl, byte ptr [rbx + 64]
	cmp	cl, 1
	jne	.LBB64_1
.LBB64_32:
	add	rsi, rax
	test	rdi, rdi
	je	.LBB64_38
	cmp	byte ptr [rsi + rdi - 1], 10
	jne	.LBB64_38
	mov	rax, rdi
	dec	rax
	je	.LBB64_35
	xor	ecx, ecx
	cmp	byte ptr [rsi + rdi - 2], 13
	lea	rdi, [rdi - 2]
	cmove	rcx, rsi
	jmp	.LBB64_37
.LBB64_1:
	xor	esi, esi
.LBB64_38:
	mov	rax, rsi
	mov	rdx, rdi
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
.LBB64_35:
	mov	rdi, -1
	xor	ecx, ecx
.LBB64_37:
	test	rcx, rcx
	cmove	rdi, rax
	cmovne	rsi, rcx
	jmp	.LBB64_38
.LBB64_28:
	lea	r9, [rip + alloc_aa47418bb16f08c24f7eacc7bfd02189]
	mov	r8d, 4
	xor	ecx, ecx
	mov	rdx, r14
	call	_ZN4core5slice5index16slice_index_fail17h30ecc7bdca4bc32bE
	int3
	.seh_endproc

	.def	_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE;
	.scl	3;
	.type	32;
	.endef
	.globl	__xmm@ffffffffffffffff0000000000000000
	.section	.rdata,"dr",discard,__xmm@ffffffffffffffff0000000000000000
	.p2align	4, 0x0
__xmm@ffffffffffffffff0000000000000000:
	.zero	8
	.quad	-1
	.section	.text,"xr",one_only,_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE,unique,65
	.p2align	4
_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE:
.seh_proc _ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	r9, qword ptr [rcx + 32]
	mov	rsi, qword ptr [rcx + 40]
	movabs	r10, 8317987319222330741
	xor	r10, r9
	movabs	r8, 7237128888997146477
	xor	r8, rsi
	movabs	rax, 7816392313619706465
	xor	rax, r9
	mov	edi, edx
	movabs	r11, 288230376151711744
	or	r11, rdi
	movabs	r9, 8098989879002948979
	xor	r9, rdi
	xor	r9, rsi
	add	r10, r8
	rol	r8, 13
	add	rax, r9
	xor	r8, r10
	rol	r9, 16
	xor	r9, rax
	rol	r10, 32
	add	rax, r8
	add	r10, r9
	rol	r8, 17
	xor	r8, rax
	rol	r9, 21
	rol	rax, 32
	xor	r9, r10
	xor	r10, r11
	xor	rax, 255
	add	r10, r8
	add	rax, r9
	rol	r8, 13
	xor	r8, r10
	rol	r9, 16
	rol	r10, 32
	xor	r9, rax
	add	rax, r8
	rol	r8, 17
	add	r10, r9
	xor	r8, rax
	rol	r9, 21
	xor	r9, r10
	rol	rax, 32
	add	r10, r8
	add	rax, r9
	rol	r8, 13
	xor	r8, r10
	rol	r9, 16
	rol	r10, 32
	xor	r9, rax
	add	rax, r8
	rol	r8, 17
	add	r10, r9
	xor	r8, rax
	rol	r9, 21
	xor	r9, r10
	rol	rax, 32
	add	r10, r8
	add	rax, r9
	rol	r8, 13
	xor	r8, r10
	rol	r9, 16
	xor	r9, rax
	add	rax, r8
	rol	r8, 17
	rol	r9, 21
	xor	r9, r8
	mov	rdi, rax
	rol	rdi, 32
	xor	rdi, r9
	xor	rdi, rax
	cmp	qword ptr [rcx + 16], 0
	je	.LBB65_1
.LBB65_2:
	mov	rax, qword ptr [rcx]
	mov	r9, qword ptr [rcx + 8]
	mov	r8, rdi
	shr	r8, 57
	movd	xmm0, r8d
	punpcklbw	xmm0, xmm0
	pshuflw	xmm0, xmm0, 0
	pshufd	xmm0, xmm0, 68
	xor	esi, esi
	pcmpeqd	xmm1, xmm1
	xor	r11d, r11d
.LBB65_3:
	and	rdi, r9
	movdqu	xmm2, xmmword ptr [rax + rdi]
	movdqa	xmm3, xmm2
	pcmpeqb	xmm3, xmm0
	pmovmskb	ebx, xmm3
	test	ebx, ebx
	je	.LBB65_6
.LBB65_4:
	rep		bsf	r14d, ebx
	add	r14, rdi
	and	r14, r9
	shl	r14, 2
	mov	r15, rax
	sub	r15, r14
	cmp	edx, dword ptr [r15 - 4]
	je	.LBB65_16
	lea	ebp, [rbx - 1]
	and	bp, bx
	mov	ebx, ebp
	jne	.LBB65_4
	.p2align	4
.LBB65_6:
	cmp	rsi, 1
	je	.LBB65_10
	pmovmskb	r10d, xmm2
	test	r10d, r10d
	je	.LBB65_8
	rep		bsf	r10d, r10d
	add	r10, rdi
	and	r10, r9
.LBB65_10:
	pcmpeqb	xmm2, xmm1
	pmovmskb	esi, xmm2
	test	esi, esi
	jne	.LBB65_13
	mov	esi, 1
	jmp	.LBB65_12
.LBB65_8:
	xor	esi, esi
.LBB65_12:
	add	rdi, r11
	add	rdi, 16
	add	r11, 16
	jmp	.LBB65_3
.LBB65_13:
	movzx	r11d, byte ptr [rax + r10]
	test	r11b, r11b
	jns	.LBB65_14
.LBB65_15:
	and	r11b, 1
	lea	rsi, [r10 - 16]
	and	rsi, r9
	mov	byte ptr [rax + r10], r8b
	mov	byte ptr [rax + rsi + 16], r8b
	movdqu	xmm0, xmmword ptr [rcx + 16]
	movzx	r8d, r11b
	movd	xmm1, r8d
	shufpd	xmm1, xmmword ptr [rip + __xmm@ffffffffffffffff0000000000000000], 2
	psubq	xmm0, xmm1
	movdqu	xmmword ptr [rcx + 16], xmm0
	shl	r10, 2
	neg	r10
	mov	dword ptr [rax + r10 - 4], edx
.LBB65_16:
	.seh_startepilogue
	add	rsp, 40
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
.LBB65_1:
	lea	r8, [rcx + 32]
	mov	ebx, edx
	mov	edx, 1
	mov	rsi, rcx
	mov	r9b, 1
	call	_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E
	mov	rcx, rsi
	mov	edx, ebx
	jmp	.LBB65_2
.LBB65_14:
	movdqa	xmm0, xmmword ptr [rax]
	pmovmskb	r10d, xmm0
	rep		bsf	r10d, r10d
	movzx	r11d, byte ptr [rax + r10]
	jmp	.LBB65_15
	.seh_endproc

	.def	_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E;
	.scl	2;
	.type	32;
	.endef
	.globl	__xmm@80808080808080808080808080808080
	.section	.rdata,"dr",discard,__xmm@80808080808080808080808080808080
	.p2align	4, 0x0
__xmm@80808080808080808080808080808080:
	.zero	16,128
	.section	.text$unlikely,"xr",one_only,_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E,unique,66
	.globl	_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E
	.p2align	4
_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E:
.seh_proc _ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E
	push	r15
	.seh_pushreg r15
	push	r14
	.seh_pushreg r14
	push	r13
	.seh_pushreg r13
	push	r12
	.seh_pushreg r12
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 88
	.seh_stackalloc 88
	.seh_endprologue
	mov	r15, r8
	mov	r8, qword ptr [rcx + 24]
	add	rdx, r8
	jb	.LBB66_52
	mov	rsi, qword ptr [rcx + 8]
	lea	r12, [rsi + 1]
	mov	rax, r12
	shr	rax, 3
	mov	rbx, r12
	and	rbx, -8
	sub	rbx, rax
	cmp	rsi, 8
	cmovb	rbx, rsi
	mov	rax, rbx
	shr	rax
	cmp	rdx, rax
	mov	qword ptr [rsp + 56], r8
	jbe	.LBB66_25
	inc	rbx
	cmp	rbx, rdx
	cmovbe	rbx, rdx
	cmp	rbx, 15
	jae	.LBB66_3
	mov	r8, rcx
	mov	eax, ebx
	and	eax, 8
	add	rax, 8
	cmp	rbx, 4
	mov	r13d, 4
	cmovae	r13, rax
	jmp	.LBB66_6
.LBB66_25:
	mov	qword ptr [rsp + 40], rcx
	test	r12, r12
	je	.LBB66_49
	mov	rax, qword ptr [rsp + 40]
	mov	r14, qword ptr [rax]
	mov	rax, r12
	shr	rax, 4
	mov	ecx, r12d
	and	ecx, 15
	cmp	rcx, 1
	sbb	rax, -1
	cmp	rax, 1
	jne	.LBB66_43
	xor	edx, edx
	jmp	.LBB66_28
.LBB66_3:
	mov	rax, rbx
	shr	rax, 61
	jne	.LBB66_52
	mov	r8, rcx
	shl	rbx, 3
	movabs	rcx, 2635249153387078803
	mov	rax, rbx
	mul	rcx
	sub	rbx, rdx
	shr	rbx
	add	rbx, rdx
	shr	rbx, 2
	dec	rbx
	bsr	rcx, rbx
	not	ecx
	mov	r13, -1
	shr	r13, cl
	movabs	rax, 4611686018427387899
	cmp	r13, rax
	ja	.LBB66_52
	inc	r13
.LBB66_6:
	lea	rdi, [4*r13 + 15]
	and	rdi, -16
	lea	r14, [r13 + 16]
	mov	r12, rdi
	add	r12, r14
	setb	al
	movabs	rcx, 9223372036854775792
	cmp	r12, rcx
	seta	cl
	or	cl, al
	je	.LBB66_7
.LBB66_52:
	mov	ecx, r9d
	call	_ZN9hashbrown3raw11Fallibility17capacity_overflow17h167bbd1a91e4964eE
	jmp	.LBB66_51
.LBB66_7:
	mov	qword ptr [rsp + 40], r8
	mov	ebp, r9d
	call	_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	edx, 16
	mov	rcx, r12
	call	_RNvCshXwFllX56pT_7___rustc12___rust_alloc
	test	rax, rax
	je	.LBB66_8
	mov	rbx, rax
	lea	r12, [r13 - 1]
	mov	rax, r13
	shr	rax, 3
	and	r13, -8
	sub	r13, rax
	cmp	r12, 8
	cmovb	r13, r12
	add	rbx, rdi
	mov	rcx, rbx
	mov	dl, -1
	mov	r8, r14
	call	memset
	mov	rdi, qword ptr [rsp + 56]
	test	rdi, rdi
	je	.LBB66_10
	mov	rax, qword ptr [rsp + 40]
	mov	r9, qword ptr [rax]
	movdqa	xmm0, xmmword ptr [r9]
	pmovmskb	ebp, xmm0
	not	ebp
	mov	rcx, qword ptr [r15]
	mov	r8, qword ptr [r15 + 8]
	movabs	rax, 8317987319222330741
	xor	rax, rcx
	movabs	rdx, 7237128888997146477
	xor	rdx, r8
	movabs	r10, 7816392313619706465
	xor	r10, rcx
	add	rax, rdx
	rol	rdx, 13
	xor	rdx, rax
	rol	rax, 32
	mov	qword ptr [rsp + 72], r10
	lea	rcx, [r10 + rdx]
	mov	qword ptr [rsp + 64], rcx
	rol	rdx, 17
	movabs	rcx, 8098989879002948979
	xor	rcx, r8
	mov	qword ptr [rsp + 80], rcx
	xor	r11d, r11d
	mov	qword ptr [rsp + 48], r9
	mov	r15, r9
	.p2align	4
.LBB66_15:
	test	bp, bp
	jne	.LBB66_18
	.p2align	4
.LBB66_16:
	movdqa	xmm0, xmmword ptr [r15 + 16]
	add	r15, 16
	pmovmskb	ebp, xmm0
	add	r11, 16
	cmp	ebp, 65535
	je	.LBB66_16
	not	ebp
.LBB66_18:
	rep		bsf	r8d, ebp
	add	r8, r11
	shl	r8, 2
	mov	rcx, qword ptr [rsp + 48]
	sub	rcx, r8
	mov	r10d, dword ptr [rcx - 4]
	movabs	r8, 288230376151711744
	add	r8, r10
	xor	r10, qword ptr [rsp + 80]
	mov	r9, qword ptr [rsp + 72]
	add	r9, r10
	mov	r14, r10
	rol	r14, 16
	xor	r14, r9
	add	r10, qword ptr [rsp + 64]
	lea	r9, [r14 + rax]
	rol	r14, 21
	xor	r14, r9
	xor	r8, r9
	mov	r9, r10
	xor	r9, rdx
	rol	r10, 32
	xor	r10, 255
	add	r8, r9
	add	r10, r14
	rol	r9, 13
	xor	r9, r8
	rol	r14, 16
	rol	r8, 32
	xor	r14, r10
	add	r10, r9
	rol	r9, 17
	add	r8, r14
	xor	r9, r10
	rol	r14, 21
	xor	r14, r8
	rol	r10, 32
	add	r8, r9
	add	r10, r14
	rol	r9, 13
	xor	r9, r8
	rol	r14, 16
	rol	r8, 32
	xor	r14, r10
	add	r10, r9
	rol	r9, 17
	add	r8, r14
	xor	r9, r10
	rol	r14, 21
	xor	r14, r8
	rol	r10, 32
	add	r8, r9
	add	r10, r14
	rol	r9, 13
	xor	r9, r8
	rol	r14, 16
	xor	r14, r10
	add	r10, r9
	rol	r9, 17
	rol	r14, 21
	mov	r8, r10
	rol	r8, 32
	xor	r8, r9
	xor	r8, r14
	xor	r8, r10
	mov	r9, r8
	and	r9, r12
	movdqu	xmm0, xmmword ptr [rbx + r9]
	pmovmskb	r10d, xmm0
	test	r10d, r10d
	je	.LBB66_19
.LBB66_21:
	rep		bsf	r10d, r10d
	add	r10, r9
	and	r10, r12
	cmp	byte ptr [rbx + r10], 0
	jns	.LBB66_22
.LBB66_23:
	lea	r9d, [rbp - 1]
	and	r9d, ebp
	dec	rdi
	shr	r8, 57
	lea	r14, [r10 - 16]
	and	r14, r12
	mov	byte ptr [rbx + r10], r8b
	mov	byte ptr [rbx + r14 + 16], r8b
	shl	r10, 2
	neg	r10
	mov	ecx, dword ptr [rcx - 4]
	mov	dword ptr [rbx + r10 - 4], ecx
	mov	ebp, r9d
	test	rdi, rdi
	jne	.LBB66_15
	jmp	.LBB66_24
.LBB66_19:
	mov	r14d, 16
.LBB66_20:
	add	r9, r14
	and	r9, r12
	movdqu	xmm0, xmmword ptr [rbx + r9]
	pmovmskb	r10d, xmm0
	add	r14, 16
	test	r10d, r10d
	jne	.LBB66_21
	jmp	.LBB66_20
.LBB66_22:
	movdqa	xmm0, xmmword ptr [rbx]
	pmovmskb	r9d, xmm0
	rep		bsf	r10d, r9d
	jmp	.LBB66_23
.LBB66_43:
	movabs	rcx, 2305843009213693950
	and	rcx, rax
	xor	r8d, r8d
	movdqa	xmm0, xmmword ptr [rip + __xmm@80808080808080808080808080808080]
	.p2align	4
.LBB66_44:
	pxor	xmm1, xmm1
	pcmpgtb	xmm1, xmmword ptr [r14 + r8]
	por	xmm1, xmm0
	movdqa	xmmword ptr [r14 + r8], xmm1
	lea	rdx, [r8 + 32]
	pxor	xmm1, xmm1
	pcmpgtb	xmm1, xmmword ptr [r14 + r8 + 16]
	por	xmm1, xmm0
	movdqa	xmmword ptr [r14 + r8 + 16], xmm1
	mov	r8, rdx
	add	rcx, -2
	jne	.LBB66_44
.LBB66_28:
	test	al, 1
	je	.LBB66_30
	pxor	xmm0, xmm0
	pcmpgtb	xmm0, xmmword ptr [r14 + rdx]
	por	xmm0, xmmword ptr [rip + __xmm@80808080808080808080808080808080]
	movdqa	xmmword ptr [r14 + rdx], xmm0
.LBB66_30:
	mov	r8d, 16
	mov	rcx, r12
	cmp	r12, 16
	jb	.LBB66_31
.LBB66_32:
	add	rcx, r14
	mov	rdx, r14
	call	memmove
	mov	r8, qword ptr [r15]
	mov	r9, qword ptr [r15 + 8]
	movabs	rax, 8317987319222330741
	xor	rax, r8
	movabs	rcx, 7237128888997146477
	xor	rcx, r9
	movabs	r10, 7816392313619706465
	xor	r10, r8
	add	rax, rcx
	rol	rcx, 13
	xor	rcx, rax
	rol	rax, 32
	mov	qword ptr [rsp + 48], r10
	lea	rdx, [r10 + rcx]
	mov	qword ptr [rsp + 72], rdx
	rol	rcx, 17
	movabs	rdx, 8098989879002948979
	xor	rdx, r9
	mov	qword ptr [rsp + 64], rdx
	mov	r8d, 1
	xor	edx, edx
	xor	edi, edi
	jmp	.LBB66_33
	.p2align	4
.LBB66_35:
	mov	r8d, dword ptr [r14 + 4*rbp - 4]
	movabs	rdx, 288230376151711744
	lea	r9, [r8 + rdx]
	xor	r8, qword ptr [rsp + 64]
	mov	rdx, qword ptr [rsp + 48]
	lea	r10, [r8 + rdx]
	mov	r11, r8
	rol	r11, 16
	xor	r11, r10
	add	r8, qword ptr [rsp + 72]
	lea	rdx, [r11 + rax]
	mov	r10, r8
	xor	r10, rcx
	rol	r11, 21
	rol	r8, 32
	xor	r11, rdx
	xor	r9, rdx
	xor	r8, 255
	add	r9, r10
	add	r8, r11
	rol	r10, 13
	xor	r10, r9
	rol	r11, 16
	rol	r9, 32
	xor	r11, r8
	add	r8, r10
	rol	r10, 17
	add	r9, r11
	xor	r10, r8
	rol	r11, 21
	xor	r11, r9
	rol	r8, 32
	add	r9, r10
	add	r8, r11
	rol	r10, 13
	xor	r10, r9
	rol	r11, 16
	rol	r9, 32
	xor	r11, r8
	add	r8, r10
	rol	r10, 17
	add	r9, r11
	xor	r10, r8
	rol	r11, 21
	xor	r11, r9
	rol	r8, 32
	add	r9, r10
	add	r8, r11
	rol	r10, 13
	xor	r10, r9
	rol	r11, 16
	xor	r11, r8
	add	r8, r10
	rol	r10, 17
	rol	r11, 21
	mov	r9, r8
	rol	r9, 32
	xor	r9, r10
	xor	r9, r11
	xor	r9, r8
	mov	r8, r9
	and	r8, rsi
	movdqu	xmm0, xmmword ptr [r14 + r8]
	pmovmskb	r10d, xmm0
	mov	r11, r8
	test	r10d, r10d
	je	.LBB66_36
.LBB66_38:
	rep		bsf	r10d, r10d
	add	r10, r11
	and	r10, rsi
	cmp	byte ptr [r14 + r10], 0
	jns	.LBB66_39
.LBB66_40:
	mov	rdx, r15
	sub	rdx, r8
	mov	r11, r10
	sub	r11, r8
	xor	r11, rdx
	and	r11, rsi
	cmp	r11, 16
	jb	.LBB66_45
	mov	r8, r10
	not	r8
	shl	r8, 2
	movzx	edx, byte ptr [r14 + r10]
	shr	r9, 57
	lea	r11, [r10 - 16]
	and	r11, rsi
	mov	byte ptr [r14 + r10], r9b
	mov	byte ptr [r14 + r11 + 16], r9b
	cmp	dl, -1
	je	.LBB66_46
	mov	edx, dword ptr [r14 + r13]
	mov	r9d, dword ptr [r14 + r8]
	mov	dword ptr [r14 + r13], r9d
	mov	dword ptr [r14 + r8], edx
	jmp	.LBB66_35
.LBB66_36:
	mov	edx, 16
	mov	r11, r8
.LBB66_37:
	add	r11, rdx
	and	r11, rsi
	movdqu	xmm0, xmmword ptr [r14 + r11]
	pmovmskb	r10d, xmm0
	add	rdx, 16
	test	r10d, r10d
	jne	.LBB66_38
	jmp	.LBB66_37
.LBB66_39:
	movdqa	xmm0, xmmword ptr [r14]
	pmovmskb	edx, xmm0
	rep		bsf	r10d, edx
	jmp	.LBB66_40
	.p2align	4
.LBB66_45:
	shr	r9, 57
	lea	rdx, [r15 - 16]
	and	rdx, rsi
	mov	byte ptr [r14 + r15], r9b
	mov	byte ptr [r14 + rdx + 16], r9b
.LBB66_47:
	xor	edx, edx
.LBB66_48:
	cmp	rdi, r12
	mov	r8, rdi
	adc	r8, 0
	cmp	rdi, r12
	jae	.LBB66_49
.LBB66_33:
	mov	r15, rdi
	mov	rdi, r8
	cmp	dl, byte ptr [r14 + r15]
	jno	.LBB66_48
	mov	r13, r15
	not	r13
	shl	r13, 2
	mov	rbp, r15
	neg	rbp
	jmp	.LBB66_35
.LBB66_46:
	lea	rdx, [r15 - 16]
	and	rdx, rsi
	mov	byte ptr [r14 + r15], -1
	mov	byte ptr [r14 + rdx + 16], -1
	mov	edx, dword ptr [r14 + r13]
	mov	dword ptr [r14 + r8], edx
	jmp	.LBB66_47
.LBB66_49:
	sub	rbx, qword ptr [rsp + 56]
	mov	rax, qword ptr [rsp + 40]
	mov	qword ptr [rax + 16], rbx
	movabs	rax, -9223372036854775807
.LBB66_50:
.LBB66_51:
	.seh_startepilogue
	add	rsp, 88
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	.seh_endepilogue
	ret
.LBB66_24:
	mov	rax, qword ptr [rsp + 40]
	mov	rdi, qword ptr [rsp + 56]
	jmp	.LBB66_11
.LBB66_8:
	mov	edx, 16
	mov	ecx, ebp
	mov	r8, r12
	call	_ZN9hashbrown3raw11Fallibility9alloc_err17h452573c969b6d13aE
	jmp	.LBB66_51
.LBB66_10:
	mov	rax, qword ptr [rsp + 40]
	mov	rcx, qword ptr [rax]
	mov	qword ptr [rsp + 48], rcx
.LBB66_11:
	sub	r13, rdi
	mov	qword ptr [rax], rbx
	mov	qword ptr [rax + 8], r12
	mov	qword ptr [rax + 16], r13
	movabs	rax, -9223372036854775807
	test	rsi, rsi
	je	.LBB66_50
	lea	rdx, [4*rsi + 19]
	and	rdx, -16
	add	rsi, rdx
	add	rsi, 17
	je	.LBB66_50
	mov	rcx, qword ptr [rsp + 48]
	sub	rcx, rdx
	mov	r8d, 16
	mov	rdx, rsi
	mov	rsi, rax
	call	_RNvCshXwFllX56pT_7___rustc14___rust_dealloc
	mov	rax, rsi
	jmp	.LBB66_50
.LBB66_31:
	mov	ecx, 16
	mov	r8, r12
	jmp	.LBB66_32
	.seh_endproc

	.section	.rdata,"dr",one_only,alloc_d0776666182ad032bd1011cf266e2f3a,unique,67
	.p2align	4, 0x0
alloc_d0776666182ad032bd1011cf266e2f3a:
	.zero	16,255

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.0,unique,68
	.p2align	3, 0x0
anon.44ffa63e8e95c400711a21744c5ea708.0:
	.quad	alloc_d0776666182ad032bd1011cf266e2f3a
	.zero	24

	.section	.rdata,"dr",one_only,alloc_2637060db1ca70c0d739ccdf7494925d,unique,69
alloc_2637060db1ca70c0d739ccdf7494925d:
	.asciz	"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\str\\mod.rs"

	.section	.rdata,"dr",one_only,alloc_90427cacc85724e4d3b32dbfd394b367,unique,70
	.p2align	3, 0x0
alloc_90427cacc85724e4d3b32dbfd394b367:
	.quad	alloc_2637060db1ca70c0d739ccdf7494925d
	.asciz	"p\000\000\000\000\000\000\000A\003\000\000\025\000\000"

	.section	.rdata,"dr",one_only,alloc_e1e44f68f3078611efc2362a115f5742,unique,71
alloc_e1e44f68f3078611efc2362a115f5742:
	.asciz	"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\str\\pattern.rs"

	.section	.rdata,"dr",one_only,alloc_960bc1bf861ba41d9b8231bbd37f66f6,unique,72
	.p2align	3, 0x0
alloc_960bc1bf861ba41d9b8231bbd37f66f6:
	.quad	alloc_e1e44f68f3078611efc2362a115f5742
	.asciz	"t\000\000\000\000\000\000\000\344\005\000\000\024\000\000"

	.section	.rdata,"dr",one_only,alloc_3c328170803c8011e86f11240ac4582e,unique,73
	.p2align	3, 0x0
alloc_3c328170803c8011e86f11240ac4582e:
	.quad	alloc_e1e44f68f3078611efc2362a115f5742
	.asciz	"t\000\000\000\000\000\000\000\344\005\000\000!\000\000"

	.section	.rdata,"dr",one_only,alloc_aeae60839ee01e593e8491fb61f2dda8,unique,74
	.p2align	3, 0x0
alloc_aeae60839ee01e593e8491fb61f2dda8:
	.quad	alloc_e1e44f68f3078611efc2362a115f5742
	.asciz	"t\000\000\000\000\000\000\000\330\005\000\000!\000\000"

	.section	.rdata,"dr",one_only,alloc_04d7ce44d7c86a9a02b346ab945bf155,unique,75
alloc_04d7ce44d7c86a9a02b346ab945bf155:
	.ascii	"description() is deprecated; use Display"

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.1,unique,76
	.p2align	3, 0x0
anon.44ffa63e8e95c400711a21744c5ea708.1:
	.quad	522596451624065841
	.quad	1648355301501614964

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.2,unique,77
	.p2align	3, 0x0
anon.44ffa63e8e95c400711a21744c5ea708.2:
	.quad	2400468019378343764
	.quad	8658629039619007897

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.3,unique,78
	.p2align	3, 0x0
anon.44ffa63e8e95c400711a21744c5ea708.3:
	.quad	-5882582162307352880
	.quad	7790121011132037916

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.4,unique,79
	.p2align	3, 0x0
anon.44ffa63e8e95c400711a21744c5ea708.4:
	.quad	3792400530729822680
	.quad	-9154057084057117283

	.section	.rdata,"dr",one_only,alloc_f7f1045810c6dd7020c5cff081af73be,unique,80
	.p2align	3, 0x0
alloc_f7f1045810c6dd7020c5cff081af73be:
	.quad	_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E
	.asciz	"\030\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E

	.section	.rdata,"dr",one_only,vtable.1,unique,81
	.p2align	3, 0x0
vtable.1:
	.quad	_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E
	.asciz	"\030\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h0607a4d5b05d13c1E
	.quad	_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E
	.quad	alloc_f7f1045810c6dd7020c5cff081af73be
	.quad	_ZN4core5error5Error5cause17h7a349d6c962996eeE
	.quad	_ZN4core5error5Error7type_id17h01cfa94765d144f3E
	.quad	_ZN4core5error5Error11description17h1649966410d8c91fE
	.quad	_ZN4core5error5Error5cause17h7a349d6c962996eeE
	.quad	_ZN4core5error5Error7provide17h9df6819c0298c953E

	.section	.rdata,"dr",one_only,alloc_0259449508b7e8fad72c8ab224522166,unique,82
	.p2align	3, 0x0
alloc_0259449508b7e8fad72c8ab224522166:
	.asciz	"\000\000\000\000\000\000\000\000\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE

	.section	.rdata,"dr",one_only,vtable.2,unique,83
	.p2align	3, 0x0
vtable.2:
	.asciz	"\000\000\000\000\000\000\000\000\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h663cff1b8df227b7E
	.quad	_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE
	.quad	alloc_0259449508b7e8fad72c8ab224522166
	.quad	_ZN4core5error5Error5cause17h7a349d6c962996eeE
	.quad	_ZN4core5error5Error7type_id17hdc7adb8570ca242eE
	.quad	_ZN4core5error5Error11description17h1649966410d8c91fE
	.quad	_ZN4core5error5Error5cause17h7a349d6c962996eeE
	.quad	_ZN4core5error5Error7provide17h9df6819c0298c953E

	.section	.rdata,"dr",one_only,alloc_1920c41e2cf13e8b20c82fe3438a9aa9,unique,84
	.p2align	3, 0x0
alloc_1920c41e2cf13e8b20c82fe3438a9aa9:
	.quad	_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE
	.asciz	"H\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE

	.section	.rdata,"dr",one_only,vtable.3,unique,85
	.p2align	3, 0x0
vtable.3:
	.quad	_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE
	.asciz	"H\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E
	.quad	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE
	.quad	alloc_1920c41e2cf13e8b20c82fe3438a9aa9
	.quad	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E
	.quad	_ZN4core5error5Error7type_id17h515f3f39467e6574E
	.quad	_ZN4core5error5Error11description17h6d684e2ae41137b1E
	.quad	_ZN4core5error5Error5cause17h3780c7e2d97e9501E
	.quad	_ZN4core5error5Error7provide17h05d6e66acb486045E

	.section	.rdata,"dr",one_only,alloc_e2f138f525d34441c9f70ee5bc3a1e34,unique,86
	.p2align	3, 0x0
alloc_e2f138f525d34441c9f70ee5bc3a1e34:
	.quad	_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E
	.asciz	"P\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE

	.section	.rdata,"dr",one_only,vtable.4,unique,87
	.p2align	3, 0x0
vtable.4:
	.quad	_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E
	.asciz	"P\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E
	.quad	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE
	.quad	alloc_e2f138f525d34441c9f70ee5bc3a1e34
	.quad	_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E
	.quad	_ZN4core5error5Error7type_id17hc7a2819971514648E
	.quad	_ZN4core5error5Error11description17h6d684e2ae41137b1E
	.quad	_ZN4core5error5Error5cause17h3780c7e2d97e9501E
	.quad	_ZN4core5error5Error7provide17h05d6e66acb486045E

	.section	.rdata,"dr",one_only,alloc_a04e47d083146d15ce3892a825ec94b0,unique,88
	.p2align	3, 0x0
alloc_a04e47d083146d15ce3892a825ec94b0:
	.quad	_ZN6anyhow5error11object_drop17h9334d5975c0625abE
	.quad	_ZN6anyhow5error10object_ref17h3950046690ebc90bE
	.quad	_ZN6anyhow5error12object_boxed17hdd48d83f6f5fca94E
	.quad	_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E
	.quad	_ZN6anyhow5error15object_downcast17h9f35c6fdf8d2f673E
	.quad	_ZN6anyhow5error17object_drop_front17h468434592611aba3E
	.quad	_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE

	.section	.rdata,"dr",one_only,alloc_00e51742134d344daa7116ffd2ad9e35,unique,89
	.p2align	3, 0x0
alloc_00e51742134d344daa7116ffd2ad9e35:
	.quad	_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE
	.quad	_ZN6anyhow5error10object_ref17h0ce20ab8fd8e770aE
	.quad	_ZN6anyhow5error12object_boxed17hde21296211696ba2E
	.quad	_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E
	.quad	_ZN6anyhow5error15object_downcast17hefd3fb1cb0d21f49E
	.quad	_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E
	.quad	_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE

	.section	.rdata,"dr",one_only,alloc_3f62f09340ec4217b72fe8840b861b6c,unique,90
alloc_3f62f09340ec4217b72fe8840b861b6c:
	.zero	2,10

	.section	.rdata,"dr",one_only,alloc_53973d2fe29b4adba8bb7390b5678745,unique,91
	.p2align	3, 0x0
alloc_53973d2fe29b4adba8bb7390b5678745:
	.zero	8

	.section	.rdata,"dr",one_only,alloc_053734e4d3889d3cd42e215952f6be3c,unique,92
alloc_053734e4d3889d3cd42e215952f6be3c:
	.asciz	"advent_of_code\\aoc2022\\src\\solver\\day02.rs"

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.5,unique,93
anon.44ffa63e8e95c400711a21744c5ea708.5:
	.ascii	"\004\b\003\001\005\t\007\002\006"

	.section	.rdata,"dr",one_only,alloc_04ec0caaeb79a0ce9c7e48871a54c01a,unique,94
	.p2align	3, 0x0
alloc_04ec0caaeb79a0ce9c7e48871a54c01a:
	.quad	alloc_053734e4d3889d3cd42e215952f6be3c
	.asciz	"*\000\000\000\000\000\000\000U\000\000\000\034\000\000"

	.section	.rdata,"dr",one_only,anon.44ffa63e8e95c400711a21744c5ea708.6,unique,95
anon.44ffa63e8e95c400711a21744c5ea708.6:
	.ascii	"\003\004\b\001\005\t\002\006\007"

	.section	.rdata,"dr",one_only,alloc_a1b3d90f42852e8e0faf60831e20d8b1,unique,96
	.p2align	3, 0x0
alloc_a1b3d90f42852e8e0faf60831e20d8b1:
	.quad	alloc_053734e4d3889d3cd42e215952f6be3c
	.asciz	"*\000\000\000\000\000\000\000`\000\000\000 \000\000"

	.section	.rdata,"dr",one_only,alloc_cb2aea7e2fdb2fba562edabf1f950868,unique,97
alloc_cb2aea7e2fdb2fba562edabf1f950868:
	.asciz	"\004Day \300; not implemented yet (valid range: 1-25). To implement day \310\000\000\027, create src/solver/day\313 \000\000i\002\000\000\0003.rs and uncomment the corresponding lines in mod.rs"

	.section	.rdata,"dr",one_only,alloc_8af13d31a0ec5c8f7fb83c7ff891ca76,unique,98
	.p2align	3, 0x0
alloc_8af13d31a0ec5c8f7fb83c7ff891ca76:
	.quad	alloc_e1e44f68f3078611efc2362a115f5742
	.asciz	"t\000\000\000\000\000\000\000h\004\000\000$\000\000"

	.section	.rdata,"dr",one_only,alloc_aa47418bb16f08c24f7eacc7bfd02189,unique,99
	.p2align	3, 0x0
alloc_aa47418bb16f08c24f7eacc7bfd02189:
	.quad	alloc_e1e44f68f3078611efc2362a115f5742
	.asciz	"t\000\000\000\000\000\000\000\315\001\000\0007\000\000"

	.section	.rdata,"dr",one_only,alloc_a931397211c33a1c8fe0d17838460834,unique,100
alloc_a931397211c33a1c8fe0d17838460834:
	.ascii	"internal error: entered unreachable code: invalid Once state"

	.section	.rdata,"dr",one_only,alloc_2b70d7ef093544d5f03f6771e2a94a2c,unique,101
alloc_2b70d7ef093544d5f03f6771e2a94a2c:
	.asciz	"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\std\\src\\sys\\sync\\once\\futex.rs"

	.section	.rdata,"dr",one_only,alloc_6a6fc231b3cb64280fdbf03fad4b13a2,unique,102
	.p2align	3, 0x0
alloc_6a6fc231b3cb64280fdbf03fad4b13a2:
	.quad	alloc_2b70d7ef093544d5f03f6771e2a94a2c
	.asciz	"{\000\000\000\000\000\000\000[\000\000\000\022\000\000"

