; ModuleID = 'step2_push_front.f21d3421313a50d0-cgu.0'
source_filename = "step2_push_front.f21d3421313a50d0-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { ptr, [1 x i64] }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE" }>, align 8
@anon.440927b5fc1324937d02a5fde4861f2c.0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_560a59ed819b9d9a5841f6e731c4c8e5 = private unnamed_addr constant [210 x i8] c"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_1be5ea12ba708d9a11b6e93a7d387a75 = private unnamed_addr constant [281 x i8] c"unsafe precondition(s) violated: Layout::from_size_align_unchecked requires that align is a power of 2 and the rounded-up allocation size does not exceed isize::MAX\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_0d9b787d66cf05195f094f98b4ff7add = private unnamed_addr constant [112 x i8] c"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\alloc\\src\\alloc.rs\00", align 1
@alloc_74f44e4ed7d96f3a05f2a9c0eea355c5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0d9b787d66cf05195f094f98b4ff7add, [16 x i8] c"p\00\00\00\00\00\00\00^\01\00\00\1B\00\00\00" }>, align 8
@alloc_d427fd65f6786178d97e53921ea83763 = private unnamed_addr constant [118 x i8] c"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\ptr\\non_null.rs\00", align 1
@alloc_0ed749e82bf9e377d35421936dcf59fe = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_d427fd65f6786178d97e53921ea83763, [16 x i8] c"v\00\00\00\00\00\00\00l\05\00\00\12\00\00\00" }>, align 8
@alloc_415c7c885de7e32b61b933860bf56ba4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_d427fd65f6786178d97e53921ea83763, [16 x i8] c"v\00\00\00\00\00\00\00\09\01\00\00\1B\00\00\00" }>, align 8
@alloc_37d2e53432a03a1f90b3e7253015eaf9 = private unnamed_addr constant [4 x i8] c"None", align 1
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E" }>, align 8
@alloc_9535bf4c204f3eb9b19ec2c83e446e52 = private unnamed_addr constant [4 x i8] c"Some", align 1
@alloc_9f4ded6d0c4846c2d34cda44c77cda48 = private unnamed_addr constant [118 x i8] c"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\alloc\\layout.rs\00", align 1
@alloc_58e30ff28bf6dbae38e7bd0623f9a2ff = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9f4ded6d0c4846c2d34cda44c77cda48, [16 x i8] c"v\00\00\00\00\00\00\00\E0\00\00\00\12\00\00\00" }>, align 8
@alloc_87e23353e3958b5265f791be5ec11c90 = private unnamed_addr constant [29 x i8] c"examples\\step2_push_front.rs\00", align 1
@alloc_e971659152cc72c28b17f5024b8a7ed1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_87e23353e3958b5265f791be5ec11c90, [16 x i8] c"\1D\00\00\00\00\00\00\00\1E\00\00\00\09\00\00\00" }>, align 8
@alloc_f9dd8b59ef017d55965bd7181b29a49a = private unnamed_addr constant [32 x i8] c"=== Step 2: Adding Elements ===\0A", align 1
@alloc_577da272e05177bb1d3b83ac08a6c489 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_f9dd8b59ef017d55965bd7181b29a49a, [8 x i8] c" \00\00\00\00\00\00\00" }>, align 8
@alloc_ec41a5cae45006b33bebd751dc3764a2 = private unnamed_addr constant [15 x i8] c"Initial state: ", align 1
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant [1 x i8] c"\0A", align 1
@alloc_dd83d120edcf9d31f68abe62bc22ff09 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_ec41a5cae45006b33bebd751dc3764a2, [8 x i8] c"\0F\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_43b9c11d63090a34c3502b314fd155c9 = private unnamed_addr constant [22 x i8] c"After push_front(42): ", align 1
@alloc_a8b04fff88f9a91c726926825c91e578 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_43b9c11d63090a34c3502b314fd155c9, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_ab2cf61edd6d48f1649f7fd89e529174 = private unnamed_addr constant [8 x i8] c"Length: ", align 1
@alloc_5301d26306a7a7874f2dc8a275fdbe65 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_ab2cf61edd6d48f1649f7fd89e529174, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_f92be2ee2e511570f205efbbf90ffbda = private unnamed_addr constant [22 x i8] c"After push_front(24): ", align 1
@alloc_deefb0527b2795a277c19980d949eb9b = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_f92be2ee2e511570f205efbbf90ffbda, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_c1121a3876f0dc30bdd2b27ec51fd992 = private unnamed_addr constant [22 x i8] c"After push_front(13): ", align 1
@alloc_9f0db0f4fd112254ad720a3327c969b1 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_c1121a3876f0dc30bdd2b27ec51fd992, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_d1a7a28b1b2f6258cb14ac97db016f09 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_e10c3a22baf4007eaa52363fe452d2d8 = private unnamed_addr constant [43 x i8] c"\F0\9F\93\9D Note: Elements are added to the FRONT\0A", align 1
@alloc_fda9d316f1b0d38ebe0d132d24b4a3a1 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_e10c3a22baf4007eaa52363fe452d2d8, [8 x i8] c"+\00\00\00\00\00\00\00" }>, align 8
@alloc_656b74272813f4986b192c05e7cfd611 = private unnamed_addr constant [24 x i8] c"Order added: 42, 24, 13\0A", align 1
@alloc_a5dedbd08c212feb5cbe28c03b02db37 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_656b74272813f4986b192c05e7cfd611, [8 x i8] c"\18\00\00\00\00\00\00\00" }>, align 8
@alloc_ac461816d5689ff19cbc4325b8fb6d3e = private unnamed_addr constant [38 x i8] c"Current order in list: 13 -> 24 -> 42\0A", align 1
@alloc_0ba7f2ea839cea16a79ffc247836c5a8 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_ac461816d5689ff19cbc4325b8fb6d3e, [8 x i8] c"&\00\00\00\00\00\00\00" }>, align 8
@alloc_8b14a4d1bbe3e5a97abca52efead4eca = private unnamed_addr constant [32 x i8] c"\E2\9C\85 push_front works correctly!\0A", align 1
@alloc_f85cd5f5dcf4f3e227f2ba5d2af9037c = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_8b14a4d1bbe3e5a97abca52efead4eca, [8 x i8] c" \00\00\00\00\00\00\00" }>, align 8
@alloc_1e2d27e83e9b70f610a2d8f4ab61fc2e = private unnamed_addr constant [4 x i8] c"Node", align 1
@alloc_1143f07fefcd705b3d341a339521c9b4 = private unnamed_addr constant [4 x i8] c"data", align 1
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00", ptr @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE" }>, align 8
@alloc_33c023552cda4833100c88f89869fb29 = private unnamed_addr constant [4 x i8] c"next", align 1
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E" }>, align 8
@alloc_0e2558d2ac4413233ac01bb9ce4b00f4 = private unnamed_addr constant [16 x i8] c"SimpleLinkedList", align 1
@alloc_e5411ed34ff3a6de11928a321e62c445 = private unnamed_addr constant [4 x i8] c"head", align 1
@vtable.4 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E" }>, align 8
@alloc_dfd3ea34a2edf442f64a301697081659 = private unnamed_addr constant [6 x i8] c"length", align 1
@vtable.5 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E" }>, align 8

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17ha97314db3f698503E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call i64 @_ZN3std2rt19lang_start_internal17hfb78be9aa1b3a79cE(ptr align 1 %_7, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE"(ptr align 8 %_1) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %_1, align 8
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E(ptr %_4)
; call <() as std::process::Termination>::report
  %self = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h82e0667bab42094bE"()
  ret i32 %self
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h93836180c44b7167E(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04e0d4a76fa50f10E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8
; call <alloc::boxed::Box<T,A> as core::fmt::Debug>::fmt
  %_0 = call zeroext i1 @"_ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E"(ptr align 8 %_3, ptr align 8 %f)
  ret i1 %_0
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha32c975f6f4a2fd2E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8
; call core::fmt::num::<impl core::fmt::Debug for usize>::fmt
  %_0 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E"(ptr align 8 %_3, ptr align 8 %f)
  ret i1 %_0
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17head4e98dba6d78e1E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8
; call <core::option::Option<T> as core::fmt::Debug>::fmt
  %_0 = call zeroext i1 @"_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E"(ptr align 8 %_3, ptr align 8 %f)
  ret i1 %_0
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_v1
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces, ptr align 8 %args) unnamed_addr #1 {
start:
  store ptr %pieces, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 2, ptr %0, align 8
  %1 = load ptr, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 16
  store ptr %args, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 1, ptr %6, align 8
  ret void
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces) unnamed_addr #1 {
start:
  store ptr %pieces, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 1, ptr %0, align 8
  %1 = load ptr, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 0, ptr %6, align 8
  ret void
}

; core::fmt::rt::Argument::new_display
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E(ptr sret([16 x i8]) align 8 %_0, ptr align 8 %x) unnamed_addr #1 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %x, ptr %_2, align 8
  %0 = getelementptr inbounds i8, ptr %_2, i64 8
  store ptr @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h78b5b37ecec6e99dE", ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_2, i64 16, i1 false)
  ret void
}

; core::fmt::rt::Argument::new_debug
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E(ptr sret([16 x i8]) align 8 %_0, ptr align 8 %x) unnamed_addr #1 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %x, ptr %_2, align 8
  %0 = getelementptr inbounds i8, ptr %_2, i64 8
  store ptr @"_ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E", ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_2, i64 16, i1 false)
  ret void
}

; core::fmt::num::<impl core::fmt::Debug for i32>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hc7dc96cca43f9dfaE"(ptr align 4 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %_0 = alloca [1 x i8], align 1
  %0 = getelementptr inbounds i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8
  %_3 = and i32 %_4, 33554432
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds i8, ptr %f, i64 16
  %_6 = load i32, ptr %2, align 8
  %_5 = and i32 %_6, 67108864
  %3 = icmp eq i32 %_5, 0
  br i1 %3, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h6c8de23d5753f856E"(ptr align 4 %self, ptr align 8 %f)
  %5 = zext i1 %4 to i8
  store i8 %5, ptr %_0, align 1
  br label %bb6

bb4:                                              ; preds = %bb2
; call core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
  %6 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he4cdc560234fe893E"(ptr align 4 %self, ptr align 8 %f)
  %7 = zext i1 %6 to i8
  store i8 %7, ptr %_0, align 1
  br label %bb5

bb3:                                              ; preds = %bb2
; call core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
  %8 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h9c04d02ee22d8c1eE"(ptr align 4 %self, ptr align 8 %f)
  %9 = zext i1 %8 to i8
  store i8 %9, ptr %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6

bb6:                                              ; preds = %bb1, %bb5
  %10 = load i8, ptr %_0, align 1
  %11 = trunc nuw i8 %10 to i1
  ret i1 %11
}

; core::fmt::num::<impl core::fmt::Debug for usize>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h3b9754b602bcbf12E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %_0 = alloca [1 x i8], align 1
  %0 = getelementptr inbounds i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8
  %_3 = and i32 %_4, 33554432
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds i8, ptr %f, i64 16
  %_6 = load i32, ptr %2, align 8
  %_5 = and i32 %_6, 67108864
  %3 = icmp eq i32 %_5, 0
  br i1 %3, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for usize>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h4385590cc0e1fda7E"(ptr align 8 %self, ptr align 8 %f)
  %5 = zext i1 %4 to i8
  store i8 %5, ptr %_0, align 1
  br label %bb6

bb4:                                              ; preds = %bb2
; call core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
  %6 = call zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h78b5b37ecec6e99dE"(ptr align 8 %self, ptr align 8 %f)
  %7 = zext i1 %6 to i8
  store i8 %7, ptr %_0, align 1
  br label %bb5

bb3:                                              ; preds = %bb2
; call core::fmt::num::<impl core::fmt::UpperHex for usize>::fmt
  %8 = call zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h3c7ef623801b380dE"(ptr align 8 %self, ptr align 8 %f)
  %9 = zext i1 %8 to i8
  store i8 %9, ptr %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6

bb6:                                              ; preds = %bb1, %bb5
  %10 = load i8, ptr %_0, align 1
  %11 = trunc nuw i8 %10 to i1
  ret i1 %11
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h09a61def7a788d77E"(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 8
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E(ptr %0)
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h592f171622784320E(ptr %0) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h899e173fbf7def6cE"(ptr align 8 %_1)
          to label %bb1 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  cleanupret from %cleanuppad unwind to caller

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hd31f45c13cbaf3f1E(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<core::option::Option<alloc::boxed::Box<step2_push_front::Node<i32>>>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E"(ptr align 8 %_1) unnamed_addr #0 {
start:
  %0 = load ptr, ptr %_1, align 8
  %1 = ptrtoint ptr %0 to i64
  %2 = icmp eq i64 %1, 0
  %_2 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_2, 0
  br i1 %3, label %bb1, label %bb2

bb1:                                              ; preds = %bb2, %start
  ret void

bb2:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::boxed::Box<step2_push_front::Node<i32>>>
  call void @"_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E"(ptr align 8 %_1)
  br label %bb1
}

; core::ptr::drop_in_place<step2_push_front::Node<i32>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call core::ptr::drop_in_place<core::option::Option<alloc::boxed::Box<step2_push_front::Node<i32>>>>
  call void @"_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<step2_push_front::SimpleLinkedList<i32>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call core::ptr::drop_in_place<core::option::Option<alloc::boxed::Box<step2_push_front::Node<i32>>>>
  call void @"_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::boxed::Box<step2_push_front::Node<i32>>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr79drop_in_place$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$17h0944d4d904e0a147E"(ptr align 8 %_1) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
  %_6 = load ptr, ptr %_1, align 8
; invoke core::ptr::drop_in_place<step2_push_front::Node<i32>>
  invoke void @"_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E"(ptr align 8 %_6)
          to label %bb3 unwind label %funclet_bb4

bb4:                                              ; preds = %funclet_bb4
; call <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE"(ptr align 8 %_1) #15 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

funclet_bb4:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb4

bb3:                                              ; preds = %start
; call <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE"(ptr align 8 %_1)
  ret void
}

; core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE"(ptr %ptr, ptr align 8 %0) unnamed_addr #3 {
start:
  %_5 = alloca [16 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  %_6 = ptrtoint ptr %ptr to i64
  %1 = icmp eq i64 %_6, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %start
  %2 = getelementptr inbounds nuw { ptr, i64 }, ptr %_5, i64 0
  store ptr @alloc_560a59ed819b9d9a5841f6e731c4c8e5, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %2, i64 8
  store i64 210, ptr %3, align 8
  store ptr %_5, ptr %_3, align 8
  %4 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 1, ptr %4, align 8
  %5 = load ptr, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, align 8
  %6 = load i64, ptr getelementptr inbounds (i8, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, i64 8), align 8
  %7 = getelementptr inbounds i8, ptr %_3, i64 32
  store ptr %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %7, i64 8
  store i64 %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %_3, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 0, ptr %10, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h967cb19ee376c79aE(ptr align 8 %_3, i1 zeroext false, ptr align 8 %0) #16
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E(i64 %size, i64 %align, ptr align 8 %0) unnamed_addr #3 personality ptr @__CxxFrameHandler3 {
start:
  %_7 = alloca [16 x i8], align 8
  %_5 = alloca [48 x i8], align 8
; invoke core::alloc::layout::Layout::is_size_align_valid
  %_3 = invoke zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h404c0a2962f1f594E(i64 %size, i64 %align)
          to label %bb1 unwind label %cs_terminate

cs_terminate:                                     ; preds = %start
  %catchswitch = catchswitch within none [label %cp_terminate] unwind to caller

cp_terminate:                                     ; preds = %cs_terminate
  %catchpad = catchpad within %catchswitch [ptr null, i32 64, ptr null]
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17h8f2b96e8056bc9b0E() #17 [ "funclet"(token %catchpad) ]
  unreachable

bb1:                                              ; preds = %start
  br i1 %_3, label %bb2, label %bb3

bb3:                                              ; preds = %bb1
  %1 = getelementptr inbounds nuw { ptr, i64 }, ptr %_7, i64 0
  store ptr @alloc_1be5ea12ba708d9a11b6e93a7d387a75, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  store i64 281, ptr %2, align 8
  store ptr %_7, ptr %_5, align 8
  %3 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 1, ptr %3, align 8
  %4 = load ptr, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, align 8
  %5 = load i64, ptr getelementptr inbounds (i8, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, i64 8), align 8
  %6 = getelementptr inbounds i8, ptr %_5, i64 32
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 0, ptr %9, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h967cb19ee376c79aE(ptr align 8 %_5, i1 zeroext false, ptr align 8 %0) #16
  unreachable

bb2:                                              ; preds = %bb1
  ret void
}

; core::option::Option<T>::take
; Function Attrs: inlinehint uwtable
define internal align 8 ptr @"_ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E"(ptr align 8 %self) unnamed_addr #1 {
start:
  %_2 = alloca [8 x i8], align 8
  store ptr null, ptr %_2, align 8
  %_0 = load ptr, ptr %self, align 8
  %0 = load ptr, ptr %_2, align 8
  store ptr %0, ptr %self, align 8
  ret ptr %_0
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h82e0667bab42094bE"() unnamed_addr #1 {
start:
  ret i32 0
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint uwtable
define internal ptr @_ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E(i64 %size, i64 %align) unnamed_addr #1 {
start:
  %_4 = alloca [16 x i8], align 8
  br label %bb4

bb4:                                              ; preds = %start
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E(i64 %size, i64 %align, ptr align 8 @alloc_74f44e4ed7d96f3a05f2a9c0eea355c5) #18
  br label %bb5

bb5:                                              ; preds = %bb4
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E(ptr align 1 inttoptr (i64 1 to ptr), i64 %align, i64 %size, i1 zeroext false)
  %1 = extractvalue { ptr, i64 } %0, 0
  %2 = extractvalue { ptr, i64 } %0, 1
  store ptr %1, ptr %_4, align 8
  %3 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 %2, ptr %3, align 8
  %4 = load ptr, ptr %_4, align 8
  %5 = getelementptr inbounds i8, ptr %_4, i64 8
  %6 = load i64, ptr %5, align 8
  %7 = ptrtoint ptr %4 to i64
  %8 = icmp eq i64 %7, 0
  %_5 = select i1 %8, i64 1, i64 0
  %9 = trunc nuw i64 %_5 to i1
  br i1 %9, label %bb2, label %bb3

bb2:                                              ; preds = %bb5
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h058b0c5b7728b769E(i64 %align, i64 %size) #19
  unreachable

bb3:                                              ; preds = %bb5
  %ptr.0 = load ptr, ptr %_4, align 8
  %10 = getelementptr inbounds i8, ptr %_4, i64 8
  %ptr.1 = load i64, ptr %10, align 8
  ret ptr %ptr.0

bb1:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h024481e0e13e0025E(ptr align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #1 {
start:
  %self4 = alloca [8 x i8], align 8
  %self3 = alloca [8 x i8], align 8
  %_12 = alloca [8 x i8], align 8
  %layout2 = alloca [16 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %size = load i64, ptr %3, align 8
  %4 = icmp eq i64 %size, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %_19 = load i64, ptr %layout, align 8
  %_20 = getelementptr i8, ptr null, i64 %_19
  %data = getelementptr i8, ptr null, i64 %_19
  br label %bb7

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb7:                                              ; preds = %bb2
  %_24 = getelementptr i8, ptr null, i64 %_19
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE"(ptr %_24, ptr align 8 @alloc_0ed749e82bf9e377d35421936dcf59fe) #18
  br label %bb9

bb9:                                              ; preds = %bb7
  store ptr %data, ptr %_0, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %5, align 8
  br label %bb6

bb6:                                              ; preds = %bb21, %bb14, %bb9
  %6 = load ptr, ptr %_0, align 8
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  %8 = load i64, ptr %7, align 8
  %9 = insertvalue { ptr, i64 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i64 } %9, i64 %8, 1
  ret { ptr, i64 } %10

bb4:                                              ; preds = %bb1
  %11 = load i64, ptr %layout, align 8
  %12 = getelementptr inbounds i8, ptr %layout, i64 8
  %13 = load i64, ptr %12, align 8
  store i64 %11, ptr %layout2, align 8
  %14 = getelementptr inbounds i8, ptr %layout2, i64 8
  store i64 %13, ptr %14, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCs73fAdSrgOJL_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18
  %_41 = load i64, ptr %layout, align 8
  %_44 = icmp uge i64 %_41, 1
  %_45 = icmp ule i64 %_41, -9223372036854775808
  %_46 = and i1 %_44, %_45
; call __rustc::__rust_alloc
  %15 = call ptr @_RNvCs73fAdSrgOJL_7___rustc12___rust_alloc(i64 %size, i64 %_41) #18
  store ptr %15, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
  %16 = load i64, ptr %layout, align 8
  %17 = getelementptr inbounds i8, ptr %layout, i64 8
  %18 = load i64, ptr %17, align 8
  store i64 %16, ptr %layout1, align 8
  %19 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %18, ptr %19, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCs73fAdSrgOJL_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18
  %_31 = load i64, ptr %layout, align 8
  %_34 = icmp uge i64 %_31, 1
  %_35 = icmp ule i64 %_31, -9223372036854775808
  %_36 = and i1 %_34, %_35
; call __rustc::__rust_alloc_zeroed
  %20 = call ptr @_RNvCs73fAdSrgOJL_7___rustc19___rust_alloc_zeroed(i64 %size, i64 %_31) #18
  store ptr %20, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %ptr = load ptr, ptr %raw_ptr, align 8
  %_49 = ptrtoint ptr %ptr to i64
  %21 = icmp eq i64 %_49, 0
  br i1 %21, label %bb14, label %bb15

bb14:                                             ; preds = %bb5
  store ptr null, ptr %self4, align 8
  store ptr null, ptr %self3, align 8
  %22 = load ptr, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, align 8
  %23 = load i64, ptr getelementptr inbounds (i8, ptr @anon.440927b5fc1324937d02a5fde4861f2c.0, i64 8), align 8
  store ptr %22, ptr %_0, align 8
  %24 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %23, ptr %24, align 8
  br label %bb6

bb15:                                             ; preds = %bb5
  br label %bb16

bb16:                                             ; preds = %bb15
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE"(ptr %ptr, ptr align 8 @alloc_415c7c885de7e32b61b933860bf56ba4) #18
  br label %bb18

bb18:                                             ; preds = %bb16
  store ptr %ptr, ptr %self4, align 8
  %v = load ptr, ptr %self4, align 8
  store ptr %v, ptr %self3, align 8
  %v5 = load ptr, ptr %self3, align 8
  store ptr %v5, ptr %_12, align 8
  %ptr6 = load ptr, ptr %_12, align 8
  br label %bb19

bb19:                                             ; preds = %bb18
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h7df8dd35d3c80d1fE"(ptr %ptr6, ptr align 8 @alloc_0ed749e82bf9e377d35421936dcf59fe) #18
  br label %bb21

bb21:                                             ; preds = %bb19
  store ptr %ptr6, ptr %_0, align 8
  %25 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %size, ptr %25, align 8
  br label %bb6
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E"(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1) unnamed_addr #1 {
start:
  %layout1 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %_4 = load i64, ptr %3, align 8
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %5 = load i64, ptr %layout, align 8
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8
  store i64 %5, ptr %layout1, align 8
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %7, ptr %8, align 8
  %_11 = load i64, ptr %layout, align 8
  %_14 = icmp uge i64 %_11, 1
  %_15 = icmp ule i64 %_11, -9223372036854775808
  %_16 = and i1 %_14, %_15
; call __rustc::__rust_dealloc
  call void @_RNvCs73fAdSrgOJL_7___rustc14___rust_dealloc(ptr %ptr, i64 %_4, i64 %_11) #18
  br label %bb2
}

; <core::option::Option<T> as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h859f5fe2cb5394d0E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %__self_0 = alloca [8 x i8], align 8
  %_0 = alloca [1 x i8], align 1
  %0 = load ptr, ptr %self, align 8
  %1 = ptrtoint ptr %0 to i64
  %2 = icmp eq i64 %1, 0
  %_3 = select i1 %2, i64 0, i64 1
  %3 = trunc nuw i64 %_3 to i1
  br i1 %3, label %bb2, label %bb3

bb2:                                              ; preds = %start
  store ptr %self, ptr %__self_0, align 8
; call core::fmt::Formatter::debug_tuple_field1_finish
  %4 = call zeroext i1 @_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h470abd3bc5ca56cfE(ptr align 8 %f, ptr align 1 @alloc_9535bf4c204f3eb9b19ec2c83e446e52, i64 4, ptr align 1 %__self_0, ptr align 8 @vtable.1)
  %5 = zext i1 %4 to i8
  store i8 %5, ptr %_0, align 1
  br label %bb5

bb3:                                              ; preds = %start
; call core::fmt::Formatter::write_str
  %6 = call zeroext i1 @_ZN4core3fmt9Formatter9write_str17h4ea819d4f057104eE(ptr align 8 %f, ptr align 1 @alloc_37d2e53432a03a1f90b3e7253015eaf9, i64 4)
  %7 = zext i1 %6 to i8
  store i8 %7, ptr %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb2, %bb3
  %8 = load i8, ptr %_0, align 1
  %9 = trunc nuw i8 %8 to i1
  ret i1 %9

bb1:                                              ; No predecessors!
  unreachable
}

; <alloc::boxed::Box<T,A> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he5b3c60959ec2300E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %_4 = load ptr, ptr %self, align 8
; call <step2_push_front::Node<T> as core::fmt::Debug>::fmt
  %_0 = call zeroext i1 @"_ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE"(ptr align 8 %_4, ptr align 8 %f)
  ret i1 %_0
}

; <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint uwtable
define internal void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9bb792b0d80e27ebE"(ptr align 8 %self) unnamed_addr #1 {
start:
  %0 = alloca [8 x i8], align 8
  %1 = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %ptr = load ptr, ptr %self, align 8
  store i64 16, ptr %1, align 8
  %size = load i64, ptr %1, align 8
  store i64 8, ptr %0, align 8
  %align = load i64, ptr %0, align 8
  br label %bb6

bb6:                                              ; preds = %start
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hac471f426a006489E(i64 %size, i64 %align, ptr align 8 @alloc_58e30ff28bf6dbae38e7bd0623f9a2ff) #18
  br label %bb7

bb7:                                              ; preds = %bb6
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %size, ptr %2, align 8
  store i64 %align, ptr %layout, align 8
  %3 = icmp eq i64 %size, 0
  br i1 %3, label %bb3, label %bb1

bb3:                                              ; preds = %bb1, %bb7
  ret void

bb1:                                              ; preds = %bb7
  %_7 = getelementptr inbounds i8, ptr %self, i64 8
  %4 = load i64, ptr %layout, align 8
  %5 = getelementptr inbounds i8, ptr %layout, i64 8
  %6 = load i64, ptr %5, align 8
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hfdd69e89059e2cc4E"(ptr align 1 %_7, ptr %ptr, i64 %4, i64 %6)
  br label %bb3
}

; step2_push_front::SimpleLinkedList<T>::new
; Function Attrs: uwtable
define internal { ptr, i64 } @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E"() unnamed_addr #0 {
start:
  %_1 = alloca [8 x i8], align 8
  store ptr null, ptr %_1, align 8
  %_0.0 = load ptr, ptr %_1, align 8
  %0 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %1 = insertvalue { ptr, i64 } %0, i64 0, 1
  ret { ptr, i64 } %1
}

; step2_push_front::SimpleLinkedList<T>::push_front
; Function Attrs: uwtable
define internal void @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE"(ptr align 8 %self, i32 %data) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
  %x.i = alloca [16 x i8], align 8
  %_8 = alloca [8 x i8], align 8
; invoke core::option::Option<T>::take
  %_6 = invoke align 8 ptr @"_ZN4core6option15Option$LT$T$GT$4take17ha3fe43a2daed7105E"(ptr align 8 %self)
          to label %bb1 unwind label %funclet_bb6

bb6:                                              ; preds = %funclet_bb6
  cleanupret from %cleanuppad unwind label %funclet_bb7

funclet_bb6:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb6

bb1:                                              ; preds = %start
  store ptr %_6, ptr %x.i, align 8
  %0 = getelementptr inbounds i8, ptr %x.i, i64 8
  store i32 %data, ptr %0, align 8
; invoke alloc::alloc::exchange_malloc
  %_4.i = invoke ptr @_ZN5alloc5alloc15exchange_malloc17hf4a8f2c4ff83c758E(i64 16, i64 8)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h320d4e76c4b3ee7eE.exit" unwind label %funclet_bb2.i

funclet_bb2.i:                                    ; preds = %bb1
  %cleanuppad.i = cleanuppad within none []
; call core::ptr::drop_in_place<step2_push_front::Node<i32>>
  call void @"_ZN4core3ptr54drop_in_place$LT$step2_push_front..Node$LT$i32$GT$$GT$17hb63bb489875fd6d6E"(ptr align 8 %x.i) #15 [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h320d4e76c4b3ee7eE.exit": ; preds = %bb1
  %1 = load ptr, ptr %x.i, align 8
  %2 = getelementptr inbounds i8, ptr %x.i, i64 8
  %3 = load i32, ptr %2, align 8
  store ptr %1, ptr %_4.i, align 8
  %4 = getelementptr inbounds i8, ptr %_4.i, i64 8
  store i32 %3, ptr %4, align 8
  store ptr %_4.i, ptr %_8, align 8
; invoke core::ptr::drop_in_place<core::option::Option<alloc::boxed::Box<step2_push_front::Node<i32>>>>
  invoke void @"_ZN4core3ptr107drop_in_place$LT$core..option..Option$LT$alloc..boxed..Box$LT$step2_push_front..Node$LT$i32$GT$$GT$$GT$$GT$17hff28cf3c8a9ed194E"(ptr align 8 %self)
          to label %bb3 unwind label %funclet_bb4

bb4:                                              ; preds = %funclet_bb4
  %5 = load ptr, ptr %_8, align 8
  store ptr %5, ptr %self, align 8
  cleanupret from %cleanuppad1 unwind label %funclet_bb7

funclet_bb4:                                      ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h320d4e76c4b3ee7eE.exit"
  %cleanuppad1 = cleanuppad within none []
  br label %bb4

bb3:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h320d4e76c4b3ee7eE.exit"
  %6 = load ptr, ptr %_8, align 8
  store ptr %6, ptr %self, align 8
  %7 = getelementptr inbounds i8, ptr %self, i64 8
  %8 = load i64, ptr %7, align 8
  %9 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %8, i64 1)
  %_9.0 = extractvalue { i64, i1 } %9, 0
  %_9.1 = extractvalue { i64, i1 } %9, 1
  br i1 %_9.1, label %panic, label %bb5

bb5:                                              ; preds = %bb3
  %10 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %_9.0, ptr %10, align 8
  ret void

panic:                                            ; preds = %bb3
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h91a6092a8a656db9E(ptr align 8 @alloc_e971659152cc72c28b17f5024b8a7ed1) #19
  unreachable

bb7:                                              ; preds = %funclet_bb7
  cleanupret from %cleanuppad2 unwind to caller

funclet_bb7:                                      ; preds = %bb6, %bb4
  %cleanuppad2 = cleanuppad within none []
  br label %bb7
}

; step2_push_front::SimpleLinkedList<T>::len
; Function Attrs: uwtable
define internal i64 @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_0 = load i64, ptr %0, align 8
  ret i64 %_0
}

; step2_push_front::main
; Function Attrs: uwtable
define internal void @_ZN16step2_push_front4main17h77510b71c3eb5aadE() unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
  %_79 = alloca [48 x i8], align 8
  %_76 = alloca [48 x i8], align 8
  %_73 = alloca [48 x i8], align 8
  %_70 = alloca [48 x i8], align 8
  %_67 = alloca [48 x i8], align 8
  %_62 = alloca [8 x i8], align 8
  %_60 = alloca [16 x i8], align 8
  %args6 = alloca [16 x i8], align 8
  %_58 = alloca [48 x i8], align 8
  %_53 = alloca [16 x i8], align 8
  %args5 = alloca [16 x i8], align 8
  %_51 = alloca [48 x i8], align 8
  %_44 = alloca [8 x i8], align 8
  %_42 = alloca [16 x i8], align 8
  %args4 = alloca [16 x i8], align 8
  %_40 = alloca [48 x i8], align 8
  %_35 = alloca [16 x i8], align 8
  %args3 = alloca [16 x i8], align 8
  %_33 = alloca [48 x i8], align 8
  %_26 = alloca [8 x i8], align 8
  %_24 = alloca [16 x i8], align 8
  %args2 = alloca [16 x i8], align 8
  %_22 = alloca [48 x i8], align 8
  %_17 = alloca [16 x i8], align 8
  %args1 = alloca [16 x i8], align 8
  %_15 = alloca [48 x i8], align 8
  %_8 = alloca [16 x i8], align 8
  %args = alloca [16 x i8], align 8
  %_6 = alloca [48 x i8], align 8
  %list = alloca [16 x i8], align 8
  %_2 = alloca [48 x i8], align 8
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_2, ptr align 8 @alloc_577da272e05177bb1d3b83ac08a6c489)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_2)
; call step2_push_front::SimpleLinkedList<T>::new
  %0 = call { ptr, i64 } @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3new17hd6de2a32ef249990E"()
  %1 = extractvalue { ptr, i64 } %0, 0
  %2 = extractvalue { ptr, i64 } %0, 1
  store ptr %1, ptr %list, align 8
  %3 = getelementptr inbounds i8, ptr %list, i64 8
  store i64 %2, ptr %3, align 8
; invoke core::fmt::rt::Argument::new_debug
  invoke void @_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E(ptr sret([16 x i8]) align 8 %_8, ptr align 8 %list)
          to label %bb4 unwind label %funclet_bb42

bb42:                                             ; preds = %funclet_bb42
; call core::ptr::drop_in_place<step2_push_front::SimpleLinkedList<i32>>
  call void @"_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E"(ptr align 8 %list) #15 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

funclet_bb42:                                     ; preds = %bb39, %bb38, %bb37, %bb36, %bb35, %bb34, %bb33, %bb32, %bb31, %bb30, %bb29, %bb28, %bb27, %bb26, %bb25, %bb24, %bb23, %bb22, %bb21, %bb20, %bb19, %bb18, %bb17, %bb16, %bb15, %bb14, %bb13, %bb12, %bb11, %bb10, %bb9, %bb8, %bb7, %bb6, %bb5, %bb4, %start
  %cleanuppad = cleanuppad within none []
  br label %bb42

bb4:                                              ; preds = %start
  %4 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %4, ptr align 8 %_8, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_6, ptr align 8 @alloc_dd83d120edcf9d31f68abe62bc22ff09, ptr align 8 %args)
          to label %bb5 unwind label %funclet_bb42

bb5:                                              ; preds = %bb4
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_6)
          to label %bb6 unwind label %funclet_bb42

bb6:                                              ; preds = %bb5
; invoke step2_push_front::SimpleLinkedList<T>::push_front
  invoke void @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE"(ptr align 8 %list, i32 42)
          to label %bb7 unwind label %funclet_bb42

bb7:                                              ; preds = %bb6
; invoke core::fmt::rt::Argument::new_debug
  invoke void @_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E(ptr sret([16 x i8]) align 8 %_17, ptr align 8 %list)
          to label %bb8 unwind label %funclet_bb42

bb8:                                              ; preds = %bb7
  %5 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args1, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %5, ptr align 8 %_17, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_15, ptr align 8 @alloc_a8b04fff88f9a91c726926825c91e578, ptr align 8 %args1)
          to label %bb9 unwind label %funclet_bb42

bb9:                                              ; preds = %bb8
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_15)
          to label %bb10 unwind label %funclet_bb42

bb10:                                             ; preds = %bb9
; invoke step2_push_front::SimpleLinkedList<T>::len
  %6 = invoke i64 @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE"(ptr align 8 %list)
          to label %bb11 unwind label %funclet_bb42

bb11:                                             ; preds = %bb10
  store i64 %6, ptr %_26, align 8
; invoke core::fmt::rt::Argument::new_display
  invoke void @_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E(ptr sret([16 x i8]) align 8 %_24, ptr align 8 %_26)
          to label %bb12 unwind label %funclet_bb42

bb12:                                             ; preds = %bb11
  %7 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args2, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %7, ptr align 8 %_24, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_22, ptr align 8 @alloc_5301d26306a7a7874f2dc8a275fdbe65, ptr align 8 %args2)
          to label %bb13 unwind label %funclet_bb42

bb13:                                             ; preds = %bb12
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_22)
          to label %bb14 unwind label %funclet_bb42

bb14:                                             ; preds = %bb13
; invoke step2_push_front::SimpleLinkedList<T>::push_front
  invoke void @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE"(ptr align 8 %list, i32 24)
          to label %bb15 unwind label %funclet_bb42

bb15:                                             ; preds = %bb14
; invoke core::fmt::rt::Argument::new_debug
  invoke void @_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E(ptr sret([16 x i8]) align 8 %_35, ptr align 8 %list)
          to label %bb16 unwind label %funclet_bb42

bb16:                                             ; preds = %bb15
  %8 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args3, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %8, ptr align 8 %_35, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_33, ptr align 8 @alloc_deefb0527b2795a277c19980d949eb9b, ptr align 8 %args3)
          to label %bb17 unwind label %funclet_bb42

bb17:                                             ; preds = %bb16
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_33)
          to label %bb18 unwind label %funclet_bb42

bb18:                                             ; preds = %bb17
; invoke step2_push_front::SimpleLinkedList<T>::len
  %9 = invoke i64 @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE"(ptr align 8 %list)
          to label %bb19 unwind label %funclet_bb42

bb19:                                             ; preds = %bb18
  store i64 %9, ptr %_44, align 8
; invoke core::fmt::rt::Argument::new_display
  invoke void @_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E(ptr sret([16 x i8]) align 8 %_42, ptr align 8 %_44)
          to label %bb20 unwind label %funclet_bb42

bb20:                                             ; preds = %bb19
  %10 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args4, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %10, ptr align 8 %_42, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_40, ptr align 8 @alloc_5301d26306a7a7874f2dc8a275fdbe65, ptr align 8 %args4)
          to label %bb21 unwind label %funclet_bb42

bb21:                                             ; preds = %bb20
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_40)
          to label %bb22 unwind label %funclet_bb42

bb22:                                             ; preds = %bb21
; invoke step2_push_front::SimpleLinkedList<T>::push_front
  invoke void @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$10push_front17hb3d466c2ce7575deE"(ptr align 8 %list, i32 13)
          to label %bb23 unwind label %funclet_bb42

bb23:                                             ; preds = %bb22
; invoke core::fmt::rt::Argument::new_debug
  invoke void @_ZN4core3fmt2rt8Argument9new_debug17hc2705ffff6474ba8E(ptr sret([16 x i8]) align 8 %_53, ptr align 8 %list)
          to label %bb24 unwind label %funclet_bb42

bb24:                                             ; preds = %bb23
  %11 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args5, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %11, ptr align 8 %_53, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_51, ptr align 8 @alloc_9f0db0f4fd112254ad720a3327c969b1, ptr align 8 %args5)
          to label %bb25 unwind label %funclet_bb42

bb25:                                             ; preds = %bb24
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_51)
          to label %bb26 unwind label %funclet_bb42

bb26:                                             ; preds = %bb25
; invoke step2_push_front::SimpleLinkedList<T>::len
  %12 = invoke i64 @"_ZN16step2_push_front25SimpleLinkedList$LT$T$GT$3len17hbad23edebfe5803cE"(ptr align 8 %list)
          to label %bb27 unwind label %funclet_bb42

bb27:                                             ; preds = %bb26
  store i64 %12, ptr %_62, align 8
; invoke core::fmt::rt::Argument::new_display
  invoke void @_ZN4core3fmt2rt8Argument11new_display17h92c59fd5fd2227e9E(ptr sret([16 x i8]) align 8 %_60, ptr align 8 %_62)
          to label %bb28 unwind label %funclet_bb42

bb28:                                             ; preds = %bb27
  %13 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args6, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %13, ptr align 8 %_60, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hf8fc62c181ca0550E"(ptr sret([48 x i8]) align 8 %_58, ptr align 8 @alloc_5301d26306a7a7874f2dc8a275fdbe65, ptr align 8 %args6)
          to label %bb29 unwind label %funclet_bb42

bb29:                                             ; preds = %bb28
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_58)
          to label %bb30 unwind label %funclet_bb42

bb30:                                             ; preds = %bb29
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_const
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_67, ptr align 8 @alloc_d1a7a28b1b2f6258cb14ac97db016f09)
          to label %bb31 unwind label %funclet_bb42

bb31:                                             ; preds = %bb30
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_67)
          to label %bb32 unwind label %funclet_bb42

bb32:                                             ; preds = %bb31
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_const
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_70, ptr align 8 @alloc_fda9d316f1b0d38ebe0d132d24b4a3a1)
          to label %bb33 unwind label %funclet_bb42

bb33:                                             ; preds = %bb32
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_70)
          to label %bb34 unwind label %funclet_bb42

bb34:                                             ; preds = %bb33
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_const
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_73, ptr align 8 @alloc_a5dedbd08c212feb5cbe28c03b02db37)
          to label %bb35 unwind label %funclet_bb42

bb35:                                             ; preds = %bb34
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_73)
          to label %bb36 unwind label %funclet_bb42

bb36:                                             ; preds = %bb35
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_const
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_76, ptr align 8 @alloc_0ba7f2ea839cea16a79ffc247836c5a8)
          to label %bb37 unwind label %funclet_bb42

bb37:                                             ; preds = %bb36
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_76)
          to label %bb38 unwind label %funclet_bb42

bb38:                                             ; preds = %bb37
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_const
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3ee5cd5eeeb711d5E"(ptr sret([48 x i8]) align 8 %_79, ptr align 8 @alloc_f85cd5f5dcf4f3e227f2ba5d2af9037c)
          to label %bb39 unwind label %funclet_bb42

bb39:                                             ; preds = %bb38
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8 %_79)
          to label %bb40 unwind label %funclet_bb42

bb40:                                             ; preds = %bb39
; call core::ptr::drop_in_place<step2_push_front::SimpleLinkedList<i32>>
  call void @"_ZN4core3ptr66drop_in_place$LT$step2_push_front..SimpleLinkedList$LT$i32$GT$$GT$17h9a049a8cd3919340E"(ptr align 8 %list)
  ret void
}

; <step2_push_front::Node<T> as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN68_$LT$step2_push_front..Node$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdabc4f3f4a4a96fcE"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %_10 = alloca [8 x i8], align 8
  %_6 = getelementptr inbounds i8, ptr %self, i64 8
  store ptr %self, ptr %_10, align 8
; call core::fmt::Formatter::debug_struct_field2_finish
  %_0 = call zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field2_finish17h4b6cb7e667c5a162E(ptr align 8 %f, ptr align 1 @alloc_1e2d27e83e9b70f610a2d8f4ab61fc2e, i64 4, ptr align 1 @alloc_1143f07fefcd705b3d341a339521c9b4, i64 4, ptr align 1 %_6, ptr align 8 @vtable.2, ptr align 1 @alloc_33c023552cda4833100c88f89869fb29, i64 4, ptr align 1 %_10, ptr align 8 @vtable.3)
  ret i1 %_0
}

; <step2_push_front::SimpleLinkedList<T> as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN80_$LT$step2_push_front..SimpleLinkedList$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0db6419ad0801a4E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %_10 = alloca [8 x i8], align 8
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  store ptr %0, ptr %_10, align 8
; call core::fmt::Formatter::debug_struct_field2_finish
  %_0 = call zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field2_finish17h4b6cb7e667c5a162E(ptr align 8 %f, ptr align 1 @alloc_0e2558d2ac4413233ac01bb9ce4b00f4, i64 16, ptr align 1 @alloc_e5411ed34ff3a6de11928a321e62c445, i64 4, ptr align 1 %self, ptr align 8 @vtable.4, ptr align 1 @alloc_dfd3ea34a2edf442f64a301697081659, i64 6, ptr align 1 %_10, ptr align 8 @vtable.5)
  ret i1 %_0
}

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17hfb78be9aa1b3a79cE(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h78b5b37ecec6e99dE"(ptr align 8, ptr align 8) unnamed_addr #0

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #4

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he4cdc560234fe893E"(ptr align 4, ptr align 8) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h9c04d02ee22d8c1eE"(ptr align 4, ptr align 8) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h6c8de23d5753f856E"(ptr align 4, ptr align 8) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h3c7ef623801b380dE"(ptr align 8, ptr align 8) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h4385590cc0e1fda7E"(ptr align 8, ptr align 8) unnamed_addr #0

declare i32 @__CxxFrameHandler3(...) unnamed_addr #5

; core::panicking::panic_nounwind_fmt
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking18panic_nounwind_fmt17h967cb19ee376c79aE(ptr align 8, i1 zeroext, ptr align 8) unnamed_addr #6

; core::alloc::layout::Layout::is_size_align_valid
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h404c0a2962f1f594E(i64, i64) unnamed_addr #0

; core::panicking::panic_cannot_unwind
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_ZN4core9panicking19panic_cannot_unwind17h8f2b96e8056bc9b0E() unnamed_addr #7

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h058b0c5b7728b769E(i64, i64) unnamed_addr #8

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCs73fAdSrgOJL_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #9

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias ptr @_RNvCs73fAdSrgOJL_7___rustc12___rust_alloc(i64, i64 allocalign) unnamed_addr #10

; __rustc::__rust_alloc_zeroed
; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias ptr @_RNvCs73fAdSrgOJL_7___rustc19___rust_alloc_zeroed(i64, i64 allocalign) unnamed_addr #11

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCs73fAdSrgOJL_7___rustc14___rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #12

; core::fmt::Formatter::write_str
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter9write_str17h4ea819d4f057104eE(ptr align 8, ptr align 1, i64) unnamed_addr #0

; core::fmt::Formatter::debug_tuple_field1_finish
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h470abd3bc5ca56cfE(ptr align 8, ptr align 1, i64, ptr align 1, ptr align 8) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #13

; core::panicking::panic_const::panic_const_add_overflow
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h91a6092a8a656db9E(ptr align 8) unnamed_addr #14

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h8b3b46477b515abcE(ptr align 8) unnamed_addr #0

; core::fmt::Formatter::debug_struct_field2_finish
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field2_finish17h4b6cb7e667c5a162E(ptr align 8, ptr align 1, i64, ptr align 1, i64, ptr align 1, ptr align 8, ptr align 1, i64, ptr align 1, ptr align 8) unnamed_addr #0

define i32 @main(i32 %0, ptr %1) unnamed_addr #5 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17ha97314db3f698503E(ptr @_ZN16step2_push_front4main17h77510b71c3eb5aadE, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #1 = { inlinehint uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #2 = { noinline uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #3 = { inlinehint nounwind uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { "target-cpu"="x86-64" }
attributes #6 = { cold noinline noreturn nounwind uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #7 = { cold minsize noinline noreturn nounwind optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #8 = { cold minsize noreturn optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #9 = { nounwind uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #10 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #11 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #12 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #13 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #14 = { cold noinline noreturn uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #15 = { cold }
attributes #16 = { noreturn nounwind }
attributes #17 = { cold noreturn nounwind }
attributes #18 = { nounwind }
attributes #19 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.89.0 (29483883e 2025-08-04)"}
!3 = !{i64 11707466671298494}
