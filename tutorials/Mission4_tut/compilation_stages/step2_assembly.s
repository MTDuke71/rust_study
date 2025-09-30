	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"step2_push_front.f21d3421313a50d0-cgu.0"
	.def	_ZN3std2rt10lang_start17ha97314db3f698503E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17ha97314db3f698503E
	.globl	_ZN3std2rt10lang_start17ha97314db3f698503E
	.p2align	4
_ZN3std2rt10lang_start17ha97314db3f698503E:
.seh_proc _ZN3std2rt10lang_start17ha97314db3f698503E
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movb	%r9b, %al
	movq	%r8, %r9
	movq	%rdx, %r8
	movq	%rcx, 48(%rsp)
	leaq	48(%rsp), %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.0(%rip), %rdx
	movb	%al, 32(%rsp)
	callq	_ZN3std2rt19lang_start_internal17hfb78be9aa1b3a79cE
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE
	.p2align	4
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h82e0667bab42094bE
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E
	.p2align	4
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E:
.seh_proc _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	_ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E
	#APP
	#NO_APP
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E
	.p2align	4
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E:
.seh_proc _ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E
	andb	$1, %al
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E
	.p2align	4
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E:
.seh_proc _ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E
	andb	$1, %al
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E
	.p2align	4
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E:
.seh_proc _ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E
	andb	$1, %al
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
	.p2align	4
_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E:
	movq	%rcx, %rax
	movq	%rdx, (%rcx)
	movq	$2, 8(%rcx)
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1(%rip), %r9
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1+8(%rip), %rdx
	movq	%r9, 32(%rcx)
	movq	%rdx, 40(%rcx)
	movq	%r8, 16(%rcx)
	movq	$1, 24(%rcx)
	retq

	.def	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
	.p2align	4
_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E:
	movq	%rcx, %rax
	movq	%rdx, (%rcx)
	movq	$1, 8(%rcx)
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1(%rip), %r8
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1+8(%rip), %rdx
	movq	%r8, 32(%rcx)
	movq	%rdx, 40(%rcx)
	movl	$8, %edx
	movq	%rdx, 16(%rcx)
	movq	$0, 24(%rcx)
	retq

	.def	_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E
	.p2align	4
_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E:
.seh_proc _ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E
	subq	$16, %rsp
	.seh_stackalloc 16
	.seh_endprologue
	movq	%rcx, %rax
	movq	%rdx, (%rsp)
	leaq	_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h78b5b37ecec6e99dE(%rip), %rdx
	movq	%rdx, 8(%rsp)
	movq	(%rsp), %rdx
	movq	%rdx, (%rcx)
	movq	8(%rsp), %rdx
	movq	%rdx, 8(%rcx)
	addq	$16, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E
	.p2align	4
_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E:
.seh_proc _ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E
	subq	$16, %rsp
	.seh_stackalloc 16
	.seh_endprologue
	movq	%rcx, %rax
	movq	%rdx, (%rsp)
	leaq	_ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E(%rip), %rdx
	movq	%rdx, 8(%rsp)
	movq	(%rsp), %rdx
	movq	%rdx, (%rcx)
	movq	8(%rsp), %rdx
	movq	%rdx, 8(%rcx)
	addq	$16, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE
	.p2align	4
_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE:
.seh_proc _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%rdx, 32(%rsp)
	movq	%rcx, 40(%rsp)
	movl	16(%rdx), %eax
	andl	$33554432, %eax
	cmpl	$0, %eax
	jne	.LBB10_2
	movq	32(%rsp), %rax
	movl	16(%rax), %eax
	andl	$67108864, %eax
	cmpl	$0, %eax
	je	.LBB10_3
	jmp	.LBB10_4
.LBB10_2:
	movq	32(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h6c8de23d5753f856E
	andb	$1, %al
	movb	%al, 55(%rsp)
	jmp	.LBB10_6
.LBB10_3:
	movq	32(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he4cdc560234fe893E
	andb	$1, %al
	movb	%al, 55(%rsp)
	jmp	.LBB10_5
.LBB10_4:
	movq	32(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h9c04d02ee22d8c1eE
	andb	$1, %al
	movb	%al, 55(%rsp)
.LBB10_5:
	jmp	.LBB10_6
.LBB10_6:
	movb	55(%rsp), %al
	andb	$1, %al
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E
	.p2align	4
_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E:
.seh_proc _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%rdx, 32(%rsp)
	movq	%rcx, 40(%rsp)
	movl	16(%rdx), %eax
	andl	$33554432, %eax
	cmpl	$0, %eax
	jne	.LBB11_2
	movq	32(%rsp), %rax
	movl	16(%rax), %eax
	andl	$67108864, %eax
	cmpl	$0, %eax
	je	.LBB11_3
	jmp	.LBB11_4
.LBB11_2:
	movq	32(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h4385590cc0e1fda7E
	andb	$1, %al
	movb	%al, 55(%rsp)
	jmp	.LBB11_6
.LBB11_3:
	movq	32(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h78b5b37ecec6e99dE
	andb	$1, %al
	movb	%al, 55(%rsp)
	jmp	.LBB11_5
.LBB11_4:
	movq	32(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h3c7ef623801b380dE
	andb	$1, %al
	movb	%al, 55(%rsp)
.LBB11_5:
	jmp	.LBB11_6
.LBB11_6:
	movb	55(%rsp), %al
	andb	$1, %al
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E
	.p2align	4
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E
	.p2align	4
_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E:
.Lfunc_begin0:
.seh_proc _ZN4core3ops8function6FnOnce9call_once17h592f171622784320E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$64, %rsp
	.seh_stackalloc 64
	leaq	64(%rsp), %rbp
	.seh_setframe %rbp, 64
	.seh_endprologue
	movq	$-2, -8(%rbp)
	movq	%rcx, -16(%rbp)
.Ltmp0:
	leaq	-16(%rbp), %rcx
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE
.Ltmp1:
	movl	%eax, -20(%rbp)
	jmp	.LBB13_2
.LBB13_2:
	movl	-20(%rbp), %eax
	addq	$64, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E)@IMGREL
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E
	.seh_endproc
	.def	"?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E@4HA":
.seh_proc "?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E@4HA"
.LBB13_1:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	64(%rdx), %rbp
	.seh_endprologue
	addq	$32, %rsp
	popq	%rbp
	retq
.Lfunc_end0:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E
	.p2align	2, 0x0
$cppxdata$_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E:
	.long	429065506
	.long	1
	.long	($stateUnwindMap$_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E)@IMGREL
	.long	0
	.long	0
	.long	3
	.long	($ip2state$_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E)@IMGREL
	.long	56
	.long	0
	.long	1
$stateUnwindMap$_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E:
	.long	-1
	.long	"?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E@4HA"@IMGREL
$ip2state$_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E

	.def	_ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E
	.p2align	4
_ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E:
.seh_proc _ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	*%rcx
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E
	.p2align	4
_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E:
.seh_proc _ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	%rcx, 32(%rsp)
	movq	(%rcx), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB15_2
.LBB15_1:
	addq	$40, %rsp
	retq
.LBB15_2:
	movq	32(%rsp), %rcx
	callq	_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E
	jmp	.LBB15_1
	.seh_endproc

	.def	_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E
	.p2align	4
_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E:
.seh_proc _ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E
	.p2align	4
_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E:
.seh_proc _ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E
	.p2align	4
_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E:
.Lfunc_begin1:
.seh_proc _ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$48, %rsp
	.seh_stackalloc 48
	leaq	48(%rsp), %rbp
	.seh_setframe %rbp, 48
	.seh_endprologue
	movq	$-2, -8(%rbp)
	movq	%rcx, -16(%rbp)
	movq	(%rcx), %rcx
.Ltmp2:
	callq	_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E
.Ltmp3:
	jmp	.LBB18_2
.LBB18_2:
	movq	-16(%rbp), %rcx
	callq	_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE
	nop
	addq	$48, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E)@IMGREL
	.section	.text,"xr",one_only,_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E
	.seh_endproc
	.def	"?dtor$1@?0?_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$1@?0?_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E@4HA":
.seh_proc "?dtor$1@?0?_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E@4HA"
.LBB18_1:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	48(%rdx), %rbp
	.seh_endprologue
	movq	-16(%rbp), %rcx
	callq	_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE
	nop
	addq	$32, %rsp
	popq	%rbp
	retq
.Lfunc_end1:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E
	.p2align	2, 0x0
$cppxdata$_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E:
	.long	429065506
	.long	1
	.long	($stateUnwindMap$_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E)@IMGREL
	.long	0
	.long	0
	.long	3
	.long	($ip2state$_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E)@IMGREL
	.long	40
	.long	0
	.long	1
$stateUnwindMap$_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E:
	.long	-1
	.long	"?dtor$1@?0?_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E@4HA"@IMGREL
$ip2state$_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E:
	.long	.Lfunc_begin1@IMGREL
	.long	-1
	.long	.Ltmp2@IMGREL+1
	.long	0
	.long	.Ltmp3@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E

	.def	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE
	.p2align	4
_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE:
.seh_proc _ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE
	subq	$104, %rsp
	.seh_stackalloc 104
	.seh_endprologue
	movq	%rdx, 32(%rsp)
	cmpq	$0, %rcx
	jne	.LBB19_2
	movq	32(%rsp), %r8
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.2(%rip), %rax
	movq	%rax, 88(%rsp)
	movq	$210, 96(%rsp)
	leaq	88(%rsp), %rax
	movq	%rax, 40(%rsp)
	movq	$1, 48(%rsp)
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1(%rip), %rcx
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1+8(%rip), %rax
	movq	%rcx, 72(%rsp)
	movq	%rax, 80(%rsp)
	movl	$8, %eax
	movq	%rax, 56(%rsp)
	movq	$0, 64(%rsp)
	leaq	40(%rsp), %rcx
	xorl	%edx, %edx
	callq	_ZN4core9panicking18panic_nounwind_fmt17h967cb19ee376c79aE
.LBB19_2:
	nop
	addq	$104, %rsp
	retq
	.seh_endproc

	.def	_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	.p2align	4
_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E:
.Lfunc_begin2:
.seh_proc _ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$128, %rsp
	.seh_stackalloc 128
	leaq	128(%rsp), %rbp
	.seh_setframe %rbp, 128
	.seh_endprologue
	movq	$-2, -8(%rbp)
	movq	%r8, -88(%rbp)
.Ltmp4:
	callq	_ZN4core5alloc6layout6Layout19is_size_align_valid17h404c0a2962f1f594E
.Ltmp5:
	movb	%al, -73(%rbp)
	jmp	.LBB20_2
.LBB20_2:
	movb	-73(%rbp), %al
	testb	$1, %al
	jne	.LBB20_4
	jmp	.LBB20_3
.LBB20_3:
	movq	-88(%rbp), %r8
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.3(%rip), %rax
	movq	%rax, -24(%rbp)
	movq	$281, -16(%rbp)
	leaq	-24(%rbp), %rax
	movq	%rax, -72(%rbp)
	movq	$1, -64(%rbp)
	movq	$0, -40(%rbp)
	movq	%rax, -32(%rbp)
	movl	$8, %eax
	movq	%rax, -56(%rbp)
	movq	$0, -48(%rbp)
	leaq	-72(%rbp), %rcx
	xorl	%edx, %edx
	callq	_ZN4core9panicking18panic_nounwind_fmt17h967cb19ee376c79aE
.LBB20_4:
	nop
	addq	$128, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E)@IMGREL
	.section	.text,"xr",one_only,_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	.seh_endproc
	.def	"?catch$1@?0?_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?catch$1@?0?_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E@4HA":
.seh_proc "?catch$1@?0?_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E@4HA"
	.seh_handler __CxxFrameHandler3, @unwind, @except
.LBB20_1:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	128(%rdx), %rbp
	.seh_endprologue
	callq	_ZN4core9panicking19panic_cannot_unwind17h8f2b96e8056bc9b0E
	int3
.Lfunc_end2:
	.seh_handlerdata
	.long	($cppxdata$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E)@IMGREL
	.section	.text,"xr",one_only,_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	.p2align	2, 0x0
$cppxdata$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E:
	.long	429065506
	.long	2
	.long	($stateUnwindMap$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E)@IMGREL
	.long	1
	.long	($tryMap$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E)@IMGREL
	.long	4
	.long	($ip2state$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E)@IMGREL
	.long	120
	.long	0
	.long	1
$stateUnwindMap$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E:
	.long	-1
	.long	0
	.long	-1
	.long	0
$tryMap$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E:
	.long	0
	.long	0
	.long	1
	.long	1
	.long	($handlerMap$0$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E)@IMGREL
$handlerMap$0$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E:
	.long	64
	.long	0
	.long	0
	.long	"?catch$1@?0?_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E@4HA"@IMGREL
	.long	56
$ip2state$_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E:
	.long	.Lfunc_begin2@IMGREL
	.long	-1
	.long	.Ltmp4@IMGREL+1
	.long	0
	.long	.Ltmp5@IMGREL+1
	.long	-1
	.long	"?catch$1@?0?_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E@4HA"@IMGREL
	.long	1
	.section	.text,"xr",one_only,_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E

	.def	_ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E
	.p2align	4
_ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E:
.seh_proc _ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E
	pushq	%rax
	.seh_stackalloc 8
	.seh_endprologue
	movq	$0, (%rsp)
	movq	(%rcx), %rax
	movq	(%rsp), %rdx
	movq	%rdx, (%rcx)
	popq	%rcx
	retq
	.seh_endproc

	.def	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h82e0667bab42094bE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h82e0667bab42094bE
	.p2align	4
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h82e0667bab42094bE:
	xorl	%eax, %eax
	retq

	.def	_ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E
	.p2align	4
_ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E:
.seh_proc _ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E
	subq	$72, %rsp
	.seh_stackalloc 72
	.seh_endprologue
	movq	%rdx, 40(%rsp)
	movq	%rcx, 48(%rsp)
	movq	40(%rsp), %rdx
	movq	48(%rsp), %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.5(%rip), %r8
	callq	_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	movq	48(%rsp), %r8
	movq	40(%rsp), %rdx
	movl	$1, %ecx
	xorl	%r9d, %r9d
	callq	_ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E
	movq	%rax, 56(%rsp)
	movq	%rdx, 64(%rsp)
	movq	56(%rsp), %rdx
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	testq	$1, %rax
	je	.LBB23_4
	movq	48(%rsp), %rdx
	movq	40(%rsp), %rcx
	callq	_ZN5alloc5alloc18handle_alloc_error17h058b0c5b7728b769E
.LBB23_4:
	movq	56(%rsp), %rax
	addq	$72, %rsp
	retq
	.seh_endproc

	.def	_ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E
	.p2align	4
_ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E:
.seh_proc _ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E
	subq	$184, %rsp
	.seh_stackalloc 184
	.seh_endprologue
	movb	%r9b, 79(%rsp)
	movq	%rdx, 88(%rsp)
	movq	%r8, 96(%rsp)
	movq	96(%rsp), %rax
	movq	%rax, 80(%rsp)
	cmpq	$0, %rax
	jne	.LBB24_2
	movq	88(%rsp), %rcx
	movq	%rcx, 56(%rsp)
	xorl	%eax, %eax
	addq	%rcx, %rax
	movq	%rax, 64(%rsp)
	jmp	.LBB24_3
.LBB24_2:
	movb	79(%rsp), %al
	testb	$1, %al
	jne	.LBB24_7
	jmp	.LBB24_6
.LBB24_3:
	movq	56(%rsp), %rax
	xorl	%ecx, %ecx
	addq	%rax, %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.7(%rip), %rdx
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE
	movq	64(%rsp), %rax
	movq	%rax, 104(%rsp)
	movq	$0, 112(%rsp)
.LBB24_5:
	movq	104(%rsp), %rax
	movq	112(%rsp), %rdx
	addq	$184, %rsp
	retq
.LBB24_6:
	movq	88(%rsp), %rcx
	movq	96(%rsp), %rax
	movq	%rcx, 144(%rsp)
	movq	%rax, 152(%rsp)
	callq	_RNvCs73fAdSrgOJL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	movq	80(%rsp), %rcx
	movq	88(%rsp), %rdx
	callq	_RNvCs73fAdSrgOJL_7___rustc12___rust_alloc
	movq	%rax, 120(%rsp)
	jmp	.LBB24_8
.LBB24_7:
	movq	88(%rsp), %rcx
	movq	96(%rsp), %rax
	movq	%rcx, 128(%rsp)
	movq	%rax, 136(%rsp)
	callq	_RNvCs73fAdSrgOJL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	movq	80(%rsp), %rcx
	movq	88(%rsp), %rdx
	callq	_RNvCs73fAdSrgOJL_7___rustc19___rust_alloc_zeroed
	movq	%rax, 120(%rsp)
.LBB24_8:
	movq	120(%rsp), %rax
	movq	%rax, 48(%rsp)
	cmpq	$0, %rax
	jne	.LBB24_10
	movq	$0, 176(%rsp)
	movq	$0, 168(%rsp)
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1(%rip), %rcx
	movq	anon.440927b5fc1324937d02a5fde4861f2c.1+8(%rip), %rax
	movq	%rcx, 104(%rsp)
	movq	%rax, 112(%rsp)
	jmp	.LBB24_5
.LBB24_10:
	jmp	.LBB24_11
.LBB24_11:
	movq	48(%rsp), %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.8(%rip), %rdx
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE
	movq	48(%rsp), %rax
	movq	%rax, 176(%rsp)
	movq	176(%rsp), %rax
	movq	%rax, 168(%rsp)
	movq	168(%rsp), %rax
	movq	%rax, 160(%rsp)
	movq	160(%rsp), %rax
	movq	%rax, 40(%rsp)
	movq	40(%rsp), %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.7(%rip), %rdx
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE
	movq	80(%rsp), %rax
	movq	40(%rsp), %rcx
	movq	%rcx, 104(%rsp)
	movq	%rax, 112(%rsp)
	jmp	.LBB24_5
	.seh_endproc

	.def	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E
	.p2align	4
_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E:
.seh_proc _ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E
	subq	$88, %rsp
	.seh_stackalloc 88
	.seh_endprologue
	movq	%rdx, 40(%rsp)
	movq	%r8, 56(%rsp)
	movq	%r9, 64(%rsp)
	movq	64(%rsp), %rax
	movq	%rax, 48(%rsp)
	cmpq	$0, %rax
	jne	.LBB25_2
.LBB25_1:
	addq	$88, %rsp
	retq
.LBB25_2:
	movq	48(%rsp), %rdx
	movq	40(%rsp), %rcx
	movq	56(%rsp), %r8
	movq	64(%rsp), %rax
	movq	%r8, 72(%rsp)
	movq	%rax, 80(%rsp)
	movq	56(%rsp), %r8
	callq	_RNvCs73fAdSrgOJL_7___rustc14___rust_dealloc
	jmp	.LBB25_1
	.seh_endproc

	.def	_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E
	.p2align	4
_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E:
.seh_proc _ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E
	subq	$72, %rsp
	.seh_stackalloc 72
	.seh_endprologue
	movq	%rdx, 40(%rsp)
	movq	%rcx, 48(%rsp)
	movq	(%rcx), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	testq	$1, %rax
	je	.LBB26_2
	movq	40(%rsp), %rcx
	movq	48(%rsp), %rax
	movq	%rax, 64(%rsp)
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.11(%rip), %rdx
	movl	$4, %r8d
	leaq	64(%rsp), %r9
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.10(%rip), %rax
	movq	%rax, 32(%rsp)
	callq	_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h470abd3bc5ca56cfE
	andb	$1, %al
	movb	%al, 63(%rsp)
	jmp	.LBB26_3
.LBB26_2:
	movq	40(%rsp), %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.9(%rip), %rdx
	movl	$4, %r8d
	callq	_ZN4core3fmt9Formatter9write_str17h4ea819d4f057104eE
	andb	$1, %al
	movb	%al, 63(%rsp)
.LBB26_3:
	movb	63(%rsp), %al
	andb	$1, %al
	addq	$72, %rsp
	retq
	.seh_endproc

	.def	_ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E
	.p2align	4
_ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E:
.seh_proc _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE
	andb	$1, %al
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE
	.p2align	4
_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE:
.seh_proc _ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE
	subq	$104, %rsp
	.seh_stackalloc 104
	.seh_endprologue
	movq	%rcx, 40(%rsp)
	movq	(%rcx), %rax
	movq	%rax, 48(%rsp)
	movq	$16, 88(%rsp)
	movq	88(%rsp), %rax
	movq	%rax, 56(%rsp)
	movq	$8, 96(%rsp)
	movq	96(%rsp), %rax
	movq	%rax, 64(%rsp)
	movq	64(%rsp), %rdx
	movq	56(%rsp), %rcx
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.13(%rip), %r8
	callq	_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E
	movq	56(%rsp), %rax
	movq	64(%rsp), %rcx
	movq	%rax, 80(%rsp)
	movq	%rcx, 72(%rsp)
	cmpq	$0, %rax
	jne	.LBB28_4
.LBB28_3:
	addq	$104, %rsp
	retq
.LBB28_4:
	movq	48(%rsp), %rdx
	movq	40(%rsp), %rcx
	addq	$8, %rcx
	movq	72(%rsp), %r8
	movq	80(%rsp), %r9
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E
	jmp	.LBB28_3
	.seh_endproc

	.def	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E
	.p2align	4
_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E:
.seh_proc _ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E
	pushq	%rax
	.seh_stackalloc 8
	.seh_endprologue
	movq	$0, (%rsp)
	movq	(%rsp), %rax
	xorl	%ecx, %ecx
	movl	%ecx, %edx
	popq	%rcx
	retq
	.seh_endproc

	.def	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.p2align	4
_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE:
.Lfunc_begin3:
.seh_proc _ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$112, %rsp
	.seh_stackalloc 112
	leaq	112(%rsp), %rbp
	.seh_setframe %rbp, 112
	.seh_endprologue
	movq	$-2, -8(%rbp)
	movl	%edx, -52(%rbp)
	movq	%rcx, -48(%rbp)
.Ltmp6:
	callq	_ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E
.Ltmp7:
	movq	%rax, -40(%rbp)
	jmp	.LBB30_2
.LBB30_2:
	movl	-52(%rbp), %eax
	movq	-40(%rbp), %rcx
	movq	%rcx, -24(%rbp)
	movl	%eax, -16(%rbp)
.Ltmp8:
	movl	$16, %ecx
	movl	$8, %edx
	callq	_ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E
.Ltmp9:
	movq	%rax, -64(%rbp)
	jmp	.LBB30_4
.LBB30_4:
	movq	-48(%rbp), %rcx
	movq	-64(%rbp), %rax
	movq	-24(%rbp), %r8
	movl	-16(%rbp), %edx
	movq	%r8, (%rax)
	movl	%edx, 8(%rax)
	movq	%rax, -32(%rbp)
.Ltmp10:
	callq	_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E
.Ltmp11:
	jmp	.LBB30_6
.LBB30_6:
	movq	-48(%rbp), %rax
	movq	-32(%rbp), %rcx
	movq	%rcx, (%rax)
	movq	8(%rax), %rax
	addq	$1, %rax
	movq	%rax, -72(%rbp)
	setb	%al
	jb	.LBB30_8
	movq	-48(%rbp), %rax
	movq	-72(%rbp), %rcx
	movq	%rcx, 8(%rax)
	addq	$112, %rsp
	popq	%rbp
	retq
.LBB30_8:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.15(%rip), %rcx
	callq	_ZN4core9panicking11panic_const24panic_const_add_overflow17h91a6092a8a656db9E
	int3
	.seh_handlerdata
	.long	($cppxdata$_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE)@IMGREL
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.seh_endproc
	.def	"?dtor$1@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$1@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA":
.seh_proc "?dtor$1@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"
.LBB30_1:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	112(%rdx), %rbp
	.seh_endprologue
	addq	$32, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.seh_endproc
	.def	"?dtor$3@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$3@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA":
.seh_proc "?dtor$3@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"
.LBB30_3:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	112(%rdx), %rbp
	.seh_endprologue
	leaq	-24(%rbp), %rcx
	callq	_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E
	nop
	addq	$32, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.seh_endproc
	.def	"?dtor$5@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$5@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA":
.seh_proc "?dtor$5@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"
.LBB30_5:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	112(%rdx), %rbp
	.seh_endprologue
	movq	-48(%rbp), %rax
	movq	-32(%rbp), %rcx
	movq	%rcx, (%rax)
	addq	$32, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.seh_endproc
	.def	"?dtor$9@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$9@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA":
.seh_proc "?dtor$9@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"
.LBB30_9:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	112(%rdx), %rbp
	.seh_endprologue
	addq	$32, %rsp
	popq	%rbp
	retq
.Lfunc_end3:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
	.p2align	2, 0x0
$cppxdata$_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE:
	.long	429065506
	.long	4
	.long	($stateUnwindMap$_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE)@IMGREL
	.long	0
	.long	0
	.long	5
	.long	($ip2state$_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE)@IMGREL
	.long	104
	.long	0
	.long	1
$stateUnwindMap$_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE:
	.long	-1
	.long	"?dtor$3@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"@IMGREL
	.long	-1
	.long	"?dtor$9@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"@IMGREL
	.long	1
	.long	"?dtor$1@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"@IMGREL
	.long	1
	.long	"?dtor$5@?0?_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE@4HA"@IMGREL
$ip2state$_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE:
	.long	.Lfunc_begin3@IMGREL
	.long	-1
	.long	.Ltmp6@IMGREL+1
	.long	2
	.long	.Ltmp8@IMGREL+1
	.long	0
	.long	.Ltmp10@IMGREL+1
	.long	3
	.long	.Ltmp11@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE

	.def	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE
	.p2align	4
_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE:
	movq	8(%rcx), %rax
	retq

	.def	_ZN16step2_push_front4main17h77510b71c3eb5aadE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN16step2_push_front4main17h77510b71c3eb5aadE
	.p2align	4
_ZN16step2_push_front4main17h77510b71c3eb5aadE:
.Lfunc_begin4:
.seh_proc _ZN16step2_push_front4main17h77510b71c3eb5aadE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$976, %rsp
	.seh_stackalloc 976
	leaq	128(%rsp), %rbp
	.seh_setframe %rbp, 128
	.seh_endprologue
	movq	$-2, 840(%rbp)
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.17(%rip), %rdx
	leaq	-64(%rbp), %rcx
	movq	%rcx, -72(%rbp)
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
	movq	-72(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E
	movq	%rax, -16(%rbp)
	movq	%rdx, -8(%rbp)
.Ltmp12:
	leaq	64(%rbp), %rcx
	leaq	-16(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E
.Ltmp13:
	jmp	.LBB32_2
.LBB32_2:
	movups	64(%rbp), %xmm0
	movaps	%xmm0, 48(%rbp)
.Ltmp14:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.20(%rip), %rdx
	movq	%rbp, %rcx
	leaq	48(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp15:
	jmp	.LBB32_3
.LBB32_3:
.Ltmp16:
	movq	%rbp, %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp17:
	jmp	.LBB32_4
.LBB32_4:
.Ltmp18:
	leaq	-16(%rbp), %rcx
	movl	$42, %edx
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
.Ltmp19:
	jmp	.LBB32_5
.LBB32_5:
.Ltmp20:
	leaq	144(%rbp), %rcx
	leaq	-16(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E
.Ltmp21:
	jmp	.LBB32_6
.LBB32_6:
	movups	144(%rbp), %xmm0
	movaps	%xmm0, 128(%rbp)
.Ltmp22:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.22(%rip), %rdx
	leaq	80(%rbp), %rcx
	leaq	128(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp23:
	jmp	.LBB32_7
.LBB32_7:
.Ltmp24:
	leaq	80(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp25:
	jmp	.LBB32_8
.LBB32_8:
.Ltmp26:
	leaq	-16(%rbp), %rcx
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE
.Ltmp27:
	movq	%rax, -80(%rbp)
	jmp	.LBB32_9
.LBB32_9:
	movq	-80(%rbp), %rax
	movq	%rax, 248(%rbp)
.Ltmp28:
	leaq	232(%rbp), %rcx
	leaq	248(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E
.Ltmp29:
	jmp	.LBB32_10
.LBB32_10:
	movups	232(%rbp), %xmm0
	movaps	%xmm0, 208(%rbp)
.Ltmp30:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.24(%rip), %rdx
	leaq	160(%rbp), %rcx
	leaq	208(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp31:
	jmp	.LBB32_11
.LBB32_11:
.Ltmp32:
	leaq	160(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp33:
	jmp	.LBB32_12
.LBB32_12:
.Ltmp34:
	leaq	-16(%rbp), %rcx
	movl	$24, %edx
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
.Ltmp35:
	jmp	.LBB32_13
.LBB32_13:
.Ltmp36:
	leaq	320(%rbp), %rcx
	leaq	-16(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E
.Ltmp37:
	jmp	.LBB32_14
.LBB32_14:
	movups	320(%rbp), %xmm0
	movaps	%xmm0, 304(%rbp)
.Ltmp38:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.26(%rip), %rdx
	leaq	256(%rbp), %rcx
	leaq	304(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp39:
	jmp	.LBB32_15
.LBB32_15:
.Ltmp40:
	leaq	256(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp41:
	jmp	.LBB32_16
.LBB32_16:
.Ltmp42:
	leaq	-16(%rbp), %rcx
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE
.Ltmp43:
	movq	%rax, -88(%rbp)
	jmp	.LBB32_17
.LBB32_17:
	movq	-88(%rbp), %rax
	movq	%rax, 424(%rbp)
.Ltmp44:
	leaq	408(%rbp), %rcx
	leaq	424(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E
.Ltmp45:
	jmp	.LBB32_18
.LBB32_18:
	movups	408(%rbp), %xmm0
	movaps	%xmm0, 384(%rbp)
.Ltmp46:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.24(%rip), %rdx
	leaq	336(%rbp), %rcx
	leaq	384(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp47:
	jmp	.LBB32_19
.LBB32_19:
.Ltmp48:
	leaq	336(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp49:
	jmp	.LBB32_20
.LBB32_20:
.Ltmp50:
	leaq	-16(%rbp), %rcx
	movl	$13, %edx
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE
.Ltmp51:
	jmp	.LBB32_21
.LBB32_21:
.Ltmp52:
	leaq	496(%rbp), %rcx
	leaq	-16(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E
.Ltmp53:
	jmp	.LBB32_22
.LBB32_22:
	movups	496(%rbp), %xmm0
	movaps	%xmm0, 480(%rbp)
.Ltmp54:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.28(%rip), %rdx
	leaq	432(%rbp), %rcx
	leaq	480(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp55:
	jmp	.LBB32_23
.LBB32_23:
.Ltmp56:
	leaq	432(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp57:
	jmp	.LBB32_24
.LBB32_24:
.Ltmp58:
	leaq	-16(%rbp), %rcx
	callq	_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE
.Ltmp59:
	movq	%rax, -96(%rbp)
	jmp	.LBB32_25
.LBB32_25:
	movq	-96(%rbp), %rax
	movq	%rax, 592(%rbp)
.Ltmp60:
	leaq	576(%rbp), %rcx
	leaq	592(%rbp), %rdx
	callq	_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E
.Ltmp61:
	jmp	.LBB32_26
.LBB32_26:
	movups	576(%rbp), %xmm0
	movaps	%xmm0, 560(%rbp)
.Ltmp62:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.24(%rip), %rdx
	leaq	512(%rbp), %rcx
	leaq	560(%rbp), %r8
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E
.Ltmp63:
	jmp	.LBB32_27
.LBB32_27:
.Ltmp64:
	leaq	512(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp65:
	jmp	.LBB32_28
.LBB32_28:
.Ltmp66:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.29(%rip), %rdx
	leaq	600(%rbp), %rcx
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
.Ltmp67:
	jmp	.LBB32_29
.LBB32_29:
.Ltmp68:
	leaq	600(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp69:
	jmp	.LBB32_30
.LBB32_30:
.Ltmp70:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.31(%rip), %rdx
	leaq	648(%rbp), %rcx
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
.Ltmp71:
	jmp	.LBB32_31
.LBB32_31:
.Ltmp72:
	leaq	648(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp73:
	jmp	.LBB32_32
.LBB32_32:
.Ltmp74:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.33(%rip), %rdx
	leaq	696(%rbp), %rcx
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
.Ltmp75:
	jmp	.LBB32_33
.LBB32_33:
.Ltmp76:
	leaq	696(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp77:
	jmp	.LBB32_34
.LBB32_34:
.Ltmp78:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.35(%rip), %rdx
	leaq	744(%rbp), %rcx
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
.Ltmp79:
	jmp	.LBB32_35
.LBB32_35:
.Ltmp80:
	leaq	744(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp81:
	jmp	.LBB32_36
.LBB32_36:
.Ltmp82:
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.37(%rip), %rdx
	leaq	792(%rbp), %rcx
	callq	_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E
.Ltmp83:
	jmp	.LBB32_37
.LBB32_37:
.Ltmp84:
	leaq	792(%rbp), %rcx
	callq	_ZN3std2io5stdio6_print17h8b3b46477b515abcE
.Ltmp85:
	jmp	.LBB32_38
.LBB32_38:
	leaq	-16(%rbp), %rcx
	callq	_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E
	nop
	addq	$976, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$_ZN16step2_push_front4main17h77510b71c3eb5aadE)@IMGREL
	.section	.text,"xr",one_only,_ZN16step2_push_front4main17h77510b71c3eb5aadE
	.seh_endproc
	.def	"?dtor$1@?0?_ZN16step2_push_front4main17h77510b71c3eb5aadE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4
"?dtor$1@?0?_ZN16step2_push_front4main17h77510b71c3eb5aadE@4HA":
.seh_proc "?dtor$1@?0?_ZN16step2_push_front4main17h77510b71c3eb5aadE@4HA"
.LBB32_1:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	128(%rdx), %rbp
	.seh_endprologue
	leaq	-16(%rbp), %rcx
	callq	_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E
	nop
	addq	$32, %rsp
	popq	%rbp
	retq
.Lfunc_end4:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN16step2_push_front4main17h77510b71c3eb5aadE
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN16step2_push_front4main17h77510b71c3eb5aadE
	.p2align	2, 0x0
$cppxdata$_ZN16step2_push_front4main17h77510b71c3eb5aadE:
	.long	429065506
	.long	1
	.long	($stateUnwindMap$_ZN16step2_push_front4main17h77510b71c3eb5aadE)@IMGREL
	.long	0
	.long	0
	.long	3
	.long	($ip2state$_ZN16step2_push_front4main17h77510b71c3eb5aadE)@IMGREL
	.long	968
	.long	0
	.long	1
$stateUnwindMap$_ZN16step2_push_front4main17h77510b71c3eb5aadE:
	.long	-1
	.long	"?dtor$1@?0?_ZN16step2_push_front4main17h77510b71c3eb5aadE@4HA"@IMGREL
$ip2state$_ZN16step2_push_front4main17h77510b71c3eb5aadE:
	.long	.Lfunc_begin4@IMGREL
	.long	-1
	.long	.Ltmp12@IMGREL+1
	.long	0
	.long	.Ltmp85@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN16step2_push_front4main17h77510b71c3eb5aadE

	.def	_ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE
	.p2align	4
_ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE:
.seh_proc _ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	subq	$104, %rsp
	.seh_stackalloc 104
	.seh_endprologue
	movq	%rdx, 88(%rsp)
	movq	%rcx, %rax
	movq	88(%rsp), %rcx
	movq	%rax, %rdi
	addq	$8, %rdi
	movq	%rax, 96(%rsp)
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.38(%rip), %rdx
	movl	$4, %r8d
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.39(%rip), %r9
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.40(%rip), %rsi
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.41(%rip), %r11
	leaq	96(%rsp), %r10
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.42(%rip), %rax
	movq	$4, 32(%rsp)
	movq	%rdi, 40(%rsp)
	movq	%rsi, 48(%rsp)
	movq	%r11, 56(%rsp)
	movq	$4, 64(%rsp)
	movq	%r10, 72(%rsp)
	movq	%rax, 80(%rsp)
	callq	_ZN4core3fmt9Formatter26debug_struct_field2_finish17h4b6cb7e667c5a162E
	andb	$1, %al
	addq	$104, %rsp
	popq	%rdi
	popq	%rsi
	retq
	.seh_endproc

	.def	_ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E
	.p2align	4
_ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E:
.seh_proc _ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	subq	$104, %rsp
	.seh_stackalloc 104
	.seh_endprologue
	movq	%rdx, 88(%rsp)
	movq	%rcx, %rdi
	movq	88(%rsp), %rcx
	movq	%rdi, %rax
	addq	$8, %rax
	movq	%rax, 96(%rsp)
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.43(%rip), %rdx
	movl	$16, %r8d
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.44(%rip), %r9
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.45(%rip), %rsi
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.46(%rip), %r11
	leaq	96(%rsp), %r10
	leaq	anon.440927b5fc1324937d02a5fde4861f2c.47(%rip), %rax
	movq	$4, 32(%rsp)
	movq	%rdi, 40(%rsp)
	movq	%rsi, 48(%rsp)
	movq	%r11, 56(%rsp)
	movq	$6, 64(%rsp)
	movq	%r10, 72(%rsp)
	movq	%rax, 80(%rsp)
	callq	_ZN4core3fmt9Formatter26debug_struct_field2_finish17h4b6cb7e667c5a162E
	andb	$1, %al
	addq	$104, %rsp
	popq	%rdi
	popq	%rsi
	retq
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main
	.globl	main
	.p2align	4
main:
.seh_proc main
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	%rdx, %r8
	movslq	%ecx, %rdx
	leaq	_ZN16step2_push_front4main17h77510b71c3eb5aadE(%rip), %rcx
	xorl	%r9d, %r9d
	callq	_ZN3std2rt10lang_start17ha97314db3f698503E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.0
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.1
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.1:
	.zero	8
	.zero	8

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.2
anon.440927b5fc1324937d02a5fde4861f2c.2:
	.ascii	"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety."

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.3
anon.440927b5fc1324937d02a5fde4861f2c.3:
	.ascii	"unsafe precondition(s) violated: Layout::from_size_align_unchecked requires that align is a power of 2 and the rounded-up allocation size does not exceed isize::MAX\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety."

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.4
anon.440927b5fc1324937d02a5fde4861f2c.4:
	.asciz	"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\alloc\\src\\alloc.rs"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.5
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.5:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.4
	.asciz	"p\000\000\000\000\000\000\000^\001\000\000\033\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.6
anon.440927b5fc1324937d02a5fde4861f2c.6:
	.asciz	"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\ptr\\non_null.rs"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.7
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.7:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.6
	.asciz	"v\000\000\000\000\000\000\000l\005\000\000\022\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.8
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.8:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.6
	.asciz	"v\000\000\000\000\000\000\000\t\001\000\000\033\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.9
anon.440927b5fc1324937d02a5fde4861f2c.9:
	.ascii	"None"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.10
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.10:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.11
anon.440927b5fc1324937d02a5fde4861f2c.11:
	.ascii	"Some"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.12
anon.440927b5fc1324937d02a5fde4861f2c.12:
	.asciz	"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\alloc\\layout.rs"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.13
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.13:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.12
	.asciz	"v\000\000\000\000\000\000\000\340\000\000\000\022\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.14
anon.440927b5fc1324937d02a5fde4861f2c.14:
	.asciz	"examples\\step2_push_front.rs"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.15
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.15:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.14
	.asciz	"\035\000\000\000\000\000\000\000\036\000\000\000\t\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.16
anon.440927b5fc1324937d02a5fde4861f2c.16:
	.ascii	"=== Step 2: Adding Elements ===\n"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.17
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.17:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.16
	.asciz	" \000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.18
anon.440927b5fc1324937d02a5fde4861f2c.18:
	.ascii	"Initial state: "

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.19
anon.440927b5fc1324937d02a5fde4861f2c.19:
	.byte	10

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.20
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.20:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.18
	.asciz	"\017\000\000\000\000\000\000"
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.19
	.asciz	"\001\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.21
anon.440927b5fc1324937d02a5fde4861f2c.21:
	.ascii	"After push_front(42): "

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.22
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.22:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.21
	.asciz	"\026\000\000\000\000\000\000"
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.19
	.asciz	"\001\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.23
anon.440927b5fc1324937d02a5fde4861f2c.23:
	.ascii	"Length: "

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.24
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.24:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.23
	.asciz	"\b\000\000\000\000\000\000"
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.19
	.asciz	"\001\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.25
anon.440927b5fc1324937d02a5fde4861f2c.25:
	.ascii	"After push_front(24): "

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.26
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.26:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.25
	.asciz	"\026\000\000\000\000\000\000"
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.19
	.asciz	"\001\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.27
anon.440927b5fc1324937d02a5fde4861f2c.27:
	.ascii	"After push_front(13): "

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.28
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.28:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.27
	.asciz	"\026\000\000\000\000\000\000"
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.19
	.asciz	"\001\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.29
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.29:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.19
	.asciz	"\001\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.30
anon.440927b5fc1324937d02a5fde4861f2c.30:
	.ascii	"\360\237\223\235 Note: Elements are added to the FRONT\n"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.31
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.31:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.30
	.asciz	"+\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.32
anon.440927b5fc1324937d02a5fde4861f2c.32:
	.ascii	"Order added: 42, 24, 13\n"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.33
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.33:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.32
	.asciz	"\030\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.34
anon.440927b5fc1324937d02a5fde4861f2c.34:
	.ascii	"Current order in list: 13 -> 24 -> 42\n"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.35
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.35:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.34
	.asciz	"&\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.36
anon.440927b5fc1324937d02a5fde4861f2c.36:
	.ascii	"\342\234\205 push_front works correctly!\n"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.37
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.37:
	.quad	anon.440927b5fc1324937d02a5fde4861f2c.36
	.asciz	" \000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.38
anon.440927b5fc1324937d02a5fde4861f2c.38:
	.ascii	"Node"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.39
anon.440927b5fc1324937d02a5fde4861f2c.39:
	.ascii	"data"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.40
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.40:
	.asciz	"\000\000\000\000\000\000\000\000\004\000\000\000\000\000\000\000\004\000\000\000\000\000\000"
	.quad	_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.41
anon.440927b5fc1324937d02a5fde4861f2c.41:
	.ascii	"next"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.42
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.42:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.43
anon.440927b5fc1324937d02a5fde4861f2c.43:
	.ascii	"SimpleLinkedList"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.44
anon.440927b5fc1324937d02a5fde4861f2c.44:
	.ascii	"head"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.45
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.45:
	.quad	_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.46
anon.440927b5fc1324937d02a5fde4861f2c.46:
	.ascii	"length"

	.section	.rdata,"dr",one_only,anon.440927b5fc1324937d02a5fde4861f2c.47
	.p2align	3, 0x0
anon.440927b5fc1324937d02a5fde4861f2c.47:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E

