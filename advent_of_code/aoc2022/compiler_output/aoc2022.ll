; ModuleID = '8fkt43vh06dzz96dgkmk541q1'
source_filename = "8fkt43vh06dzz96dgkmk541q1"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<u8>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<u8>" = type {}
%"std::backtrace::BacktraceFrame" = type { %"alloc::vec::Vec<std::backtrace::BacktraceSymbol>", %"std::backtrace::RawFrame::Actual" }
%"alloc::vec::Vec<std::backtrace::BacktraceSymbol>" = type { %"alloc::raw_vec::RawVec<std::backtrace::BacktraceSymbol>", i64 }
%"alloc::raw_vec::RawVec<std::backtrace::BacktraceSymbol>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<std::backtrace::BacktraceSymbol>" }
%"core::marker::PhantomData<std::backtrace::BacktraceSymbol>" = type {}
%"std::backtrace::RawFrame::Actual" = type { %"std::backtrace_rs::backtrace::Frame" }
%"std::backtrace_rs::backtrace::Frame" = type { %"std::backtrace_rs::backtrace::win64::Frame" }
%"std::backtrace_rs::backtrace::win64::Frame" = type { %"core::option::Option<u32>", ptr, ptr, ptr }
%"core::option::Option<u32>" = type { i32, [1 x i32] }
%"std::backtrace::BacktraceSymbol" = type { %"core::option::Option<std::backtrace::BytesOrWide>", %"core::option::Option<alloc::vec::Vec<u8>>", %"core::option::Option<u32>", %"core::option::Option<u32>" }
%"core::option::Option<std::backtrace::BytesOrWide>" = type { i64, [3 x i64] }
%"core::option::Option<alloc::vec::Vec<u8>>" = type { i64, [2 x i64] }

@alloc_d0776666182ad032bd1011cf266e2f3a = private unnamed_addr constant [16 x i8] c"\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF", align 16
@anon.44ffa63e8e95c400711a21744c5ea708.0 = private unnamed_addr constant <{ ptr, [24 x i8] }> <{ ptr @alloc_d0776666182ad032bd1011cf266e2f3a, [24 x i8] zeroinitializer }>, align 8
@alloc_2637060db1ca70c0d739ccdf7494925d = private unnamed_addr constant [113 x i8] c"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\str\\mod.rs\00", align 1
@alloc_90427cacc85724e4d3b32dbfd394b367 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2637060db1ca70c0d739ccdf7494925d, [16 x i8] c"p\00\00\00\00\00\00\00A\03\00\00\15\00\00\00" }>, align 8
@alloc_e1e44f68f3078611efc2362a115f5742 = private unnamed_addr constant [117 x i8] c"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\core\\src\\str\\pattern.rs\00", align 1
@alloc_960bc1bf861ba41d9b8231bbd37f66f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e1e44f68f3078611efc2362a115f5742, [16 x i8] c"t\00\00\00\00\00\00\00\E4\05\00\00\14\00\00\00" }>, align 8
@alloc_3c328170803c8011e86f11240ac4582e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e1e44f68f3078611efc2362a115f5742, [16 x i8] c"t\00\00\00\00\00\00\00\E4\05\00\00!\00\00\00" }>, align 8
@alloc_aeae60839ee01e593e8491fb61f2dda8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e1e44f68f3078611efc2362a115f5742, [16 x i8] c"t\00\00\00\00\00\00\00\D8\05\00\00!\00\00\00" }>, align 8
@alloc_04d7ce44d7c86a9a02b346ab945bf155 = private unnamed_addr constant [40 x i8] c"description() is deprecated; use Display", align 1
@anon.44ffa63e8e95c400711a21744c5ea708.1 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 522596451624065841 to ptr), ptr inttoptr (i64 1648355301501614964 to ptr) }>, align 8
@anon.44ffa63e8e95c400711a21744c5ea708.2 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 2400468019378343764 to ptr), ptr inttoptr (i64 8658629039619007897 to ptr) }>, align 8
@anon.44ffa63e8e95c400711a21744c5ea708.3 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 -5882582162307352880 to ptr), ptr inttoptr (i64 7790121011132037916 to ptr) }>, align 8
@anon.44ffa63e8e95c400711a21744c5ea708.4 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 3792400530729822680 to ptr), ptr inttoptr (i64 -9154057084057117283 to ptr) }>, align 8
@_ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE = external dllimport local_unnamed_addr global [256 x i8]
@alloc_f7f1045810c6dd7020c5cff081af73be = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E", [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E" }>, align 8
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E", [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h0607a4d5b05d13c1E", ptr @"_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E", ptr @alloc_f7f1045810c6dd7020c5cff081af73be, ptr @_ZN4core5error5Error5cause17h7a349d6c962996eeE, ptr @_ZN4core5error5Error7type_id17h01cfa94765d144f3E, ptr @_ZN4core5error5Error11description17h1649966410d8c91fE, ptr @_ZN4core5error5Error5cause17h7a349d6c962996eeE, ptr @_ZN4core5error5Error7provide17h9df6819c0298c953E }>, align 8
@alloc_0259449508b7e8fad72c8ab224522166 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h663cff1b8df227b7E", ptr @"_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE", ptr @alloc_0259449508b7e8fad72c8ab224522166, ptr @_ZN4core5error5Error5cause17h7a349d6c962996eeE, ptr @_ZN4core5error5Error7type_id17hdc7adb8570ca242eE, ptr @_ZN4core5error5Error11description17h1649966410d8c91fE, ptr @_ZN4core5error5Error5cause17h7a349d6c962996eeE, ptr @_ZN4core5error5Error7provide17h9df6819c0298c953E }>, align 8
@alloc_1920c41e2cf13e8b20c82fe3438a9aa9 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE", [16 x i8] c"H\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE" }>, align 8
@vtable.3 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE", [16 x i8] c"H\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E", ptr @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE", ptr @alloc_1920c41e2cf13e8b20c82fe3438a9aa9, ptr @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E", ptr @_ZN4core5error5Error7type_id17h515f3f39467e6574E, ptr @_ZN4core5error5Error11description17h6d684e2ae41137b1E, ptr @_ZN4core5error5Error5cause17h3780c7e2d97e9501E, ptr @_ZN4core5error5Error7provide17h05d6e66acb486045E }>, align 8
@alloc_e2f138f525d34441c9f70ee5bc3a1e34 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E", [16 x i8] c"P\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE" }>, align 8
@vtable.4 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E", [16 x i8] c"P\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E", ptr @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE", ptr @alloc_e2f138f525d34441c9f70ee5bc3a1e34, ptr @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E", ptr @_ZN4core5error5Error7type_id17hc7a2819971514648E, ptr @_ZN4core5error5Error11description17h6d684e2ae41137b1E, ptr @_ZN4core5error5Error5cause17h3780c7e2d97e9501E, ptr @_ZN4core5error5Error7provide17h05d6e66acb486045E }>, align 8
@alloc_a04e47d083146d15ce3892a825ec94b0 = private unnamed_addr constant <{ ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_ZN6anyhow5error11object_drop17h9334d5975c0625abE, ptr @_ZN6anyhow5error10object_ref17h3950046690ebc90bE, ptr @_ZN6anyhow5error12object_boxed17hdd48d83f6f5fca94E, ptr @_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E, ptr @_ZN6anyhow5error15object_downcast17h9f35c6fdf8d2f673E, ptr @_ZN6anyhow5error17object_drop_front17h468434592611aba3E, ptr @_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE }>, align 8
@alloc_00e51742134d344daa7116ffd2ad9e35 = private unnamed_addr constant <{ ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE, ptr @_ZN6anyhow5error10object_ref17h0ce20ab8fd8e770aE, ptr @_ZN6anyhow5error12object_boxed17hde21296211696ba2E, ptr @_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E, ptr @_ZN6anyhow5error15object_downcast17hefd3fb1cb0d21f49E, ptr @_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E, ptr @_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE }>, align 8
@alloc_3f62f09340ec4217b72fe8840b861b6c = private unnamed_addr constant [2 x i8] c"\0A\0A", align 1
@alloc_53973d2fe29b4adba8bb7390b5678745 = private unnamed_addr constant [8 x i8] zeroinitializer, align 8
@alloc_053734e4d3889d3cd42e215952f6be3c = private unnamed_addr constant [43 x i8] c"advent_of_code\\aoc2022\\src\\solver\\day02.rs\00", align 1
@anon.44ffa63e8e95c400711a21744c5ea708.5 = private unnamed_addr constant [9 x i8] c"\04\08\03\01\05\09\07\02\06", align 1
@alloc_04ec0caaeb79a0ce9c7e48871a54c01a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_053734e4d3889d3cd42e215952f6be3c, [16 x i8] c"*\00\00\00\00\00\00\00U\00\00\00\1C\00\00\00" }>, align 8
@anon.44ffa63e8e95c400711a21744c5ea708.6 = private unnamed_addr constant [9 x i8] c"\03\04\08\01\05\09\02\06\07", align 1
@alloc_a1b3d90f42852e8e0faf60831e20d8b1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_053734e4d3889d3cd42e215952f6be3c, [16 x i8] c"*\00\00\00\00\00\00\00`\00\00\00 \00\00\00" }>, align 8
@alloc_cb2aea7e2fdb2fba562edabf1f950868 = private unnamed_addr constant [155 x i8] c"\04Day \C0; not implemented yet (valid range: 1-25). To implement day \C8\00\00\17, create src/solver/day\CB \00\00i\02\00\00\003.rs and uncomment the corresponding lines in mod.rs\00", align 1
@alloc_8af13d31a0ec5c8f7fb83c7ff891ca76 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e1e44f68f3078611efc2362a115f5742, [16 x i8] c"t\00\00\00\00\00\00\00h\04\00\00$\00\00\00" }>, align 8
@alloc_aa47418bb16f08c24f7eacc7bfd02189 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e1e44f68f3078611efc2362a115f5742, [16 x i8] c"t\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8
@alloc_a931397211c33a1c8fe0d17838460834 = private unnamed_addr constant [60 x i8] c"internal error: entered unreachable code: invalid Once state", align 1
@alloc_2b70d7ef093544d5f03f6771e2a94a2c = private unnamed_addr constant [124 x i8] c"C:\\Users\\m_lad\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\std\\src\\sys\\sync\\once\\futex.rs\00", align 1
@alloc_6a6fc231b3cb64280fdbf03fad4b13a2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2b70d7ef093544d5f03f6771e2a94a2c, [16 x i8] c"{\00\00\00\00\00\00\00[\00\00\00\12\00\00\00" }>, align 8

; <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { i64, i64 } @"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E"(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(128) %self) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
  %_4.i.i.i.i3 = alloca [72 x i8], align 8
  %_5.i.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 121
  %1 = load i8, ptr %0, align 1, !range !8, !alias.scope !9, !noundef !10
  %_2.i.i = trunc nuw i8 %1 to i1
  br i1 %_2.i.i, label %bb2, label %bb2.i.i

bb2.i.i:                                          ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %self.val.i.i = load ptr, ptr %2, align 8, !alias.scope !9, !nonnull !10, !align !11, !noundef !10
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 80
  %self.val1.i.i = load i64, ptr %3, align 8, !alias.scope !9, !noundef !10
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i), !noalias !9
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15)
  %_2.i.i.i = load i64, ptr %self, align 8, !range !17, !alias.scope !18, !noalias !12, !noundef !10
  %4 = trunc nuw i64 %_2.i.i.i to i1
  br i1 %4, label %bb2.i.i.i, label %bb3.i.critedge.i.i.i

bb2.i.i.i:                                        ; preds = %bb2.i.i
  %searcher.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_10.i.i.i = load i64, ptr %5, align 8, !alias.scope !18, !noalias !12, !noundef !10
  %is_long.i.i.i = icmp eq i64 %_10.i.i.i, -1
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 88
  %self.05.i.i.i = load ptr, ptr %6, align 8, !alias.scope !18, !noalias !12, !nonnull !10, !align !11, !noundef !10
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %self.16.i.i.i = load i64, ptr %7, align 8, !alias.scope !18, !noalias !12, !noundef !10
  br i1 %is_long.i.i.i, label %bb8.i.i.i, label %bb9.i.i.i

bb3.i.critedge.i.i.i:                             ; preds = %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19)
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 26
  %10 = load i8, ptr %9, align 2, !range !8, !alias.scope !22, !noalias !23, !noundef !10
  %_4.i.i.i.i = trunc nuw i8 %10 to i1
  br i1 %_4.i.i.i.i, label %bb10.i.i.i, label %bb5.i.lr.ph.i.i.i

bb5.i.lr.ph.i.i.i:                                ; preds = %bb3.i.critedge.i.i.i
  %.promoted.i.i.i = load i64, ptr %8, align 8, !alias.scope !18, !noalias !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_48.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %self.val1.i.i
  %.promoted29.i.i.i = load i8, ptr %11, align 8, !alias.scope !22, !noalias !23
  %is_match.i.i.peel.i.i = trunc nuw i8 %.promoted29.i.i.i to i1
  %_8.i.i.i.peel.i.i = icmp eq i64 %.promoted.i.i.i, 0
  br i1 %_8.i.i.i.peel.i.i, label %bb19.i.i.peel.i.i, label %bb5.i.i.i.peel.i.i

bb5.i.i.i.peel.i.i:                               ; preds = %bb5.i.lr.ph.i.i.i
  %_9.not.i.i.i.peel.i.i = icmp ult i64 %.promoted.i.i.i, %self.val1.i.i
  br i1 %_9.not.i.i.i.peel.i.i, label %bb9.i.i.i.peel.i.i, label %bb6.i.i.i.peel.i.i

bb6.i.i.i.peel.i.i:                               ; preds = %bb5.i.i.i.peel.i.i
  %12 = icmp eq i64 %.promoted.i.i.i, %self.val1.i.i
  br i1 %12, label %bb19.i.i.peel.i.i, label %bb18.i.i.i.i

bb9.i.i.i.peel.i.i:                               ; preds = %bb5.i.i.i.peel.i.i
  %13 = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %.promoted.i.i.i
  %self1.i.i.i.peel.i.i = load i8, ptr %13, align 1, !alias.scope !25, !noalias !28, !noundef !10
  %14 = icmp sgt i8 %self1.i.i.i.peel.i.i, -65
  br i1 %14, label %bb19.i.i.peel.i.i, label %bb18.i.i.i.i

bb19.i.i.peel.i.i:                                ; preds = %bb9.i.i.i.peel.i.i, %bb6.i.i.i.peel.i.i, %bb5.i.lr.ph.i.i.i
  %data.i.i.i.peel.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %.promoted.i.i.i
  %_7.i.i.i.i.peel.i.i = icmp samesign eq i64 %.promoted.i.i.i, %self.val1.i.i
  br i1 %_7.i.i.i.i.peel.i.i, label %bb21.i.i.i.i, label %bb14.i.i.i.peel.i.i

bb14.i.i.i.peel.i.i:                              ; preds = %bb19.i.i.peel.i.i
  %x.i.i.i.peel.i.i = load i8, ptr %data.i.i.i.peel.i.i, align 1, !noalias !30, !noundef !10
  %_6.i.i.i.peel.i.i = icmp sgt i8 %x.i.i.i.peel.i.i, -1
  br i1 %_6.i.i.i.peel.i.i, label %bb3.i.i.i.peel.i.i, label %bb4.i.i.i.peel.i.i

bb4.i.i.i.peel.i.i:                               ; preds = %bb14.i.i.i.peel.i.i
  %_18.i.i.i.i.peel.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i.i, i64 1
  %_30.i.i.i.peel.i.i = and i8 %x.i.i.i.peel.i.i, 31
  %init.i.i.i.peel.i.i = zext nneg i8 %_30.i.i.i.peel.i.i to i32
  %_7.i10.i.i.i.peel.i.i = icmp ne ptr %_18.i.i.i.i.peel.i.i, %_48.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i10.i.i.i.peel.i.i)
  %y.i.i.i.peel.i.i = load i8, ptr %_18.i.i.i.i.peel.i.i, align 1, !noalias !30, !noundef !10
  %_34.i.i.i.peel.i.i = shl nuw nsw i32 %init.i.i.i.peel.i.i, 6
  %_36.i.i.i.peel.i.i = and i8 %y.i.i.i.peel.i.i, 63
  %_35.i.i.i.peel.i.i = zext nneg i8 %_36.i.i.i.peel.i.i to i32
  %15 = or disjoint i32 %_34.i.i.i.peel.i.i, %_35.i.i.i.peel.i.i
  %_13.i.i.i.peel.i.i = icmp samesign ugt i8 %x.i.i.i.peel.i.i, -33
  br i1 %_13.i.i.i.peel.i.i, label %bb6.i21.i.i.peel.i.i, label %bb22.i.i.peel.i.i

bb6.i21.i.i.peel.i.i:                             ; preds = %bb4.i.i.i.peel.i.i
  %_18.i12.i.i.i.peel.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i.i, i64 2
  %_7.i17.i.i.i.peel.i.i = icmp ne ptr %_18.i12.i.i.i.peel.i.i, %_48.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i17.i.i.i.peel.i.i)
  %z.i.i.i.peel.i.i = load i8, ptr %_18.i12.i.i.i.peel.i.i, align 1, !noalias !30, !noundef !10
  %_40.i.i.i.peel.i.i = shl nuw nsw i32 %_35.i.i.i.peel.i.i, 6
  %_42.i.i.i.peel.i.i = and i8 %z.i.i.i.peel.i.i, 63
  %_41.i.i.i.peel.i.i = zext nneg i8 %_42.i.i.i.peel.i.i to i32
  %y_z.i.i.i.peel.i.i = or disjoint i32 %_40.i.i.i.peel.i.i, %_41.i.i.i.peel.i.i
  %_20.i.i.i.peel.i.i = shl nuw nsw i32 %init.i.i.i.peel.i.i, 12
  %16 = or disjoint i32 %y_z.i.i.i.peel.i.i, %_20.i.i.i.peel.i.i
  %_21.i.i.i.peel.i.i = icmp samesign ugt i8 %x.i.i.i.peel.i.i, -17
  br i1 %_21.i.i.i.peel.i.i, label %bb8.i.i.i.peel.i.i, label %bb22.i.i.peel.i.i

bb8.i.i.i.peel.i.i:                               ; preds = %bb6.i21.i.i.peel.i.i
  %_18.i19.i.i.i.peel.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i.i, i64 3
  %_7.i24.i.i.i.peel.i.i = icmp ne ptr %_18.i19.i.i.i.peel.i.i, %_48.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i24.i.i.i.peel.i.i)
  %w.i.i.i.peel.i.i = load i8, ptr %_18.i19.i.i.i.peel.i.i, align 1, !noalias !30, !noundef !10
  %_26.i.i.i.peel.i.i = shl nuw nsw i32 %init.i.i.i.peel.i.i, 18
  %_25.i.i.i.peel.i.i = and i32 %_26.i.i.i.peel.i.i, 1835008
  %_46.i.i.i.peel.i.i = shl nuw nsw i32 %y_z.i.i.i.peel.i.i, 6
  %_48.i.i.i.peel.i.i = and i8 %w.i.i.i.peel.i.i, 63
  %_47.i.i.i.peel.i.i = zext nneg i8 %_48.i.i.i.peel.i.i to i32
  %_27.i.i.i.peel.i.i = or disjoint i32 %_46.i.i.i.peel.i.i, %_47.i.i.i.peel.i.i
  %17 = or disjoint i32 %_27.i.i.i.peel.i.i, %_25.i.i.i.peel.i.i
  br label %bb22.i.i.peel.i.i

bb3.i.i.i.peel.i.i:                               ; preds = %bb14.i.i.i.peel.i.i
  %_7.i.i.i.peel.i.i = zext nneg i8 %x.i.i.i.peel.i.i to i32
  br label %bb22.i.i.peel.i.i

bb22.i.i.peel.i.i:                                ; preds = %bb3.i.i.i.peel.i.i, %bb8.i.i.i.peel.i.i, %bb6.i21.i.i.peel.i.i, %bb4.i.i.i.peel.i.i
  %_0.sroa.4.0.i.ph.i.i.peel.i.i = phi i32 [ %15, %bb4.i.i.i.peel.i.i ], [ %16, %bb6.i21.i.i.peel.i.i ], [ %17, %bb8.i.i.i.peel.i.i ], [ %_7.i.i.i.peel.i.i, %bb3.i.i.i.peel.i.i ]
  %18 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.i, 1114112
  tail call void @llvm.assume(i1 %18)
  br i1 %is_match.i.i.peel.i.i, label %bb7.i.sink.split.i.i, label %bb39.i.i.peel.i.i

bb39.i.i.peel.i.i:                                ; preds = %bb22.i.i.peel.i.i
  %_60.i.i.peel.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.i, 128
  br i1 %_60.i.i.peel.i.i, label %bb5.i.i.i.i, label %bb26.i.i.peel.i.i

bb26.i.i.peel.i.i:                                ; preds = %bb39.i.i.peel.i.i
  %_61.i.i.peel.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.i, 2048
  br i1 %_61.i.i.peel.i.i, label %bb5.i.i.i.i, label %bb27.i.i.peel.i.i

bb27.i.i.peel.i.i:                                ; preds = %bb26.i.i.peel.i.i
  %_62.i.i.peel.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.i, 65536
  %..i.i.peel.i.i = select i1 %_62.i.i.peel.i.i, i64 3, i64 4
  br label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb27.i.i.peel.i.i, %bb26.i.i.peel.i.i, %bb39.i.i.peel.i.i
  %_14.sroa.0.0.i.i.peel.i.i = phi i64 [ 1, %bb39.i.i.peel.i.i ], [ %..i.i.peel.i.i, %bb27.i.i.peel.i.i ], [ 2, %bb26.i.i.peel.i.i ]
  %19 = add i64 %_14.sroa.0.0.i.i.peel.i.i, %.promoted.i.i.i
  store i64 %19, ptr %8, align 8, !alias.scope !33, !noalias !23
  tail call void @llvm.experimental.noalias.scope.decl(metadata !34)
  %_8.i.i.i.i.i = icmp eq i64 %19, 0
  br i1 %_8.i.i.i.i.i, label %bb19.i.i.i.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i
  %_9.not.i.i.i.i.i = icmp ult i64 %19, %self.val1.i.i
  br i1 %_9.not.i.i.i.i.i, label %bb9.i.i.i.i.i, label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i.i
  %20 = icmp eq i64 %19, %self.val1.i.i
  br i1 %20, label %bb19.i.i.i.i, label %bb18.i.i.i.i

bb9.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i.i
  %21 = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %19
  %self1.i.i.i.i.i = load i8, ptr %21, align 1, !alias.scope !25, !noalias !35, !noundef !10
  %22 = icmp sgt i8 %self1.i.i.i.i.i, -65
  br i1 %22, label %bb19.i.i.i.i, label %bb18.i.i.i.i

bb19.i.i.i.i:                                     ; preds = %bb9.i.i.i.i.i, %bb6.i.i.i.i.i, %bb5.i.i.i.i
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %19
  %_7.i.i.i.i.i.i = icmp samesign eq i64 %19, %self.val1.i.i
  br i1 %_7.i.i.i.i.i.i, label %bb7.i.sink.split.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb19.i.i.i.i
  %x.i.i.i.i.i = load i8, ptr %data.i.i.i.i.i, align 1, !noalias !36, !noundef !10
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_18.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i.i, i64 1
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_7.i10.i.i.i.i.i = icmp ne ptr %_18.i.i.i.i.i.i, %_48.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i10.i.i.i.i.i)
  %y.i.i.i.i.i = load i8, ptr %_18.i.i.i.i.i.i, align 1, !noalias !36, !noundef !10
  %_34.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_36.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_35.i.i.i.i.i = zext nneg i8 %_36.i.i.i.i.i to i32
  %23 = or disjoint i32 %_34.i.i.i.i.i, %_35.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i21.i.i.i.i, label %bb7.loopexit.i.loopexit.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb7.loopexit.i.loopexit.i.i

bb6.i21.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i
  %_18.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i.i, i64 2
  %_7.i17.i.i.i.i.i = icmp ne ptr %_18.i12.i.i.i.i.i, %_48.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i17.i.i.i.i.i)
  %z.i.i.i.i.i = load i8, ptr %_18.i12.i.i.i.i.i, align 1, !noalias !36, !noundef !10
  %_40.i.i.i.i.i = shl nuw nsw i32 %_35.i.i.i.i.i, 6
  %_42.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_41.i.i.i.i.i = zext nneg i8 %_42.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_40.i.i.i.i.i, %_41.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %24 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb7.loopexit.i.loopexit.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i21.i.i.i.i
  %_18.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i.i, i64 3
  %_7.i24.i.i.i.i.i = icmp ne ptr %_18.i19.i.i.i.i.i, %_48.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i24.i.i.i.i.i)
  %w.i.i.i.i.i = load i8, ptr %_18.i19.i.i.i.i.i, align 1, !noalias !36, !noundef !10
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_46.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_48.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_47.i.i.i.i.i = zext nneg i8 %_48.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_46.i.i.i.i.i, %_47.i.i.i.i.i
  %25 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb7.loopexit.i.loopexit.i.i

bb18.i.i.i.i:                                     ; preds = %bb9.i.i.i.i.i, %bb6.i.i.i.i.i, %bb9.i.i.i.peel.i.i, %bb6.i.i.i.peel.i.i
  %.lcssa16.i.i = phi i8 [ %.promoted29.i.i.i, %bb6.i.i.i.peel.i.i ], [ %.promoted29.i.i.i, %bb9.i.i.i.peel.i.i ], [ 1, %bb6.i.i.i.i.i ], [ 1, %bb9.i.i.i.i.i ]
  %.lcssa.i.i = phi i64 [ %.promoted.i.i.i, %bb6.i.i.i.peel.i.i ], [ %.promoted.i.i.i, %bb9.i.i.i.peel.i.i ], [ %19, %bb6.i.i.i.i.i ], [ %19, %bb9.i.i.i.i.i ]
  %26 = xor i8 %.lcssa16.i.i, 1
  store i8 %26, ptr %11, align 8, !alias.scope !22, !noalias !23
; call core::str::slice_error_fail
  tail call void @_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val.i.i, i64 noundef %self.val1.i.i, i64 noundef %.lcssa.i.i, i64 noundef %self.val1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8af13d31a0ec5c8f7fb83c7ff891ca76) #31, !noalias !35
  unreachable

bb21.i.i.i.i:                                     ; preds = %bb19.i.i.peel.i.i
  %27 = xor i8 %.promoted29.i.i.i, 1
  store i8 %27, ptr %11, align 8, !alias.scope !22, !noalias !23
  br i1 %is_match.i.i.peel.i.i, label %bb7.i.i.i, label %bb40.i.i.i.i

bb40.i.i.i.i:                                     ; preds = %bb21.i.i.i.i
  store i8 1, ptr %9, align 2, !alias.scope !22, !noalias !23
  br label %bb10.i.i.i

bb7.loopexit.i.loopexit.i.i:                      ; preds = %bb8.i.i.i.i.i, %bb6.i21.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %23, %bb4.i.i.i.i.i ], [ %24, %bb6.i21.i.i.i.i ], [ %25, %bb8.i.i.i.i.i ], [ %_7.i.i.i.i.i, %bb3.i.i.i.i.i ]
  %28 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i, 1114112
  tail call void @llvm.assume(i1 %28)
  br label %bb7.i.sink.split.i.i

bb7.i.sink.split.i.i:                             ; preds = %bb7.loopexit.i.loopexit.i.i, %bb19.i.i.i.i, %bb22.i.i.peel.i.i
  %.ph.i.i = phi i64 [ %self.val1.i.i, %bb19.i.i.i.i ], [ %.promoted.i.i.i, %bb22.i.i.peel.i.i ], [ %19, %bb7.loopexit.i.loopexit.i.i ]
  store i8 0, ptr %11, align 8, !alias.scope !22, !noalias !23
  br label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %bb7.i.sink.split.i.i, %bb21.i.i.i.i
  %29 = phi i64 [ %self.val1.i.i, %bb21.i.i.i.i ], [ %.ph.i.i, %bb7.i.sink.split.i.i ]
  %30 = getelementptr inbounds nuw i8, ptr %_5.i.i, i64 8
  store i64 %29, ptr %30, align 8, !alias.scope !12, !noalias !18
  %31 = getelementptr inbounds nuw i8, ptr %_5.i.i, i64 16
  store i64 %29, ptr %31, align 8, !alias.scope !12, !noalias !18
  br label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %bb7.i.i.i, %bb40.i.i.i.i, %bb3.i.critedge.i.i.i
  %storemerge.i.i.i = phi i64 [ 1, %bb7.i.i.i ], [ 0, %bb40.i.i.i.i ], [ 0, %bb3.i.critedge.i.i.i ]
  store i64 %storemerge.i.i.i, ptr %_5.i.i, align 8, !alias.scope !12, !noalias !18
  br label %"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE.exit.i.i"

bb9.i.i.i:                                        ; preds = %bb2.i.i.i
; call core::str::pattern::TwoWaySearcher::next
  call fastcc void @_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(24) %_5.i.i, ptr noalias noundef align 8 dereferenceable(64) %searcher.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val.i.i, i64 noundef %self.val1.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.05.i.i.i, i64 noundef %self.16.i.i.i, i1 noundef zeroext false) #32
  br label %"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE.exit.i.i"

bb8.i.i.i:                                        ; preds = %bb2.i.i.i
; call core::str::pattern::TwoWaySearcher::next
  call fastcc void @_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(24) %_5.i.i, ptr noalias noundef align 8 dereferenceable(64) %searcher.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val.i.i, i64 noundef %self.val1.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.05.i.i.i, i64 noundef %self.16.i.i.i, i1 noundef zeroext true) #32
  br label %"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE.exit.i.i"

"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE.exit.i.i": ; preds = %bb8.i.i.i, %bb9.i.i.i, %bb10.i.i.i
  %_7.i.i = load i64, ptr %_5.i.i, align 8, !range !17, !noalias !9, !noundef !10
  %32 = trunc nuw i64 %_7.i.i to i1
  br i1 %32, label %bb7.i.i, label %bb6.i.i

bb7.i.i:                                          ; preds = %"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE.exit.i.i"
  %33 = getelementptr inbounds nuw i8, ptr %_5.i.i, i64 8
  %a.i.i = load i64, ptr %33, align 8, !noalias !9, !noundef !10
  %34 = getelementptr inbounds nuw i8, ptr %_5.i.i, i64 16
  %b.i.i = load i64, ptr %34, align 8, !noalias !9, !noundef !10
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 104
  %i.i.i = load i64, ptr %35, align 8, !alias.scope !9, !noundef !10
  %new_len.i.i = sub nuw i64 %a.i.i, %i.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %i.i.i
  store i64 %b.i.i, ptr %35, align 8, !alias.scope !9
  br label %bb5

bb6.i.i:                                          ; preds = %"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE.exit.i.i"
  %36 = load i8, ptr %0, align 1, !range !8, !alias.scope !37, !noundef !10
  %_2.i2.i.i = trunc nuw i8 %36 to i1
  br i1 %_2.i2.i.i, label %"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E.exit.thread8", label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb6.i.i
  store i8 1, ptr %0, align 1, !alias.scope !37
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %38 = load i8, ptr %37, align 8, !range !8, !alias.scope !37, !noundef !10
  %_3.i.i.i = trunc nuw i8 %38 to i1
  br i1 %_3.i.i.i, label %bb1.bb4_crit_edge.i.i.i, label %bb2.i3.i.i

bb1.bb4_crit_edge.i.i.i:                          ; preds = %bb1.i.i.i
  %.phi.trans.insert.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 104
  %i.pre.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i, align 8, !alias.scope !37
  %.phi.trans.insert4.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 112
  %i1.pre.i.i.i = load i64, ptr %.phi.trans.insert4.i.i.i, align 8, !alias.scope !37
  br label %bb4.i.i.i

bb2.i3.i.i:                                       ; preds = %bb1.i.i.i
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 112
  %_6.i.i.i = load i64, ptr %39, align 8, !alias.scope !37, !noundef !10
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 104
  %_7.i.i.i = load i64, ptr %40, align 8, !alias.scope !37, !noundef !10
  %_4.not.i.i.i = icmp eq i64 %_6.i.i.i, %_7.i.i.i
  br i1 %_4.not.i.i.i, label %"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E.exit.thread8", label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb2.i3.i.i, %bb1.bb4_crit_edge.i.i.i
  %i1.i.i.i = phi i64 [ %i1.pre.i.i.i, %bb1.bb4_crit_edge.i.i.i ], [ %_6.i.i.i, %bb2.i3.i.i ]
  %i.i.i.i = phi i64 [ %i.pre.i.i.i, %bb1.bb4_crit_edge.i.i.i ], [ %_7.i.i.i, %bb2.i3.i.i ]
  %self.val.i.i.i = load ptr, ptr %2, align 8, !alias.scope !37, !nonnull !10, !align !11, !noundef !10
  %new_len.i.i.i = sub nuw i64 %i1.i.i.i, %i.i.i.i
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i.i, i64 %i.i.i.i
  br label %bb5

"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E.exit.thread8": ; preds = %bb2.i3.i.i, %bb6.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i), !noalias !9
  br label %bb2

bb5:                                              ; preds = %bb4.i.i.i, %bb7.i.i
  %_0.sroa.4.0.i.i = phi i64 [ %new_len.i.i, %bb7.i.i ], [ %new_len.i.i.i, %bb4.i.i.i ]
  %_0.sroa.0.0.i.i = phi ptr [ %data.i.i, %bb7.i.i ], [ %data.i.i.i, %bb4.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i), !noalias !9
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_4.i.i.i.i3), !noalias !40
  store i64 0, ptr %_4.i.i.i.i3, align 8, !noalias !49
  %_3.sroa.4.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 8
  store i64 %_0.sroa.4.0.i.i, ptr %_3.sroa.4.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.5.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 16
  store ptr %_0.sroa.0.0.i.i, ptr %_3.sroa.5.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.6.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 24
  store i64 %_0.sroa.4.0.i.i, ptr %_3.sroa.6.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.7.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 32
  store i64 0, ptr %_3.sroa.7.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.8.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 40
  store i64 %_0.sroa.4.0.i.i, ptr %_3.sroa.8.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.9.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 48
  store i32 10, ptr %_3.sroa.9.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.10.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 52
  store i32 10, ptr %_3.sroa.10.0._4.i.i.i.sroa_idx.i, align 4, !noalias !49
  %_3.sroa.11.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 56
  store i8 1, ptr %_3.sroa.11.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.13.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 64
  store i8 0, ptr %_3.sroa.13.0._4.i.i.i.sroa_idx.i, align 8, !noalias !49
  %_3.sroa.14.0._4.i.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i3, i64 65
  store i8 0, ptr %_3.sroa.14.0._4.i.i.i.sroa_idx.i, align 1, !noalias !49
; call <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
  %41 = call fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 dereferenceable(72) %_4.i.i.i.i3), !noalias !50
  %42 = extractvalue { ptr, i64 } %41, 0
  %.not7.i.i.i.i.i = icmp eq ptr %42, null
  br i1 %.not7.i.i.i.i.i, label %"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE.exit", label %bb3.i.i.i.i.i4

bb3.i.i.i.i.i4:                                   ; preds = %bb5, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i"
  %43 = phi ptr [ %56, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i" ], [ %42, %bb5 ]
  %44 = phi { ptr, i64 } [ %55, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i" ], [ %41, %bb5 ]
  %accum.sroa.0.08.i.i.i.i.i = phi i64 [ %_0.sroa.0.0.i.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i" ], [ 0, %bb5 ]
  %45 = extractvalue { ptr, i64 } %44, 1
; call core::str::<impl str>::trim
  %46 = tail call fastcc { ptr, i64 } @"_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %43, i64 noundef %45) #32, !noalias !51
  %_4.0.i.i.i.i.i.i.i = extractvalue { ptr, i64 } %46, 0
  %_4.1.i.i.i.i.i.i.i = extractvalue { ptr, i64 } %46, 1
  switch i64 %_4.1.i.i.i.i.i.i.i, label %bb9thread-pre-split.i.i.i.i.i.i.i.i.i.i [
    i64 0, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i"
    i64 1, label %bb7.i.i.i.i.i.i.i.i.i.i
  ]

bb7.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i4
  %47 = load i8, ptr %_4.0.i.i.i.i.i.i.i, align 1, !alias.scope !54, !noalias !61, !noundef !10
  switch i8 %47, label %bb9.i.i.i.i.i.i.i.i.i.i [
    i8 43, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i"
    i8 45, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i"
  ]

bb9thread-pre-split.i.i.i.i.i.i.i.i.i.i:          ; preds = %bb3.i.i.i.i.i4
  %.pr.i.i.i.i.i.i.i.i.i.i = load i8, ptr %_4.0.i.i.i.i.i.i.i, align 1, !alias.scope !54, !noalias !61
  br label %bb9.i.i.i.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb9thread-pre-split.i.i.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i.i
  %48 = phi i8 [ %.pr.i.i.i.i.i.i.i.i.i.i, %bb9thread-pre-split.i.i.i.i.i.i.i.i.i.i ], [ %47, %bb7.i.i.i.i.i.i.i.i.i.i ]
  %cond.i.i.i.i.i.i.i.i.i.i = icmp eq i8 %48, 43
  %rest.1.i.i.i.i.i.i.i.i.i.i = sext i1 %cond.i.i.i.i.i.i.i.i.i.i to i64
  %src.sroa.15.0.i.i.i.i.i.i.i.i.i.i = add nsw i64 %_4.1.i.i.i.i.i.i.i, %rest.1.i.i.i.i.i.i.i.i.i.i
  %src.sroa.0.0.idx.i.i.i.i.i.i.i.i.i.i = zext i1 %cond.i.i.i.i.i.i.i.i.i.i to i64
  %src.sroa.0.0.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.0.i.i.i.i.i.i.i, i64 %src.sroa.0.0.idx.i.i.i.i.i.i.i.i.i.i
  %_12.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i64 %src.sroa.15.0.i.i.i.i.i.i.i.i.i.i, 17
  br i1 %_12.i.i.i.i.i.i.i.i.i.i, label %bb15.preheader.i.i.i.i.i.i.i.i.i.i, label %bb22.i.i.i.i.i.i.i.i.i.i

bb15.preheader.i.i.i.i.i.i.i.i.i.i:               ; preds = %bb9.i.i.i.i.i.i.i.i.i.i
  %_15.not43.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %src.sroa.15.0.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_15.not43.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i

bb22.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb9.i.i.i.i.i.i.i.i.i.i, %bb26.i.i.i.i.i.i.i.i.i.i
  %result.sroa.0.0.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_69.0.i.i.i.i.i.i.i.i.i.i, %bb26.i.i.i.i.i.i.i.i.i.i ], [ 0, %bb9.i.i.i.i.i.i.i.i.i.i ]
  %src.sroa.15.1.i.i.i.i.i.i.i.i.i.i = phi i64 [ %rest.12.i.i.i.i.i.i.i.i.i.i, %bb26.i.i.i.i.i.i.i.i.i.i ], [ %src.sroa.15.0.i.i.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i.i.i.i ]
  %src.sroa.0.1.i.i.i.i.i.i.i.i.i.i = phi ptr [ %rest.01.i.i.i.i.i.i.i.i.i.i, %bb26.i.i.i.i.i.i.i.i.i.i ], [ %src.sroa.0.0.i.i.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i.i.i.i ]
  %_32.not.i.i.i.not.i.i.i.i.i.i.i = icmp eq i64 %src.sroa.15.1.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_32.not.i.i.i.not.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i, label %bb23.i.i.i.i.i.i.i.i.i.i

bb23.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb22.i.i.i.i.i.i.i.i.i.i
  %_40.i.i.i.i.i.i.i.i.i.i = load i8, ptr %src.sroa.0.1.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !54, !noalias !61, !noundef !10
  %_39.i.i.i.i.i.i.i.i.i.i = zext i8 %_40.i.i.i.i.i.i.i.i.i.i to i32
  %49 = add nsw i32 %_39.i.i.i.i.i.i.i.i.i.i, -48
  %_14.i.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %49, 10
  br i1 %_14.i.i.i.i.i.i.i.i.i.i.i, label %bb26.i.i.i.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i"

bb26.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb23.i.i.i.i.i.i.i.i.i.i
  %50 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %result.sroa.0.0.i.i.i.i.i.i.i.i.i.i, i64 10)
  %_63.0.i.i.i.i.i.i.i.i.i.i = extractvalue { i64, i1 } %50, 0
  %rest.12.i.i.i.i.i.i.i.i.i.i = add nsw i64 %src.sroa.15.1.i.i.i.i.i.i.i.i.i.i, -1
  %rest.01.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i.i.i.i.i.i.i.i.i, i64 1
  %_63.1.i.i.i.i.i.i.i.i.i.i = extractvalue { i64, i1 } %50, 1
  %x.i.i.i.i.i.i.i.i.i.i = zext nneg i32 %49 to i64
  %_69.0.i.i.i.i.i.i.i.i.i.i = add i64 %_63.0.i.i.i.i.i.i.i.i.i.i, %x.i.i.i.i.i.i.i.i.i.i
  %_69.1.not.i.i.i.i.i.i.i.i.i.i = icmp ult i64 %_69.0.i.i.i.i.i.i.i.i.i.i, %_63.0.i.i.i.i.i.i.i.i.i.i
  %or.cond.i.i.i.i.i.i.i = select i1 %_63.1.i.i.i.i.i.i.i.i.i.i, i1 true, i1 %_69.1.not.i.i.i.i.i.i.i.i.i.i
  br i1 %or.cond.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i", label %bb22.i.i.i.i.i.i.i.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb15.preheader.i.i.i.i.i.i.i.i.i.i, %bb20.i.i.i.i.i.i.i.i.i.i
  %src.sroa.0.246.i.i.i.i.i.i.i.i.i.i = phi ptr [ %rest.04.i.i.i.i.i.i.i.i.i.i, %bb20.i.i.i.i.i.i.i.i.i.i ], [ %src.sroa.0.0.i.i.i.i.i.i.i.i.i.i, %bb15.preheader.i.i.i.i.i.i.i.i.i.i ]
  %src.sroa.15.245.i.i.i.i.i.i.i.i.i.i = phi i64 [ %rest.15.i.i.i.i.i.i.i.i.i.i, %bb20.i.i.i.i.i.i.i.i.i.i ], [ %src.sroa.15.0.i.i.i.i.i.i.i.i.i.i, %bb15.preheader.i.i.i.i.i.i.i.i.i.i ]
  %result.sroa.0.244.i.i.i.i.i.i.i.i.i.i = phi i64 [ %53, %bb20.i.i.i.i.i.i.i.i.i.i ], [ 0, %bb15.preheader.i.i.i.i.i.i.i.i.i.i ]
  %_22.i.i.i.i.i.i.i.i.i.i = load i8, ptr %src.sroa.0.246.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !54, !noalias !61, !noundef !10
  %_21.i.i.i.i.i.i.i.i.i.i = zext i8 %_22.i.i.i.i.i.i.i.i.i.i to i32
  %51 = add nsw i32 %_21.i.i.i.i.i.i.i.i.i.i, -48
  %_14.i36.i.i.i.i.i.i.i.i.i.i = icmp ugt i32 %51, 9
  br i1 %_14.i36.i.i.i.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i", label %bb20.i.i.i.i.i.i.i.i.i.i

bb20.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  %52 = mul i64 %result.sroa.0.244.i.i.i.i.i.i.i.i.i.i, 10
  %rest.15.i.i.i.i.i.i.i.i.i.i = add nsw i64 %src.sroa.15.245.i.i.i.i.i.i.i.i.i.i, -1
  %rest.04.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.246.i.i.i.i.i.i.i.i.i.i, i64 1
  %_26.i.i.i.i.i.i.i.i.i.i = zext nneg i32 %51 to i64
  %53 = add i64 %52, %_26.i.i.i.i.i.i.i.i.i.i
  %_15.not.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %rest.15.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_15.not.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb22.i.i.i.i.i.i.i.i.i.i, %bb20.i.i.i.i.i.i.i.i.i.i, %bb15.preheader.i.i.i.i.i.i.i.i.i.i
  %54 = phi i64 [ 0, %bb15.preheader.i.i.i.i.i.i.i.i.i.i ], [ %53, %bb20.i.i.i.i.i.i.i.i.i.i ], [ %result.sroa.0.0.i.i.i.i.i.i.i.i.i.i, %bb22.i.i.i.i.i.i.i.i.i.i ]
  %_4.0.i2.i.i.i.i.i.i = add i64 %54, %accum.sroa.0.08.i.i.i.i.i
  br label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i"

"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i": ; preds = %bb26.i.i.i.i.i.i.i.i.i.i, %bb23.i.i.i.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i4
  %_0.sroa.0.0.i.i.i.i.i.i = phi i64 [ %_4.0.i2.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %accum.sroa.0.08.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i.i ], [ %accum.sroa.0.08.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i.i ], [ %accum.sroa.0.08.i.i.i.i.i, %bb3.i.i.i.i.i4 ], [ %accum.sroa.0.08.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i ], [ %accum.sroa.0.08.i.i.i.i.i, %bb23.i.i.i.i.i.i.i.i.i.i ], [ %accum.sroa.0.08.i.i.i.i.i, %bb26.i.i.i.i.i.i.i.i.i.i ]
; call <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
  %55 = call fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 dereferenceable(72) %_4.i.i.i.i3), !noalias !50
  %56 = extractvalue { ptr, i64 } %55, 0
  %.not.i.i.i.i.i = icmp eq ptr %56, null
  br i1 %.not.i.i.i.i.i, label %"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE.exit", label %bb3.i.i.i.i.i4

"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE.exit": ; preds = %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i", %bb5
  %accum.sroa.0.0.lcssa.i.i.i.i.i = phi i64 [ 0, %bb5 ], [ %_0.sroa.0.0.i.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h6b224f95adfcb80eE.exit.i.i.i.i.i" ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_4.i.i.i.i3), !noalias !40
  br label %bb2

bb2:                                              ; preds = %start, %"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E.exit.thread8", %"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE.exit"
  %_0.sroa.3.0 = phi i64 [ %accum.sroa.0.0.lcssa.i.i.i.i.i, %"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE.exit" ], [ undef, %"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E.exit.thread8" ], [ undef, %start ]
  %_0.sroa.0.0 = phi i64 [ 1, %"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE.exit" ], [ 0, %"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E.exit.thread8" ], [ 0, %start ]
  %57 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0, 0
  %58 = insertvalue { i64, i64 } %57, i64 %_0.sroa.3.0, 1
  ret { i64, i64 } %58
}

; core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(80) %_1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef align 8 dereferenceable(48) %0)
          to label %bb4 unwind label %funclet_bb3

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 56
  %_1.val.i = load i64, ptr %1, align 8, !alias.scope !65
  %_6.i.i.i.i4.i.i.i = icmp eq i64 %_1.val.i, 0
  br i1 %_6.i.i.i.i4.i.i.i, label %"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit", label %bb2.i.i.i5.i.i.i

bb2.i.i.i5.i.i.i:                                 ; preds = %funclet_bb3
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %_1.val1.i = load ptr, ptr %2, align 8, !alias.scope !65, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad) ]
  br label %"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit"

"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit": ; preds = %funclet_bb3, %bb2.i.i.i5.i.i.i
  cleanupret from %cleanuppad unwind to caller

bb4:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !68)
  %_1.val.i1 = load i64, ptr %3, align 8, !alias.scope !68
  %_6.i.i.i.i4.i.i.i2 = icmp eq i64 %_1.val.i1, 0
  br i1 %_6.i.i.i.i4.i.i.i2, label %"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit5", label %bb2.i.i.i5.i.i.i3

bb2.i.i.i5.i.i.i3:                                ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %_1.val1.i4 = load ptr, ptr %4, align 8, !alias.scope !68, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i4, i64 noundef %_1.val.i1, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !68
  br label %"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit5"

"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit5": ; preds = %bb4, %bb2.i.i.i5.i.i.i3
  ret void
}

; core::ptr::drop_in_place<aoc2022::solver::day03::Rucksacks>
; Function Attrs: nounwind uwtable
define internal fastcc void @"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E"(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !71)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val.i = load ptr, ptr %0, align 8, !alias.scope !71, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1.i = load i64, ptr %1, align 8, !alias.scope !71, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !74)
  %_711.i.i.i = icmp eq i64 %_1.val1.i, 0
  br i1 %_711.i.i.i, label %bb4.i, label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %start, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i"
  %_3.sroa.0.012.i.i.i = phi i64 [ %2, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i" ], [ 0, %start ]
  %_6.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.val.i, i64 %_3.sroa.0.012.i.i.i
  %2 = add nuw i64 %_3.sroa.0.012.i.i.i, 1
  %_6.val.i.i.i = load i64, ptr %_6.i.i.i, align 8, !alias.scope !74, !noalias !71
  %_6.i.i.i.i4.i.i.i.i.i = icmp eq i64 %_6.val.i.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i", label %bb2.i.i.i5.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i:                             ; preds = %bb5.i.i.i
  %3 = getelementptr i8, ptr %_6.i.i.i, i64 8
  %_6.val7.i.i.i = load ptr, ptr %3, align 8, !alias.scope !74, !noalias !71, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i, i64 noundef %_6.val.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !77
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i, %bb5.i.i.i
  %_7.i.i.i = icmp eq i64 %2, %_1.val1.i
  br i1 %_7.i.i.i, label %bb4.i, label %bb5.i.i.i

bb4.i:                                            ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i", %start
  %_1.val4.i = load i64, ptr %_1, align 8, !range !78, !alias.scope !71, !noundef !10
  %_6.i.i.i.i6.i = icmp eq i64 %_1.val4.i, 0
  br i1 %_6.i.i.i.i6.i, label %"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E.exit", label %bb2.i.i.i7.i

bb2.i.i.i7.i:                                     ; preds = %bb4.i
  %4 = mul nuw i64 %_1.val4.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 8) #33, !noalias !71
  br label %"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E.exit"

"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E.exit": ; preds = %bb4.i, %bb2.i.i.i7.i
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
; Function Attrs: nounwind uwtable
define internal fastcc void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E"(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !79)
  %_711.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_711.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i"
  %_3.sroa.0.012.i.i = phi i64 [ %2, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i" ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.val, i64 %_3.sroa.0.012.i.i
  %2 = add nuw i64 %_3.sroa.0.012.i.i, 1
  %_6.val.i.i = load i64, ptr %_6.i.i, align 8, !alias.scope !79
  %_6.i.i.i.i4.i.i.i.i = icmp eq i64 %_6.val.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i", label %bb2.i.i.i5.i.i.i.i

bb2.i.i.i5.i.i.i.i:                               ; preds = %bb5.i.i
  %3 = getelementptr i8, ptr %_6.i.i, i64 8
  %_6.val7.i.i = load ptr, ptr %3, align 8, !alias.scope !79, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i, i64 noundef %_6.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !79
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i": ; preds = %bb2.i.i.i5.i.i.i.i, %bb5.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb4:                                              ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i", %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !78, !noundef !10
  %_6.i.i.i.i6 = icmp eq i64 %_1.val4, 0
  br i1 %_6.i.i.i.i6, label %"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17h043cfb4b536bdaf3E.exit8", label %bb2.i.i.i7

bb2.i.i.i7:                                       ; preds = %bb4
  %4 = mul nuw i64 %_1.val4, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 8) #33
  br label %"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17h043cfb4b536bdaf3E.exit8"

"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17h043cfb4b536bdaf3E.exit8": ; preds = %bb4, %bb2.i.i.i7
  ret void
}

; core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
; Function Attrs: nounwind uwtable
define internal fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %_1.0.val, i64 %_1.8.val) unnamed_addr #2 {
start:
  %_4.i.i.i.i.i = icmp eq i64 %_1.8.val, 0
  br i1 %_4.i.i.i.i.i, label %"_ZN4core3ptr89drop_in_place$LT$hashbrown..set..HashSet$LT$char$C$std..hash..random..RandomState$GT$$GT$17h494f4ed7fd4a9519E.exit", label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %start
  %_10.i.i.i.i.i = shl i64 %_1.8.val, 2
  %_32.0.i.i.i.i.i.i = add i64 %_10.i.i.i.i.i, 19
  %ctrl_offset.i.i.i.i.i.i = and i64 %_32.0.i.i.i.i.i.i, -16
  %rhs5.i.i.i.i.i.i = add i64 %_1.8.val, 17
  %_37.0.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i
  %_37.1.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i
  %_19.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i)
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_4.not.i.i.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i.i.i, label %"_ZN4core3ptr89drop_in_place$LT$hashbrown..set..HashSet$LT$char$C$std..hash..random..RandomState$GT$$GT$17h494f4ed7fd4a9519E.exit", label %bb1.i2.i.i.i.i.i

bb1.i2.i.i.i.i.i:                                 ; preds = %bb1.i.i.i.i.i
  %_18.i.i.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i
  %ptr.i.i.i.i.i = getelementptr inbounds i8, ptr %_1.0.val, i64 %_18.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33
  br label %"_ZN4core3ptr89drop_in_place$LT$hashbrown..set..HashSet$LT$char$C$std..hash..random..RandomState$GT$$GT$17h494f4ed7fd4a9519E.exit"

"_ZN4core3ptr89drop_in_place$LT$hashbrown..set..HashSet$LT$char$C$std..hash..random..RandomState$GT$$GT$17h494f4ed7fd4a9519E.exit": ; preds = %start, %bb1.i.i.i.i.i, %bb1.i2.i.i.i.i.i
  ret void
}

; core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
; Function Attrs: uwtable
define internal fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %_1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = load i64, ptr %_1, align 8, !range !82, !noundef !10
  %1 = icmp eq i64 %0, 3
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %bb2.i.i.i7.i.i.i.i.i, %bb4.i.i.i.i.i, %bb2.i.i, %bb2, %start
  ret void

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !83)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !86)
  %switch.i.i = icmp samesign ult i64 %0, 2
  br i1 %switch.i.i, label %bb1, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb2
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !89)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !92)
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 40
  %4 = load i32, ptr %3, align 8, !alias.scope !95, !noundef !10
  switch i32 %4, label %bb2.i.i.i.i [
    i32 3, label %bb1.sink.split.i.i.i.i
    i32 2, label %bb1
    i32 0, label %bb1.sink.split.i.i.i.i
  ], !prof !96

bb2.i.i.i.i:                                      ; preds = %bb2.i.i
; call core::panicking::panic_fmt
  tail call void @_ZN4core9panicking9panic_fmt17hdddacd639c98ccdaE(ptr noundef nonnull @alloc_a931397211c33a1c8fe0d17838460834, ptr noundef nonnull inttoptr (i64 121 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6a6fc231b3cb64280fdbf03fad4b13a2) #31, !noalias !95
  unreachable

bb1.sink.split.i.i.i.i:                           ; preds = %bb2.i.i, %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !97)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !100)
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val.i.i.i.i.i = load ptr, ptr %5, align 8, !alias.scope !103, !nonnull !10, !noundef !10
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %_1.val1.i.i.i.i.i = load i64, ptr %6, align 8, !alias.scope !103, !noundef !10
  %_76.i.i.i.i.i.i.i = icmp eq i64 %_1.val1.i.i.i.i.i, 0
  br i1 %_76.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb1.sink.split.i.i.i.i, %"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE.exit.i.i.i.i.i.i"
  %_3.sroa.0.07.i.i.i.i.i.i.i = phi i64 [ %7, %"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE.exit.i.i.i.i.i.i" ], [ 0, %bb1.sink.split.i.i.i.i ]
  %_6.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::backtrace::BacktraceFrame", ptr %_1.val.i.i.i.i.i, i64 %_3.sroa.0.07.i.i.i.i.i.i.i
  %7 = add nuw i64 %_3.sroa.0.07.i.i.i.i.i.i.i, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !104)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !107)
  %8 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i, i64 8
  %_1.val.i.i.i.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !110, !noalias !103, !nonnull !10, !noundef !10
  %9 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i, i64 16
  %_1.val1.i.i.i.i.i.i.i.i = load i64, ptr %9, align 8, !alias.scope !110, !noalias !103, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !111)
  %_76.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_1.val1.i.i.i.i.i.i.i.i, 0
  br i1 %_76.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb5.i.i.i.i.i.i.i, %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i"
  %_3.sroa.0.07.i.i.i.i.i.i.i.i.i.i = phi i64 [ %10, %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i" ], [ 0, %bb5.i.i.i.i.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::backtrace::BacktraceSymbol", ptr %_1.val.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.07.i.i.i.i.i.i.i.i.i.i
  %10 = add nuw i64 %_3.sroa.0.07.i.i.i.i.i.i.i.i.i.i, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !114)
  %11 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 32
  %.val.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %11, align 8, !range !117, !alias.scope !118, !noalias !119, !noundef !10
  switch i64 %.val.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i.i.i.i.i.i.i.i
    i64 0, label %bb4.i.i.i.i.i.i.i.i.i.i.i
  ]

bb2.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i:             ; preds = %bb5.i.i.i.i.i.i.i.i.i.i
  %12 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 40
  %.val1.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %12, align 8, !alias.scope !118, !noalias !119, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %.val.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !120
  br label %bb4.i.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %bb2.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !121)
  %13 = load i64, ptr %_6.i.i.i.i.i.i.i.i.i.i, align 8, !range !124, !alias.scope !125, !noalias !119, !noundef !10
  %14 = icmp eq i64 %13, 2
  br i1 %14, label %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i", label %bb2.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %bb4.i.i.i.i.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !126)
  %15 = icmp eq i64 %13, 0
  %16 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 8
  %.val.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %16, align 8, !alias.scope !129, !noalias !119
  %_6.i.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %.val.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %15, label %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_6.i.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i.i:           ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i
  %17 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 16
  %.val1.i.i.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %17, align 8, !alias.scope !129, !noalias !119, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %.val.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !130
  br label %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i"

bb3.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_6.i.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i", label %bb2.i.i.i5.i5.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i5.i5.i.i.i.i.i.i.i.i.i.i.i.i.i:          ; preds = %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i
  %18 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 16
  %.val3.i.i.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %18, align 8, !alias.scope !129, !noalias !119, !nonnull !10, !noundef !10
  %19 = shl nuw i64 %.val.i.i.i.i.i.i.i.i.i.i.i.i.i, 1
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %19, i64 noundef range(i64 1, -9223372036854775807) 2) #33, !noalias !130
  br label %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i"

"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i": ; preds = %bb2.i.i.i5.i5.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %10, %_1.val1.i.i.i.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE.exit.i.i.i.i.i.i.i.i.i.i", %bb5.i.i.i.i.i.i.i
  %_1.val4.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i, align 8, !range !78, !alias.scope !110, !noalias !103, !noundef !10
  %_6.i.i.i.i6.i.i.i.i.i.i.i.i = icmp eq i64 %_1.val4.i.i.i.i.i.i.i.i, 0
  br i1 %_6.i.i.i.i6.i.i.i.i.i.i.i.i, label %"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE.exit.i.i.i.i.i.i", label %bb2.i.i.i7.i.i.i.i.i.i.i.i

bb2.i.i.i7.i.i.i.i.i.i.i.i:                       ; preds = %bb4.i.i.i.i.i.i.i.i
  %20 = mul nuw i64 %_1.val4.i.i.i.i.i.i.i.i, 72
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i.i.i.i.i.i.i.i, i64 noundef %20, i64 noundef range(i64 1, -9223372036854775807) 8) #33, !noalias !119
  br label %"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE.exit.i.i.i.i.i.i"

"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE.exit.i.i.i.i.i.i": ; preds = %bb2.i.i.i7.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = icmp eq i64 %7, %_1.val1.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE.exit.i.i.i.i.i.i", %bb1.sink.split.i.i.i.i
  %_1.val4.i.i.i.i.i = load i64, ptr %2, align 8, !range !78, !alias.scope !103, !noundef !10
  %_6.i.i.i.i6.i.i.i.i.i = icmp eq i64 %_1.val4.i.i.i.i.i, 0
  br i1 %_6.i.i.i.i6.i.i.i.i.i, label %bb1, label %bb2.i.i.i7.i.i.i.i.i

bb2.i.i.i7.i.i.i.i.i:                             ; preds = %bb4.i.i.i.i.i
  %21 = mul nuw i64 %_1.val4.i.i.i.i.i, 56
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i.i.i.i.i, i64 noundef %21, i64 noundef range(i64 1, -9223372036854775807) 8) #33, !noalias !103
  br label %bb1
}

; core::ptr::drop_in_place<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: nounwind uwtable
define internal void @"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  %_1.val = load i64, ptr %_1, align 8
  %_6.i.i.i.i4.i.i = icmp eq i64 %_1.val, 0
  br i1 %_6.i.i.i.i4.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit", label %bb2.i.i.i5.i.i

bb2.i.i.i5.i.i:                                   ; preds = %start
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #33
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit": ; preds = %start, %bb2.i.i.i5.i.i
  ret void
}

; core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<&str>>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr97drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$17hcea1bef47dc7f53eE"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(72) %_1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; call core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  tail call fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef align 8 dereferenceable(48) %0)
  ret void
}

; core::str::<impl str>::trim
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: readwrite) uwtable
define internal fastcc { ptr, i64 } @"_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #3 personality ptr @__CxxFrameHandler3 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !131)
  %_7.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %_7.i.i.i.i11.i.i.i = icmp samesign eq i64 %self.1, 0
  br i1 %_7.i.i.i.i11.i.i.i, label %bb5.i, label %bb14.i.i.i.i.i.i

bb14.i.i.i.i.i.i:                                 ; preds = %start, %bb5.i.i.i
  %0 = phi i64 [ %7, %bb5.i.i.i ], [ 0, %start ]
  %_18.i26.i.i.i1012.i.i.i = phi ptr [ %_23.i.i.i.i, %bb5.i.i.i ], [ %self.0, %start ]
  %1 = ptrtoint ptr %_18.i26.i.i.i1012.i.i.i to i64
  %_18.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_18.i26.i.i.i1012.i.i.i, i64 1
  %x.i.i.i.i.i.i = load i8, ptr %_18.i26.i.i.i1012.i.i.i, align 1, !alias.scope !131, !noalias !134, !noundef !10
  %_6.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_30.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i to i32
  %_7.i10.i.i.i.i.i.i = icmp ne ptr %_18.i.i.i.i.i.i.i, %_7.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i10.i.i.i.i.i.i)
  %_18.i12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_18.i26.i.i.i1012.i.i.i, i64 2
  %y.i.i.i.i.i.i = load i8, ptr %_18.i.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !134, !noundef !10
  %_34.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 6
  %_36.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 63
  %_35.i.i.i.i.i.i = zext nneg i8 %_36.i.i.i.i.i.i to i32
  %2 = or disjoint i32 %_34.i.i.i.i.i.i, %_35.i.i.i.i.i.i
  %_13.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %bb2.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_7.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i to i32
  br label %bb2.i.i.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i.i
  %_7.i17.i.i.i.i.i.i = icmp ne ptr %_18.i12.i.i.i.i.i.i, %_7.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i17.i.i.i.i.i.i)
  %_18.i19.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_18.i26.i.i.i1012.i.i.i, i64 3
  %z.i.i.i.i.i.i = load i8, ptr %_18.i12.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !134, !noundef !10
  %_40.i.i.i.i.i.i = shl nuw nsw i32 %_35.i.i.i.i.i.i, 6
  %_42.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 63
  %_41.i.i.i.i.i.i = zext nneg i8 %_42.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i = or disjoint i32 %_40.i.i.i.i.i.i, %_41.i.i.i.i.i.i
  %_20.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 12
  %3 = or disjoint i32 %y_z.i.i.i.i.i.i, %_20.i.i.i.i.i.i
  %_21.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, label %bb2.i.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i.i
  %_7.i24.i.i.i.i.i.i = icmp ne ptr %_18.i19.i.i.i.i.i.i, %_7.i.i.i.i
  tail call void @llvm.assume(i1 %_7.i24.i.i.i.i.i.i)
  %_18.i26.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_18.i26.i.i.i1012.i.i.i, i64 4
  %w.i.i.i.i.i.i = load i8, ptr %_18.i19.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !134, !noundef !10
  %_26.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i, 1835008
  %_46.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i, 6
  %_48.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i, 63
  %_47.i.i.i.i.i.i = zext nneg i8 %_48.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i = or disjoint i32 %_46.i.i.i.i.i.i, %_47.i.i.i.i.i.i
  %4 = or disjoint i32 %_27.i.i.i.i.i.i, %_25.i.i.i.i.i.i
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb8.i.i.i.i.i.i, %bb6.i.i.i.i.i.i, %bb3.i.i.i.i.i.i, %bb4.i.i.i.i.i.i
  %_23.i.i.i.i = phi ptr [ %_18.i12.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %_18.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ], [ %_18.i26.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ %_18.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i = phi i32 [ %2, %bb4.i.i.i.i.i.i ], [ %3, %bb6.i.i.i.i.i.i ], [ %4, %bb8.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %5 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 1114112
  tail call void @llvm.assume(i1 %5)
  %6 = ptrtoint ptr %_23.i.i.i.i to i64
  %_10.i.i.i.i.i = sub i64 %6, %1
  %7 = add i64 %_10.i.i.i.i.i, %0
  switch i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i [
    i32 32, label %bb5.i.i.i
    i32 13, label %bb5.i.i.i
    i32 12, label %bb5.i.i.i
    i32 11, label %bb5.i.i.i
    i32 10, label %bb5.i.i.i
    i32 9, label %bb5.i.i.i
  ]

bb1.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i
  %_4.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 127
  br i1 %_4.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i, label %bb5.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb1.i.i.i.i.i.i.i
  %_3.i.i.i.i.i.i.i.i = lshr i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 8
  switch i32 %_3.i.i.i.i.i.i.i.i, label %bb5.i [
    i32 0, label %bb6.i.i.i.i.i.i.i.i
    i32 22, label %bb4.i.i.i.i.i.i.i.i
    i32 32, label %bb7.i.i.i.i.i.i.i.i
    i32 48, label %bb2.i.i.i.i.i.i.i.i
  ]

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i.i
  %8 = icmp eq i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 5760
  %9 = zext i1 %8 to i8
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i"

bb2.i.i.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i.i
  %10 = icmp eq i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 12288
  %11 = zext i1 %10 to i8
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i"

bb6.i.i.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i.i
  %12 = and i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 255
  %_8.i.i.i.i.i.i.i.i = zext nneg i32 %12 to i64
  %13 = getelementptr inbounds nuw i8, ptr @_ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE, i64 %_8.i.i.i.i.i.i.i.i
  %_6.i.i.i.i.i.i.i.i = load i8, ptr %13, align 1, !noalias !148, !noundef !10
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i"

bb7.i.i.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i.i
  %14 = and i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 255
  %_14.i.i.i.i.i.i.i.i = zext nneg i32 %14 to i64
  %15 = getelementptr inbounds nuw i8, ptr @_ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE, i64 %_14.i.i.i.i.i.i.i.i
  %_12.i.i.i.i.i.i.i.i = load i8, ptr %15, align 1, !noalias !148, !noundef !10
  %_11.i.i.i.i.i.i.i.i = lshr i8 %_12.i.i.i.i.i.i.i.i, 1
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i"

"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i": ; preds = %bb7.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i
  %_0.sroa.0.0.i.i.i.i.i.i.i.i = phi i8 [ %_6.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i ], [ %9, %bb4.i.i.i.i.i.i.i.i ], [ %_11.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i ], [ %11, %bb2.i.i.i.i.i.i.i.i ]
  %16 = trunc i8 %_0.sroa.0.0.i.i.i.i.i.i.i.i to i1
  br i1 %16, label %bb5.i.i.i, label %bb5.i

bb5.i.i.i:                                        ; preds = %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i", %bb2.i.i.i.i, %bb2.i.i.i.i, %bb2.i.i.i.i, %bb2.i.i.i.i, %bb2.i.i.i.i, %bb2.i.i.i.i
  %_7.i.i.i.i.i.i.i = icmp eq ptr %_23.i.i.i.i, %_7.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i, label %"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE.exit", label %bb14.i.i.i.i.i.i

bb5.i:                                            ; preds = %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i", %bb5.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i, %start
  %matcher.sroa.4.047.i = phi ptr [ %self.0, %start ], [ %_23.i.i.i.i, %bb1.i.i.i.i.i.i.i ], [ %_23.i.i.i.i, %bb5.i.i.i.i.i.i.i ], [ %_23.i.i.i.i, %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i" ]
  %matcher.sroa.14.045.i = phi i64 [ 0, %start ], [ %7, %bb1.i.i.i.i.i.i.i ], [ %7, %bb5.i.i.i.i.i.i.i ], [ %7, %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i" ]
  %i.sroa.0.0.i = phi i64 [ 0, %start ], [ %0, %bb1.i.i.i.i.i.i.i ], [ %0, %bb5.i.i.i.i.i.i.i ], [ %0, %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i.i" ]
  %17 = icmp eq ptr %matcher.sroa.4.047.i, %_7.i.i.i.i
  br i1 %17, label %"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE.exit", label %bb17.i.i.i.i.i.i

bb17.i.i.i.i.i.i:                                 ; preds = %bb5.i, %bb5.i.i10.i
  %_24.i25.i.i.i1213.i.i.i = phi ptr [ %_22.i.i.i.i, %bb5.i.i10.i ], [ %_7.i.i.i.i, %bb5.i ]
  %_24.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_24.i25.i.i.i1213.i.i.i, i64 -1
  %w.i.i.i.i.i4.i = load i8, ptr %_24.i.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !149, !noundef !10
  %_6.i.i.i.i.i5.i = icmp sgt i8 %w.i.i.i.i.i4.i, -1
  br i1 %_6.i.i.i.i.i5.i, label %bb3.i.i.i.i.i37.i, label %bb4.i.i.i.i.i6.i

bb4.i.i.i.i.i6.i:                                 ; preds = %bb17.i.i.i.i.i.i
  %18 = icmp ne ptr %matcher.sroa.4.047.i, %_24.i.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %18)
  %_24.i13.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_24.i25.i.i.i1213.i.i.i, i64 -2
  %z.i.i.i.i.i7.i = load i8, ptr %_24.i13.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !149, !noundef !10
  %_26.i.i.i.i.i8.i = and i8 %z.i.i.i.i.i7.i, 31
  %19 = zext nneg i8 %_26.i.i.i.i.i8.i to i32
  %_12.i.i.i.i.i.i = icmp slt i8 %z.i.i.i.i.i7.i, -64
  br i1 %_12.i.i.i.i.i.i, label %bb6.i.i.i.i.i29.i, label %bb13.i.i.i.i.i.i

bb3.i.i.i.i.i37.i:                                ; preds = %bb17.i.i.i.i.i.i
  %_8.i.i.i.i.i.i = zext nneg i8 %w.i.i.i.i.i4.i to i32
  br label %bb2.i.i.i9.i

bb6.i.i.i.i.i29.i:                                ; preds = %bb4.i.i.i.i.i6.i
  %20 = icmp ne ptr %matcher.sroa.4.047.i, %_24.i13.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %20)
  %_24.i19.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_24.i25.i.i.i1213.i.i.i, i64 -3
  %y.i.i.i.i.i30.i = load i8, ptr %_24.i19.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !149, !noundef !10
  %_31.i.i.i.i.i.i = and i8 %y.i.i.i.i.i30.i, 15
  %21 = zext nneg i8 %_31.i.i.i.i.i.i to i32
  %_16.i.i.i.i.i.i = icmp slt i8 %y.i.i.i.i.i30.i, -64
  br i1 %_16.i.i.i.i.i.i, label %bb8.i.i.i.i.i34.i, label %bb11.i.i.i.i.i.i

bb13.i.i.i.i.i.i:                                 ; preds = %bb11.i.i.i.i.i.i, %bb4.i.i.i.i.i6.i
  %_2116.i.i.i.i.i = phi ptr [ %_2117.i.i.i.i.i, %bb11.i.i.i.i.i.i ], [ %_24.i13.i.i.i.i.i.i, %bb4.i.i.i.i.i6.i ]
  %ch.sroa.0.0.i.i.i.i.i.i = phi i32 [ %26, %bb11.i.i.i.i.i.i ], [ %19, %bb4.i.i.i.i.i6.i ]
  %_43.i.i.i.i.i.i = shl nuw nsw i32 %ch.sroa.0.0.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i = and i8 %w.i.i.i.i.i4.i, 63
  %_44.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i to i32
  %22 = or disjoint i32 %_43.i.i.i.i.i.i, %_44.i.i.i.i.i.i
  br label %bb2.i.i.i9.i

bb8.i.i.i.i.i34.i:                                ; preds = %bb6.i.i.i.i.i29.i
  %23 = icmp ne ptr %matcher.sroa.4.047.i, %_24.i19.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %23)
  %_24.i25.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_24.i25.i.i.i1213.i.i.i, i64 -4
  %x.i.i.i.i.i35.i = load i8, ptr %_24.i25.i.i.i.i.i.i, align 1, !alias.scope !131, !noalias !149, !noundef !10
  %_36.i.i.i.i.i36.i = and i8 %x.i.i.i.i.i35.i, 7
  %24 = zext nneg i8 %_36.i.i.i.i.i36.i to i32
  %_37.i.i.i.i.i.i = shl nuw nsw i32 %24, 6
  %_39.i.i.i.i.i.i = and i8 %y.i.i.i.i.i30.i, 63
  %_38.i.i.i.i.i.i = zext nneg i8 %_39.i.i.i.i.i.i to i32
  %25 = or disjoint i32 %_37.i.i.i.i.i.i, %_38.i.i.i.i.i.i
  br label %bb11.i.i.i.i.i.i

bb11.i.i.i.i.i.i:                                 ; preds = %bb8.i.i.i.i.i34.i, %bb6.i.i.i.i.i29.i
  %_2117.i.i.i.i.i = phi ptr [ %_24.i25.i.i.i.i.i.i, %bb8.i.i.i.i.i34.i ], [ %_24.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i29.i ]
  %ch.sroa.0.1.i.i.i.i.i.i = phi i32 [ %25, %bb8.i.i.i.i.i34.i ], [ %21, %bb6.i.i.i.i.i29.i ]
  %_40.i.i.i.i.i31.i = shl nuw nsw i32 %ch.sroa.0.1.i.i.i.i.i.i, 6
  %_42.i.i.i.i.i32.i = and i8 %z.i.i.i.i.i7.i, 63
  %_41.i.i.i.i.i33.i = zext nneg i8 %_42.i.i.i.i.i32.i to i32
  %26 = or disjoint i32 %_40.i.i.i.i.i31.i, %_41.i.i.i.i.i33.i
  br label %bb13.i.i.i.i.i.i

bb2.i.i.i9.i:                                     ; preds = %bb13.i.i.i.i.i.i, %bb3.i.i.i.i.i37.i
  %_22.i.i.i.i = phi ptr [ %_24.i.i.i.i.i.i.i, %bb3.i.i.i.i.i37.i ], [ %_2116.i.i.i.i.i, %bb13.i.i.i.i.i.i ]
  %_0.sroa.4.1.i.ph.i.i.i.i.i = phi i32 [ %_8.i.i.i.i.i.i, %bb3.i.i.i.i.i37.i ], [ %22, %bb13.i.i.i.i.i.i ]
  %27 = icmp samesign ult i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 1114112
  tail call void @llvm.assume(i1 %27)
  switch i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, label %bb1.i.i.i.i.i.i13.i [
    i32 32, label %bb5.i.i10.i
    i32 13, label %bb5.i.i10.i
    i32 12, label %bb5.i.i10.i
    i32 11, label %bb5.i.i10.i
    i32 10, label %bb5.i.i10.i
    i32 9, label %bb5.i.i10.i
  ]

bb1.i.i.i.i.i.i13.i:                              ; preds = %bb2.i.i.i9.i
  %_4.i.i.i.i.i.i14.i = icmp samesign ugt i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 127
  br i1 %_4.i.i.i.i.i.i14.i, label %bb5.i.i.i.i.i.i16.i, label %bb7.i

bb5.i.i.i.i.i.i16.i:                              ; preds = %bb1.i.i.i.i.i.i13.i
  %_3.i.i.i.i.i.i.i17.i = lshr i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 8
  switch i32 %_3.i.i.i.i.i.i.i17.i, label %bb7.i [
    i32 0, label %bb6.i.i.i.i.i.i.i26.i
    i32 22, label %bb4.i.i.i.i.i.i.i25.i
    i32 32, label %bb7.i.i.i.i.i.i.i21.i
    i32 48, label %bb2.i.i.i.i.i.i.i18.i
  ]

bb4.i.i.i.i.i.i.i25.i:                            ; preds = %bb5.i.i.i.i.i.i16.i
  %28 = icmp eq i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 5760
  %29 = zext i1 %28 to i8
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i"

bb2.i.i.i.i.i.i.i18.i:                            ; preds = %bb5.i.i.i.i.i.i16.i
  %30 = icmp eq i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 12288
  %31 = zext i1 %30 to i8
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i"

bb6.i.i.i.i.i.i.i26.i:                            ; preds = %bb5.i.i.i.i.i.i16.i
  %32 = and i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 255
  %_8.i.i.i.i.i.i.i27.i = zext nneg i32 %32 to i64
  %33 = getelementptr inbounds nuw i8, ptr @_ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE, i64 %_8.i.i.i.i.i.i.i27.i
  %_6.i.i.i.i.i.i.i28.i = load i8, ptr %33, align 1, !noalias !163, !noundef !10
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i"

bb7.i.i.i.i.i.i.i21.i:                            ; preds = %bb5.i.i.i.i.i.i16.i
  %34 = and i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 255
  %_14.i.i.i.i.i.i.i22.i = zext nneg i32 %34 to i64
  %35 = getelementptr inbounds nuw i8, ptr @_ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h49f287ce5984536aE, i64 %_14.i.i.i.i.i.i.i22.i
  %_12.i.i.i.i.i.i.i23.i = load i8, ptr %35, align 1, !noalias !163, !noundef !10
  %_11.i.i.i.i.i.i.i24.i = lshr i8 %_12.i.i.i.i.i.i.i23.i, 1
  br label %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i"

"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i": ; preds = %bb7.i.i.i.i.i.i.i21.i, %bb6.i.i.i.i.i.i.i26.i, %bb2.i.i.i.i.i.i.i18.i, %bb4.i.i.i.i.i.i.i25.i
  %_0.sroa.0.0.i.i.i.i.i.i.i20.i = phi i8 [ %_6.i.i.i.i.i.i.i28.i, %bb6.i.i.i.i.i.i.i26.i ], [ %29, %bb4.i.i.i.i.i.i.i25.i ], [ %_11.i.i.i.i.i.i.i24.i, %bb7.i.i.i.i.i.i.i21.i ], [ %31, %bb2.i.i.i.i.i.i.i18.i ]
  %36 = trunc i8 %_0.sroa.0.0.i.i.i.i.i.i.i20.i to i1
  br i1 %36, label %bb5.i.i10.i, label %bb7.i

bb5.i.i10.i:                                      ; preds = %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i", %bb2.i.i.i9.i, %bb2.i.i.i9.i, %bb2.i.i.i9.i, %bb2.i.i.i9.i, %bb2.i.i.i9.i, %bb2.i.i.i9.i
  %37 = icmp eq ptr %matcher.sroa.4.047.i, %_22.i.i.i.i
  br i1 %37, label %"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE.exit", label %bb17.i.i.i.i.i.i

bb7.i:                                            ; preds = %"_ZN53_$LT$F$u20$as$u20$core..str..pattern..MultiCharEq$GT$7matches17h3547392c82c329f0E.exit.i.i.i19.i", %bb5.i.i.i.i.i.i16.i, %bb1.i.i.i.i.i.i13.i
  %38 = ptrtoint ptr %_24.i25.i.i.i1213.i.i.i to i64
  %39 = ptrtoint ptr %matcher.sroa.4.047.i to i64
  %40 = sub i64 %matcher.sroa.14.045.i, %39
  %_15.i6.i.i.i = add i64 %40, %38
  br label %"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE.exit"

"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE.exit": ; preds = %bb5.i.i.i, %bb5.i.i10.i, %bb5.i, %bb7.i
  %i.sroa.0.058.i = phi i64 [ %i.sroa.0.0.i, %bb7.i ], [ %i.sroa.0.0.i, %bb5.i ], [ %i.sroa.0.0.i, %bb5.i.i10.i ], [ 0, %bb5.i.i.i ]
  %j.sroa.0.1.i = phi i64 [ %_15.i6.i.i.i, %bb7.i ], [ %matcher.sroa.14.045.i, %bb5.i ], [ %matcher.sroa.14.045.i, %bb5.i.i10.i ], [ 0, %bb5.i.i.i ]
  %new_len.i = sub nuw i64 %j.sroa.0.1.i, %i.sroa.0.058.i
  %data.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %i.sroa.0.058.i
  %41 = insertvalue { ptr, i64 } poison, ptr %data.i, 0
  %42 = insertvalue { ptr, i64 } %41, i64 %new_len.i, 1
  ret { ptr, i64 } %42
}

; core::str::pattern::TwoWaySearcher::next
; Function Attrs: inlinehint uwtable
define internal fastcc void @_ZN4core3str7pattern14TwoWaySearcher4next17h02553c3b689b0c5fE(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(64) %self, ptr noalias noundef nonnull readonly align 1 captures(none) %haystack.0, i64 noundef range(i64 0, -9223372036854775808) %haystack.1, ptr noalias noundef nonnull readonly align 1 captures(none) %needle.0, i64 noundef range(i64 0, -9223372036854775808) %needle.1, i1 noundef zeroext %long_period) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %needle_last = add nsw i64 %needle.1, -1
  %.promoted = load i64, ptr %0, align 8
  %index24 = add i64 %needle_last, %.promoted
  %_5425 = icmp ult i64 %index24, %haystack.1
  br i1 %_5425, label %bb39.lr.ph, label %bb40

bb39.lr.ph:                                       ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_59 = load i64, ptr %1, align 8, !noundef !10
  %v1 = load i64, ptr %self, align 8
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_49 = load i64, ptr %3, align 8
  %4 = sub i64 %needle.1, %_49
  %.promoted27 = load i64, ptr %2, align 8
  br label %bb39

bb40:                                             ; preds = %bb37, %start
  store i64 %haystack.1, ptr %0, align 8
  br label %bb38

bb39:                                             ; preds = %bb39.lr.ph, %bb37
  %v229 = phi i64 [ %.promoted27, %bb39.lr.ph ], [ %v228, %bb37 ]
  %index26 = phi i64 [ %index24, %bb39.lr.ph ], [ %index, %bb37 ]
  %5 = phi i64 [ %.promoted, %bb39.lr.ph ], [ %9, %bb37 ]
  %_56 = getelementptr inbounds nuw i8, ptr %haystack.0, i64 %index26
  %tail_byte = load i8, ptr %_56, align 1, !noundef !10
  %_61 = and i8 %tail_byte, 63
  %_60 = zext nneg i8 %_61 to i64
  %6 = shl nuw i64 1, %_60
  %7 = and i64 %6, %_59
  %_17.not = icmp eq i64 %7, 0
  br i1 %_17.not, label %bb10, label %bb9

bb38:                                             ; preds = %bb34, %bb40
  %storemerge = phi i64 [ 0, %bb40 ], [ 1, %bb34 ]
  store i64 %storemerge, ptr %_0, align 8
  ret void

bb10:                                             ; preds = %bb39
  %8 = add i64 %5, %needle.1
  store i64 %8, ptr %0, align 8
  br i1 %long_period, label %bb37, label %bb37.sink.split

bb9:                                              ; preds = %bb39
  %..i = tail call i64 @llvm.umax.i64(i64 %v229, i64 %v1)
  %start1.sroa.0.0 = select i1 %long_period, i64 %v1, i64 %..i
  br label %bb16

bb37.sink.split:                                  ; preds = %bb10, %bb19, %bb29
  %.sink = phi i64 [ %4, %bb29 ], [ 0, %bb19 ], [ 0, %bb10 ]
  %.ph = phi i64 [ %15, %bb29 ], [ %19, %bb19 ], [ %8, %bb10 ]
  store i64 %.sink, ptr %2, align 8
  br label %bb37

bb37:                                             ; preds = %bb37.sink.split, %bb19, %bb29, %bb10
  %v228 = phi i64 [ %v229, %bb19 ], [ %v229, %bb29 ], [ %v229, %bb10 ], [ %.sink, %bb37.sink.split ]
  %9 = phi i64 [ %19, %bb19 ], [ %15, %bb29 ], [ %8, %bb10 ], [ %.ph, %bb37.sink.split ]
  %index = add i64 %needle_last, %9
  %_54 = icmp ult i64 %index, %haystack.1
  br i1 %_54, label %bb39, label %bb40

bb16:                                             ; preds = %bb18, %bb9
  %iter.sroa.0.0 = phi i64 [ %start1.sroa.0.0, %bb9 ], [ %_66, %bb18 ]
  %_62 = icmp ult i64 %iter.sroa.0.0, %needle.1
  br i1 %_62, label %bb42, label %bb43

bb43:                                             ; preds = %bb16
  %start2.sroa.0.0 = select i1 %long_period, i64 0, i64 %v229
  br label %bb26

bb42:                                             ; preds = %bb16
  %_29 = add i64 %iter.sroa.0.0, %5
  %_31 = icmp ult i64 %_29, %haystack.1
  br i1 %_31, label %bb18, label %panic8

bb26:                                             ; preds = %bb28, %bb43
  %iter3.sroa.2.0 = phi i64 [ %v1, %bb43 ], [ %_75, %bb28 ]
  %_72 = icmp ult i64 %start2.sroa.0.0, %iter3.sroa.2.0
  br i1 %_72, label %bb46, label %bb47

bb47:                                             ; preds = %bb26
  %10 = add i64 %5, %needle.1
  store i64 %10, ptr %0, align 8
  br i1 %long_period, label %bb34, label %bb33

bb46:                                             ; preds = %bb26
  %_75 = add i64 %iter3.sroa.2.0, -1
  %_44 = icmp ult i64 %_75, %needle.1
  br i1 %_44, label %bb27, label %panic

bb33:                                             ; preds = %bb47
  store i64 0, ptr %2, align 8
  br label %bb34

bb34:                                             ; preds = %bb33, %bb47
  %11 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %5, ptr %11, align 8, !alias.scope !164
  %12 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %10, ptr %12, align 8, !alias.scope !164
  br label %bb38

bb27:                                             ; preds = %bb46
  %_46 = add i64 %_75, %5
  %_48 = icmp ult i64 %_46, %haystack.1
  br i1 %_48, label %bb28, label %panic5

panic:                                            ; preds = %bb46
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %_75, i64 noundef %needle.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_960bc1bf861ba41d9b8231bbd37f66f6) #31
  unreachable

bb28:                                             ; preds = %bb27
  %13 = getelementptr inbounds nuw i8, ptr %needle.0, i64 %_75
  %_43 = load i8, ptr %13, align 1, !noundef !10
  %14 = getelementptr inbounds nuw i8, ptr %haystack.0, i64 %_46
  %_45 = load i8, ptr %14, align 1, !noundef !10
  %_42.not = icmp eq i8 %_43, %_45
  br i1 %_42.not, label %bb26, label %bb29

panic5:                                           ; preds = %bb27
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %_46, i64 noundef %haystack.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3c328170803c8011e86f11240ac4582e) #31
  unreachable

bb29:                                             ; preds = %bb28
  %15 = add i64 %_49, %5
  store i64 %15, ptr %0, align 8
  br i1 %long_period, label %bb37, label %bb37.sink.split

bb18:                                             ; preds = %bb42
  %_66 = add nuw nsw i64 %iter.sroa.0.0, 1
  %16 = getelementptr inbounds nuw i8, ptr %needle.0, i64 %iter.sroa.0.0
  %_26 = load i8, ptr %16, align 1, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %haystack.0, i64 %_29
  %_28 = load i8, ptr %17, align 1, !noundef !10
  %_25.not = icmp eq i8 %_26, %_28
  br i1 %_25.not, label %bb16, label %bb19

panic8:                                           ; preds = %bb42
  %18 = add i64 %start1.sroa.0.0, %5
  %umax = tail call i64 @llvm.umax.i64(i64 %haystack.1, i64 %18)
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %umax, i64 noundef %haystack.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aeae60839ee01e593e8491fb61f2dda8) #31
  unreachable

bb19:                                             ; preds = %bb18
  %_33 = add i64 %5, 1
  %_32 = add i64 %_33, %iter.sroa.0.0
  %19 = sub i64 %_32, %v1
  store i64 %19, ptr %0, align 8
  br i1 %long_period, label %bb37, label %bb37.sink.split
}

; core::iter::traits::iterator::Iterator::collect
; Function Attrs: inlinehint uwtable
define internal fastcc void @_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(48) %_0, ptr noundef nonnull %self.0, ptr noundef nonnull %self.1) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
  %set.i = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %set.i), !noalias !167
; call std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL{{tls.shim}}
  %_3.i.i.i.i.i.i = tail call noundef nonnull align 8 ptr @"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$51__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$17hed5e461344c1f9f9E"(), !noalias !170
  %_12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i, i64 16
  %0 = load i8, ptr %_12.i.i.i.i.i.i.i, align 8, !range !8, !noalias !177, !noundef !10
  %_4.i.i.i.i.i.i.i = trunc nuw i8 %0 to i1
  br i1 %_4.i.i.i.i.i.i.i, label %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i, label %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i", !prof !180

start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i: ; preds = %start
  %_9.i.pre.i.i.i.i = load i64, ptr %_3.i.i.i.i.i.i, align 8, !noalias !181
  %.phi.trans.insert.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i, i64 8
  %_10.i.pre.i.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i.i, align 8, !noalias !181
  br label %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i"

"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i": ; preds = %start
; call std::sys::random::hashmap_random_keys
  %1 = tail call { i64, i64 } @_ZN3std3sys6random19hashmap_random_keys17hc3f03c6d163b2da2E(), !noalias !182
  %2 = extractvalue { i64, i64 } %1, 0
  %3 = extractvalue { i64, i64 } %1, 1
  %4 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i, i64 8
  store i64 %3, ptr %4, align 8, !noalias !182
  store i8 1, ptr %_12.i.i.i.i.i.i.i, align 8, !noalias !182
  br label %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i"

"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i": ; preds = %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i", %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i
  %hasher.1.pre-phi.i = phi i64 [ %_10.i.pre.i.i.i.i, %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i ], [ %3, %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i" ]
  %_9.i.i.i.i.i = phi i64 [ %_9.i.pre.i.i.i.i, %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i ], [ %2, %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i" ]
  %_4.i.i.i.i.i = add i64 %_9.i.i.i.i.i, 1
  store i64 %_4.i.i.i.i.i, ptr %_3.i.i.i.i.i.i, align 8, !noalias !181
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %set.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.44ffa63e8e95c400711a21744c5ea708.0, i64 32, i1 false), !noalias !167
  %_7.sroa.4.0.set.sroa_idx.i = getelementptr inbounds nuw i8, ptr %set.i, i64 32
  store i64 %_9.i.i.i.i.i, ptr %_7.sroa.4.0.set.sroa_idx.i, align 8, !noalias !167
  %_7.sroa.5.0.set.sroa_idx.i = getelementptr inbounds nuw i8, ptr %set.i, i64 40
  store i64 %hasher.1.pre-phi.i, ptr %_7.sroa.5.0.set.sroa_idx.i, align 8, !noalias !167
  %b.i.i.i.not.i = icmp eq ptr %self.1, %self.0
  br i1 %b.i.i.i.not.i, label %"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1b209ee349209c18E.exit", label %bb8.i.i.i.i, !prof !180

bb8.i.i.i.i:                                      ; preds = %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i"
  %5 = ptrtoint ptr %self.0 to i64
  %6 = ptrtoint ptr %self.1 to i64
  %7 = sub nuw i64 %6, %5
  %d1.i.i4.i.i.i = lshr i64 %7, 2
  %r.i.i5.i.i.i = and i64 %7, 3
  %_12.not.i.i6.i.i.i = icmp ne i64 %r.i.i5.i.i.i, 0
  %8 = zext i1 %_12.not.i.i6.i.i.i to i64
  %_4.sroa.0.0.i.i7.i.i.i = add nuw nsw i64 %d1.i.i4.i.i.i, %8
; invoke hashbrown::raw::RawTable<T,A>::reserve_rehash
  %9 = invoke { i64, i64 } @"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E"(ptr noalias noundef nonnull align 8 dereferenceable(48) %set.i, i64 noundef %_4.sroa.0.0.i.i7.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %_7.sroa.4.0.set.sroa_idx.i, i1 noundef zeroext true)
          to label %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i.i.i" unwind label %funclet_bb1.i, !noalias !167

"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i.i.i": ; preds = %bb8.i.i.i.i
  %_8.0.i.i.i.i = extractvalue { i64, i64 } %9, 0
  %10 = icmp eq i64 %_8.0.i.i.i.i, -9223372036854775807
  tail call void @llvm.assume(i1 %10)
  br label %bb14.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i:                             ; preds = %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i.i.i", %_5.i.i.i.i.i.i.i.i.noexc.i
  %self.sroa.0.013.i.i.i.i.i.i = phi ptr [ %self.sroa.0.16.i.i.i.i.i.i, %_5.i.i.i.i.i.i.i.i.noexc.i ], [ %self.0, %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i.i.i" ]
  %_18.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.013.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i = load i8, ptr %self.sroa.0.013.i.i.i.i.i.i, align 1, !noalias !185, !noundef !10
  %_6.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i to i32
  %_7.i10.i.i.i.i.i.i.i.i = icmp ne ptr %_18.i.i.i.i.i.i.i.i.i, %self.1
  tail call void @llvm.assume(i1 %_7.i10.i.i.i.i.i.i.i.i)
  %_18.i12.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.013.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i = load i8, ptr %_18.i.i.i.i.i.i.i.i.i, align 1, !noalias !185, !noundef !10
  %_34.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i, 6
  %_36.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i, 63
  %_35.i.i.i.i.i.i.i.i = zext nneg i8 %_36.i.i.i.i.i.i.i.i to i32
  %11 = or disjoint i32 %_34.i.i.i.i.i.i.i.i, %_35.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i to i32
  br label %bb3.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i:                              ; preds = %bb4.i.i.i.i.i.i.i.i
  %_7.i17.i.i.i.i.i.i.i.i = icmp ne ptr %_18.i12.i.i.i.i.i.i.i.i, %self.1
  tail call void @llvm.assume(i1 %_7.i17.i.i.i.i.i.i.i.i)
  %_18.i19.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.013.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i = load i8, ptr %_18.i12.i.i.i.i.i.i.i.i, align 1, !noalias !185, !noundef !10
  %_40.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_35.i.i.i.i.i.i.i.i, 6
  %_42.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i, 63
  %_41.i.i.i.i.i.i.i.i = zext nneg i8 %_42.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i = or disjoint i32 %_40.i.i.i.i.i.i.i.i, %_41.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i, 12
  %12 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i, label %"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE.exit.i.i.i.i.i.i", label %bb3.i.i.i.i.i.i

"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE.exit.i.i.i.i.i.i": ; preds = %bb6.i.i.i.i.i.i.i.i
  %_7.i24.i.i.i.i.i.i.i.i = icmp ne ptr %_18.i19.i.i.i.i.i.i.i.i, %self.1
  tail call void @llvm.assume(i1 %_7.i24.i.i.i.i.i.i.i.i)
  %_18.i26.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.013.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i = load i8, ptr %_18.i19.i.i.i.i.i.i.i.i, align 1, !noalias !185, !noundef !10
  %_26.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i, 1835008
  %_46.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i, 6
  %_48.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i, 63
  %_47.i.i.i.i.i.i.i.i = zext nneg i8 %_48.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i = or disjoint i32 %_46.i.i.i.i.i.i.i.i, %_47.i.i.i.i.i.i.i.i
  %13 = or disjoint i32 %_27.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i
  %.not.i.i.i.i.i.i = icmp eq i32 %13, 1114112
  br i1 %.not.i.i.i.i.i.i, label %"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1b209ee349209c18E.exit", label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE.exit.i.i.i.i.i.i", %bb6.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i
  %spec.select.i7.i.i.i.i.i.i = phi i32 [ %13, %"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE.exit.i.i.i.i.i.i" ], [ %11, %bb4.i.i.i.i.i.i.i.i ], [ %12, %bb6.i.i.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i ]
  %self.sroa.0.16.i.i.i.i.i.i = phi ptr [ %_18.i26.i.i.i.i.i.i.i.i, %"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE.exit.i.i.i.i.i.i" ], [ %_18.i12.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i ], [ %_18.i19.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i ], [ %_18.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i ]
; invoke hashbrown::map::HashMap<K,V,S,A>::insert
  invoke fastcc void @"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE"(ptr noalias noundef nonnull align 8 dereferenceable(48) %set.i, i32 noundef range(i32 0, 1114112) %spec.select.i7.i.i.i.i.i.i)
          to label %_5.i.i.i.i.i.i.i.i.noexc.i unwind label %funclet_bb1.i

_5.i.i.i.i.i.i.i.i.noexc.i:                       ; preds = %bb3.i.i.i.i.i.i
  %_7.i.i.not.i.i.i.i.i.i.i = icmp eq ptr %self.sroa.0.16.i.i.i.i.i.i, %self.1
  br i1 %_7.i.i.not.i.i.i.i.i.i.i, label %"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1b209ee349209c18E.exit", label %bb14.i.i.i.i.i.i.i.i

funclet_bb1.i:                                    ; preds = %bb3.i.i.i.i.i.i, %bb8.i.i.i.i
  %cleanuppad1.i = cleanuppad within none []
  %set.val.i = load ptr, ptr %set.i, align 8, !noalias !167
  %14 = getelementptr inbounds nuw i8, ptr %set.i, i64 8
  %set.val2.i = load i64, ptr %14, align 8, !noalias !167, !noundef !10
; call core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
  call fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %set.val.i, i64 %set.val2.i) #34 [ "funclet"(token %cleanuppad1.i) ], !noalias !167
  cleanupret from %cleanuppad1.i unwind to caller

"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1b209ee349209c18E.exit": ; preds = %"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE.exit.i.i.i.i.i.i", %_5.i.i.i.i.i.i.i.i.noexc.i, %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i"
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_0, ptr noundef nonnull align 8 dereferenceable(48) %set.i, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %set.i), !noalias !167
  ret void
}

; core::error::Error::description
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, i64 } @_ZN4core5error5Error11description17h1649966410d8c91fE(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #4 {
start:
  ret { ptr, i64 } { ptr @alloc_04d7ce44d7c86a9a02b346ab945bf155, i64 40 }
}

; core::error::Error::description
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, i64 } @_ZN4core5error5Error11description17h6d684e2ae41137b1E(ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #4 {
start:
  ret { ptr, i64 } { ptr @alloc_04d7ce44d7c86a9a02b346ab945bf155, i64 40 }
}

; core::error::Error::cause
; Function Attrs: uwtable
define internal { ptr, ptr } @_ZN4core5error5Error5cause17h3780c7e2d97e9501E(ptr noundef nonnull align 8 %self) unnamed_addr #1 {
start:
; call anyhow::error::ErrorImpl::error
  %0 = tail call { ptr, ptr } @_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E(ptr noundef nonnull align 8 %self)
  %_2.0.i = extractvalue { ptr, ptr } %0, 0
  %_2.1.i = extractvalue { ptr, ptr } %0, 1
  %1 = getelementptr inbounds nuw i8, ptr %_2.1.i, i64 48
  %2 = load ptr, ptr %1, align 8, !invariant.load !10, !nonnull !10
  %3 = tail call { ptr, ptr } %2(ptr noundef align 1 %_2.0.i) #32
  ret { ptr, ptr } %3
}

; core::error::Error::cause
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_ZN4core5error5Error5cause17h7a349d6c962996eeE(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #4 {
start:
  ret { ptr, ptr } { ptr null, ptr undef }
}

; core::error::Error::provide
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal void @_ZN4core5error5Error7provide17h05d6e66acb486045E(ptr nonnull readnone align 8 captures(none) %self, ptr nonnull readnone align 8 captures(none) %request.0, ptr noalias readonly align 8 captures(none) %request.1) unnamed_addr #4 {
start:
  ret void
}

; core::error::Error::provide
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal void @_ZN4core5error5Error7provide17h9df6819c0298c953E(ptr noalias readonly align 8 captures(none) %self, ptr nonnull readnone align 8 captures(none) %request.0, ptr noalias readonly align 8 captures(none) %request.1) unnamed_addr #4 {
start:
  ret void
}

; core::error::Error::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_ZN4core5error5Error7type_id17h01cfa94765d144f3E(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #5 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.44ffa63e8e95c400711a21744c5ea708.1, i64 16, i1 false)
  ret void
}

; core::error::Error::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_ZN4core5error5Error7type_id17h515f3f39467e6574E(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #5 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.44ffa63e8e95c400711a21744c5ea708.2, i64 16, i1 false)
  ret void
}

; core::error::Error::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_ZN4core5error5Error7type_id17hc7a2819971514648E(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #5 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.44ffa63e8e95c400711a21744c5ea708.3, i64 16, i1 false)
  ret void
}

; core::error::Error::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_ZN4core5error5Error7type_id17hdc7adb8570ca242eE(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #5 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.44ffa63e8e95c400711a21744c5ea708.4, i64 16, i1 false)
  ret void
}

; core::slice::sort::shared::pivot::median3_rec
; Function Attrs: nofree nosync nounwind memory(read, inaccessiblemem: none) uwtable
define internal fastcc noundef nonnull ptr @_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE(ptr noundef nonnull readonly %0, ptr noundef nonnull readonly %1, ptr noundef nonnull readonly %2, i64 noundef range(i64 0, 144115188075855872) %n) unnamed_addr #6 {
start:
  %_6 = icmp samesign ugt i64 %n, 7
  br i1 %_6, label %bb1, label %bb6

bb1:                                              ; preds = %start
  %n84 = lshr i64 %n, 3
  %count = shl nuw nsw i64 %n84, 2
  %_10 = getelementptr inbounds nuw i64, ptr %0, i64 %count
  %count1 = mul nuw nsw i64 %n84, 7
  %_13 = getelementptr inbounds nuw i64, ptr %0, i64 %count1
; call core::slice::sort::shared::pivot::median3_rec
  %3 = tail call fastcc noundef ptr @_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE(ptr noundef %0, ptr noundef %_10, ptr noundef %_13, i64 noundef %n84)
  %_16 = getelementptr inbounds nuw i64, ptr %1, i64 %count
  %_18 = getelementptr inbounds nuw i64, ptr %1, i64 %count1
; call core::slice::sort::shared::pivot::median3_rec
  %4 = tail call fastcc noundef ptr @_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE(ptr noundef %1, ptr noundef %_16, ptr noundef %_18, i64 noundef %n84)
  %_20 = getelementptr inbounds nuw i64, ptr %2, i64 %count
  %_22 = getelementptr inbounds nuw i64, ptr %2, i64 %count1
; call core::slice::sort::shared::pivot::median3_rec
  %5 = tail call fastcc noundef ptr @_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE(ptr noundef %2, ptr noundef %_20, ptr noundef %_22, i64 noundef %n84)
  br label %bb6

bb6:                                              ; preds = %start, %bb1
  %c.sroa.0.0 = phi ptr [ %5, %bb1 ], [ %2, %start ]
  %b.sroa.0.0 = phi ptr [ %4, %bb1 ], [ %1, %start ]
  %a.sroa.0.0 = phi ptr [ %3, %bb1 ], [ %0, %start ]
  %a.sroa.0.0.val6 = load i64, ptr %a.sroa.0.0, align 8, !noundef !10
  %b.sroa.0.0.val7 = load i64, ptr %b.sroa.0.0, align 8, !noundef !10
  %_0.i = icmp ult i64 %b.sroa.0.0.val7, %a.sroa.0.0.val6
  %c.sroa.0.0.val5 = load i64, ptr %c.sroa.0.0, align 8, !noundef !10
  %_0.i8 = icmp ult i64 %c.sroa.0.0.val5, %a.sroa.0.0.val6
  %6 = xor i1 %_0.i, %_0.i8
  %_0.i9 = icmp ult i64 %c.sroa.0.0.val5, %b.sroa.0.0.val7
  %_12.i = xor i1 %_0.i, %_0.i9
  %c.b.i = select i1 %_12.i, ptr %c.sroa.0.0, ptr %b.sroa.0.0
  %_0.sroa.0.0.i = select i1 %6, ptr %a.sroa.0.0, ptr %c.b.i
  ret ptr %_0.sroa.0.0.i
}

; core::slice::sort::shared::smallsort::insertion_sort_shift_left
; Function Attrs: nounwind memory(argmem: readwrite, inaccessiblemem: write) uwtable
define internal fastcc void @_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E(ptr noalias noundef nonnull align 8 captures(address) %v.0, i64 noundef range(i64 1, 32) %v.1, i64 noundef range(i64 1, 14) %offset) unnamed_addr #7 personality ptr @__CxxFrameHandler3 {
start:
  %_6 = icmp samesign ugt i64 %offset, %v.1
  br i1 %_6, label %bb2, label %bb3

bb2:                                              ; preds = %start
  tail call void @llvm.trap()
  unreachable

bb3:                                              ; preds = %start
  %v_end = getelementptr inbounds nuw i64, ptr %v.0, i64 %v.1
  %_11.not1 = icmp samesign eq i64 %offset, %v.1
  br i1 %_11.not1, label %bb7, label %bb5.preheader

bb5.preheader:                                    ; preds = %bb3
  %0 = getelementptr inbounds nuw i64, ptr %v.0, i64 %offset
  br label %bb5

bb7:                                              ; preds = %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit, %bb3
  ret void

bb5:                                              ; preds = %bb5.preheader, %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit
  %tail.sroa.0.02 = phi ptr [ %_15, %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit ], [ %0, %bb5.preheader ]
  %1 = getelementptr inbounds i8, ptr %tail.sroa.0.02, i64 -8
  %tail.val.i = load i64, ptr %tail.sroa.0.02, align 8, !noundef !10
  %.val1.i = load i64, ptr %1, align 8, !noundef !10
  %_0.i.i = icmp ult i64 %.val1.i, %tail.val.i
  br i1 %_0.i.i, label %bb4.i, label %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit

bb4.i:                                            ; preds = %bb5, %bb7.i
  %2 = phi i64 [ %.val.i, %bb7.i ], [ %.val1.i, %bb5 ]
  %gap_guard.sroa.5.0.i = phi ptr [ %sift.sroa.0.0.i, %bb7.i ], [ %tail.sroa.0.02, %bb5 ]
  %sift.sroa.0.0.i = phi ptr [ %3, %bb7.i ], [ %1, %bb5 ]
  store i64 %2, ptr %gap_guard.sroa.5.0.i, align 8
  %_18.i = icmp eq ptr %sift.sroa.0.0.i, %v.0
  br i1 %_18.i, label %bb10.i, label %bb7.i

bb7.i:                                            ; preds = %bb4.i
  %3 = getelementptr inbounds i8, ptr %sift.sroa.0.0.i, i64 -8
  %.val.i = load i64, ptr %3, align 8, !noundef !10
  %_0.i2.i = icmp ult i64 %.val.i, %tail.val.i
  br i1 %_0.i2.i, label %bb4.i, label %bb10.i

bb10.i:                                           ; preds = %bb7.i, %bb4.i
  %sift.sroa.0.0.i.lcssa = phi ptr [ %sift.sroa.0.0.i, %bb7.i ], [ %v.0, %bb4.i ]
  store i64 %tail.val.i, ptr %sift.sroa.0.0.i.lcssa, align 8, !noalias !200
  br label %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit

_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit: ; preds = %bb5, %bb10.i
  %_15 = getelementptr inbounds nuw i8, ptr %tail.sroa.0.02, i64 8
  %_11.not = icmp eq ptr %_15, %v_end
  br i1 %_11.not, label %bb7, label %bb5
}

; core::slice::sort::unstable::ipnsort
; Function Attrs: noinline uwtable
define void @_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE(ptr noalias noundef nonnull align 8 %v.0, i64 noundef range(i64 0, 1152921504606846976) %v.1, ptr noalias noundef readnone align 8 captures(none) dereferenceable(8) %is_less) unnamed_addr #8 {
start:
  %_4.i = icmp samesign ult i64 %v.1, 2
  br i1 %_4.i, label %bb6, label %bb2.i

bb2.i:                                            ; preds = %start
  %_28.i = getelementptr inbounds nuw i8, ptr %v.0, i64 8
  %_28.i.val = load i64, ptr %_28.i, align 8, !noundef !10
  %v.0.val = load i64, ptr %v.0, align 8, !noundef !10
  %_0.i = icmp ult i64 %v.0.val, %_28.i.val
  %_10.i15.not = icmp eq i64 %v.1, 2
  br i1 %_0.i, label %bb4.i.preheader, label %bb11.i.preheader

bb11.i.preheader:                                 ; preds = %bb2.i
  br i1 %_10.i15.not, label %_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit, label %bb12.i

bb4.i.preheader:                                  ; preds = %bb2.i
  br i1 %_10.i15.not, label %_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit, label %bb5.i

bb12.i:                                           ; preds = %bb11.i.preheader, %bb15.i
  %_44.i.val = phi i64 [ %.val2, %bb15.i ], [ %_28.i.val, %bb11.i.preheader ]
  %run_len.sroa.0.0.i13 = phi i64 [ %1, %bb15.i ], [ 2, %bb11.i.preheader ]
  %0 = getelementptr inbounds nuw i64, ptr %v.0, i64 %run_len.sroa.0.0.i13
  %.val2 = load i64, ptr %0, align 8, !noundef !10
  %_0.i3 = icmp ult i64 %_44.i.val, %.val2
  br i1 %_0.i3, label %_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit, label %bb15.i

bb15.i:                                           ; preds = %bb12.i
  %1 = add nuw nsw i64 %run_len.sroa.0.0.i13, 1
  %exitcond.not = icmp eq i64 %1, %v.1
  br i1 %exitcond.not, label %bb2, label %bb12.i

bb5.i:                                            ; preds = %bb4.i.preheader, %bb7.i
  %_38.i.val = phi i64 [ %.val, %bb7.i ], [ %_28.i.val, %bb4.i.preheader ]
  %run_len.sroa.0.1.i16 = phi i64 [ %3, %bb7.i ], [ 2, %bb4.i.preheader ]
  %2 = getelementptr inbounds nuw i64, ptr %v.0, i64 %run_len.sroa.0.1.i16
  %.val = load i64, ptr %2, align 8, !noundef !10
  %_0.i4 = icmp ult i64 %_38.i.val, %.val
  br i1 %_0.i4, label %bb7.i, label %_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit

bb7.i:                                            ; preds = %bb5.i
  %3 = add nuw nsw i64 %run_len.sroa.0.1.i16, 1
  %exitcond21.not = icmp eq i64 %3, %v.1
  br i1 %exitcond21.not, label %bb2, label %bb5.i

_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit: ; preds = %bb12.i, %bb5.i, %bb11.i.preheader, %bb4.i.preheader
  %_0.sroa.0.0.i = phi i64 [ 2, %bb4.i.preheader ], [ 2, %bb11.i.preheader ], [ %run_len.sroa.0.1.i16, %bb5.i ], [ %run_len.sroa.0.0.i13, %bb12.i ]
  %_8 = icmp samesign ule i64 %_0.sroa.0.0.i, %v.1
  tail call void @llvm.assume(i1 %_8)
  %_9 = icmp eq i64 %_0.sroa.0.0.i, %v.1
  br i1 %_9, label %bb2, label %bb9

bb2:                                              ; preds = %bb15.i, %bb7.i, %_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit
  br i1 %_0.i, label %bb5.preheader.i.i, label %bb6

bb9:                                              ; preds = %_ZN4core5slice4sort6shared17find_existing_run17h1d3d46de54b3b97cE.exit
  %self = or i64 %v.1, 1
  %4 = tail call range(i64 4, 64) i64 @llvm.ctlz.i64(i64 %self, i1 true)
  %5 = trunc nuw nsw i64 %4 to i32
  %log = shl nuw nsw i32 %5, 1
  %limit = xor i32 %log, 126
; call core::slice::sort::unstable::quicksort::quicksort
  tail call fastcc void @_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E(ptr noalias noundef nonnull align 8 %v.0, i64 noundef %v.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(8) null, i32 noundef %limit, ptr noalias noundef align 8 dereferenceable(8) %is_less)
  br label %bb6

bb6:                                              ; preds = %bb6.i.i, %middle.block, %start, %bb2, %bb9
  ret void

bb5.preheader.i.i:                                ; preds = %bb2
  %half_len2.i = lshr i64 %v.1, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !205)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !208)
  %end.i = getelementptr inbounds nuw i64, ptr %v.0, i64 %v.1
  %min.iters.check = icmp samesign ult i64 %v.1, 8
  br i1 %min.iters.check, label %bb6.i.i.preheader, label %vector.ph

vector.ph:                                        ; preds = %bb5.preheader.i.i
  %n.vec = and i64 %half_len2.i, 576460752303423484
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %6 = xor i64 %index, -1
  %7 = getelementptr inbounds nuw i64, ptr %v.0, i64 %index
  %8 = getelementptr i64, ptr %end.i, i64 %6
  %9 = getelementptr inbounds nuw i8, ptr %7, i64 16
  %wide.load = load <2 x i64>, ptr %7, align 8, !alias.scope !210, !noalias !208
  %wide.load35 = load <2 x i64>, ptr %9, align 8, !alias.scope !210, !noalias !208
  %10 = getelementptr i8, ptr %8, i64 -8
  %11 = getelementptr i8, ptr %8, i64 -24
  %wide.load36 = load <2 x i64>, ptr %10, align 8, !alias.scope !213, !noalias !205
  %reverse = shufflevector <2 x i64> %wide.load36, <2 x i64> poison, <2 x i32> <i32 1, i32 0>
  %wide.load37 = load <2 x i64>, ptr %11, align 8, !alias.scope !213, !noalias !205
  %reverse38 = shufflevector <2 x i64> %wide.load37, <2 x i64> poison, <2 x i32> <i32 1, i32 0>
  store <2 x i64> %reverse, ptr %7, align 8, !alias.scope !210, !noalias !208
  store <2 x i64> %reverse38, ptr %9, align 8, !alias.scope !210, !noalias !208
  %reverse39 = shufflevector <2 x i64> %wide.load, <2 x i64> poison, <2 x i32> <i32 1, i32 0>
  store <2 x i64> %reverse39, ptr %10, align 8, !alias.scope !213, !noalias !205
  %reverse40 = shufflevector <2 x i64> %wide.load35, <2 x i64> poison, <2 x i32> <i32 1, i32 0>
  store <2 x i64> %reverse40, ptr %11, align 8, !alias.scope !213, !noalias !205
  %index.next = add nuw i64 %index, 4
  %12 = icmp eq i64 %index.next, %n.vec
  br i1 %12, label %middle.block, label %vector.body, !llvm.loop !214

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %half_len2.i, %n.vec
  br i1 %cmp.n, label %bb6, label %bb6.i.i.preheader

bb6.i.i.preheader:                                ; preds = %bb5.preheader.i.i, %middle.block
  %i.sroa.0.016.i.i.ph = phi i64 [ 0, %bb5.preheader.i.i ], [ %n.vec, %middle.block ]
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb6.i.i.preheader, %bb6.i.i
  %i.sroa.0.016.i.i = phi i64 [ %16, %bb6.i.i ], [ %i.sroa.0.016.i.i.ph, %bb6.i.i.preheader ]
  %13 = xor i64 %i.sroa.0.016.i.i, -1
  %x.i.i = getelementptr inbounds nuw i64, ptr %v.0, i64 %i.sroa.0.016.i.i
  %y.i.i = getelementptr i64, ptr %end.i, i64 %13
  %14 = load i64, ptr %x.i.i, align 8, !alias.scope !210, !noalias !208, !noundef !10
  %15 = load i64, ptr %y.i.i, align 8, !alias.scope !213, !noalias !205
  store i64 %15, ptr %x.i.i, align 8, !alias.scope !210, !noalias !208
  store i64 %14, ptr %y.i.i, align 8, !alias.scope !213, !noalias !205
  %16 = add nuw nsw i64 %i.sroa.0.016.i.i, 1
  %exitcond.not.i.i = icmp eq i64 %16, %half_len2.i
  br i1 %exitcond.not.i.i, label %bb6, label %bb6.i.i, !llvm.loop !217
}

; core::slice::sort::unstable::heapsort::heapsort
; Function Attrs: nofree noinline norecurse nosync nounwind memory(argmem: readwrite, inaccessiblemem: readwrite) uwtable
define void @_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE(ptr noalias noundef nonnull align 8 captures(none) %v.0, i64 noundef range(i64 0, 1152921504606846976) %v.1, ptr noalias readnone align 8 captures(none) %is_less) unnamed_addr #9 personality ptr @__CxxFrameHandler3 {
start:
  %_75 = lshr i64 %v.1, 1
  %_6 = add nuw nsw i64 %_75, %v.1
  %_17.not11 = icmp eq i64 %_6, 0
  br i1 %_17.not11, label %bb7, label %bb6

bb7:                                              ; preds = %_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit, %start
  ret void

bb6:                                              ; preds = %start, %_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit
  %iter.sroa.2.012 = phi i64 [ %_20, %_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit ], [ %_6, %start ]
  %_20 = add nsw i64 %iter.sroa.2.012, -1
  %_12.not = icmp ult i64 %_20, %v.1
  br i1 %_12.not, label %bb11, label %bb2

bb2:                                              ; preds = %bb6
  %0 = sub nuw nsw i64 %_20, %v.1
  br label %bb14

bb11:                                             ; preds = %bb6
  %pb = getelementptr inbounds nuw i64, ptr %v.0, i64 %_20
  %tmp.sroa.0.0.copyload.i = load i64, ptr %v.0, align 8
  %1 = load i64, ptr %pb, align 8
  store i64 %1, ptr %v.0, align 8
  store i64 %tmp.sroa.0.0.copyload.i, ptr %pb, align 8
  br label %bb14

bb14:                                             ; preds = %bb11, %bb2
  %sift_idx.sroa.0.0 = phi i64 [ %0, %bb2 ], [ 0, %bb11 ]
  %..i = tail call noundef i64 @llvm.umin.i64(i64 %v.1, i64 %_20)
  %_4.i = icmp ule i64 %sift_idx.sroa.0.0, %..i
  tail call void @llvm.assume(i1 %_4.i)
  %_9.i7 = shl nuw nsw i64 %sift_idx.sroa.0.0, 1
  %2 = or disjoint i64 %_9.i7, 1
  %_11.not.i8 = icmp samesign ult i64 %2, %..i
  br i1 %_11.not.i8, label %bb3.i, label %_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit

bb3.i:                                            ; preds = %bb14, %bb9.i
  %3 = phi i64 [ %5, %bb9.i ], [ %2, %bb14 ]
  %_9.i10 = phi i64 [ %_9.i, %bb9.i ], [ %_9.i7, %bb14 ]
  %node.sroa.0.0.i9 = phi i64 [ %child.sroa.0.0.i, %bb9.i ], [ %sift_idx.sroa.0.0, %bb14 ]
  %_14.i = add nuw nsw i64 %_9.i10, 2
  %_13.i = icmp samesign ult i64 %_14.i, %..i
  br i1 %_13.i, label %bb4.i, label %bb7.i

bb4.i:                                            ; preds = %bb3.i
  %_20.i = getelementptr inbounds nuw i64, ptr %v.0, i64 %3
  %_23.i = getelementptr inbounds nuw i64, ptr %v.0, i64 %_14.i
  %_20.i.val = load i64, ptr %_20.i, align 8, !noundef !10
  %_23.i.val = load i64, ptr %_23.i, align 8, !noundef !10
  %_0.i = icmp ult i64 %_23.i.val, %_20.i.val
  %_16.i = zext i1 %_0.i to i64
  %4 = add nuw nsw i64 %3, %_16.i
  br label %bb7.i

bb7.i:                                            ; preds = %bb4.i, %bb3.i
  %child.sroa.0.0.i = phi i64 [ %4, %bb4.i ], [ %3, %bb3.i ]
  %_29.i = getelementptr inbounds nuw i64, ptr %v.0, i64 %node.sroa.0.0.i9
  %_32.i = getelementptr inbounds nuw i64, ptr %v.0, i64 %child.sroa.0.0.i
  %_29.i.val = load i64, ptr %_29.i, align 8, !noundef !10
  %_32.i.val = load i64, ptr %_32.i, align 8, !noundef !10
  %_0.i6 = icmp ult i64 %_32.i.val, %_29.i.val
  br i1 %_0.i6, label %bb9.i, label %_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit

bb9.i:                                            ; preds = %bb7.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !218)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !221)
  store i64 %_32.i.val, ptr %_29.i, align 8, !alias.scope !218, !noalias !221
  store i64 %_29.i.val, ptr %_32.i, align 8, !alias.scope !221, !noalias !218
  %_9.i = shl nuw nsw i64 %child.sroa.0.0.i, 1
  %5 = or disjoint i64 %_9.i, 1
  %_11.not.i = icmp samesign ult i64 %5, %..i
  br i1 %_11.not.i, label %bb3.i, label %_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit

_ZN4core5slice4sort8unstable8heapsort9sift_down17hc39d1fe3120e0d7fE.exit: ; preds = %bb7.i, %bb9.i, %bb14
  %_17.not = icmp eq i64 %_20, 0
  br i1 %_17.not, label %bb7, label %bb6
}

; core::slice::sort::unstable::quicksort::quicksort
; Function Attrs: uwtable
define internal fastcc void @_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E(ptr noalias noundef nonnull align 8 %0, i64 noundef range(i64 0, 1152921504606846976) %1, ptr noalias noundef readonly align 8 captures(address) dereferenceable_or_null(8) %2, i32 noundef range(i32 0, 127) %3, ptr noalias noundef nonnull readnone align 8 captures(none) dereferenceable(8) %is_less) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %stack_array.i = alloca [256 x i8], align 8
  %_589 = icmp samesign ult i64 %1, 33
  br i1 %_589, label %bb3, label %bb5

bb5:                                              ; preds = %start, %bb1.backedge
  %v.sroa.0.093 = phi ptr [ %v.sroa.0.0.be, %bb1.backedge ], [ %0, %start ]
  %v.sroa.15.092 = phi i64 [ %v.sroa.15.0.be, %bb1.backedge ], [ %1, %start ]
  %ancestor_pivot.sroa.0.091 = phi ptr [ %ancestor_pivot.sroa.0.0.be, %bb1.backedge ], [ %2, %start ]
  %limit.sroa.0.090 = phi i32 [ %90, %bb1.backedge ], [ %3, %start ]
  %_10 = icmp eq i32 %limit.sroa.0.090, 0
  br i1 %_10, label %bb6, label %bb7

bb3:                                              ; preds = %bb1.backedge, %start
  %v.sroa.15.0.lcssa = phi i64 [ %1, %start ], [ %v.sroa.15.0.be, %bb1.backedge ]
  %v.sroa.0.0.lcssa = phi ptr [ %0, %start ], [ %v.sroa.0.0.be, %bb1.backedge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !223)
  %_4.i = icmp samesign ult i64 %v.sroa.15.0.lcssa, 2
  br i1 %_4.i, label %bb22, label %bb4.i

bb4.i:                                            ; preds = %bb3
  call void @llvm.lifetime.start.p0(i64 256, ptr nonnull %stack_array.i), !noalias !223
  %len_div_26.i = lshr i64 %v.sroa.15.0.lcssa, 1
  %no_merge.i = icmp samesign ult i64 %v.sroa.15.0.lcssa, 18
  %v.1.len_div_26.i = select i1 %no_merge.i, i64 %v.sroa.15.0.lcssa, i64 %len_div_26.i
  %data.i = getelementptr i64, ptr %v.sroa.0.0.lcssa, i64 %len_div_26.i
  %len.i = sub nuw nsw i64 %v.sroa.15.0.lcssa, %len_div_26.i
  br label %bb8.i

bb8.i:                                            ; preds = %bb19.i, %bb4.i
  %region.sroa.8.0.i = phi i64 [ %v.1.len_div_26.i, %bb4.i ], [ %len.i, %bb19.i ]
  %region.sroa.0.0.i = phi ptr [ %v.sroa.0.0.lcssa, %bb4.i ], [ %data.i, %bb19.i ]
  %_14.i = icmp ugt i64 %region.sroa.8.0.i, 12
  br i1 %_14.i, label %bb9.i, label %bb11.i

bb11.i:                                           ; preds = %bb8.i
  %_19.i = icmp samesign ugt i64 %region.sroa.8.0.i, 8
  br i1 %_19.i, label %bb12.i, label %bb16.i

bb9.i:                                            ; preds = %bb8.i
  %v_b.i.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 96
  %v_b.val.i.i.i = load i64, ptr %v_b.i.i.i, align 8, !alias.scope !226, !noundef !10
  %v_a.val.i.i.i = load i64, ptr %region.sroa.0.0.i, align 8, !alias.scope !226, !noundef !10
  %value.i.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i.i.i, i64 %v_b.val.i.i.i)
  %4 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i.i.i, i64 %v_b.val.i.i.i)
  %v_a.i.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 8
  %v_b.i1.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 80
  %v_b.val.i2.i.i = load i64, ptr %v_b.i1.i.i, align 8, !alias.scope !226, !noundef !10
  %v_a.val.i3.i.i = load i64, ptr %v_a.i.i.i, align 8, !alias.scope !226, !noundef !10
  %value.i4.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i3.i.i, i64 %v_b.val.i2.i.i)
  %5 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i3.i.i, i64 %v_b.val.i2.i.i)
  %v_a.i5.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 16
  %v_b.i6.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 72
  %v_b.val.i7.i.i = load i64, ptr %v_b.i6.i.i, align 8, !alias.scope !226, !noundef !10
  %v_a.val.i8.i.i = load i64, ptr %v_a.i5.i.i, align 8, !alias.scope !226, !noundef !10
  %value.i9.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i8.i.i, i64 %v_b.val.i7.i.i)
  %6 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i8.i.i, i64 %v_b.val.i7.i.i)
  %v_a.i10.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 24
  %v_b.i11.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 56
  %v_b.val.i12.i.i = load i64, ptr %v_b.i11.i.i, align 8, !alias.scope !226, !noundef !10
  %v_a.val.i13.i.i = load i64, ptr %v_a.i10.i.i, align 8, !alias.scope !226, !noundef !10
  %value.i14.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i13.i.i, i64 %v_b.val.i12.i.i)
  %7 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i13.i.i, i64 %v_b.val.i12.i.i)
  %v_a.i15.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 40
  %v_b.i16.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 88
  %v_b.val.i17.i.i = load i64, ptr %v_b.i16.i.i, align 8, !alias.scope !226, !noundef !10
  %v_a.val.i18.i.i = load i64, ptr %v_a.i15.i.i, align 8, !alias.scope !226, !noundef !10
  %value.i19.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i18.i.i, i64 %v_b.val.i17.i.i)
  %8 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i18.i.i, i64 %v_b.val.i17.i.i)
  %v_a.i20.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 48
  %v_b.i21.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 64
  %v_b.val.i22.i.i = load i64, ptr %v_b.i21.i.i, align 8, !alias.scope !226, !noundef !10
  %v_a.val.i23.i.i = load i64, ptr %v_a.i20.i.i, align 8, !alias.scope !226, !noundef !10
  %value.i24.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i23.i.i, i64 %v_b.val.i22.i.i)
  %9 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i23.i.i, i64 %v_b.val.i22.i.i)
  %value.i29.i.i = tail call i64 @llvm.umin.i64(i64 %5, i64 %9)
  %10 = tail call i64 @llvm.umax.i64(i64 %5, i64 %9)
  %value.i34.i.i = tail call i64 @llvm.umin.i64(i64 %6, i64 %7)
  %11 = tail call i64 @llvm.umax.i64(i64 %6, i64 %7)
  %v_a.i35.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 32
  %v_a.val.i38.i.i = load i64, ptr %v_a.i35.i.i, align 8, !alias.scope !226, !noundef !10
  %value.i39.i.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i38.i.i, i64 %value.i19.i.i)
  %12 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i38.i.i, i64 %value.i19.i.i)
  %value.i44.i.i = tail call i64 @llvm.umin.i64(i64 %value.i14.i.i, i64 %value.i9.i.i)
  %13 = tail call i64 @llvm.umax.i64(i64 %value.i14.i.i, i64 %value.i9.i.i)
  %value.i49.i.i = tail call i64 @llvm.umin.i64(i64 %value.i24.i.i, i64 %value.i4.i.i)
  %14 = tail call i64 @llvm.umax.i64(i64 %value.i24.i.i, i64 %value.i4.i.i)
  %value.i54.i.i = tail call i64 @llvm.umin.i64(i64 %4, i64 %12)
  %15 = tail call i64 @llvm.umax.i64(i64 %4, i64 %12)
  %value.i59.i.i = tail call i64 @llvm.umin.i64(i64 %10, i64 %11)
  %16 = tail call i64 @llvm.umax.i64(i64 %10, i64 %11)
  %value.i64.i.i = tail call i64 @llvm.umin.i64(i64 %value.i34.i.i, i64 %value.i29.i.i)
  %17 = tail call i64 @llvm.umax.i64(i64 %value.i34.i.i, i64 %value.i29.i.i)
  %value.i69.i.i = tail call i64 @llvm.umin.i64(i64 %13, i64 %14)
  %18 = tail call i64 @llvm.umax.i64(i64 %13, i64 %14)
  %value.i74.i.i = tail call i64 @llvm.umin.i64(i64 %value.i44.i.i, i64 %value.i49.i.i)
  %19 = tail call i64 @llvm.umax.i64(i64 %value.i44.i.i, i64 %value.i49.i.i)
  %value.i79.i.i = tail call i64 @llvm.umin.i64(i64 %value.i39.i.i, i64 %value.i.i.i)
  %20 = tail call i64 @llvm.umax.i64(i64 %value.i39.i.i, i64 %value.i.i.i)
  %value.i84.i.i = tail call i64 @llvm.umin.i64(i64 %value.i54.i.i, i64 %value.i64.i.i)
  %21 = tail call i64 @llvm.umax.i64(i64 %value.i54.i.i, i64 %value.i64.i.i)
  %value.i89.i.i = tail call i64 @llvm.umin.i64(i64 %8, i64 %19)
  %22 = tail call i64 @llvm.umax.i64(i64 %8, i64 %19)
  %value.i94.i.i = tail call i64 @llvm.umin.i64(i64 %value.i69.i.i, i64 %20)
  %23 = tail call i64 @llvm.umax.i64(i64 %value.i69.i.i, i64 %20)
  %value.i99.i.i = tail call i64 @llvm.umin.i64(i64 %value.i74.i.i, i64 %value.i79.i.i)
  %24 = tail call i64 @llvm.umax.i64(i64 %value.i74.i.i, i64 %value.i79.i.i)
  store i64 %value.i99.i.i, ptr %v_b.i.i.i, align 8, !alias.scope !226
  %value.i104.i.i = tail call i64 @llvm.umin.i64(i64 %15, i64 %22)
  %25 = tail call i64 @llvm.umax.i64(i64 %15, i64 %22)
  %value.i109.i.i = tail call i64 @llvm.umin.i64(i64 %17, i64 %23)
  %26 = tail call i64 @llvm.umax.i64(i64 %17, i64 %23)
  %value.i114.i.i = tail call i64 @llvm.umin.i64(i64 %21, i64 %18)
  %27 = tail call i64 @llvm.umax.i64(i64 %21, i64 %18)
  %value.i119.i.i = tail call i64 @llvm.umin.i64(i64 %value.i84.i.i, i64 %value.i94.i.i)
  %28 = tail call i64 @llvm.umax.i64(i64 %value.i84.i.i, i64 %value.i94.i.i)
  %value.i124.i.i = tail call i64 @llvm.umin.i64(i64 %value.i89.i.i, i64 %24)
  %29 = tail call i64 @llvm.umax.i64(i64 %value.i89.i.i, i64 %24)
  %value.i129.i.i = tail call i64 @llvm.umin.i64(i64 %25, i64 %16)
  %30 = tail call i64 @llvm.umax.i64(i64 %25, i64 %16)
  store i64 %30, ptr %region.sroa.0.0.i, align 8, !alias.scope !226
  %value.i134.i.i = tail call i64 @llvm.umin.i64(i64 %value.i59.i.i, i64 %value.i104.i.i)
  %31 = tail call i64 @llvm.umax.i64(i64 %value.i59.i.i, i64 %value.i104.i.i)
  %value.i139.i.i = tail call i64 @llvm.umin.i64(i64 %28, i64 %29)
  %32 = tail call i64 @llvm.umax.i64(i64 %28, i64 %29)
  %value.i144.i.i = tail call i64 @llvm.umin.i64(i64 %value.i114.i.i, i64 %value.i109.i.i)
  %33 = tail call i64 @llvm.umax.i64(i64 %value.i114.i.i, i64 %value.i109.i.i)
  %value.i149.i.i = tail call i64 @llvm.umin.i64(i64 %value.i124.i.i, i64 %value.i119.i.i)
  %34 = tail call i64 @llvm.umax.i64(i64 %value.i124.i.i, i64 %value.i119.i.i)
  store i64 %value.i149.i.i, ptr %v_b.i16.i.i, align 8, !alias.scope !226
  %value.i154.i.i = tail call i64 @llvm.umin.i64(i64 %value.i129.i.i, i64 %26)
  %35 = tail call i64 @llvm.umax.i64(i64 %value.i129.i.i, i64 %26)
  %value.i159.i.i = tail call i64 @llvm.umin.i64(i64 %31, i64 %27)
  %36 = tail call i64 @llvm.umax.i64(i64 %31, i64 %27)
  %value.i164.i.i = tail call i64 @llvm.umin.i64(i64 %value.i134.i.i, i64 %32)
  %37 = tail call i64 @llvm.umax.i64(i64 %value.i134.i.i, i64 %32)
  %value.i169.i.i = tail call i64 @llvm.umin.i64(i64 %value.i139.i.i, i64 %34)
  %38 = tail call i64 @llvm.umax.i64(i64 %value.i139.i.i, i64 %34)
  store i64 %value.i169.i.i, ptr %v_b.i1.i.i, align 8, !alias.scope !226
  %value.i174.i.i = tail call i64 @llvm.umin.i64(i64 %35, i64 %36)
  %39 = tail call i64 @llvm.umax.i64(i64 %35, i64 %36)
  store i64 %39, ptr %v_a.i.i.i, align 8, !alias.scope !226
  %value.i179.i.i = tail call i64 @llvm.umin.i64(i64 %value.i154.i.i, i64 %value.i159.i.i)
  %40 = tail call i64 @llvm.umax.i64(i64 %value.i154.i.i, i64 %value.i159.i.i)
  %value.i184.i.i = tail call i64 @llvm.umin.i64(i64 %37, i64 %33)
  %41 = tail call i64 @llvm.umax.i64(i64 %37, i64 %33)
  %value.i189.i.i = tail call i64 @llvm.umin.i64(i64 %value.i164.i.i, i64 %value.i144.i.i)
  %42 = tail call i64 @llvm.umax.i64(i64 %value.i164.i.i, i64 %value.i144.i.i)
  %value.i194.i.i = tail call i64 @llvm.umin.i64(i64 %value.i174.i.i, i64 %40)
  %43 = tail call i64 @llvm.umax.i64(i64 %value.i174.i.i, i64 %40)
  store i64 %43, ptr %v_a.i5.i.i, align 8, !alias.scope !226
  %value.i199.i.i = tail call i64 @llvm.umin.i64(i64 %value.i179.i.i, i64 %41)
  %44 = tail call i64 @llvm.umax.i64(i64 %value.i179.i.i, i64 %41)
  %value.i204.i.i = tail call i64 @llvm.umin.i64(i64 %42, i64 %value.i184.i.i)
  %45 = tail call i64 @llvm.umax.i64(i64 %42, i64 %value.i184.i.i)
  store i64 %value.i204.i.i, ptr %v_b.i11.i.i, align 8, !alias.scope !226
  %value.i209.i.i = tail call i64 @llvm.umin.i64(i64 %value.i189.i.i, i64 %38)
  %46 = tail call i64 @llvm.umax.i64(i64 %value.i189.i.i, i64 %38)
  store i64 %46, ptr %v_b.i21.i.i, align 8, !alias.scope !226
  store i64 %value.i209.i.i, ptr %v_b.i6.i.i, align 8, !alias.scope !226
  %value.i214.i.i = tail call i64 @llvm.umin.i64(i64 %value.i194.i.i, i64 %44)
  %47 = tail call i64 @llvm.umax.i64(i64 %value.i194.i.i, i64 %44)
  store i64 %47, ptr %v_a.i10.i.i, align 8, !alias.scope !226
  store i64 %value.i214.i.i, ptr %v_a.i35.i.i, align 8, !alias.scope !226
  %value.i219.i.i = tail call i64 @llvm.umin.i64(i64 %value.i199.i.i, i64 %45)
  %48 = tail call i64 @llvm.umax.i64(i64 %value.i199.i.i, i64 %45)
  store i64 %48, ptr %v_a.i15.i.i, align 8, !alias.scope !226
  store i64 %value.i219.i.i, ptr %v_a.i20.i.i, align 8, !alias.scope !226
  br label %bb16.i

bb12.i:                                           ; preds = %bb11.i
  %v_b.i.i7.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 24
  %v_b.val.i.i8.i = load i64, ptr %v_b.i.i7.i, align 8, !alias.scope !229, !noundef !10
  %v_a.val.i.i9.i = load i64, ptr %region.sroa.0.0.i, align 8, !alias.scope !229, !noundef !10
  %value.i.i10.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i.i9.i, i64 %v_b.val.i.i8.i)
  %49 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i.i9.i, i64 %v_b.val.i.i8.i)
  %v_a.i.i11.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 8
  %v_b.i1.i12.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 56
  %v_b.val.i2.i13.i = load i64, ptr %v_b.i1.i12.i, align 8, !alias.scope !229, !noundef !10
  %v_a.val.i3.i14.i = load i64, ptr %v_a.i.i11.i, align 8, !alias.scope !229, !noundef !10
  %value.i4.i15.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i3.i14.i, i64 %v_b.val.i2.i13.i)
  %50 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i3.i14.i, i64 %v_b.val.i2.i13.i)
  %v_a.i5.i16.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 16
  %v_b.i6.i17.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 40
  %v_b.val.i7.i18.i = load i64, ptr %v_b.i6.i17.i, align 8, !alias.scope !229, !noundef !10
  %v_a.val.i8.i19.i = load i64, ptr %v_a.i5.i16.i, align 8, !alias.scope !229, !noundef !10
  %value.i9.i20.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i8.i19.i, i64 %v_b.val.i7.i18.i)
  %51 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i8.i19.i, i64 %v_b.val.i7.i18.i)
  %v_a.i10.i21.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 32
  %v_b.i11.i22.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 64
  %v_b.val.i12.i23.i = load i64, ptr %v_b.i11.i22.i, align 8, !alias.scope !229, !noundef !10
  %v_a.val.i13.i24.i = load i64, ptr %v_a.i10.i21.i, align 8, !alias.scope !229, !noundef !10
  %value.i14.i25.i = tail call i64 @llvm.umin.i64(i64 %v_a.val.i13.i24.i, i64 %v_b.val.i12.i23.i)
  %52 = tail call i64 @llvm.umax.i64(i64 %v_a.val.i13.i24.i, i64 %v_b.val.i12.i23.i)
  %value.i19.i26.i = tail call i64 @llvm.umin.i64(i64 %49, i64 %value.i4.i15.i)
  %53 = tail call i64 @llvm.umax.i64(i64 %49, i64 %value.i4.i15.i)
  %value.i24.i27.i = tail call i64 @llvm.umin.i64(i64 %51, i64 %52)
  %54 = tail call i64 @llvm.umax.i64(i64 %51, i64 %52)
  %value.i29.i28.i = tail call i64 @llvm.umin.i64(i64 %value.i.i10.i, i64 %value.i14.i25.i)
  %55 = tail call i64 @llvm.umax.i64(i64 %value.i.i10.i, i64 %value.i14.i25.i)
  %v_b.i31.i.i = getelementptr inbounds nuw i8, ptr %region.sroa.0.0.i, i64 48
  %v_b.val.i32.i.i = load i64, ptr %v_b.i31.i.i, align 8, !alias.scope !229, !noundef !10
  %value.i34.i29.i = tail call i64 @llvm.umin.i64(i64 %value.i9.i20.i, i64 %v_b.val.i32.i.i)
  %56 = tail call i64 @llvm.umax.i64(i64 %value.i9.i20.i, i64 %v_b.val.i32.i.i)
  %value.i39.i30.i = tail call i64 @llvm.umin.i64(i64 %53, i64 %54)
  %57 = tail call i64 @llvm.umax.i64(i64 %53, i64 %54)
  %value.i44.i31.i = tail call i64 @llvm.umin.i64(i64 %50, i64 %55)
  %58 = tail call i64 @llvm.umax.i64(i64 %50, i64 %55)
  %value.i49.i32.i = tail call i64 @llvm.umin.i64(i64 %value.i24.i27.i, i64 %56)
  %59 = tail call i64 @llvm.umax.i64(i64 %value.i24.i27.i, i64 %56)
  %value.i54.i33.i = tail call i64 @llvm.umin.i64(i64 %value.i19.i26.i, i64 %value.i29.i28.i)
  %60 = tail call i64 @llvm.umax.i64(i64 %value.i19.i26.i, i64 %value.i29.i28.i)
  %value.i59.i34.i = tail call i64 @llvm.umin.i64(i64 %58, i64 %59)
  %61 = tail call i64 @llvm.umax.i64(i64 %58, i64 %59)
  %value.i64.i35.i = tail call i64 @llvm.umin.i64(i64 %value.i44.i31.i, i64 %value.i34.i29.i)
  %62 = tail call i64 @llvm.umax.i64(i64 %value.i44.i31.i, i64 %value.i34.i29.i)
  %value.i69.i36.i = tail call i64 @llvm.umin.i64(i64 %value.i49.i32.i, i64 %60)
  %63 = tail call i64 @llvm.umax.i64(i64 %value.i49.i32.i, i64 %60)
  %value.i74.i37.i = tail call i64 @llvm.umin.i64(i64 %57, i64 %61)
  %64 = tail call i64 @llvm.umax.i64(i64 %57, i64 %61)
  store i64 %64, ptr %region.sroa.0.0.i, align 8, !alias.scope !229
  %value.i79.i38.i = tail call i64 @llvm.umin.i64(i64 %value.i39.i30.i, i64 %value.i59.i34.i)
  %65 = tail call i64 @llvm.umax.i64(i64 %value.i39.i30.i, i64 %value.i59.i34.i)
  %value.i84.i39.i = tail call i64 @llvm.umin.i64(i64 %62, i64 %63)
  %66 = tail call i64 @llvm.umax.i64(i64 %62, i64 %63)
  %value.i89.i40.i = tail call i64 @llvm.umin.i64(i64 %value.i64.i35.i, i64 %value.i54.i33.i)
  %67 = tail call i64 @llvm.umax.i64(i64 %value.i64.i35.i, i64 %value.i54.i33.i)
  store i64 %value.i89.i40.i, ptr %v_b.i11.i22.i, align 8, !alias.scope !229
  %value.i94.i41.i = tail call i64 @llvm.umin.i64(i64 %65, i64 %66)
  %68 = tail call i64 @llvm.umax.i64(i64 %65, i64 %66)
  %value.i99.i42.i = tail call i64 @llvm.umin.i64(i64 %value.i79.i38.i, i64 %value.i84.i39.i)
  %69 = tail call i64 @llvm.umax.i64(i64 %value.i79.i38.i, i64 %value.i84.i39.i)
  %value.i104.i43.i = tail call i64 @llvm.umin.i64(i64 %67, i64 %value.i69.i36.i)
  %70 = tail call i64 @llvm.umax.i64(i64 %67, i64 %value.i69.i36.i)
  store i64 %value.i104.i43.i, ptr %v_b.i1.i12.i, align 8, !alias.scope !229
  %value.i109.i44.i = tail call i64 @llvm.umin.i64(i64 %value.i74.i37.i, i64 %68)
  %71 = tail call i64 @llvm.umax.i64(i64 %value.i74.i37.i, i64 %68)
  store i64 %71, ptr %v_a.i.i11.i, align 8, !alias.scope !229
  store i64 %value.i109.i44.i, ptr %v_a.i5.i16.i, align 8, !alias.scope !229
  %value.i114.i45.i = tail call i64 @llvm.umin.i64(i64 %value.i94.i41.i, i64 %69)
  %72 = tail call i64 @llvm.umax.i64(i64 %value.i94.i41.i, i64 %69)
  store i64 %72, ptr %v_b.i.i7.i, align 8, !alias.scope !229
  store i64 %value.i114.i45.i, ptr %v_a.i10.i21.i, align 8, !alias.scope !229
  %value.i119.i46.i = tail call i64 @llvm.umin.i64(i64 %value.i99.i42.i, i64 %70)
  %73 = tail call i64 @llvm.umax.i64(i64 %value.i99.i42.i, i64 %70)
  store i64 %73, ptr %v_b.i6.i17.i, align 8, !alias.scope !229
  store i64 %value.i119.i46.i, ptr %v_b.i31.i.i, align 8, !alias.scope !229
  br label %bb16.i

bb16.i:                                           ; preds = %bb12.i, %bb9.i, %bb11.i
  %presorted_len.sroa.0.0.i = phi i64 [ 13, %bb9.i ], [ 9, %bb12.i ], [ 1, %bb11.i ]
  %_6.i.i = icmp samesign ugt i64 %presorted_len.sroa.0.0.i, %region.sroa.8.0.i
  br i1 %_6.i.i, label %bb2.i.i, label %bb3.i.i

bb2.i.i:                                          ; preds = %bb16.i
  tail call void @llvm.trap()
  unreachable

bb3.i.i:                                          ; preds = %bb16.i
  %v_end.i.i = getelementptr inbounds nuw i64, ptr %region.sroa.0.0.i, i64 %region.sroa.8.0.i
  %_11.not1.i.i = icmp samesign eq i64 %presorted_len.sroa.0.0.i, %region.sroa.8.0.i
  br i1 %_11.not1.i.i, label %_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E.exit.i, label %bb5.preheader.i.i

bb5.preheader.i.i:                                ; preds = %bb3.i.i
  %74 = getelementptr inbounds nuw i64, ptr %region.sroa.0.0.i, i64 %presorted_len.sroa.0.0.i
  br label %bb5.i.i

bb5.i.i:                                          ; preds = %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit.i.i, %bb5.preheader.i.i
  %tail.sroa.0.02.i.i = phi ptr [ %_15.i.i, %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit.i.i ], [ %74, %bb5.preheader.i.i ]
  %75 = getelementptr inbounds i8, ptr %tail.sroa.0.02.i.i, i64 -8
  %tail.val.i.i.i = load i64, ptr %tail.sroa.0.02.i.i, align 8, !alias.scope !232, !noundef !10
  %.val1.i.i.i = load i64, ptr %75, align 8, !alias.scope !232, !noundef !10
  %_0.i.i.i.i = icmp ult i64 %.val1.i.i.i, %tail.val.i.i.i
  br i1 %_0.i.i.i.i, label %bb4.i.i.i, label %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit.i.i

bb4.i.i.i:                                        ; preds = %bb5.i.i, %bb7.i.i.i
  %76 = phi i64 [ %.val.i.i.i, %bb7.i.i.i ], [ %.val1.i.i.i, %bb5.i.i ]
  %gap_guard.sroa.5.0.i.i.i = phi ptr [ %sift.sroa.0.0.i.i.i, %bb7.i.i.i ], [ %tail.sroa.0.02.i.i, %bb5.i.i ]
  %sift.sroa.0.0.i.i.i = phi ptr [ %77, %bb7.i.i.i ], [ %75, %bb5.i.i ]
  store i64 %76, ptr %gap_guard.sroa.5.0.i.i.i, align 8, !alias.scope !232
  %_18.i.i.i = icmp eq ptr %sift.sroa.0.0.i.i.i, %region.sroa.0.0.i
  br i1 %_18.i.i.i, label %bb10.i.i.i, label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %bb4.i.i.i
  %77 = getelementptr inbounds i8, ptr %sift.sroa.0.0.i.i.i, i64 -8
  %.val.i.i.i = load i64, ptr %77, align 8, !alias.scope !232, !noundef !10
  %_0.i2.i.i.i = icmp ult i64 %.val.i.i.i, %tail.val.i.i.i
  br i1 %_0.i2.i.i.i, label %bb4.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %bb7.i.i.i, %bb4.i.i.i
  %sift.sroa.0.0.i.lcssa.i.i = phi ptr [ %sift.sroa.0.0.i.i.i, %bb7.i.i.i ], [ %region.sroa.0.0.i, %bb4.i.i.i ]
  store i64 %tail.val.i.i.i, ptr %sift.sroa.0.0.i.lcssa.i.i, align 8, !alias.scope !232, !noalias !235
  br label %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit.i.i

_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit.i.i: ; preds = %bb10.i.i.i, %bb5.i.i
  %_15.i.i = getelementptr inbounds nuw i8, ptr %tail.sroa.0.02.i.i, i64 8
  %_11.not.i.i = icmp eq ptr %_15.i.i, %v_end.i.i
  br i1 %_11.not.i.i, label %_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E.exit.i, label %bb5.i.i

_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E.exit.i: ; preds = %_ZN4core5slice4sort6shared9smallsort11insert_tail17h341e9ac0413ebac4E.exit.i.i, %bb3.i.i
  br i1 %no_merge.i, label %bb23.sink.split.i, label %bb19.i

bb19.i:                                           ; preds = %_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E.exit.i
  %_26.not.i = icmp eq ptr %region.sroa.0.0.i, %v.sroa.0.0.lcssa
  br i1 %_26.not.i, label %bb8.i, label %bb20.i

bb20.i:                                           ; preds = %bb19.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !240)
  %count1.i.i = add nsw i64 %v.sroa.15.0.lcssa, -1
  %78 = getelementptr inbounds nuw i64, ptr %stack_array.i, i64 %count1.i.i
  %79 = getelementptr inbounds nuw i64, ptr %v.sroa.0.0.lcssa, i64 %count1.i.i
  %80 = getelementptr i8, ptr %data.i, i64 -8
  br label %bb15.i.i

bb16.i.i:                                         ; preds = %bb15.i.i
  %81 = getelementptr i8, ptr %86, i64 8
  %82 = getelementptr i8, ptr %85, i64 8
  %_44.i.i = and i64 %v.sroa.15.0.lcssa, 1
  %_22.i.i = icmp eq i64 %_44.i.i, 0
  br i1 %_22.i.i, label %bb9.i.i, label %bb5.i47.i

bb15.i.i:                                         ; preds = %bb15.i.i, %bb20.i
  %dst.sroa.0.012.i.i = phi ptr [ %_16.i.i.i, %bb15.i.i ], [ %stack_array.i, %bb20.i ]
  %iter.sroa.0.011.i.i = phi i64 [ %_39.i.i, %bb15.i.i ], [ 0, %bb20.i ]
  %left.sroa.0.010.i.i = phi ptr [ %_14.i.i.i, %bb15.i.i ], [ %v.sroa.0.0.lcssa, %bb20.i ]
  %right.sroa.0.09.i.i = phi ptr [ %_12.i.i.i, %bb15.i.i ], [ %data.i, %bb20.i ]
  %left_rev.sroa.0.08.i.i = phi ptr [ %86, %bb15.i.i ], [ %80, %bb20.i ]
  %right_rev.sroa.0.07.i.i = phi ptr [ %85, %bb15.i.i ], [ %79, %bb20.i ]
  %dst_rev.sroa.0.06.i.i = phi ptr [ %87, %bb15.i.i ], [ %78, %bb20.i ]
  %_39.i.i = add nuw nsw i64 %iter.sroa.0.011.i.i, 1
  %right.sroa.0.0.val.i.i = load i64, ptr %right.sroa.0.09.i.i, align 8, !alias.scope !243, !noundef !10
  %left.sroa.0.0.val.i.i = load i64, ptr %left.sroa.0.010.i.i, align 8, !alias.scope !243, !noundef !10
  %_0.i.i.i = icmp ult i64 %left.sroa.0.0.val.i.i, %right.sroa.0.0.val.i.i
  %is_l.i18.i.i = xor i1 %_0.i.i.i, true
  %83 = tail call i64 @llvm.umax.i64(i64 %left.sroa.0.0.val.i.i, i64 %right.sroa.0.0.val.i.i)
  store i64 %83, ptr %dst.sroa.0.012.i.i, align 8, !noalias !244
  %count.i.i.i = zext i1 %_0.i.i.i to i64
  %_12.i.i.i = getelementptr inbounds nuw i64, ptr %right.sroa.0.09.i.i, i64 %count.i.i.i
  %count2.i.i.i = zext i1 %is_l.i18.i.i to i64
  %_14.i.i.i = getelementptr inbounds nuw i64, ptr %left.sroa.0.010.i.i, i64 %count2.i.i.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %dst.sroa.0.012.i.i, i64 8
  %right_rev.sroa.0.0.val.i.i = load i64, ptr %right_rev.sroa.0.07.i.i, align 8, !alias.scope !243, !noundef !10
  %left_rev.sroa.0.0.val.i.i = load i64, ptr %left_rev.sroa.0.08.i.i, align 8, !alias.scope !243, !noundef !10
  %_0.i19.i.i = icmp ult i64 %left_rev.sroa.0.0.val.i.i, %right_rev.sroa.0.0.val.i.i
  %is_l.i.i.i = xor i1 %_0.i19.i.i, true
  %84 = tail call i64 @llvm.umin.i64(i64 %left_rev.sroa.0.0.val.i.i, i64 %right_rev.sroa.0.0.val.i.i)
  store i64 %84, ptr %dst_rev.sroa.0.06.i.i, align 8, !noalias !248
  %count.neg.i.i.i = sext i1 %is_l.i.i.i to i64
  %85 = getelementptr i64, ptr %right_rev.sroa.0.07.i.i, i64 %count.neg.i.i.i
  %count3.neg.i.i.i = sext i1 %_0.i19.i.i to i64
  %86 = getelementptr i64, ptr %left_rev.sroa.0.08.i.i, i64 %count3.neg.i.i.i
  %87 = getelementptr inbounds i8, ptr %dst_rev.sroa.0.06.i.i, i64 -8
  %exitcond.not.i.i = icmp eq i64 %_39.i.i, %len_div_26.i
  br i1 %exitcond.not.i.i, label %bb16.i.i, label %bb15.i.i

bb5.i47.i:                                        ; preds = %bb16.i.i
  %left_nonempty.i.i = icmp ult ptr %_14.i.i.i, %81
  %left.sroa.0.0.right.sroa.0.0.i.i = select i1 %left_nonempty.i.i, ptr %_14.i.i.i, ptr %_12.i.i.i
  %88 = load i64, ptr %left.sroa.0.0.right.sroa.0.0.i.i, align 8, !alias.scope !243
  store i64 %88, ptr %_16.i.i.i, align 8, !noalias !243
  %count2.i.i = zext i1 %left_nonempty.i.i to i64
  %_26.i.i = getelementptr inbounds nuw i64, ptr %_14.i.i.i, i64 %count2.i.i
  %_30.i.i = xor i1 %left_nonempty.i.i, true
  %count3.i.i = zext i1 %_30.i.i to i64
  %_28.i.i = getelementptr inbounds nuw i64, ptr %_12.i.i.i, i64 %count3.i.i
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb5.i47.i, %bb16.i.i
  %right.sroa.0.1.i.i = phi ptr [ %_12.i.i.i, %bb16.i.i ], [ %_28.i.i, %bb5.i47.i ]
  %left.sroa.0.1.i.i = phi ptr [ %_14.i.i.i, %bb16.i.i ], [ %_26.i.i, %bb5.i47.i ]
  %_31.i.i = icmp ne ptr %left.sroa.0.1.i.i, %81
  %_32.i.i = icmp ne ptr %right.sroa.0.1.i.i, %82
  %or.cond.i.i = select i1 %_31.i.i, i1 true, i1 %_32.i.i
  br i1 %or.cond.i.i, label %bb13.i.i, label %_ZN4core5slice4sort6shared9smallsort19bidirectional_merge17hd46cedbb47ad2770E.exit.i, !prof !252

bb13.i.i:                                         ; preds = %bb9.i.i
; call core::slice::sort::shared::smallsort::panic_on_ord_violation
  tail call void @_ZN4core5slice4sort6shared9smallsort22panic_on_ord_violation17ha8ac69acadf1c3c7E() #31, !noalias !240
  unreachable

_ZN4core5slice4sort6shared9smallsort19bidirectional_merge17hd46cedbb47ad2770E.exit.i: ; preds = %bb9.i.i
  %89 = shl nuw nsw i64 %v.sroa.15.0.lcssa, 3
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %v.sroa.0.0.lcssa, ptr nonnull align 8 %stack_array.i, i64 %89, i1 false)
  br label %bb23.sink.split.i

bb23.sink.split.i:                                ; preds = %_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E.exit.i, %_ZN4core5slice4sort6shared9smallsort19bidirectional_merge17hd46cedbb47ad2770E.exit.i
  call void @llvm.lifetime.end.p0(i64 256, ptr nonnull %stack_array.i), !noalias !223
  br label %bb22

bb7:                                              ; preds = %bb5
  %90 = add nsw i32 %limit.sroa.0.090, -1
  %len_div_84.i = lshr i64 %v.sroa.15.092, 3
  %b.idx.i = shl nuw nsw i64 %len_div_84.i, 5
  %b.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 %b.idx.i
  %c.idx.i = mul nuw nsw i64 %len_div_84.i, 56
  %c.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 %c.idx.i
  %_12.i = icmp samesign ult i64 %v.sroa.15.092, 64
  br i1 %_12.i, label %bb3.i, label %bb5.i

bb5.i:                                            ; preds = %bb7
; call core::slice::sort::shared::pivot::median3_rec
  %self.i = tail call fastcc noundef ptr @_ZN4core5slice4sort6shared5pivot11median3_rec17h1bedfef3af1cf54cE(ptr noundef nonnull readonly align 8 %v.sroa.0.093, ptr noundef readonly %b.i, ptr noundef readonly %c.i, i64 noundef %len_div_84.i)
  br label %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit

bb3.i:                                            ; preds = %bb7
  %v.0.val6.i = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !253, !noundef !10
  %b.val7.i = load i64, ptr %b.i, align 8, !alias.scope !253, !noundef !10
  %_0.i.i = icmp ult i64 %b.val7.i, %v.0.val6.i
  %c.val5.i = load i64, ptr %c.i, align 8, !alias.scope !253, !noundef !10
  %_0.i8.i = icmp ult i64 %c.val5.i, %v.0.val6.i
  %91 = xor i1 %_0.i.i, %_0.i8.i
  %_0.i9.i = icmp ult i64 %c.val5.i, %b.val7.i
  %_12.i.i = xor i1 %_0.i.i, %_0.i9.i
  %c.b.i.i = select i1 %_12.i.i, ptr %c.i, ptr %b.i
  %_0.sroa.0.0.i.i = select i1 %91, ptr %v.sroa.0.093, ptr %c.b.i.i
  br label %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit

_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit: ; preds = %bb5.i, %bb3.i
  %_0.sroa.0.0.i.sink.i = phi ptr [ %_0.sroa.0.0.i.i, %bb3.i ], [ %self.i, %bb5.i ]
  %92 = ptrtoint ptr %_0.sroa.0.0.i.sink.i to i64
  %93 = ptrtoint ptr %v.sroa.0.093 to i64
  %94 = sub nuw i64 %92, %93
  %index.sroa.0.0.i = lshr exact i64 %94, 3
  %cond.i = icmp samesign ult i64 %index.sroa.0.0.i, %v.sroa.15.092
  tail call void @llvm.assume(i1 %cond.i)
  %.not = icmp eq ptr %ancestor_pivot.sroa.0.091, null
  br i1 %.not, label %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit.bb7.i_crit_edge, label %bb11

_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit.bb7.i_crit_edge: ; preds = %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit
  %tmp.sroa.0.0.copyload.i.i.i.pre = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !256
  %_10.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 %94
  %.pre = load i64, ptr %_10.i.i.phi.trans.insert, align 8, !alias.scope !256
  br label %bb7.i

bb6:                                              ; preds = %bb5
; call core::slice::sort::unstable::heapsort::heapsort
  tail call void @_ZN4core5slice4sort8unstable8heapsort8heapsort17h636bf08246baac0aE(ptr noalias noundef nonnull align 8 %v.sroa.0.093, i64 noundef %v.sroa.15.092, ptr noalias nonnull align 8 poison) #35
  br label %bb22

bb7.i:                                            ; preds = %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit.bb7.i_crit_edge, %bb11
  %95 = phi i64 [ %.pre, %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit.bb7.i_crit_edge ], [ %_19.val, %bb11 ]
  %tmp.sroa.0.0.copyload.i.i.i = phi i64 [ %tmp.sroa.0.0.copyload.i.i.i.pre, %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit.bb7.i_crit_edge ], [ %tmp.sroa.0.0.copyload.i.i.i.pre106, %bb11 ]
  %_10.i.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 %94
  store i64 %95, ptr %v.sroa.0.093, align 8, !alias.scope !256
  store i64 %tmp.sroa.0.0.copyload.i.i.i, ptr %_10.i.i, align 8, !alias.scope !256
  %96 = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !261)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !264)
  %value.i.i = load i64, ptr %96, align 8, !alias.scope !266, !noalias !264, !noundef !10
  %97 = getelementptr i64, ptr %v.sroa.0.093, i64 %v.sroa.15.092
  %unroll_end.i.i = getelementptr i8, ptr %97, i64 -8
  %state.sroa.13.033.i.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 16
  %_2134.i.i = icmp ult ptr %state.sroa.13.033.i.i, %unroll_end.i.i
  %_21.val.i.pre.i.i = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !267, !noalias !268
  br i1 %_2134.i.i, label %bb6.i.i, label %bb16.preheader.i.i

bb16.preheader.i.i:                               ; preds = %bb6.i.i, %bb7.i
  %state.sroa.0.0.lcssa.i.i = phi ptr [ %96, %bb7.i ], [ %_17.i17.i.i, %bb6.i.i ]
  %state.sroa.23.0.lcssa.i.i = phi i64 [ 0, %bb7.i ], [ %103, %bb6.i.i ]
  %state.sroa.13.0.lcssa.i.i = phi ptr [ %state.sroa.13.033.i.i, %bb7.i ], [ %state.sroa.13.0.i.i, %bb6.i.i ]
  %is_done.i4.i = icmp eq ptr %state.sroa.13.0.lcssa.i.i, %97
  br i1 %is_done.i4.i, label %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE.exit.i, label %bb16.cont.i.i

bb16.cont.i.i:                                    ; preds = %bb16.preheader.i.i, %bb16.cont.i.i
  %state.sroa.23.1.i7.i = phi i64 [ %99, %bb16.cont.i.i ], [ %state.sroa.23.0.lcssa.i.i, %bb16.preheader.i.i ]
  %state.sroa.13.1.i6.i = phi ptr [ %state.sroa.13.1.sroa.gep.i.i, %bb16.cont.i.i ], [ %state.sroa.13.0.lcssa.i.i, %bb16.preheader.i.i ]
  %state.sroa.0.1.i5.i = phi ptr [ %state.sroa.13.1.i6.i, %bb16.cont.i.i ], [ %state.sroa.0.0.lcssa.i.i, %bb16.preheader.i.i ]
  %_20.val.i.else.val.i.i = load i64, ptr %state.sroa.13.1.i6.i, align 8, !alias.scope !266, !noalias !271, !noundef !10
  %_0.i.i.i.i22 = icmp ult i64 %_21.val.i.pre.i.i, %_20.val.i.else.val.i.i
  %left.i.i.i = getelementptr inbounds nuw i64, ptr %96, i64 %state.sroa.23.1.i7.i
  %98 = load i64, ptr %left.i.i.i, align 8, !alias.scope !266, !noalias !271
  store i64 %98, ptr %state.sroa.0.1.i5.i, align 8, !alias.scope !266, !noalias !271
  store i64 %_20.val.i.else.val.i.i, ptr %left.i.i.i, align 8, !alias.scope !266, !noalias !271
  %_16.i.i.i23 = zext i1 %_0.i.i.i.i22 to i64
  %99 = add i64 %state.sroa.23.1.i7.i, %_16.i.i.i23
  %state.sroa.13.1.sroa.gep.i.i = getelementptr inbounds nuw i8, ptr %state.sroa.13.1.i6.i, i64 8
  %is_done.i.i = icmp eq ptr %state.sroa.13.1.sroa.gep.i.i, %97
  br i1 %is_done.i.i, label %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE.exit.i, label %bb16.cont.i.i

bb6.i.i:                                          ; preds = %bb7.i, %bb6.i.i
  %state.sroa.13.037.i.i = phi ptr [ %state.sroa.13.0.i.i, %bb6.i.i ], [ %state.sroa.13.033.i.i, %bb7.i ]
  %state.sroa.23.036.i.i = phi i64 [ %103, %bb6.i.i ], [ 0, %bb7.i ]
  %state.sroa.0.035.i.i = phi ptr [ %_17.i17.i.i, %bb6.i.i ], [ %96, %bb7.i ]
  %_20.val.i9.i.i = load i64, ptr %state.sroa.13.037.i.i, align 8, !alias.scope !266, !noalias !272, !noundef !10
  %_0.i.i11.i.i = icmp ult i64 %_21.val.i.pre.i.i, %_20.val.i9.i.i
  %left.i14.i.i = getelementptr inbounds nuw i64, ptr %96, i64 %state.sroa.23.036.i.i
  %100 = load i64, ptr %left.i14.i.i, align 8, !alias.scope !266, !noalias !272
  store i64 %100, ptr %state.sroa.0.035.i.i, align 8, !alias.scope !266, !noalias !272
  store i64 %_20.val.i9.i.i, ptr %left.i14.i.i, align 8, !alias.scope !266, !noalias !272
  %_16.i16.i.i = zext i1 %_0.i.i11.i.i to i64
  %101 = add i64 %state.sroa.23.036.i.i, %_16.i16.i.i
  %_17.i17.i.i = getelementptr inbounds nuw i8, ptr %state.sroa.0.035.i.i, i64 16
  %_20.val.i19.i.i = load i64, ptr %_17.i17.i.i, align 8, !alias.scope !266, !noalias !275, !noundef !10
  %_0.i.i21.i.i = icmp ult i64 %_21.val.i.pre.i.i, %_20.val.i19.i.i
  %left.i24.i.i = getelementptr inbounds nuw i64, ptr %96, i64 %101
  %102 = load i64, ptr %left.i24.i.i, align 8, !alias.scope !266, !noalias !275
  store i64 %102, ptr %state.sroa.13.037.i.i, align 8, !alias.scope !266, !noalias !275
  store i64 %_20.val.i19.i.i, ptr %left.i24.i.i, align 8, !alias.scope !266, !noalias !275
  %_16.i26.i.i = zext i1 %_0.i.i21.i.i to i64
  %103 = add i64 %101, %_16.i26.i.i
  %state.sroa.13.0.i.i = getelementptr inbounds nuw i8, ptr %state.sroa.0.035.i.i, i64 24
  %_21.i.i = icmp ult ptr %state.sroa.13.0.i.i, %unroll_end.i.i
  br i1 %_21.i.i, label %bb6.i.i, label %bb16.preheader.i.i

_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE.exit.i: ; preds = %bb16.cont.i.i, %bb16.preheader.i.i
  %state.sroa.0.1.i.lcssa.i = phi ptr [ %state.sroa.0.0.lcssa.i.i, %bb16.preheader.i.i ], [ %state.sroa.13.1.i6.i, %bb16.cont.i.i ]
  %state.sroa.23.1.i.lcssa.i = phi i64 [ %state.sroa.23.0.lcssa.i.i, %bb16.preheader.i.i ], [ %99, %bb16.cont.i.i ]
  %_0.i.i46.i.i = icmp ult i64 %_21.val.i.pre.i.i, %value.i.i
  %left.i47.i.i = getelementptr inbounds nuw i64, ptr %96, i64 %state.sroa.23.1.i.lcssa.i
  %104 = load i64, ptr %left.i47.i.i, align 8, !alias.scope !266, !noalias !271
  store i64 %104, ptr %state.sroa.0.1.i.lcssa.i, align 8, !alias.scope !266, !noalias !271
  store i64 %value.i.i, ptr %left.i47.i.i, align 8, !alias.scope !266, !noalias !271
  %_16.i52.i.i = zext i1 %_0.i.i46.i.i to i64
  %105 = add i64 %state.sroa.23.1.i.lcssa.i, %_16.i52.i.i
  %_16.not.i = icmp ult i64 %105, %v.sroa.15.092
  br i1 %_16.not.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$12split_at_mut17hbce4c383f5e73d7fE.exit", label %bb9.i24

bb9.i24:                                          ; preds = %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE.exit.i
  tail call void @llvm.trap()
  unreachable

"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$12split_at_mut17hbce4c383f5e73d7fE.exit": ; preds = %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE.exit.i
  %_10.i5.i = getelementptr inbounds nuw i64, ptr %v.sroa.0.093, i64 %105
  %tmp.sroa.0.0.copyload.i.i6.i = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !278
  %106 = load i64, ptr %_10.i5.i, align 8, !alias.scope !278
  store i64 %106, ptr %v.sroa.0.093, align 8, !alias.scope !278
  store i64 %tmp.sroa.0.0.copyload.i.i6.i, ptr %_10.i5.i, align 8, !alias.scope !278
  %107 = getelementptr inbounds nuw i8, ptr %_10.i5.i, i64 8
  %108 = xor i64 %105, -1
  %109 = add i64 %v.sroa.15.092, %108
; call core::slice::sort::unstable::quicksort::quicksort
  tail call fastcc void @_ZN4core5slice4sort8unstable9quicksort9quicksort17h1e72079e011d70e5E(ptr noalias noundef nonnull align 8 %v.sroa.0.093, i64 noundef %105, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(8) %ancestor_pivot.sroa.0.091, i32 noundef %90, ptr noalias noundef align 8 dereferenceable(8) %is_less)
  br label %bb1.backedge

bb11:                                             ; preds = %_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E.exit
  %_19 = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 %94
  %ancestor_pivot.sroa.0.0.val = load i64, ptr %ancestor_pivot.sroa.0.091, align 8, !noundef !10
  %_19.val = load i64, ptr %_19, align 8, !noundef !10
  %_0.i = icmp ult i64 %_19.val, %ancestor_pivot.sroa.0.0.val
  %tmp.sroa.0.0.copyload.i.i.i.pre106 = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !256
  br i1 %_0.i, label %bb7.i, label %bb7.i35

bb7.i35:                                          ; preds = %bb11
  store i64 %_19.val, ptr %v.sroa.0.093, align 8, !alias.scope !281
  store i64 %tmp.sroa.0.0.copyload.i.i.i.pre106, ptr %_19, align 8, !alias.scope !281
  %110 = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !286)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !289)
  %value.i.i38 = load i64, ptr %110, align 8, !alias.scope !291, !noalias !289, !noundef !10
  %111 = getelementptr i64, ptr %v.sroa.0.093, i64 %v.sroa.15.092
  %unroll_end.i.i39 = getelementptr i8, ptr %111, i64 -8
  %state.sroa.13.033.i.i40 = getelementptr inbounds nuw i8, ptr %v.sroa.0.093, i64 16
  %_2134.i.i41 = icmp ult ptr %state.sroa.13.033.i.i40, %unroll_end.i.i39
  %_21.val.i.pre.i.i42 = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !292, !noalias !293
  br i1 %_2134.i.i41, label %bb6.i.i65, label %bb16.preheader.i.i43

bb16.preheader.i.i43:                             ; preds = %bb6.i.i65, %bb7.i35
  %state.sroa.0.0.lcssa.i.i44 = phi ptr [ %110, %bb7.i35 ], [ %_17.i17.i.i72, %bb6.i.i65 ]
  %state.sroa.23.0.lcssa.i.i45 = phi i64 [ 0, %bb7.i35 ], [ %117, %bb6.i.i65 ]
  %state.sroa.13.0.lcssa.i.i46 = phi ptr [ %state.sroa.13.033.i.i40, %bb7.i35 ], [ %state.sroa.13.0.i.i76, %bb6.i.i65 ]
  %is_done.i4.i47 = icmp eq ptr %state.sroa.13.0.lcssa.i.i46, %111
  br i1 %is_done.i4.i47, label %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE.exit.i, label %bb16.cont.i.i48

bb16.cont.i.i48:                                  ; preds = %bb16.preheader.i.i43, %bb16.cont.i.i48
  %state.sroa.23.1.i7.i49 = phi i64 [ %113, %bb16.cont.i.i48 ], [ %state.sroa.23.0.lcssa.i.i45, %bb16.preheader.i.i43 ]
  %state.sroa.13.1.i6.i50 = phi ptr [ %state.sroa.13.1.sroa.gep.i.i55, %bb16.cont.i.i48 ], [ %state.sroa.13.0.lcssa.i.i46, %bb16.preheader.i.i43 ]
  %state.sroa.0.1.i5.i51 = phi ptr [ %state.sroa.13.1.i6.i50, %bb16.cont.i.i48 ], [ %state.sroa.0.0.lcssa.i.i44, %bb16.preheader.i.i43 ]
  %_20.val.i.else.val.i.i52 = load i64, ptr %state.sroa.13.1.i6.i50, align 8, !alias.scope !291, !noalias !296, !noundef !10
  %_0.i.i.i.i.i = icmp uge i64 %_20.val.i.else.val.i.i52, %_21.val.i.pre.i.i42
  %left.i.i.i53 = getelementptr inbounds nuw i64, ptr %110, i64 %state.sroa.23.1.i7.i49
  %112 = load i64, ptr %left.i.i.i53, align 8, !alias.scope !291, !noalias !296
  store i64 %112, ptr %state.sroa.0.1.i5.i51, align 8, !alias.scope !291, !noalias !296
  store i64 %_20.val.i.else.val.i.i52, ptr %left.i.i.i53, align 8, !alias.scope !291, !noalias !296
  %_16.i.i.i54 = zext i1 %_0.i.i.i.i.i to i64
  %113 = add i64 %state.sroa.23.1.i7.i49, %_16.i.i.i54
  %state.sroa.13.1.sroa.gep.i.i55 = getelementptr inbounds nuw i8, ptr %state.sroa.13.1.i6.i50, i64 8
  %is_done.i.i56 = icmp eq ptr %state.sroa.13.1.sroa.gep.i.i55, %111
  br i1 %is_done.i.i56, label %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE.exit.i, label %bb16.cont.i.i48

bb6.i.i65:                                        ; preds = %bb7.i35, %bb6.i.i65
  %state.sroa.13.037.i.i66 = phi ptr [ %state.sroa.13.0.i.i76, %bb6.i.i65 ], [ %state.sroa.13.033.i.i40, %bb7.i35 ]
  %state.sroa.23.036.i.i67 = phi i64 [ %117, %bb6.i.i65 ], [ 0, %bb7.i35 ]
  %state.sroa.0.035.i.i68 = phi ptr [ %_17.i17.i.i72, %bb6.i.i65 ], [ %110, %bb7.i35 ]
  %_20.val.i9.i.i69 = load i64, ptr %state.sroa.13.037.i.i66, align 8, !alias.scope !291, !noalias !297, !noundef !10
  %_0.i.i.i11.i.i = icmp uge i64 %_20.val.i9.i.i69, %_21.val.i.pre.i.i42
  %left.i14.i.i70 = getelementptr inbounds nuw i64, ptr %110, i64 %state.sroa.23.036.i.i67
  %114 = load i64, ptr %left.i14.i.i70, align 8, !alias.scope !291, !noalias !297
  store i64 %114, ptr %state.sroa.0.035.i.i68, align 8, !alias.scope !291, !noalias !297
  store i64 %_20.val.i9.i.i69, ptr %left.i14.i.i70, align 8, !alias.scope !291, !noalias !297
  %_16.i16.i.i71 = zext i1 %_0.i.i.i11.i.i to i64
  %115 = add i64 %state.sroa.23.036.i.i67, %_16.i16.i.i71
  %_17.i17.i.i72 = getelementptr inbounds nuw i8, ptr %state.sroa.0.035.i.i68, i64 16
  %_20.val.i19.i.i73 = load i64, ptr %_17.i17.i.i72, align 8, !alias.scope !291, !noalias !300, !noundef !10
  %_0.i.i.i21.i.i = icmp uge i64 %_20.val.i19.i.i73, %_21.val.i.pre.i.i42
  %left.i24.i.i74 = getelementptr inbounds nuw i64, ptr %110, i64 %115
  %116 = load i64, ptr %left.i24.i.i74, align 8, !alias.scope !291, !noalias !300
  store i64 %116, ptr %state.sroa.13.037.i.i66, align 8, !alias.scope !291, !noalias !300
  store i64 %_20.val.i19.i.i73, ptr %left.i24.i.i74, align 8, !alias.scope !291, !noalias !300
  %_16.i26.i.i75 = zext i1 %_0.i.i.i21.i.i to i64
  %117 = add i64 %115, %_16.i26.i.i75
  %state.sroa.13.0.i.i76 = getelementptr inbounds nuw i8, ptr %state.sroa.0.035.i.i68, i64 24
  %_21.i.i77 = icmp ult ptr %state.sroa.13.0.i.i76, %unroll_end.i.i39
  br i1 %_21.i.i77, label %bb6.i.i65, label %bb16.preheader.i.i43

_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE.exit.i: ; preds = %bb16.cont.i.i48, %bb16.preheader.i.i43
  %state.sroa.0.1.i.lcssa.i57 = phi ptr [ %state.sroa.0.0.lcssa.i.i44, %bb16.preheader.i.i43 ], [ %state.sroa.13.1.i6.i50, %bb16.cont.i.i48 ]
  %state.sroa.23.1.i.lcssa.i58 = phi i64 [ %state.sroa.23.0.lcssa.i.i45, %bb16.preheader.i.i43 ], [ %113, %bb16.cont.i.i48 ]
  %_0.i.i.i46.i.i = icmp uge i64 %value.i.i38, %_21.val.i.pre.i.i42
  %left.i47.i.i59 = getelementptr inbounds nuw i64, ptr %110, i64 %state.sroa.23.1.i.lcssa.i58
  %118 = load i64, ptr %left.i47.i.i59, align 8, !alias.scope !291, !noalias !296
  store i64 %118, ptr %state.sroa.0.1.i.lcssa.i57, align 8, !alias.scope !291, !noalias !296
  store i64 %value.i.i38, ptr %left.i47.i.i59, align 8, !alias.scope !291, !noalias !296
  %_16.i52.i.i60 = zext i1 %_0.i.i.i46.i.i to i64
  %119 = add i64 %state.sroa.23.1.i.lcssa.i58, %_16.i52.i.i60
  %_16.not.i61 = icmp ult i64 %119, %v.sroa.15.092
  br i1 %_16.not.i61, label %bb24, label %bb9.i62

bb9.i62:                                          ; preds = %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE.exit.i
  tail call void @llvm.trap()
  unreachable

bb24:                                             ; preds = %_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE.exit.i
  %_10.i5.i63 = getelementptr inbounds nuw i64, ptr %v.sroa.0.093, i64 %119
  %tmp.sroa.0.0.copyload.i.i6.i64 = load i64, ptr %v.sroa.0.093, align 8, !alias.scope !303
  %120 = load i64, ptr %_10.i5.i63, align 8, !alias.scope !303
  store i64 %120, ptr %v.sroa.0.093, align 8, !alias.scope !303
  store i64 %tmp.sroa.0.0.copyload.i.i6.i64, ptr %_10.i5.i63, align 8, !alias.scope !303
  %index = add nuw nsw i64 %119, 1
  %new_len = sub nuw i64 %v.sroa.15.092, %index
  %_50 = getelementptr inbounds nuw i64, ptr %v.sroa.0.093, i64 %index
  br label %bb1.backedge

bb1.backedge:                                     ; preds = %bb24, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$12split_at_mut17hbce4c383f5e73d7fE.exit"
  %ancestor_pivot.sroa.0.0.be = phi ptr [ %_10.i5.i, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$12split_at_mut17hbce4c383f5e73d7fE.exit" ], [ null, %bb24 ]
  %v.sroa.15.0.be = phi i64 [ %109, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$12split_at_mut17hbce4c383f5e73d7fE.exit" ], [ %new_len, %bb24 ]
  %v.sroa.0.0.be = phi ptr [ %107, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$12split_at_mut17hbce4c383f5e73d7fE.exit" ], [ %_50, %bb24 ]
  %_5 = icmp ult i64 %v.sroa.15.0.be, 33
  br i1 %_5, label %bb3, label %bb5

bb22:                                             ; preds = %bb23.sink.split.i, %bb3, %bb6
  ret void
}

; alloc::raw_vec::RawVecInner<A>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E"(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 2, 25) %elem_layout.1) unnamed_addr #10 {
start:
  %_10.i = add nsw i64 %elem_layout.0, -1
  %_12.i = add nuw nsw i64 %_10.i, %elem_layout.1
  %_14.i = sub nsw i64 0, %elem_layout.0
  %new_size.i = and i64 %_12.i, %_14.i
  %0 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %new_size.i, i64 %cap)
  %_21.0.i = extractvalue { i64, i1 } %0, 0
  %_21.1.i = extractvalue { i64, i1 } %0, 1
  %_27.i = sub nuw i64 -9223372036854775808, %elem_layout.0
  %_26.i = icmp ugt i64 %_21.0.i, %_27.i
  %or.cond.i = select i1 %_21.1.i, i1 true, i1 %_26.i
  br i1 %or.cond.i, label %bb8, label %bb11, !prof !306

bb11:                                             ; preds = %start
  %_6.i = icmp eq i64 %self.0.val, 0
  br i1 %_6.i, label %bb5, label %bb3

bb3:                                              ; preds = %bb11
  %1 = mul nuw i64 %elem_layout.1, %self.0.val
  %2 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %2)
  %cond.i.i = icmp uge i64 %_21.0.i, %1
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef ptr @_RNvCshXwFllX56pT_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %_21.0.i) #33
  br label %bb7

bb5:                                              ; preds = %bb11
  %3 = icmp eq i64 %_21.0.i, 0
  br i1 %3, label %bb7.thread, label %bb4.i.i16

bb7.thread:                                       ; preds = %bb5
  %_17.i.i = inttoptr i64 %elem_layout.0 to ptr
  br label %bb16

bb4.i.i16:                                        ; preds = %bb5
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef %_21.0.i, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0) #33
  br label %bb7

bb7:                                              ; preds = %bb4.i.i16, %bb3
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3 ], [ %4, %bb4.i.i16 ]
  %5 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %5, label %bb15, label %bb16

bb15:                                             ; preds = %bb7
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %elem_layout.0, ptr %6, align 8
  br label %bb8

bb16:                                             ; preds = %bb7.thread, %bb7
  %raw_ptr.i.i.pn10 = phi ptr [ %_17.i.i, %bb7.thread ], [ %raw_ptr.i.i.pn, %bb7 ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn10, ptr %7, align 8
  br label %bb8

bb8:                                              ; preds = %start, %bb16, %bb15
  %.sink12 = phi i64 [ 16, %bb16 ], [ 16, %bb15 ], [ 8, %start ]
  %_21.0.i.sink = phi i64 [ %_21.0.i, %bb16 ], [ %_21.0.i, %bb15 ], [ 0, %start ]
  %.sink = phi i64 [ 0, %bb16 ], [ 1, %bb15 ], [ 1, %start ]
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink12
  store i64 %_21.0.i.sink, ptr %8, align 8
  store i64 %.sink, ptr %_0, align 8
  ret void
}

; alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
; Function Attrs: cold uwtable
define internal fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E"(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef range(i64 1, 0) %additional, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 2, 25) %elem_layout.1) unnamed_addr #11 personality ptr @__CxxFrameHandler3 {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !307)
  %_26.0.i = add i64 %additional, %len
  %_26.1.i = icmp ult i64 %_26.0.i, %len
  br i1 %_26.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !78, !alias.scope !307, !noundef !10
  %v16.i = shl nuw i64 %self5.i, 1
  %..i.i = tail call noundef i64 @llvm.umax.i64(i64 %_26.0.i, i64 %v16.i)
  %..i16.i = tail call noundef i64 @llvm.umax.i64(i64 %..i.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !307
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !307
; call alloc::raw_vec::RawVecInner<A>::finish_grow
  call fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$11finish_grow17h6767e0518b1a1520E"(ptr noalias noundef align 8 captures(address) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %..i16.i, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 2, 25) %elem_layout.1), !noalias !307
  %_37.i = load i64, ptr %self3.i, align 8, !range !17, !noalias !307, !noundef !10
  %1 = trunc nuw i64 %_37.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !117, !noalias !307, !noundef !10
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !307
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !307
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #36
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !307, !nonnull !10, !noundef !10
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !307
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !307
  %4 = icmp sgt i64 %..i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %..i16.i, ptr %slf, align 8, !alias.scope !307
  ret void
}

; anyhow::error::object_ref
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_ZN6anyhow5error10object_ref17h0ce20ab8fd8e770aE(ptr noundef nonnull %e) unnamed_addr #4 {
start:
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %0 = insertvalue { ptr, ptr } poison, ptr %_5, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.1, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_ref
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_ZN6anyhow5error10object_ref17h3950046690ebc90bE(ptr noundef nonnull %e) unnamed_addr #4 {
start:
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %0 = insertvalue { ptr, ptr } poison, ptr %_5, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.2, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_drop
; Function Attrs: uwtable
define internal void @_ZN6anyhow5error11object_drop17h11826f6ea70ae0fdE(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !310)
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %bb4.i.i unwind label %funclet_bb3.i.i

funclet_bb3.i.i:                                  ; preds = %start
  %cleanuppad.i.i = cleanuppad within none []
  %1 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_1.val.i.i.i = load i64, ptr %1, align 8, !alias.scope !313
  %_6.i.i.i.i4.i.i.i.i.i = icmp eq i64 %_1.val.i.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i.i, label %"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit.i.i", label %bb2.i.i.i5.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i:                             ; preds = %funclet_bb3.i.i
  %2 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_1.val1.i.i.i = load ptr, ptr %2, align 8, !alias.scope !313, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i, i64 noundef %_1.val.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad.i.i) ], !noalias !310
  br label %"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit.i.i"

"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E.exit.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i, %funclet_bb3.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33 [ "funclet"(token %cleanuppad.i.i) ]
  cleanupret from %cleanuppad.i.i unwind to caller

bb4.i.i:                                          ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %e, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !316)
  %_1.val.i1.i.i = load i64, ptr %3, align 8, !alias.scope !319
  %_6.i.i.i.i4.i.i.i2.i.i = icmp eq i64 %_1.val.i1.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i2.i.i, label %"_ZN4core3ptr136drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$$GT$17h6d9f17498705c1a9E.exit", label %bb2.i.i.i5.i.i.i3.i.i

bb2.i.i.i5.i.i.i3.i.i:                            ; preds = %bb4.i.i
  %4 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_1.val1.i4.i.i = load ptr, ptr %4, align 8, !alias.scope !319, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i4.i.i, i64 noundef %_1.val.i1.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !319
  br label %"_ZN4core3ptr136drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$$GT$17h6d9f17498705c1a9E.exit"

"_ZN4core3ptr136drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$$GT$17h6d9f17498705c1a9E.exit": ; preds = %bb4.i.i, %bb2.i.i.i5.i.i.i3.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33
  ret void
}

; anyhow::error::object_drop
; Function Attrs: uwtable
define internal void @_ZN6anyhow5error11object_drop17h9334d5975c0625abE(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %"_ZN4core3ptr122drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$$GT$17ha1ea932c95026485E.exit" unwind label %funclet_bb4.i

funclet_bb4.i:                                    ; preds = %start
  %cleanuppad.i = cleanuppad within none []
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33 [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

"_ZN4core3ptr122drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$$RF$str$GT$$GT$$GT$$GT$17ha1ea932c95026485E.exit": ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33
  ret void
}

; anyhow::error::no_backtrace
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal noalias noundef align 8 ptr @_ZN6anyhow5error12no_backtrace17h03abfc442485f28cE(ptr nonnull readnone captures(none) %e) unnamed_addr #4 {
start:
  ret ptr null
}

; anyhow::error::object_boxed
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_ZN6anyhow5error12object_boxed17hdd48d83f6f5fca94E(ptr noundef nonnull %e) unnamed_addr #4 {
start:
  %0 = insertvalue { ptr, ptr } poison, ptr %e, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.3, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_boxed
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_ZN6anyhow5error12object_boxed17hde21296211696ba2E(ptr noundef nonnull %e) unnamed_addr #4 {
start:
  %0 = insertvalue { ptr, ptr } poison, ptr %e, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.4, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_downcast
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal noundef ptr @_ZN6anyhow5error15object_downcast17h9f35c6fdf8d2f673E(ptr noundef nonnull readnone captures(ret: address, provenance) %e, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(16) %target) unnamed_addr #12 {
start:
  %_7 = load i128, ptr %target, align 8, !noundef !10
  %_3 = icmp eq i128 %_7, -93652901832424836513689306266955195027
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_0.sroa.0.0 = select i1 %_3, ptr %_5, ptr null
  ret ptr %_0.sroa.0.0
}

; anyhow::error::object_downcast
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal noundef ptr @_ZN6anyhow5error15object_downcast17hefd3fb1cb0d21f49E(ptr noundef nonnull readnone captures(ret: address, provenance) %e, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(16) %target) unnamed_addr #12 {
start:
  %_7 = load i128, ptr %target, align 8, !noundef !10
  %_3 = icmp eq i128 %_7, -156079246856905522541904935584057136664
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_0.sroa.0.0 = select i1 %_3, ptr %_5, ptr null
  ret ptr %_0.sroa.0.0
}

; anyhow::error::object_drop_front
; Function Attrs: uwtable
define internal void @_ZN6anyhow5error17object_drop_front17h468434592611aba3E(ptr noundef nonnull %e, ptr dead_on_return noalias readnone align 8 captures(none) %target) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %"_ZN4core3ptr131drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$core..mem..manually_drop..ManuallyDrop$LT$$RF$str$GT$$GT$$GT$$GT$17he76bb154db5b111aE.exit" unwind label %funclet_bb4.i

funclet_bb4.i:                                    ; preds = %start
  %cleanuppad.i = cleanuppad within none []
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33 [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

"_ZN4core3ptr131drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$core..mem..manually_drop..ManuallyDrop$LT$$RF$str$GT$$GT$$GT$$GT$17he76bb154db5b111aE.exit": ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33
  ret void
}

; anyhow::error::object_drop_front
; Function Attrs: uwtable
define internal void @_ZN6anyhow5error17object_drop_front17hd509813b2a1e9440E(ptr noundef nonnull %e, ptr dead_on_return noalias readnone align 8 captures(none) %target) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %"_ZN4core3ptr145drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$core..mem..manually_drop..ManuallyDrop$LT$alloc..string..String$GT$$GT$$GT$$GT$17h1130f00af3a46401E.exit" unwind label %funclet_bb4.i

funclet_bb4.i:                                    ; preds = %start
  %cleanuppad.i = cleanuppad within none []
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33 [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

"_ZN4core3ptr145drop_in_place$LT$alloc..boxed..Box$LT$anyhow..error..ErrorImpl$LT$core..mem..manually_drop..ManuallyDrop$LT$alloc..string..String$GT$$GT$$GT$$GT$17h1130f00af3a46401E.exit": ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33
  ret void
}

; anyhow::error::object_reallocate_boxed
; Function Attrs: uwtable
define internal { ptr, ptr } @_ZN6anyhow5error23object_reallocate_boxed17h23b914d0daf126d1E(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_3.0 = load ptr, ptr %0, align 8, !nonnull !10, !align !11, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_3.1 = load i64, ptr %1, align 8, !noundef !10
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 dereferenceable_or_null(16) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 16, i64 noundef 8) #33
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb2.i, label %bb1, !prof !320

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E(i64 noundef 8, i64 noundef 16) #36
          to label %.noexc unwind label %funclet_bb6

.noexc:                                           ; preds = %bb2.i
  unreachable

funclet_bb6:                                      ; preds = %bb2.i
  %cleanuppad = cleanuppad within none []
  %4 = getelementptr inbounds nuw i8, ptr %e, i64 8
; call core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  call fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef align 8 dereferenceable(48) %4) #34 [ "funclet"(token %cleanuppad) ]
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

bb1:                                              ; preds = %start
  store ptr %_3.0, ptr %2, align 8, !noalias !321
  %5 = getelementptr inbounds nuw i8, ptr %2, i64 8
  store i64 %_3.1, ptr %5, align 8
  %6 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef align 8 dereferenceable(48) %6)
          to label %bb7 unwind label %funclet_bb3

funclet_bb3:                                      ; preds = %bb1
  %cleanuppad1 = cleanuppad within none []
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33 [ "funclet"(token %cleanuppad1) ]
  cleanupret from %cleanuppad1 unwind to caller

bb7:                                              ; preds = %bb1
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #33
  %7 = insertvalue { ptr, ptr } poison, ptr %2, 0
  %8 = insertvalue { ptr, ptr } %7, ptr @vtable.2, 1
  ret { ptr, ptr } %8
}

; anyhow::error::object_reallocate_boxed
; Function Attrs: uwtable
define internal { ptr, ptr } @_ZN6anyhow5error23object_reallocate_boxed17had37fecdb166d209E(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_3.sroa.0.0.copyload = load i64, ptr %0, align 8
  %_3.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_3.sroa.5.0.copyload = load ptr, ptr %_3.sroa.5.0..sroa_idx, align 8
  %_3.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %e, i64 72
  %_3.sroa.6.0.copyload = load i64, ptr %_3.sroa.6.0..sroa_idx, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !324
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(24) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 24, i64 noundef 8) #33, !noalias !324
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb2.i, label %bb1, !prof !320

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E(i64 noundef 8, i64 noundef 24) #36
          to label %.noexc5 unwind label %funclet_bb2.i

.noexc5:                                          ; preds = %bb2.i
  unreachable

funclet_bb2.i:                                    ; preds = %bb2.i
  %cleanuppad.i = cleanuppad within none []
  %_6.i.i.i.i4.i.i.i = icmp eq i64 %_3.sroa.0.0.copyload, 0
  br i1 %_6.i.i.i.i4.i.i.i, label %.noexc, label %bb2.i.i.i5.i.i.i

bb2.i.i.i5.i.i.i:                                 ; preds = %funclet_bb2.i
  %3 = icmp ne ptr %_3.sroa.5.0.copyload, null
  tail call void @llvm.assume(i1 %3)
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.sroa.5.0.copyload, i64 noundef %_3.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad.i) ]
  br label %.noexc

.noexc:                                           ; preds = %bb2.i.i.i5.i.i.i, %funclet_bb2.i
  %4 = getelementptr inbounds nuw i8, ptr %e, i64 8
; call core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  call fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef align 8 dereferenceable(48) %4) #34 [ "funclet"(token %cleanuppad.i) ]
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33 [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

bb1:                                              ; preds = %start
  store i64 %_3.sroa.0.0.copyload, ptr %1, align 8
  %_3.sroa.5.0..sroa_idx7 = getelementptr inbounds nuw i8, ptr %1, i64 8
  store ptr %_3.sroa.5.0.copyload, ptr %_3.sroa.5.0..sroa_idx7, align 8
  %_3.sroa.6.0..sroa_idx9 = getelementptr inbounds nuw i8, ptr %1, i64 16
  store i64 %_3.sroa.6.0.copyload, ptr %_3.sroa.6.0..sroa_idx9, align 8
  %5 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef align 8 dereferenceable(48) %5)
          to label %bb7 unwind label %funclet_bb3

funclet_bb3:                                      ; preds = %bb1
  %cleanuppad1 = cleanuppad within none []
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33 [ "funclet"(token %cleanuppad1) ]
  cleanupret from %cleanuppad1 unwind to caller

bb7:                                              ; preds = %bb1
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #33
  %6 = insertvalue { ptr, ptr } poison, ptr %1, 0
  %7 = insertvalue { ptr, ptr } %6, ptr @vtable.1, 1
  ret { ptr, ptr } %7
}

; anyhow::error::<impl anyhow::Error>::msg
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E"(i64 noundef range(i64 0, -9223372036854775808) %message.1) unnamed_addr #11 personality ptr @__CxxFrameHandler3 {
start:
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
; call std::backtrace::Backtrace::capture
  call void @_ZN3std9backtrace9Backtrace7capture17hf7fc842bd7b2c58bE(ptr noalias noundef nonnull sret([48 x i8]) align 8 captures(address) dereferenceable(48) %_3)
; call anyhow::error::<impl anyhow::Error>::construct
  %_0.i = call fastcc noalias noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E"(i64 noundef range(i64 0, -9223372036854775808) %message.1, ptr noalias noundef nonnull readonly align 8 captures(address) dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret ptr %_0.i
}

; anyhow::error::<impl anyhow::Error>::msg
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E"(ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %message) unnamed_addr #11 personality ptr @__CxxFrameHandler3 {
start:
  %_4 = alloca [48 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_4)
; invoke std::backtrace::Backtrace::capture
  invoke void @_ZN3std9backtrace9Backtrace7capture17hf7fc842bd7b2c58bE(ptr noalias noundef nonnull sret([48 x i8]) align 8 captures(address) dereferenceable(48) %_4)
          to label %bb1 unwind label %funclet_bb5

funclet_bb5:                                      ; preds = %bb1, %start
  %_5.sroa.0.0 = phi i1 [ true, %start ], [ false, %bb1 ]
  %cleanuppad = cleanuppad within none []
  br i1 %_5.sroa.0.0, label %bb4, label %bb3

bb1:                                              ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3, ptr noundef nonnull align 8 dereferenceable(48) %_4, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_4)
; invoke anyhow::error::<impl anyhow::Error>::construct
  %_0.i2 = invoke fastcc noalias noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E"(ptr noalias noundef nonnull readonly align 8 captures(address) dereferenceable(24) %message, ptr noalias noundef nonnull readonly align 8 captures(address) dereferenceable(48) %_3)
          to label %bb2 unwind label %funclet_bb5

bb2:                                              ; preds = %bb1
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret ptr %_0.i2

bb3:                                              ; preds = %bb2.i.i.i5.i.i, %bb4, %funclet_bb5
  cleanupret from %cleanuppad unwind to caller

bb4:                                              ; preds = %funclet_bb5
  %_2.val = load i64, ptr %message, align 8
  %_6.i.i.i.i4.i.i = icmp eq i64 %_2.val, 0
  br i1 %_6.i.i.i.i4.i.i, label %bb3, label %bb2.i.i.i5.i.i

bb2.i.i.i5.i.i:                                   ; preds = %bb4
  %0 = getelementptr inbounds nuw i8, ptr %message, i64 8
  %_2.val1 = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val1, i64 noundef %_2.val, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad) ]
  br label %bb3
}

; anyhow::error::<impl anyhow::Error>::construct
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17h14a599e5ccef9669E"(i64 noundef range(i64 0, -9223372036854775808) %error.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %backtrace) unnamed_addr #11 personality ptr @__CxxFrameHandler3 {
start:
  %_5 = alloca [72 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_5)
  store ptr @alloc_a04e47d083146d15ce3892a825ec94b0, ptr %_5, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, ptr noundef nonnull align 8 dereferenceable(48) %backtrace, i64 48, i1 false)
  %1 = getelementptr inbounds nuw i8, ptr %_5, i64 56
  store ptr @alloc_cb2aea7e2fdb2fba562edabf1f950868, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_5, i64 64
  store i64 %error.1, ptr %2, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !327
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 dereferenceable_or_null(72) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 72, i64 noundef 8) #33, !noalias !327
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb2.i, label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h3e21f0fb660f3a91E.exit", !prof !320

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E(i64 noundef 8, i64 noundef 72) #36
          to label %.noexc unwind label %funclet_bb2.i

.noexc:                                           ; preds = %bb2.i
  unreachable

funclet_bb2.i:                                    ; preds = %bb2.i
  %cleanuppad.i = cleanuppad within none []
; call core::ptr::drop_in_place<core::option::Option<std::backtrace::Backtrace>>
  call fastcc void @"_ZN4core3ptr74drop_in_place$LT$core..option..Option$LT$std..backtrace..Backtrace$GT$$GT$17h14b75456a58a7afaE"(ptr noalias noundef readonly align 8 dereferenceable(48) %0) [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h3e21f0fb660f3a91E.exit": ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %3, ptr noundef nonnull align 8 dereferenceable(72) %_5, i64 72, i1 false)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_5)
  ret ptr %3
}

; anyhow::error::<impl anyhow::Error>::construct
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$9construct17hfb9bc515e14da7b4E"(ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %error, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %backtrace) unnamed_addr #11 personality ptr @__CxxFrameHandler3 {
start:
  %_5 = alloca [80 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 80, ptr nonnull %_5)
  store ptr @alloc_00e51742134d344daa7116ffd2ad9e35, ptr %_5, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, ptr noundef nonnull align 8 dereferenceable(48) %backtrace, i64 48, i1 false)
  %1 = getelementptr inbounds nuw i8, ptr %_5, i64 56
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %1, ptr noundef nonnull align 8 dereferenceable(24) %error, i64 24, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !330
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 dereferenceable_or_null(80) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 80, i64 noundef 8) #33, !noalias !330
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb2.i, label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17hcb17a0cacde96825E.exit", !prof !320

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E(i64 noundef 8, i64 noundef 80) #36
          to label %.noexc unwind label %funclet_bb2.i

.noexc:                                           ; preds = %bb2.i
  unreachable

funclet_bb2.i:                                    ; preds = %bb2.i
  %cleanuppad.i = cleanuppad within none []
; call core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>>>
  call void @"_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E"(ptr noalias noundef nonnull align 8 dereferenceable(80) %_5) #34 [ "funclet"(token %cleanuppad.i) ]
  cleanupret from %cleanuppad.i unwind to caller

"_ZN5alloc5boxed12Box$LT$T$GT$3new17hcb17a0cacde96825E.exit": ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(80) %2, ptr noundef nonnull align 8 dereferenceable(80) %_5, i64 80, i1 false)
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %_5)
  ret ptr %2
}

; anyhow::__private::format_err
; Function Attrs: cold inlinehint uwtable
define internal fastcc noalias noundef nonnull ptr @_ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE(ptr noundef nonnull %args.1) unnamed_addr #13 personality ptr @__CxxFrameHandler3 {
start:
  %_4 = alloca [24 x i8], align 8
  %_5 = ptrtoint ptr %args.1 to i64
  %_8 = and i64 %_5, 1
  %_7.not = icmp eq i64 %_8, 0
  br i1 %_7.not, label %_ZN5alloc3fmt6format17h88fc7d6adde53cfcE.exit, label %bb4

_ZN5alloc3fmt6format17h88fc7d6adde53cfcE.exit:    ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
; call alloc::fmt::format::format_inner
  call void @_ZN5alloc3fmt6format12format_inner17hbb70ff8f9f00ea6cE(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_4, ptr noundef nonnull @alloc_cb2aea7e2fdb2fba562edabf1f950868, ptr noundef nonnull %args.1), !noalias !333
; call anyhow::error::<impl anyhow::Error>::msg
  %0 = call fastcc noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17hb8c91782bfdfaac6E"(ptr noalias noundef align 8 captures(address) dereferenceable(24) %_4)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  br label %bb3

bb4:                                              ; preds = %start
  %_13 = lshr i64 %_5, 1
; call anyhow::error::<impl anyhow::Error>::msg
  %1 = tail call fastcc noundef nonnull ptr @"_ZN6anyhow5error31_$LT$impl$u20$anyhow..Error$GT$3msg17h2039c5aa29505ce5E"(i64 noundef %_13)
  br label %bb3

bb3:                                              ; preds = %bb4, %_ZN5alloc3fmt6format17h88fc7d6adde53cfcE.exit
  %_0.sroa.0.0 = phi ptr [ %1, %bb4 ], [ %0, %_ZN5alloc3fmt6format17h88fc7d6adde53cfcE.exit ]
  ret ptr %_0.sroa.0.0
}

; <anyhow::error::ErrorImpl<E> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN70_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd677e354803a6a53E"(ptr noundef nonnull align 8 %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #1 {
start:
; call anyhow::fmt::<impl anyhow::error::ErrorImpl>::debug
  %_0 = tail call noundef zeroext i1 @"_ZN6anyhow3fmt42_$LT$impl$u20$anyhow..error..ErrorImpl$GT$5debug17h7876388f4297a68bE"(ptr noundef nonnull %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter)
  ret i1 %_0
}

; <anyhow::error::ErrorImpl<E> as core::error::Error>::source
; Function Attrs: uwtable
define internal { ptr, ptr } @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..error..Error$GT$6source17hb797cb8c82fe5106E"(ptr noundef nonnull align 8 %self) unnamed_addr #1 {
start:
; call anyhow::error::ErrorImpl::error
  %0 = tail call { ptr, ptr } @_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E(ptr noundef nonnull %self)
  %_2.0 = extractvalue { ptr, ptr } %0, 0
  %_2.1 = extractvalue { ptr, ptr } %0, 1
  %1 = getelementptr inbounds nuw i8, ptr %_2.1, i64 48
  %2 = load ptr, ptr %1, align 8, !invariant.load !10, !nonnull !10
  %3 = tail call { ptr, ptr } %2(ptr noundef align 1 %_2.0) #32
  ret { ptr, ptr } %3
}

; <anyhow::error::ErrorImpl<E> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN72_$LT$anyhow..error..ErrorImpl$LT$E$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h79cad4c46422c79dE"(ptr noundef nonnull align 8 %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #1 {
start:
; call anyhow::error::ErrorImpl::error
  %0 = tail call { ptr, ptr } @_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E(ptr noundef nonnull %self)
  %_3.0 = extractvalue { ptr, ptr } %0, 0
  %_3.1 = extractvalue { ptr, ptr } %0, 1
  %1 = getelementptr inbounds nuw i8, ptr %_3.1, i64 32
  %2 = load ptr, ptr %1, align 8, !invariant.load !10, !nonnull !10
  %_0 = tail call noundef zeroext i1 %2(ptr noundef align 1 %_3.0, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter) #32
  ret i1 %_0
}

; <anyhow::wrapper::MessageError<M> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h0607a4d5b05d13c1E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val1 = load i64, ptr %1, align 8, !noundef !10
; call <str as core::fmt::Debug>::fmt
  %_0.i = tail call noundef zeroext i1 @"_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hdd75f4eba36c23acE"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <anyhow::wrapper::MessageError<M> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN75_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h663cff1b8df227b7E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %self.val = load ptr, ptr %self, align 8, !nonnull !10, !align !11, !noundef !10
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val1 = load i64, ptr %0, align 8, !noundef !10
; call <str as core::fmt::Debug>::fmt
  %_0.i = tail call noundef zeroext i1 @"_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hdd75f4eba36c23acE"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <anyhow::wrapper::MessageError<M> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17h4b8336a14db8ed90E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val1 = load i64, ptr %1, align 8, !noundef !10
; call <str as core::fmt::Display>::fmt
  %_0.i = tail call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h2e02e0ff298d12e0E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <anyhow::wrapper::MessageError<M> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN77_$LT$anyhow..wrapper..MessageError$LT$M$GT$$u20$as$u20$core..fmt..Display$GT$3fmt17hbf14f8175864174fE"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %self.val = load ptr, ptr %self, align 8, !nonnull !10, !align !11, !noundef !10
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val1 = load i64, ptr %0, align 8, !noundef !10
; call <str as core::fmt::Display>::fmt
  %_0.i = tail call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h2e02e0ff298d12e0E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; aoc2022::solver::day01::parse_input
; Function Attrs: uwtable
define void @_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_19.i.i.i.i = alloca [128 x i8], align 8
  %vector.i.i.i.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [128 x i8], align 8
  %_4 = alloca [128 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 128, ptr nonnull %_4)
; call core::str::pattern::StrSearcher::new
  call void @_ZN4core3str7pattern11StrSearcher3new17h068a94d23c181adaE(ptr noalias noundef nonnull sret([104 x i8]) align 8 captures(address) dereferenceable(128) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3f62f09340ec4217b72fe8840b861b6c, i64 noundef 2)
  %_3.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4, i64 104
  store i64 0, ptr %_3.sroa.4.0._0.sroa_idx.i, align 8, !alias.scope !336, !noalias !339
  %_3.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4, i64 112
  store i64 %input.1, ptr %_3.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !336, !noalias !339
  %_3.sroa.6.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4, i64 120
  store i8 1, ptr %_3.sroa.6.0._0.sroa_idx.i, align 8, !alias.scope !336, !noalias !339
  %_3.sroa.7.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4, i64 121
  store i8 0, ptr %_3.sroa.7.0._0.sroa_idx.i, align 1, !alias.scope !336, !noalias !339
  call void @llvm.lifetime.start.p0(i64 128, ptr nonnull %_2.i.i), !noalias !341
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(128) %_2.i.i, ptr noundef nonnull readonly align 8 dereferenceable(128) %_4, i64 128, i1 false)
  call void @llvm.lifetime.end.p0(i64 128, ptr nonnull %_4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !348
; call <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
  %0 = call fastcc { i64, i64 } @"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E"(ptr noalias noundef nonnull align 8 dereferenceable(128) %_2.i.i), !noalias !355
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = trunc nuw i64 %1 to i1
  br i1 %2, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i", label %_ZN4core4iter6traits8iterator8Iterator7collect17h591a7ad1524e9751E.exit

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i": ; preds = %start
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !356
; call __rustc::__rust_alloc
  %3 = call noundef align 8 dereferenceable_or_null(32) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 32, i64 noundef range(i64 1, 9) 8) #33, !noalias !356
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb3.i.i.i.i.i, label %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i"

bb3.i.i.i.i.i:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 8, i64 32) #36, !noalias !348
  unreachable

"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"
  %5 = extractvalue { i64, i64 } %0, 1
  store i64 %5, ptr %3, align 8, !noalias !348
  store i64 4, ptr %vector.i.i.i.i, align 8, !noalias !348
  %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 8
  store ptr %3, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !348
  %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !348
  call void @llvm.lifetime.start.p0(i64 128, ptr nonnull %_19.i.i.i.i), !noalias !348
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(128) %_19.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(128) %_2.i.i, i64 128, i1 false), !noalias !355
  call void @llvm.experimental.noalias.scope.decl(metadata !359)
  call void @llvm.experimental.noalias.scope.decl(metadata !362)
; invoke <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
  %6 = invoke fastcc { i64, i64 } @"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E"(ptr noalias noundef nonnull align 8 dereferenceable(128) %_19.i.i.i.i)
          to label %.noexc.i.i.i.i unwind label %funclet_bb7.i.i.i.i, !noalias !348

.noexc.i.i.i.i:                                   ; preds = %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i"
  %7 = extractvalue { i64, i64 } %6, 0
  %8 = trunc nuw i64 %7 to i1
  br i1 %8, label %bb3.i.i.i.i.i.i, label %bb5.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %.noexc.i.i.i.i, %.noexc8.i.i.i.i
  %_24.i.i9.i.i.i.i = phi ptr [ %_24.i.i.i.i.i.i, %.noexc8.i.i.i.i ], [ %3, %.noexc.i.i.i.i ]
  %len.i.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i.i, %.noexc8.i.i.i.i ], [ 1, %.noexc.i.i.i.i ]
  %.pn.i.i.i.i.i.i = phi { i64, i64 } [ %10, %.noexc8.i.i.i.i ], [ %6, %.noexc.i.i.i.i ]
  %9 = extractvalue { i64, i64 } %.pn.i.i.i.i.i.i, 1
  %_19.i.i.i.i.i.i = icmp samesign ult i64 %len.i.i.i.i.i.i, 1152921504606846976
  call void @llvm.assume(i1 %_19.i.i.i.i.i.i)
  %self1.i.i.i.i.i.i = load i64, ptr %vector.i.i.i.i, align 8, !range !78, !alias.scope !365, !noalias !366, !noundef !10
  %_8.i.i6.i.i.i.i = icmp eq i64 %len.i.i.i.i.i.i, %self1.i.i.i.i.i.i
  br i1 %_8.i.i6.i.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.i.i.i.i", label %bb8.i.i.i.i.i.i

"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.i.i.i.i": ; preds = %bb3.i.i.i.i.i.i
; invoke alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
  invoke fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E"(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i.i.i, i64 noundef %len.i.i.i.i.i.i, i64 noundef range(i64 1, 0) 1, i64 noundef 8, i64 noundef 8)
          to label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i" unwind label %funclet_bb7.i.i.i.i, !noalias !348

"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i": ; preds = %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.i.i.i.i"
  %_24.i.i.pre.i.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !365, !noalias !366
  br label %bb8.i.i.i.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i", %bb3.i.i.i.i.i.i
  %_24.i.i.i.i.i.i = phi ptr [ %_24.i.i.pre.i.i.i.i, %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i" ], [ %_24.i.i9.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %dst.i.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %_24.i.i.i.i.i.i, i64 %len.i.i.i.i.i.i
  store i64 %9, ptr %dst.i.i.i.i.i.i, align 8, !noalias !369
  %new_len.i.i.i.i.i.i = add nuw nsw i64 %len.i.i.i.i.i.i, 1
  store i64 %new_len.i.i.i.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !365, !noalias !366
; invoke <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
  %10 = invoke fastcc { i64, i64 } @"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hd8194a182afe9b42E"(ptr noalias noundef nonnull align 8 dereferenceable(128) %_19.i.i.i.i)
          to label %.noexc8.i.i.i.i unwind label %funclet_bb7.i.i.i.i, !noalias !348

.noexc8.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i
  %11 = extractvalue { i64, i64 } %10, 0
  %12 = trunc nuw i64 %11 to i1
  br i1 %12, label %bb3.i.i.i.i.i.i, label %bb5.i.i.i.i.loopexit

funclet_bb7.i.i.i.i:                              ; preds = %bb8.i.i.i.i.i.i, %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7e5a8b8291d0d285E.exit.i.i.i.i.i.i", %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i"
  %cleanuppad3.i.i.i.i = cleanuppad within none []
  %vector.val.i.i.i.i = load i64, ptr %vector.i.i.i.i, align 8, !noalias !348
  %_6.i.i.i.i4.i.i.i.i.i = icmp eq i64 %vector.val.i.i.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i.i, label %"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i:                             ; preds = %funclet_bb7.i.i.i.i
  %vector.val5.i.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !348, !nonnull !10, !noundef !10
  %13 = shl nuw i64 %vector.val.i.i.i.i, 3
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %vector.val5.i.i.i.i, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) 8) #33 [ "funclet"(token %cleanuppad3.i.i.i.i) ], !noalias !348
  br label %"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit.i.i.i.i"

"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit.i.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i, %funclet_bb7.i.i.i.i
  cleanupret from %cleanuppad3.i.i.i.i unwind to caller

bb5.i.i.i.i.loopexit:                             ; preds = %.noexc8.i.i.i.i
  %totals.sroa.0.0.copyload1.pre = load i64, ptr %vector.i.i.i.i, align 8, !noalias !370
  %totals.sroa.3.0.copyload2.pre = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !370
  br label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb5.i.i.i.i.loopexit, %.noexc.i.i.i.i
  %totals.sroa.4.0.copyload3 = phi i64 [ %new_len.i.i.i.i.i.i, %bb5.i.i.i.i.loopexit ], [ 1, %.noexc.i.i.i.i ]
  %totals.sroa.3.0.copyload2 = phi ptr [ %totals.sroa.3.0.copyload2.pre, %bb5.i.i.i.i.loopexit ], [ %3, %.noexc.i.i.i.i ]
  %totals.sroa.0.0.copyload1 = phi i64 [ %totals.sroa.0.0.copyload1.pre, %bb5.i.i.i.i.loopexit ], [ 4, %.noexc.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 128, ptr nonnull %_19.i.i.i.i), !noalias !348
  br label %_ZN4core4iter6traits8iterator8Iterator7collect17h591a7ad1524e9751E.exit

_ZN4core4iter6traits8iterator8Iterator7collect17h591a7ad1524e9751E.exit: ; preds = %start, %bb5.i.i.i.i
  %totals.sroa.4.0 = phi i64 [ %totals.sroa.4.0.copyload3, %bb5.i.i.i.i ], [ 0, %start ]
  %totals.sroa.3.0 = phi ptr [ %totals.sroa.3.0.copyload2, %bb5.i.i.i.i ], [ inttoptr (i64 8 to ptr), %start ]
  %totals.sroa.0.0 = phi i64 [ %totals.sroa.0.0.copyload1, %bb5.i.i.i.i ], [ 0, %start ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !348
  call void @llvm.lifetime.end.p0(i64 128, ptr nonnull %_2.i.i), !noalias !341
  store i64 %totals.sroa.0.0, ptr %_0, align 8
  %totals.sroa.3.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %totals.sroa.3.0, ptr %totals.sroa.3.0._0.sroa_idx, align 8
  %totals.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %totals.sroa.4.0, ptr %totals.sroa.4.0._0.sroa_idx, align 8
  ret void
}

; aoc2022::solver::day01::solve_part1
; Function Attrs: uwtable
define noundef i64 @_ZN7aoc20226solver5day0111solve_part117hdf4b875787c5cc5dE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_3 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call aoc2022::solver::day01::parse_input
  call void @_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_3.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %_3.val1 = load i64, ptr %1, align 8, !noundef !10
  %_7.i.i.i.i.i = icmp eq i64 %_3.val1, 0
  br i1 %_7.i.i.i.i.i, label %bb2, label %bb12.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %start
  %_18.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.val, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !371)
  %2 = icmp eq i64 %_3.val1, 1
  br i1 %2, label %bb2, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %3 = add i64 %_3.val1, 2305843009213693951
  %4 = and i64 %3, 2305843009213693951
  %_3.val.i.i.i.i.i.pre.i.i.i.i.i = load i64, ptr %_3.val, align 8, !alias.scope !374, !noalias !379
  %5 = add nsw i64 %4, -1
  %xtraiter = and i64 %3, 3
  %6 = icmp ult i64 %5, 3
  br i1 %6, label %bb2.loopexit.unr-lcssa, label %bb5.i.i.i.i.i.new

bb5.i.i.i.i.i.new:                                ; preds = %bb5.i.i.i.i.i
  %unroll_iter = and i64 %3, 2305843009213693948
  %invariant.gep = getelementptr i8, ptr %_18.i.i.i.i.i, i64 8
  %invariant.gep10 = getelementptr i8, ptr %_18.i.i.i.i.i, i64 16
  %invariant.gep12 = getelementptr i8, ptr %_18.i.i.i.i.i, i64 24
  br label %bb10.i.i.i.i.i

bb10.i.i.i.i.i:                                   ; preds = %bb10.i.i.i.i.i, %bb5.i.i.i.i.i.new
  %_3.val.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_3.val.i.i.i.i.i.pre.i.i.i.i.i, %bb5.i.i.i.i.i.new ], [ %10, %bb10.i.i.i.i.i ]
  %i.sroa.0.0.i.i.i.i.i = phi i64 [ 0, %bb5.i.i.i.i.i.new ], [ %_27.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %acc.sroa.0.0.i.i.i.i.i = phi ptr [ %_3.val, %bb5.i.i.i.i.i.new ], [ %..i.i.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %niter = phi i64 [ 0, %bb5.i.i.i.i.i.new ], [ %niter.next.3, %bb10.i.i.i.i.i ]
  %_36.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %_18.i.i.i.i.i, i64 %i.sroa.0.0.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !382)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !383)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !384)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !385)
  %_4.val.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_36.i.i.i.i.i, align 8, !alias.scope !379, !noalias !374, !noundef !10
  %_4.i.i.i.i.i.i.i = icmp ugt i64 %_3.val.i.i.i.i.i.i.i.i.i.i, %_4.val.i.i.i.i.i.i.i.i.i.i
  %7 = tail call i64 @llvm.umax.i64(i64 %_3.val.i.i.i.i.i.i.i.i.i.i, i64 %_4.val.i.i.i.i.i.i.i.i.i.i)
  %..i.i.i.i.i.i.i = select i1 %_4.i.i.i.i.i.i.i, ptr %acc.sroa.0.0.i.i.i.i.i, ptr %_36.i.i.i.i.i
  %gep = getelementptr i64, ptr %invariant.gep, i64 %i.sroa.0.0.i.i.i.i.i
  %_4.val.i.i.i.i.i.i.i.i.i.i.1 = load i64, ptr %gep, align 8, !alias.scope !386, !noalias !389, !noundef !10
  %_4.i.i.i.i.i.i.i.1 = icmp ugt i64 %7, %_4.val.i.i.i.i.i.i.i.i.i.i.1
  %8 = tail call i64 @llvm.umax.i64(i64 %7, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.1)
  %..i.i.i.i.i.i.i.1 = select i1 %_4.i.i.i.i.i.i.i.1, ptr %..i.i.i.i.i.i.i, ptr %gep
  %gep11 = getelementptr i64, ptr %invariant.gep10, i64 %i.sroa.0.0.i.i.i.i.i
  %_4.val.i.i.i.i.i.i.i.i.i.i.2 = load i64, ptr %gep11, align 8, !alias.scope !392, !noalias !395, !noundef !10
  %_4.i.i.i.i.i.i.i.2 = icmp ugt i64 %8, %_4.val.i.i.i.i.i.i.i.i.i.i.2
  %9 = tail call i64 @llvm.umax.i64(i64 %8, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.2)
  %..i.i.i.i.i.i.i.2 = select i1 %_4.i.i.i.i.i.i.i.2, ptr %..i.i.i.i.i.i.i.1, ptr %gep11
  %gep13 = getelementptr i64, ptr %invariant.gep12, i64 %i.sroa.0.0.i.i.i.i.i
  %_4.val.i.i.i.i.i.i.i.i.i.i.3 = load i64, ptr %gep13, align 8, !alias.scope !398, !noalias !401, !noundef !10
  %_4.i.i.i.i.i.i.i.3 = icmp ugt i64 %9, %_4.val.i.i.i.i.i.i.i.i.i.i.3
  %10 = tail call i64 @llvm.umax.i64(i64 %9, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.3)
  %..i.i.i.i.i.i.i.3 = select i1 %_4.i.i.i.i.i.i.i.3, ptr %..i.i.i.i.i.i.i.2, ptr %gep13
  %_27.i.i.i.i.i.3 = add nuw i64 %i.sroa.0.0.i.i.i.i.i, 4
  %niter.next.3 = add i64 %niter, 4
  %niter.ncmp.3 = icmp eq i64 %niter.next.3, %unroll_iter
  br i1 %niter.ncmp.3, label %bb2.loopexit.unr-lcssa, label %bb10.i.i.i.i.i

bb2.loopexit.unr-lcssa:                           ; preds = %bb10.i.i.i.i.i, %bb5.i.i.i.i.i
  %..i.i.i.i.i.i.i.lcssa.ph = phi ptr [ poison, %bb5.i.i.i.i.i ], [ %..i.i.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %_3.val.i.i.i.i.i.i.i.i.i.i.unr = phi i64 [ %_3.val.i.i.i.i.i.pre.i.i.i.i.i, %bb5.i.i.i.i.i ], [ %10, %bb10.i.i.i.i.i ]
  %i.sroa.0.0.i.i.i.i.i.unr = phi i64 [ 0, %bb5.i.i.i.i.i ], [ %_27.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %acc.sroa.0.0.i.i.i.i.i.unr = phi ptr [ %_3.val, %bb5.i.i.i.i.i ], [ %..i.i.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %lcmp.mod.not = icmp eq i64 %xtraiter, 0
  br i1 %lcmp.mod.not, label %bb2, label %bb10.i.i.i.i.i.epil

bb10.i.i.i.i.i.epil:                              ; preds = %bb2.loopexit.unr-lcssa, %bb10.i.i.i.i.i.epil
  %_3.val.i.i.i.i.i.i.i.i.i.i.epil = phi i64 [ %11, %bb10.i.i.i.i.i.epil ], [ %_3.val.i.i.i.i.i.i.i.i.i.i.unr, %bb2.loopexit.unr-lcssa ]
  %i.sroa.0.0.i.i.i.i.i.epil = phi i64 [ %_27.i.i.i.i.i.epil, %bb10.i.i.i.i.i.epil ], [ %i.sroa.0.0.i.i.i.i.i.unr, %bb2.loopexit.unr-lcssa ]
  %acc.sroa.0.0.i.i.i.i.i.epil = phi ptr [ %..i.i.i.i.i.i.i.epil, %bb10.i.i.i.i.i.epil ], [ %acc.sroa.0.0.i.i.i.i.i.unr, %bb2.loopexit.unr-lcssa ]
  %epil.iter = phi i64 [ %epil.iter.next, %bb10.i.i.i.i.i.epil ], [ 0, %bb2.loopexit.unr-lcssa ]
  %_36.i.i.i.i.i.epil = getelementptr inbounds nuw i64, ptr %_18.i.i.i.i.i, i64 %i.sroa.0.0.i.i.i.i.i.epil
  tail call void @llvm.experimental.noalias.scope.decl(metadata !382)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !383)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !384)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !385)
  %_4.val.i.i.i.i.i.i.i.i.i.i.epil = load i64, ptr %_36.i.i.i.i.i.epil, align 8, !alias.scope !379, !noalias !374, !noundef !10
  %_4.i.i.i.i.i.i.i.epil = icmp ugt i64 %_3.val.i.i.i.i.i.i.i.i.i.i.epil, %_4.val.i.i.i.i.i.i.i.i.i.i.epil
  %11 = tail call i64 @llvm.umax.i64(i64 %_3.val.i.i.i.i.i.i.i.i.i.i.epil, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.epil)
  %..i.i.i.i.i.i.i.epil = select i1 %_4.i.i.i.i.i.i.i.epil, ptr %acc.sroa.0.0.i.i.i.i.i.epil, ptr %_36.i.i.i.i.i.epil
  %_27.i.i.i.i.i.epil = add nuw i64 %i.sroa.0.0.i.i.i.i.i.epil, 1
  %epil.iter.next = add i64 %epil.iter, 1
  %epil.iter.cmp.not = icmp eq i64 %epil.iter.next, %xtraiter
  br i1 %epil.iter.cmp.not, label %bb2, label %bb10.i.i.i.i.i.epil, !llvm.loop !404

bb2:                                              ; preds = %bb2.loopexit.unr-lcssa, %bb10.i.i.i.i.i.epil, %bb12.i.i.i.i, %start
  %_0.sroa.0.0.i.i.i.i = phi ptr [ %_3.val, %bb12.i.i.i.i ], [ null, %start ], [ %..i.i.i.i.i.i.i.lcssa.ph, %bb2.loopexit.unr-lcssa ], [ %..i.i.i.i.i.i.i.epil, %bb10.i.i.i.i.i.epil ]
  %.not.i.i = icmp eq ptr %_0.sroa.0.0.i.i.i.i, null
  %alloc_53973d2fe29b4adba8bb7390b5678745..i.i = select i1 %.not.i.i, ptr @alloc_53973d2fe29b4adba8bb7390b5678745, ptr %_0.sroa.0.0.i.i.i.i
  %_0.i = load i64, ptr %alloc_53973d2fe29b4adba8bb7390b5678745..i.i, align 8, !noundef !10
  %_3.val4 = load i64, ptr %_3, align 8
  %_6.i.i.i.i4.i.i6 = icmp eq i64 %_3.val4, 0
  br i1 %_6.i.i.i.i4.i.i6, label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit8", label %bb2.i.i.i5.i.i7

bb2.i.i.i5.i.i7:                                  ; preds = %bb2
  %12 = shl nuw i64 %_3.val4, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.val, i64 noundef %12, i64 noundef range(i64 1, -9223372036854775807) 8) #33
  br label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit8"

"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit8": ; preds = %bb2, %bb2.i.i.i5.i.i7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  ret i64 %_0.i
}

; aoc2022::solver::day01::solve_part2
; Function Attrs: uwtable
define noundef i64 @_ZN7aoc20226solver5day0111solve_part217hb84eafde43b9ab90E(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_3 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call aoc2022::solver::day01::parse_input
  call void @_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_3.val4 = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %_3.val5 = load i64, ptr %1, align 8, !noundef !10
; invoke aoc2022::solver::day01::solve_part2_impl
  %_0 = invoke fastcc noundef i64 @_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE(ptr nonnull %_3.val4, i64 %_3.val5)
          to label %bb2 unwind label %funclet_bb4

funclet_bb4:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  %_3.val = load i64, ptr %_3, align 8
  %_6.i.i.i.i4.i.i = icmp eq i64 %_3.val, 0
  br i1 %_6.i.i.i.i4.i.i, label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit", label %bb2.i.i.i5.i.i

bb2.i.i.i5.i.i:                                   ; preds = %funclet_bb4
  %2 = shl nuw i64 %_3.val, 3
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.val4, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 8) #33 [ "funclet"(token %cleanuppad) ]
  br label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit"

"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit": ; preds = %funclet_bb4, %bb2.i.i.i5.i.i
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %start
  %_3.val2 = load i64, ptr %_3, align 8
  %_6.i.i.i.i4.i.i6 = icmp eq i64 %_3.val2, 0
  br i1 %_6.i.i.i.i4.i.i6, label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit8", label %bb2.i.i.i5.i.i7

bb2.i.i.i5.i.i7:                                  ; preds = %bb2
  %3 = shl nuw i64 %_3.val2, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.val4, i64 noundef %3, i64 noundef range(i64 1, -9223372036854775807) 8) #33
  br label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit8"

"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit8": ; preds = %bb2, %bb2.i.i.i5.i.i7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  ret i64 %_0
}

; aoc2022::solver::day01::solve_part2_impl
; Function Attrs: uwtable
define internal fastcc noundef i64 @_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE(ptr readonly captures(address_is_null) %data.8.val, i64 %data.16.val) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_4.i = alloca [8 x i8], align 8
  %0 = icmp ne ptr %data.8.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = shl nuw i64 %data.16.val, 3
  %_8.i.i.i.i = icmp eq i64 %data.16.val, 0
  br i1 %_8.i.i.i.i, label %bb7.thread, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i": ; preds = %start
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !406
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef %1, i64 noundef range(i64 1, 9) 8) #33, !noalias !406
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb3.i.i.i, label %bb2

bb3.i.i.i:                                        ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i"
; call alloc::raw_vec::handle_error
  tail call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 8, i64 %1) #36, !noalias !414
  unreachable

"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit": ; preds = %bb10.i
  %cleanuppad = cleanuppad within none []
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 8) #33 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i"
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %2, ptr nonnull readonly align 8 %data.8.val, i64 %1, i1 false), !noalias !415
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_4.i), !noalias !416
  %b.i = icmp eq i64 %data.16.val, 1
  br i1 %b.i, label %bb13.i.i.i.i.preheader, label %bb7.i, !prof !419

bb7.i:                                            ; preds = %bb2
  %b1.i = icmp samesign ult i64 %data.16.val, 21
  br i1 %b1.i, label %bb9.i, label %bb10.i, !prof !180

bb10.i:                                           ; preds = %bb7.i
; invoke core::slice::sort::unstable::ipnsort
  invoke void @_ZN4core5slice4sort8unstable7ipnsort17h0de3b552f2d2971aE(ptr noalias noundef nonnull align 8 %2, i64 noundef range(i64 0, 1152921504606846976) %data.16.val, ptr noalias noundef nonnull align 8 dereferenceable(8) %_4.i)
          to label %bb13.i.i.i.i.preheader unwind label %"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit"

bb9.i:                                            ; preds = %bb7.i
; call core::slice::sort::shared::smallsort::insertion_sort_shift_left
  tail call fastcc void @_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E(ptr noalias noundef nonnull align 8 %2, i64 noundef range(i64 0, 1152921504606846976) %data.16.val, i64 noundef 1)
  br label %bb13.i.i.i.i.preheader

bb7.thread:                                       ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 inttoptr (i64 8 to ptr), ptr nonnull readonly align 8 %data.8.val, i64 %1, i1 false), !noalias !415
  br label %"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit11"

bb13.i.i.i.i.preheader:                           ; preds = %bb10.i, %bb2, %bb9.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_4.i), !noalias !416
  %..i.i.i.i.i7 = tail call noundef i64 @llvm.umin.i64(i64 %data.16.val, i64 3)
  br label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb13.i.i.i.i.preheader, %bb13.i.i.i.i
  %acc.sroa.0.08.i.i.i.i = phi i64 [ %_5.0.i.i.i.i.i, %bb13.i.i.i.i ], [ 0, %bb13.i.i.i.i.preheader ]
  %iter.sroa.0.07.i.i.i.i = phi i64 [ %_24.i.i.i.i, %bb13.i.i.i.i ], [ 0, %bb13.i.i.i.i.preheader ]
  %_24.i.i.i.i = add nuw nsw i64 %iter.sroa.0.07.i.i.i.i, 1
  %_3.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %2, i64 %iter.sroa.0.07.i.i.i.i
  %val.val.i.i.i.i = load i64, ptr %_3.i.i.i.i.i, align 8, !noalias !420, !noundef !10
  %_5.0.i.i.i.i.i = add i64 %val.val.i.i.i.i, %acc.sroa.0.08.i.i.i.i
  %exitcond.not.i.i.i.i = icmp eq i64 %_24.i.i.i.i, %..i.i.i.i.i7
  br i1 %exitcond.not.i.i.i.i, label %bb2.i.i.i5.i10, label %bb13.i.i.i.i, !llvm.loop !429

bb2.i.i.i5.i10:                                   ; preds = %bb13.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 8) #33
  br label %"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit11"

"_ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17h516fccd102a3adcaE.exit11": ; preds = %bb7.thread, %bb2.i.i.i5.i10
  %acc.sroa.0.0.lcssa.i.i.i.i13 = phi i64 [ 0, %bb7.thread ], [ %_5.0.i.i.i.i.i, %bb2.i.i.i5.i10 ]
  ret i64 %acc.sroa.0.0.lcssa.i.i.i.i13
}

; aoc2022::solver::day01::solve
; Function Attrs: uwtable
define { i64, i64 } @_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %data = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %data)
; call aoc2022::solver::day01::parse_input
  call void @_ZN7aoc20226solver5day0111parse_input17hd5ab857934e97ff3E(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %data, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %0 = getelementptr inbounds nuw i8, ptr %data, i64 8
  %data.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %data, i64 16
  %data.val1 = load i64, ptr %1, align 8, !noundef !10
  %_7.i.i.i.i.i = icmp eq i64 %data.val1, 0
  br i1 %_7.i.i.i.i.i, label %bb2, label %bb12.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %start
  %_18.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.val, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !430)
  %2 = icmp eq i64 %data.val1, 1
  br i1 %2, label %bb2, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %3 = add i64 %data.val1, 2305843009213693951
  %4 = and i64 %3, 2305843009213693951
  %_3.val.i.i.i.i.i.pre.i.i.i.i.i = load i64, ptr %data.val, align 8, !alias.scope !433, !noalias !438
  %5 = add nsw i64 %4, -1
  %xtraiter = and i64 %3, 3
  %6 = icmp ult i64 %5, 3
  br i1 %6, label %bb2.loopexit.unr-lcssa, label %bb5.i.i.i.i.i.new

bb5.i.i.i.i.i.new:                                ; preds = %bb5.i.i.i.i.i
  %unroll_iter = and i64 %3, 2305843009213693948
  %invariant.gep = getelementptr i8, ptr %_18.i.i.i.i.i, i64 8
  %invariant.gep12 = getelementptr i8, ptr %_18.i.i.i.i.i, i64 16
  %invariant.gep14 = getelementptr i8, ptr %_18.i.i.i.i.i, i64 24
  br label %bb10.i.i.i.i.i

bb10.i.i.i.i.i:                                   ; preds = %bb10.i.i.i.i.i, %bb5.i.i.i.i.i.new
  %_3.val.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_3.val.i.i.i.i.i.pre.i.i.i.i.i, %bb5.i.i.i.i.i.new ], [ %10, %bb10.i.i.i.i.i ]
  %i.sroa.0.0.i.i.i.i.i = phi i64 [ 0, %bb5.i.i.i.i.i.new ], [ %_27.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %acc.sroa.0.0.i.i.i.i.i = phi ptr [ %data.val, %bb5.i.i.i.i.i.new ], [ %..i.i.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %niter = phi i64 [ 0, %bb5.i.i.i.i.i.new ], [ %niter.next.3, %bb10.i.i.i.i.i ]
  %_36.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %_18.i.i.i.i.i, i64 %i.sroa.0.0.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !441)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !442)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !443)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !444)
  %_4.val.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_36.i.i.i.i.i, align 8, !alias.scope !438, !noalias !433, !noundef !10
  %_4.i.i.i.i.i.i.i = icmp ugt i64 %_3.val.i.i.i.i.i.i.i.i.i.i, %_4.val.i.i.i.i.i.i.i.i.i.i
  %7 = tail call i64 @llvm.umax.i64(i64 %_3.val.i.i.i.i.i.i.i.i.i.i, i64 %_4.val.i.i.i.i.i.i.i.i.i.i)
  %..i.i.i.i.i.i.i = select i1 %_4.i.i.i.i.i.i.i, ptr %acc.sroa.0.0.i.i.i.i.i, ptr %_36.i.i.i.i.i
  %gep = getelementptr i64, ptr %invariant.gep, i64 %i.sroa.0.0.i.i.i.i.i
  %_4.val.i.i.i.i.i.i.i.i.i.i.1 = load i64, ptr %gep, align 8, !alias.scope !445, !noalias !448, !noundef !10
  %_4.i.i.i.i.i.i.i.1 = icmp ugt i64 %7, %_4.val.i.i.i.i.i.i.i.i.i.i.1
  %8 = tail call i64 @llvm.umax.i64(i64 %7, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.1)
  %..i.i.i.i.i.i.i.1 = select i1 %_4.i.i.i.i.i.i.i.1, ptr %..i.i.i.i.i.i.i, ptr %gep
  %gep13 = getelementptr i64, ptr %invariant.gep12, i64 %i.sroa.0.0.i.i.i.i.i
  %_4.val.i.i.i.i.i.i.i.i.i.i.2 = load i64, ptr %gep13, align 8, !alias.scope !451, !noalias !454, !noundef !10
  %_4.i.i.i.i.i.i.i.2 = icmp ugt i64 %8, %_4.val.i.i.i.i.i.i.i.i.i.i.2
  %9 = tail call i64 @llvm.umax.i64(i64 %8, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.2)
  %..i.i.i.i.i.i.i.2 = select i1 %_4.i.i.i.i.i.i.i.2, ptr %..i.i.i.i.i.i.i.1, ptr %gep13
  %gep15 = getelementptr i64, ptr %invariant.gep14, i64 %i.sroa.0.0.i.i.i.i.i
  %_4.val.i.i.i.i.i.i.i.i.i.i.3 = load i64, ptr %gep15, align 8, !alias.scope !457, !noalias !460, !noundef !10
  %_4.i.i.i.i.i.i.i.3 = icmp ugt i64 %9, %_4.val.i.i.i.i.i.i.i.i.i.i.3
  %10 = tail call i64 @llvm.umax.i64(i64 %9, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.3)
  %..i.i.i.i.i.i.i.3 = select i1 %_4.i.i.i.i.i.i.i.3, ptr %..i.i.i.i.i.i.i.2, ptr %gep15
  %_27.i.i.i.i.i.3 = add nuw i64 %i.sroa.0.0.i.i.i.i.i, 4
  %niter.next.3 = add i64 %niter, 4
  %niter.ncmp.3 = icmp eq i64 %niter.next.3, %unroll_iter
  br i1 %niter.ncmp.3, label %bb2.loopexit.unr-lcssa, label %bb10.i.i.i.i.i

funclet_bb5:                                      ; preds = %bb2
  %cleanuppad = cleanuppad within none []
  %data.val2 = load i64, ptr %data, align 8
  %_6.i.i.i.i4.i.i = icmp eq i64 %data.val2, 0
  br i1 %_6.i.i.i.i4.i.i, label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit", label %bb2.i.i.i5.i.i

bb2.i.i.i5.i.i:                                   ; preds = %funclet_bb5
  %11 = shl nuw i64 %data.val2, 3
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %data.val, i64 noundef %11, i64 noundef range(i64 1, -9223372036854775807) 8) #33 [ "funclet"(token %cleanuppad) ]
  br label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit"

"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit": ; preds = %funclet_bb5, %bb2.i.i.i5.i.i
  cleanupret from %cleanuppad unwind to caller

bb2.loopexit.unr-lcssa:                           ; preds = %bb10.i.i.i.i.i, %bb5.i.i.i.i.i
  %..i.i.i.i.i.i.i.lcssa.ph = phi ptr [ poison, %bb5.i.i.i.i.i ], [ %..i.i.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %_3.val.i.i.i.i.i.i.i.i.i.i.unr = phi i64 [ %_3.val.i.i.i.i.i.pre.i.i.i.i.i, %bb5.i.i.i.i.i ], [ %10, %bb10.i.i.i.i.i ]
  %i.sroa.0.0.i.i.i.i.i.unr = phi i64 [ 0, %bb5.i.i.i.i.i ], [ %_27.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %acc.sroa.0.0.i.i.i.i.i.unr = phi ptr [ %data.val, %bb5.i.i.i.i.i ], [ %..i.i.i.i.i.i.i.3, %bb10.i.i.i.i.i ]
  %lcmp.mod.not = icmp eq i64 %xtraiter, 0
  br i1 %lcmp.mod.not, label %bb2, label %bb10.i.i.i.i.i.epil

bb10.i.i.i.i.i.epil:                              ; preds = %bb2.loopexit.unr-lcssa, %bb10.i.i.i.i.i.epil
  %_3.val.i.i.i.i.i.i.i.i.i.i.epil = phi i64 [ %12, %bb10.i.i.i.i.i.epil ], [ %_3.val.i.i.i.i.i.i.i.i.i.i.unr, %bb2.loopexit.unr-lcssa ]
  %i.sroa.0.0.i.i.i.i.i.epil = phi i64 [ %_27.i.i.i.i.i.epil, %bb10.i.i.i.i.i.epil ], [ %i.sroa.0.0.i.i.i.i.i.unr, %bb2.loopexit.unr-lcssa ]
  %acc.sroa.0.0.i.i.i.i.i.epil = phi ptr [ %..i.i.i.i.i.i.i.epil, %bb10.i.i.i.i.i.epil ], [ %acc.sroa.0.0.i.i.i.i.i.unr, %bb2.loopexit.unr-lcssa ]
  %epil.iter = phi i64 [ %epil.iter.next, %bb10.i.i.i.i.i.epil ], [ 0, %bb2.loopexit.unr-lcssa ]
  %_36.i.i.i.i.i.epil = getelementptr inbounds nuw i64, ptr %_18.i.i.i.i.i, i64 %i.sroa.0.0.i.i.i.i.i.epil
  tail call void @llvm.experimental.noalias.scope.decl(metadata !441)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !442)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !443)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !444)
  %_4.val.i.i.i.i.i.i.i.i.i.i.epil = load i64, ptr %_36.i.i.i.i.i.epil, align 8, !alias.scope !438, !noalias !433, !noundef !10
  %_4.i.i.i.i.i.i.i.epil = icmp ugt i64 %_3.val.i.i.i.i.i.i.i.i.i.i.epil, %_4.val.i.i.i.i.i.i.i.i.i.i.epil
  %12 = tail call i64 @llvm.umax.i64(i64 %_3.val.i.i.i.i.i.i.i.i.i.i.epil, i64 %_4.val.i.i.i.i.i.i.i.i.i.i.epil)
  %..i.i.i.i.i.i.i.epil = select i1 %_4.i.i.i.i.i.i.i.epil, ptr %acc.sroa.0.0.i.i.i.i.i.epil, ptr %_36.i.i.i.i.i.epil
  %_27.i.i.i.i.i.epil = add nuw i64 %i.sroa.0.0.i.i.i.i.i.epil, 1
  %epil.iter.next = add i64 %epil.iter, 1
  %epil.iter.cmp.not = icmp eq i64 %epil.iter.next, %xtraiter
  br i1 %epil.iter.cmp.not, label %bb2, label %bb10.i.i.i.i.i.epil, !llvm.loop !463

bb2:                                              ; preds = %bb2.loopexit.unr-lcssa, %bb10.i.i.i.i.i.epil, %bb12.i.i.i.i, %start
  %_0.sroa.0.0.i.i.i.i = phi ptr [ %data.val, %bb12.i.i.i.i ], [ null, %start ], [ %..i.i.i.i.i.i.i.lcssa.ph, %bb2.loopexit.unr-lcssa ], [ %..i.i.i.i.i.i.i.epil, %bb10.i.i.i.i.i.epil ]
  %.not.i.i = icmp eq ptr %_0.sroa.0.0.i.i.i.i, null
  %alloc_53973d2fe29b4adba8bb7390b5678745..i.i = select i1 %.not.i.i, ptr @alloc_53973d2fe29b4adba8bb7390b5678745, ptr %_0.sroa.0.0.i.i.i.i
  %_0.i = load i64, ptr %alloc_53973d2fe29b4adba8bb7390b5678745..i.i, align 8, !noundef !10
; invoke aoc2022::solver::day01::solve_part2_impl
  %_5 = invoke fastcc noundef i64 @_ZN7aoc20226solver5day0116solve_part2_impl17hc09554692c14eebcE(ptr nonnull %data.val, i64 %data.val1)
          to label %bb3 unwind label %funclet_bb5

bb3:                                              ; preds = %bb2
  %data.val4 = load i64, ptr %data, align 8
  %_6.i.i.i.i4.i.i8 = icmp eq i64 %data.val4, 0
  br i1 %_6.i.i.i.i4.i.i8, label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit10", label %bb2.i.i.i5.i.i9

bb2.i.i.i5.i.i9:                                  ; preds = %bb3
  %13 = shl nuw i64 %data.val4, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %data.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) 8) #33
  br label %"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit10"

"_ZN4core3ptr59drop_in_place$LT$aoc2022..solver..day01..ElfInventories$GT$17h4c7fcf40314aeee8E.exit10": ; preds = %bb3, %bb2.i.i.i5.i.i9
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %data)
  %14 = insertvalue { i64, i64 } poison, i64 %_0.i, 0
  %15 = insertvalue { i64, i64 } %14, i64 %_5, 1
  ret { i64, i64 } %15
}

; aoc2022::solver::day02::parse_input
; Function Attrs: uwtable
define void @_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_19.i.i.i.i = alloca [72 x i8], align 8
  %vector.i.i.i.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [72 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_2.i.i), !noalias !464
  store i64 0, ptr %_2.i.i, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.4.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  store i64 %input.1, ptr %_3.sroa.4.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.5.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  store ptr %input.0, ptr %_3.sroa.5.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.6.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 24
  store i64 %input.1, ptr %_3.sroa.6.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.7.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 32
  store i64 0, ptr %_3.sroa.7.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.8.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 40
  store i64 %input.1, ptr %_3.sroa.8.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.9.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 48
  store i32 10, ptr %_3.sroa.9.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.10.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 52
  store i32 10, ptr %_3.sroa.10.0._2.i.i.sroa_idx, align 4, !alias.scope !471, !noalias !475
  %_3.sroa.11.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 56
  store i8 1, ptr %_3.sroa.11.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.13.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 64
  store i8 0, ptr %_3.sroa.13.0._2.i.i.sroa_idx, align 8, !alias.scope !471, !noalias !475
  %_3.sroa.14.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 65
  store i8 0, ptr %_3.sroa.14.0._2.i.i.sroa_idx, align 1, !alias.scope !471, !noalias !475
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !476
  br label %bb1.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb3.i.i.i.i.i.i.i.i, %start
; call <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
  %0 = call fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 dereferenceable(72) %_2.i.i), !noalias !483
  %1 = extractvalue { ptr, i64 } %0, 0
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %1, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %_ZN4core4iter6traits8iterator8Iterator7collect17h1a19bbf95a9ccf9cE.exit, label %bb3.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb1.i.i.i.i.i.i.i.i
  %2 = extractvalue { ptr, i64 } %0, 1
; call core::str::<impl str>::trim
  %3 = tail call fastcc { ptr, i64 } @"_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %1, i64 noundef %2) #32, !noalias !484
  %_4.1.i.i.i.i.i.i.i.i.i.i = extractvalue { ptr, i64 } %3, 1
  %_0.i.i.not.i.i.i.i.i.i.i.i.i = icmp ne i64 %_4.1.i.i.i.i.i.i.i.i.i.i, 0
  %_4.i.i.i.i.i.i.i.i.i.i.i.i = icmp ugt i64 %2, 2
  %or.cond.i.i.i.i.i.i.i.i = and i1 %_4.i.i.i.i.i.i.i.i.i.i.i.i, %_0.i.i.not.i.i.i.i.i.i.i.i.i
  br i1 %or.cond.i.i.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i", label %bb1.i.i.i.i.i.i.i.i

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i": ; preds = %bb3.i.i.i.i.i.i.i.i
  %4 = getelementptr inbounds nuw i8, ptr %1, i64 2
  %_10.i.i.i.i.i.i.i.i.i.i.i.i = load i8, ptr %4, align 1, !alias.scope !493, !noalias !484, !noundef !10
  %_7.i.i.i.i.i.i.i.i.i.i.i.i = load i8, ptr %1, align 1, !alias.scope !493, !noalias !484, !noundef !10
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !502
; call __rustc::__rust_alloc
  %5 = tail call noundef dereferenceable_or_null(8) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 8, i64 noundef range(i64 1, 9) 1) #33, !noalias !502
  %6 = icmp eq ptr %5, null
  br i1 %6, label %bb3.i.i.i.i.i, label %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i"

bb3.i.i.i.i.i:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"
; call alloc::raw_vec::handle_error
  tail call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 8) #36, !noalias !476
  unreachable

"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"
  %_0.i2.i.i.i.i.i.i.i.i.i.i.i.i = add i8 %_7.i.i.i.i.i.i.i.i.i.i.i.i, -65
  %_0.i.i.i.i.i.i.i.i.i.i.i.i.i = add i8 %_10.i.i.i.i.i.i.i.i.i.i.i.i, -88
  store i8 %_0.i2.i.i.i.i.i.i.i.i.i.i.i.i, ptr %5, align 1, !noalias !476
  %7 = getelementptr inbounds nuw i8, ptr %5, i64 1
  store i8 %_0.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %7, align 1, !noalias !476
  store i64 4, ptr %vector.i.i.i.i, align 8, !noalias !476
  %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 8
  store ptr %5, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !476
  %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !476
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_19.i.i.i.i), !noalias !476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %_19.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(72) %_2.i.i, i64 72, i1 false), !noalias !483
  tail call void @llvm.experimental.noalias.scope.decl(metadata !505)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !508)
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i, %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i"
  %_24.i.i14.i.i.i.i = phi ptr [ %_24.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ %5, %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i" ]
  %rounds.sroa.4.0.copyload3 = phi i64 [ %new_len.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ 1, %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha4e0df058ecaa51aE.exit.i.i.i.i" ]
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i
; invoke <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
  %8 = invoke fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 dereferenceable(72) %_19.i.i.i.i)
          to label %.noexc.i.i.i.i unwind label %funclet_bb7.i.i.i.i, !noalias !476

.noexc.i.i.i.i:                                   ; preds = %bb1.i.i.i.i.i.i.i.i.i.i
  %9 = extractvalue { ptr, i64 } %8, 0
  %.not.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %.noexc.i.i.i.i
  %10 = extractvalue { ptr, i64 } %8, 1
; call core::str::<impl str>::trim
  %11 = tail call fastcc { ptr, i64 } @"_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %9, i64 noundef %10) #32, !noalias !511
  %_4.1.i.i.i.i.i.i.i.i.i.i.i.i = extractvalue { ptr, i64 } %11, 1
  %_0.i.i.not.i.i.i.i.i.i.i.i.i.i.i = icmp ne i64 %_4.1.i.i.i.i.i.i.i.i.i.i.i.i, 0
  %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp ugt i64 %10, 2
  %or.cond.i.i.i.i.i.i.i.i.i.i = and i1 %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %_0.i.i.not.i.i.i.i.i.i.i.i.i.i.i
  br i1 %or.cond.i.i.i.i.i.i.i.i.i.i, label %"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E.exit.i.i.i.i.i.i", label %bb1.i.i.i.i.i.i.i.i.i.i

"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E.exit.i.i.i.i.i.i": ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load i8, ptr %9, align 1, !alias.scope !522, !noalias !511, !noundef !10
  %_0.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i8 %_7.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -65
  %12 = getelementptr inbounds nuw i8, ptr %9, i64 2
  %_10.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load i8, ptr %12, align 1, !alias.scope !522, !noalias !511, !noundef !10
  %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i8 %_10.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -88
  %_19.i.i.i.i.i.i = icmp samesign ult i64 %rounds.sroa.4.0.copyload3, 4611686018427387904
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i)
  %self1.i.i.i.i.i.i = load i64, ptr %vector.i.i.i.i, align 8, !range !78, !alias.scope !531, !noalias !532, !noundef !10
  %_8.i.i6.i.i.i.i = icmp eq i64 %rounds.sroa.4.0.copyload3, %self1.i.i.i.i.i.i
  br i1 %_8.i.i6.i.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.i.i.i.i", label %bb8.i.i.i.i.i.i

"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.i.i.i.i": ; preds = %"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E.exit.i.i.i.i.i.i"
; invoke alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
  invoke fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E"(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i.i.i, i64 noundef %rounds.sroa.4.0.copyload3, i64 noundef range(i64 1, 0) 1, i64 noundef 1, i64 noundef 2)
          to label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i" unwind label %funclet_bb7.i.i.i.i, !noalias !476

"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i": ; preds = %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.i.i.i.i"
  %_24.i.i.pre.i.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !531, !noalias !532
  br label %bb8.i.i.i.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i", %"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E.exit.i.i.i.i.i.i"
  %_24.i.i.i.i.i.i = phi ptr [ %_24.i.i.pre.i.i.i.i, %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.bb8.i.i_crit_edge.i.i.i.i" ], [ %_24.i.i14.i.i.i.i, %"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E.exit.i.i.i.i.i.i" ]
  %dst.i.i.i.i.i.i = getelementptr inbounds nuw { i8, i8 }, ptr %_24.i.i.i.i.i.i, i64 %rounds.sroa.4.0.copyload3
  store i8 %_0.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %dst.i.i.i.i.i.i, align 1, !noalias !533
  %13 = getelementptr inbounds nuw i8, ptr %dst.i.i.i.i.i.i, i64 1
  store i8 %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %13, align 1, !noalias !533
  %new_len.i.i.i.i.i.i = add nuw nsw i64 %rounds.sroa.4.0.copyload3, 1
  store i64 %new_len.i.i.i.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !531, !noalias !532
  br label %bb1.i.i.i.i.i.i

funclet_bb7.i.i.i.i:                              ; preds = %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h47e7775cc2f3ee18E.exit.i.i.i.i.i.i", %bb1.i.i.i.i.i.i.i.i.i.i
  %cleanuppad3.i.i.i.i = cleanuppad within none []
  %vector.val.i.i.i.i = load i64, ptr %vector.i.i.i.i, align 8, !noalias !476
  %_6.i.i.i.i4.i.i.i.i.i = icmp eq i64 %vector.val.i.i.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i.i, label %"_ZN4core3ptr59drop_in_place$LT$alloc..vec..Vec$LT$$LP$u8$C$u8$RP$$GT$$GT$17h9ac9550dae85bb90E.exit.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i:                             ; preds = %funclet_bb7.i.i.i.i
  %vector.val5.i.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !476, !nonnull !10, !noundef !10
  %14 = shl nuw i64 %vector.val.i.i.i.i, 1
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %vector.val5.i.i.i.i, i64 noundef %14, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad3.i.i.i.i) ], !noalias !476
  br label %"_ZN4core3ptr59drop_in_place$LT$alloc..vec..Vec$LT$$LP$u8$C$u8$RP$$GT$$GT$17h9ac9550dae85bb90E.exit.i.i.i.i"

"_ZN4core3ptr59drop_in_place$LT$alloc..vec..Vec$LT$$LP$u8$C$u8$RP$$GT$$GT$17h9ac9550dae85bb90E.exit.i.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i, %funclet_bb7.i.i.i.i
  cleanupret from %cleanuppad3.i.i.i.i unwind to caller

bb5.i.i.i.i:                                      ; preds = %.noexc.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_19.i.i.i.i), !noalias !476
  %rounds.sroa.0.0.copyload1 = load i64, ptr %vector.i.i.i.i, align 8, !noalias !534
  %rounds.sroa.3.0.copyload2 = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !534
  br label %_ZN4core4iter6traits8iterator8Iterator7collect17h1a19bbf95a9ccf9cE.exit

_ZN4core4iter6traits8iterator8Iterator7collect17h1a19bbf95a9ccf9cE.exit: ; preds = %bb1.i.i.i.i.i.i.i.i, %bb5.i.i.i.i
  %rounds.sroa.4.0 = phi i64 [ %rounds.sroa.4.0.copyload3, %bb5.i.i.i.i ], [ 0, %bb1.i.i.i.i.i.i.i.i ]
  %rounds.sroa.3.0 = phi ptr [ %rounds.sroa.3.0.copyload2, %bb5.i.i.i.i ], [ inttoptr (i64 1 to ptr), %bb1.i.i.i.i.i.i.i.i ]
  %rounds.sroa.0.0 = phi i64 [ %rounds.sroa.0.0.copyload1, %bb5.i.i.i.i ], [ 0, %bb1.i.i.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !476
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_2.i.i), !noalias !464
  store i64 %rounds.sroa.0.0, ptr %_0, align 8
  %rounds.sroa.3.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %rounds.sroa.3.0, ptr %rounds.sroa.3.0._0.sroa_idx, align 8
  %rounds.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %rounds.sroa.4.0, ptr %rounds.sroa.4.0._0.sroa_idx, align 8
  ret void
}

; aoc2022::solver::day02::solve
; Function Attrs: uwtable
define { i64, i64 } @_ZN7aoc20226solver5day025solve17h15f04a137efee39bE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %data = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %data)
; call aoc2022::solver::day02::parse_input
  call void @_ZN7aoc20226solver5day0211parse_input17h4b1637d3054b7602E(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %data, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %0 = getelementptr inbounds nuw i8, ptr %data, i64 8
  %data.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %data, i64 16
  %data.val1 = load i64, ptr %1, align 8, !noundef !10
  %2 = icmp eq i64 %data.val1, 0
  br i1 %2, label %bb3, label %bb10.i.i.i.i.i

bb10.i.i.i.i.i:                                   ; preds = %start, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i"
  %i.sroa.0.0.i.i.i.i.i = phi i64 [ %_27.i.i.i.i.i, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i" ], [ 0, %start ]
  %acc.sroa.0.0.i.i.i.i.i = phi i64 [ %_4.0.i.i.i.i.i.i.i, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i" ], [ 0, %start ]
  %_36.i.i.i.i.i = getelementptr inbounds nuw { i8, i8 }, ptr %data.val, i64 %i.sroa.0.0.i.i.i.i.i
  %_36.val.i.i.i.i.i = load i8, ptr %_36.i.i.i.i.i, align 1, !noundef !10
  %_7.i.i.i.i.i.i.i = zext i8 %_36.val.i.i.i.i.i to i64
  %_8.i.i.i.i.i.i.i = icmp ult i8 %_36.val.i.i.i.i.i, 3
  br i1 %_8.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i, label %panic.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %bb10.i.i.i.i.i
  %3 = getelementptr i8, ptr %_36.i.i.i.i.i, i64 1
  %_36.val5.i.i.i.i.i = load i8, ptr %3, align 1
  %_9.i.i.i.i.i.i.i = zext i8 %_36.val5.i.i.i.i.i to i64
  %_10.i.i.i.i.i.i.i = icmp ult i8 %_36.val5.i.i.i.i.i, 3
  br i1 %_10.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i", label %panic1.i.i.i.i.i.i.i

panic.i.i.i.i.i.i.i:                              ; preds = %bb10.i.i.i.i.i
; invoke core::panicking::panic_bounds_check
  invoke void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %_7.i.i.i.i.i.i.i, i64 noundef 3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_04ec0caaeb79a0ce9c7e48871a54c01a) #31
          to label %.noexc unwind label %funclet_bb5

.noexc:                                           ; preds = %panic.i.i.i.i.i.i.i
  unreachable

panic1.i.i.i.i.i.i.i:                             ; preds = %bb1.i.i.i.i.i.i.i
; invoke core::panicking::panic_bounds_check
  invoke void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %_9.i.i.i.i.i.i.i, i64 noundef 3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_04ec0caaeb79a0ce9c7e48871a54c01a) #31
          to label %.noexc8 unwind label %funclet_bb5

.noexc8:                                          ; preds = %panic1.i.i.i.i.i.i.i
  unreachable

"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i": ; preds = %bb1.i.i.i.i.i.i.i
  %4 = getelementptr inbounds nuw [3 x i8], ptr @anon.44ffa63e8e95c400711a21744c5ea708.5, i64 %_7.i.i.i.i.i.i.i
  %5 = getelementptr inbounds nuw i8, ptr %4, i64 %_9.i.i.i.i.i.i.i
  %_5.i.i.i.i.i.i.i = load i8, ptr %5, align 1, !noundef !10
  %_0.i.i.i.i.i.i.i = zext i8 %_5.i.i.i.i.i.i.i to i64
  %_4.0.i.i.i.i.i.i.i = add i64 %acc.sroa.0.0.i.i.i.i.i, %_0.i.i.i.i.i.i.i
  %_27.i.i.i.i.i = add nuw i64 %i.sroa.0.0.i.i.i.i.i, 1
  %_28.i.i.i.i.i = icmp eq i64 %_27.i.i.i.i.i, %data.val1
  br i1 %_28.i.i.i.i.i, label %bb10.i.i.i.i.i9, label %bb10.i.i.i.i.i

funclet_bb5:                                      ; preds = %panic1.i.i.i.i.i.i.i21, %panic.i.i.i.i.i.i.i16, %panic1.i.i.i.i.i.i.i, %panic.i.i.i.i.i.i.i
  %cleanuppad = cleanuppad within none []
  %data.val2 = load i64, ptr %data, align 8
  %_6.i.i.i.i4.i.i = icmp eq i64 %data.val2, 0
  br i1 %_6.i.i.i.i4.i.i, label %"_ZN4core3ptr55drop_in_place$LT$aoc2022..solver..day02..GameRounds$GT$17h7fcb991822deb84fE.exit", label %bb2.i.i.i5.i.i

bb2.i.i.i5.i.i:                                   ; preds = %funclet_bb5
  %6 = shl nuw i64 %data.val2, 1
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %data.val, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad) ]
  br label %"_ZN4core3ptr55drop_in_place$LT$aoc2022..solver..day02..GameRounds$GT$17h7fcb991822deb84fE.exit"

"_ZN4core3ptr55drop_in_place$LT$aoc2022..solver..day02..GameRounds$GT$17h7fcb991822deb84fE.exit": ; preds = %funclet_bb5, %bb2.i.i.i5.i.i
  cleanupret from %cleanuppad unwind to caller

bb10.i.i.i.i.i9:                                  ; preds = %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i", %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i"
  %i.sroa.0.0.i.i.i.i.i10 = phi i64 [ %_27.i.i.i.i.i25, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i" ], [ 0, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i" ]
  %acc.sroa.0.0.i.i.i.i.i11 = phi i64 [ %_4.0.i.i.i.i.i.i.i24, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i" ], [ 0, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h8fa55e531943c05bE.exit.i.i.i.i.i" ]
  %_36.i.i.i.i.i12 = getelementptr inbounds nuw { i8, i8 }, ptr %data.val, i64 %i.sroa.0.0.i.i.i.i.i10
  %_36.val.i.i.i.i.i13 = load i8, ptr %_36.i.i.i.i.i12, align 1, !noundef !10
  %_7.i.i.i.i.i.i.i14 = zext i8 %_36.val.i.i.i.i.i13 to i64
  %_8.i.i.i.i.i.i.i15 = icmp ult i8 %_36.val.i.i.i.i.i13, 3
  br i1 %_8.i.i.i.i.i.i.i15, label %bb1.i.i.i.i.i.i.i17, label %panic.i.i.i.i.i.i.i16

bb1.i.i.i.i.i.i.i17:                              ; preds = %bb10.i.i.i.i.i9
  %7 = getelementptr i8, ptr %_36.i.i.i.i.i12, i64 1
  %_36.val5.i.i.i.i.i18 = load i8, ptr %7, align 1
  %_9.i.i.i.i.i.i.i19 = zext i8 %_36.val5.i.i.i.i.i18 to i64
  %_10.i.i.i.i.i.i.i20 = icmp ult i8 %_36.val5.i.i.i.i.i18, 3
  br i1 %_10.i.i.i.i.i.i.i20, label %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i", label %panic1.i.i.i.i.i.i.i21

panic.i.i.i.i.i.i.i16:                            ; preds = %bb10.i.i.i.i.i9
; invoke core::panicking::panic_bounds_check
  invoke void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %_7.i.i.i.i.i.i.i14, i64 noundef 3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a1b3d90f42852e8e0faf60831e20d8b1) #31
          to label %.noexc28 unwind label %funclet_bb5

.noexc28:                                         ; preds = %panic.i.i.i.i.i.i.i16
  unreachable

panic1.i.i.i.i.i.i.i21:                           ; preds = %bb1.i.i.i.i.i.i.i17
; invoke core::panicking::panic_bounds_check
  invoke void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef %_9.i.i.i.i.i.i.i19, i64 noundef 3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a1b3d90f42852e8e0faf60831e20d8b1) #31
          to label %.noexc29 unwind label %funclet_bb5

.noexc29:                                         ; preds = %panic1.i.i.i.i.i.i.i21
  unreachable

"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i": ; preds = %bb1.i.i.i.i.i.i.i17
  %8 = getelementptr inbounds nuw [3 x i8], ptr @anon.44ffa63e8e95c400711a21744c5ea708.6, i64 %_7.i.i.i.i.i.i.i14
  %9 = getelementptr inbounds nuw i8, ptr %8, i64 %_9.i.i.i.i.i.i.i19
  %_5.i.i.i.i.i.i.i22 = load i8, ptr %9, align 1, !noundef !10
  %_0.i.i.i.i.i.i.i23 = zext i8 %_5.i.i.i.i.i.i.i22 to i64
  %_4.0.i.i.i.i.i.i.i24 = add i64 %acc.sroa.0.0.i.i.i.i.i11, %_0.i.i.i.i.i.i.i23
  %_27.i.i.i.i.i25 = add nuw i64 %i.sroa.0.0.i.i.i.i.i10, 1
  %_28.i.i.i.i.i26 = icmp eq i64 %_27.i.i.i.i.i25, %data.val1
  br i1 %_28.i.i.i.i.i26, label %bb3, label %bb10.i.i.i.i.i9

bb3:                                              ; preds = %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i", %start
  %_0.sroa.0.0.i.i.i.i.i34 = phi i64 [ 0, %start ], [ %_4.0.i.i.i.i.i.i.i, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i" ]
  %_0.sroa.0.0.i.i.i.i.i27 = phi i64 [ 0, %start ], [ %_4.0.i.i.i.i.i.i.i24, %"_ZN4core4iter8adapters3map8map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hb383363ff96fddcaE.exit.i.i.i.i.i" ]
  %data.val4 = load i64, ptr %data, align 8
  %_6.i.i.i.i4.i.i30 = icmp eq i64 %data.val4, 0
  br i1 %_6.i.i.i.i4.i.i30, label %"_ZN4core3ptr55drop_in_place$LT$aoc2022..solver..day02..GameRounds$GT$17h7fcb991822deb84fE.exit32", label %bb2.i.i.i5.i.i31

bb2.i.i.i5.i.i31:                                 ; preds = %bb3
  %10 = shl nuw i64 %data.val4, 1
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %data.val, i64 noundef %10, i64 noundef range(i64 1, -9223372036854775807) 1) #33
  br label %"_ZN4core3ptr55drop_in_place$LT$aoc2022..solver..day02..GameRounds$GT$17h7fcb991822deb84fE.exit32"

"_ZN4core3ptr55drop_in_place$LT$aoc2022..solver..day02..GameRounds$GT$17h7fcb991822deb84fE.exit32": ; preds = %bb3, %bb2.i.i.i5.i.i31
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %data)
  %11 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0.i.i.i.i.i34, 0
  %12 = insertvalue { i64, i64 } %11, i64 %_0.sroa.0.0.i.i.i.i.i27, 1
  ret { i64, i64 } %12
}

; aoc2022::solver::day03::parse_input
; Function Attrs: uwtable
define void @_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %_19.i.i.i.i = alloca [72 x i8], align 8
  %vector.i.i.i.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [72 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_2.i.i), !noalias !535
  store i64 0, ptr %_2.i.i, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.4.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  store i64 %input.1, ptr %_3.sroa.4.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.5.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  store ptr %input.0, ptr %_3.sroa.5.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.6.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 24
  store i64 %input.1, ptr %_3.sroa.6.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.7.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 32
  store i64 0, ptr %_3.sroa.7.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.8.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 40
  store i64 %input.1, ptr %_3.sroa.8.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.9.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 48
  store i32 10, ptr %_3.sroa.9.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.10.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 52
  store i32 10, ptr %_3.sroa.10.0._2.i.i.sroa_idx, align 4, !alias.scope !542, !noalias !546
  %_3.sroa.11.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 56
  store i8 1, ptr %_3.sroa.11.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.13.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 64
  store i8 0, ptr %_3.sroa.13.0._2.i.i.sroa_idx, align 8, !alias.scope !542, !noalias !546
  %_3.sroa.14.0._2.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 65
  store i8 0, ptr %_3.sroa.14.0._2.i.i.sroa_idx, align 1, !alias.scope !542, !noalias !546
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !547
  br label %bb1.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb3.i.i.i.i.i.i.i.i, %start
; call <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
  %0 = call fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 dereferenceable(72) %_2.i.i), !noalias !554
  %1 = extractvalue { ptr, i64 } %0, 0
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %1, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E.exit, label %bb3.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb1.i.i.i.i.i.i.i.i
  %2 = extractvalue { ptr, i64 } %0, 1
; call core::str::<impl str>::trim
  %3 = tail call fastcc { ptr, i64 } @"_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %1, i64 noundef %2) #32, !noalias !557
  %_4.1.i.i.i.i.i.i.i.i.i.i.i = extractvalue { ptr, i64 } %3, 1
  %_0.i.i.i.not.i.i.i.i.i.i.i.i.i = icmp eq i64 %_4.1.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_0.i.i.i.not.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb3.i.i.i.i.i.i.i.i
  %_8.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %2, 0
  br i1 %_8.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i": ; preds = %bb5.i.i.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !565
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %2, i64 noundef range(i64 1, 9) 1) #33, !noalias !565
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb3.i.i.i.i.i.i.i.i.i.i, label %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i"

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i"
; call alloc::raw_vec::handle_error
  tail call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %2) #36, !noalias !580
  unreachable

"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i"
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %4, ptr nonnull readonly align 1 %1, i64 range(i64 0, -9223372036854775808) %2, i1 false), !noalias !581
  %.not.i.i.i.i = icmp eq i64 %2, -9223372036854775808
  br i1 %.not.i.i.i.i, label %_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E.exit, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"

funclet_bb9.i.i.i.i:                              ; preds = %bb3.i.i.i.i.i
  %cleanuppad2.i.i.i.i = cleanuppad within none []
  br i1 %_8.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i.i:                           ; preds = %funclet_bb9.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i.i.i.i.i.i17.i.i.i.i, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad2.i.i.i.i) ], !noalias !547
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i.i, %funclet_bb9.i.i.i.i
  cleanupret from %cleanuppad2.i.i.i.i unwind to caller

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i": ; preds = %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i", %bb5.i.i.i.i.i
  %_4.sroa.10.0.i.i.i.i.i.i17.i.i.i.i = phi ptr [ %4, %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i" ], [ inttoptr (i64 1 to ptr), %bb5.i.i.i.i.i ]
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !582
; call __rustc::__rust_alloc
  %6 = tail call noundef align 8 dereferenceable_or_null(96) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef 96, i64 noundef range(i64 1, 9) 8) #33, !noalias !582
  %7 = icmp eq ptr %6, null
  br i1 %7, label %bb3.i.i.i.i.i, label %bb15.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 8, i64 96) #36
          to label %.noexc.i.i.i.i unwind label %funclet_bb9.i.i.i.i, !noalias !547

.noexc.i.i.i.i:                                   ; preds = %bb3.i.i.i.i.i
  unreachable

bb15.i.i.i.i:                                     ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i"
  store i64 %2, ptr %6, align 8, !noalias !547
  %src.sroa.4.0._28.1.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %6, i64 8
  store ptr %_4.sroa.10.0.i.i.i.i.i.i17.i.i.i.i, ptr %src.sroa.4.0._28.1.sroa_idx.i.i.i.i, align 8, !noalias !547
  %src.sroa.5.0._28.1.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %6, i64 16
  store i64 %2, ptr %src.sroa.5.0._28.1.sroa_idx.i.i.i.i, align 8, !noalias !547
  store i64 4, ptr %vector.i.i.i.i, align 8, !noalias !547
  %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 8
  store ptr %6, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !547
  %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !547
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_19.i.i.i.i), !noalias !547
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %_19.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(72) %_2.i.i, i64 72, i1 false), !noalias !585
  tail call void @llvm.experimental.noalias.scope.decl(metadata !586)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !589)
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i, %bb15.i.i.i.i
  %_24.i.i37.i.i.i.i = phi ptr [ %_24.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ %6, %bb15.i.i.i.i ]
  %lines.sroa.4.0.copyload3 = phi i64 [ %new_len.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ 1, %bb15.i.i.i.i ]
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i6.i.i.i.i, %bb1.i.i.i.i.i.i
; invoke <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
  %8 = invoke fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 dereferenceable(72) %_19.i.i.i.i)
          to label %.noexc8.i.i.i.i unwind label %funclet_bb7.i.i.i.i, !noalias !547

.noexc8.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i.i.i.i.i
  %9 = extractvalue { ptr, i64 } %8, 0
  %.not.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i, label %bb3.i.i.i.i.i.i6.i.i.i.i

bb3.i.i.i.i.i.i6.i.i.i.i:                         ; preds = %.noexc8.i.i.i.i
  %10 = extractvalue { ptr, i64 } %8, 1
; call core::str::<impl str>::trim
  %11 = tail call fastcc { ptr, i64 } @"_ZN4core3str21_$LT$impl$u20$str$GT$4trim17h65a5d3511a103624E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %9, i64 noundef %10) #32, !noalias !592
  %_4.1.i.i.i.i.i.i.i.i.i.i.i.i.i = extractvalue { ptr, i64 } %11, 1
  %_0.i.i.i.not.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_4.1.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_0.i.i.i.not.i.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i.i6.i.i.i.i
  %_8.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %10, 0
  br i1 %_8.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i": ; preds = %bb5.i.i.i.i.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !604
; call __rustc::__rust_alloc
  %12 = tail call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %10, i64 noundef range(i64 1, 9) 1) #33, !noalias !604
  %13 = icmp eq ptr %12, null
  br i1 %13, label %bb3.i.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i.i.i"

bb3.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i"
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %10) #36
          to label %.noexc9.i.i.i.i unwind label %funclet_bb7.i.i.i.i, !noalias !547

.noexc9.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i.i.i.i.i.i.i.i
  unreachable

"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i.i.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i"
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %12, ptr nonnull readonly align 1 %9, i64 range(i64 0, -9223372036854775808) %10, i1 false), !noalias !619
  %.not.i.i.i.i.i.i = icmp eq i64 %10, -9223372036854775808
  br i1 %.not.i.i.i.i.i.i, label %bb5.i.i.i.i, label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i.i.i", %bb5.i.i.i.i.i.i.i
  %_4.sroa.10.0.i.i.i.i.i.i11.i.i.i.i.i.i = phi ptr [ %12, %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i.i.i" ], [ inttoptr (i64 1 to ptr), %bb5.i.i.i.i.i.i.i ]
  %_19.i.i.i.i.i.i = icmp samesign ult i64 %lines.sroa.4.0.copyload3, 384307168202282326
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i)
  %self1.i.i.i.i.i.i = load i64, ptr %vector.i.i.i.i, align 8, !range !78, !alias.scope !620, !noalias !621, !noundef !10
  %_8.i.i7.i.i.i.i = icmp eq i64 %lines.sroa.4.0.copyload3, %self1.i.i.i.i.i.i
  br i1 %_8.i.i7.i.i.i.i, label %bb1.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i, %bb3.i.i.i.i.i.i
  %_24.i.i.i.i.i.i = phi ptr [ %_24.i.i.pre.i.i.i.i, %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i ], [ %_24.i.i37.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %dst.i.i.i.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_24.i.i.i.i.i.i, i64 %lines.sroa.4.0.copyload3
  store i64 %10, ptr %dst.i.i.i.i.i.i, align 8, !noalias !622
  %src.sroa.4.0.dst.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %dst.i.i.i.i.i.i, i64 8
  store ptr %_4.sroa.10.0.i.i.i.i.i.i11.i.i.i.i.i.i, ptr %src.sroa.4.0.dst.sroa_idx.i.i.i.i.i.i, align 8, !noalias !622
  %src.sroa.5.0.dst.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %dst.i.i.i.i.i.i, i64 16
  store i64 %10, ptr %src.sroa.5.0.dst.sroa_idx.i.i.i.i.i.i, align 8, !noalias !622
  %new_len.i.i.i.i.i.i = add nuw nsw i64 %lines.sroa.4.0.copyload3, 1
  store i64 %new_len.i.i.i.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !620, !noalias !621
  br label %bb1.i.i.i.i.i.i

funclet_bb13.i.i.i.i.i.i:                         ; preds = %bb1.i.i.i.i.i.i.i
  %cleanuppad2.i.i.i.i.i.i = cleanuppad within none []
  br i1 %_8.i.i.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i.i.i.i:                       ; preds = %funclet_bb13.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i.i.i.i.i.i11.i.i.i.i.i.i, i64 noundef %10, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad2.i.i.i.i.i.i) ], !noalias !622
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i.i.i.i, %funclet_bb13.i.i.i.i.i.i
  cleanupret from %cleanuppad2.i.i.i.i.i.i unwind label %funclet_bb7.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i.i
; invoke alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
  invoke fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h8aa7982a6c7a3c55E"(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i.i.i, i64 noundef %lines.sroa.4.0.copyload3, i64 noundef range(i64 1, 0) 1, i64 noundef 8, i64 noundef 24)
          to label %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i unwind label %funclet_bb13.i.i.i.i.i.i, !noalias !621

bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i:              ; preds = %bb1.i.i.i.i.i.i.i
  %_24.i.i.pre.i.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !620, !noalias !621
  br label %bb8.i.i.i.i.i.i

funclet_bb7.i.i.i.i:                              ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i.i.i", %bb3.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %cleanuppad3.i.i.i.i = cleanuppad within none []
; call core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
  call fastcc void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E"(ptr noalias noundef align 8 dereferenceable(24) %vector.i.i.i.i) #34 [ "funclet"(token %cleanuppad3.i.i.i.i) ], !noalias !547
  cleanupret from %cleanuppad3.i.i.i.i unwind to caller

bb5.i.i.i.i:                                      ; preds = %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i.i.i", %.noexc8.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_19.i.i.i.i), !noalias !547
  %lines.sroa.0.0.copyload1 = load i64, ptr %vector.i.i.i.i, align 8, !noalias !623
  %lines.sroa.3.0.copyload2 = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !623
  br label %_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E.exit

_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E.exit: ; preds = %bb1.i.i.i.i.i.i.i.i, %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i", %bb5.i.i.i.i
  %lines.sroa.4.0 = phi i64 [ %lines.sroa.4.0.copyload3, %bb5.i.i.i.i ], [ 0, %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i" ], [ 0, %bb1.i.i.i.i.i.i.i.i ]
  %lines.sroa.3.0 = phi ptr [ %lines.sroa.3.0.copyload2, %bb5.i.i.i.i ], [ inttoptr (i64 8 to ptr), %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i" ], [ inttoptr (i64 8 to ptr), %bb1.i.i.i.i.i.i.i.i ]
  %lines.sroa.0.0 = phi i64 [ %lines.sroa.0.0.copyload1, %bb5.i.i.i.i ], [ 0, %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E.exit.i.i.i.i" ], [ 0, %bb1.i.i.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !547
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_2.i.i), !noalias !535
  store i64 %lines.sroa.0.0, ptr %_0, align 8
  %lines.sroa.3.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %lines.sroa.3.0, ptr %lines.sroa.3.0._0.sroa_idx, align 8
  %lines.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %lines.sroa.4.0, ptr %lines.sroa.4.0._0.sroa_idx, align 8
  ret void
}

; aoc2022::solver::day03::solve_bitset
; Function Attrs: uwtable
define { i32, i32 } @_ZN7aoc20226solver5day0312solve_bitset17hfb8e228b61119ee7E(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %data = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %data)
; call aoc2022::solver::day03::parse_input
  call void @_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %data, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %0 = getelementptr inbounds nuw i8, ptr %data, i64 8
  %data.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %data, i64 16
  %data.val1 = load i64, ptr %1, align 8, !noundef !10
  %2 = icmp eq i64 %data.val1, 0
  br i1 %2, label %bb4.i.i, label %bb10.i.i.i.i.i

bb10.i.i.i.i.i:                                   ; preds = %start, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i"
  %i.sroa.0.0.i.i.i.i.i = phi i64 [ %_27.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i" ], [ 0, %start ]
  %acc.sroa.0.0.i.i.i.i.i = phi i32 [ %_0.sroa.0.0.i.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i" ], [ 0, %start ]
  %_36.i.i.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %data.val, i64 %i.sroa.0.0.i.i.i.i.i
  %3 = getelementptr i8, ptr %_36.i.i.i.i.i, i64 8
  %_36.val.i.i.i.i.i = load ptr, ptr %3, align 8, !nonnull !10, !noundef !10
  %4 = getelementptr i8, ptr %_36.i.i.i.i.i, i64 16
  %_36.val5.i.i.i.i.i = load i64, ptr %4, align 8, !noundef !10
  %_2.i.i.i.i.i.i.i.i = icmp sgt i64 %_36.val5.i.i.i.i.i, -1
  tail call void @llvm.assume(i1 %_2.i.i.i.i.i.i.i.i)
  %mid1.i.i.i.i.i.i.i = lshr i64 %_36.val5.i.i.i.i.i, 1
  %_5.i.i.i.i.i.i.i.i.i = icmp samesign ult i64 %_36.val5.i.i.i.i.i, 2
  br i1 %_5.i.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i.i

bb10.i.i.i.i.i.i.i.i.i:                           ; preds = %bb10.i.i.i.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_36.val.i.i.i.i.i, i64 %mid1.i.i.i.i.i.i.i
  %self.i.i.i.i.i.i.i.i.i = load i8, ptr %5, align 1, !alias.scope !624, !noalias !629, !noundef !10
  %6 = icmp sgt i8 %self.i.i.i.i.i.i.i.i.i, -65
  br i1 %6, label %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i", label %bb4.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb10.i.i.i.i.i.i.i.i.i
; invoke core::str::slice_error_fail
  invoke void @_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_36.val.i.i.i.i.i, i64 noundef %_36.val5.i.i.i.i.i, i64 noundef 0, i64 noundef range(i64 0, 4611686018427387904) %mid1.i.i.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90427cacc85724e4d3b32dbfd394b367) #31
          to label %.noexc unwind label %funclet_bb5

.noexc:                                           ; preds = %bb4.i.i.i.i.i.i.i.i
  unreachable

"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i": ; preds = %bb10.i.i.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !632)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !635)
  %7 = add nsw i64 %mid1.i.i.i.i.i.i.i, -1
  %xtraiter = and i64 %mid1.i.i.i.i.i.i.i, 3
  %8 = icmp ult i64 %7, 3
  br i1 %8, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa, label %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i.new"

"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i.new": ; preds = %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i"
  %unroll_iter = and i64 %mid1.i.i.i.i.i.i.i, 4611686018427387900
  br label %bb3.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i.i.i.i, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i.new"
  %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i = phi i128 [ 0, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i.new" ], [ %_0.i.i.i.i.i.i.i.i.i.i.i.3, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_36.val.i.i.i.i.i, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i.new" ], [ %_18.i.i.i.i.i.i.i.i.i.i.i.i.3, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %niter = phi i64 [ 0, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i.new" ], [ %niter.next.3, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %_18.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i, i64 1
  %v.i.i.i.i.i.i.i.i.i.i.i = load i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !637, !noalias !640, !noundef !10
  %9 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i, 127
  %10 = zext nneg i8 %9 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i = shl nuw i128 1, %10
  %_0.i.i.i.i.i.i.i.i.i.i.i = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i, %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.1 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i, i64 2
  %v.i.i.i.i.i.i.i.i.i.i.i.1 = load i8, ptr %_18.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !637, !noalias !640, !noundef !10
  %11 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i.1, 127
  %12 = zext nneg i8 %11 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i.1 = shl nuw i128 1, %12
  %_0.i.i.i.i.i.i.i.i.i.i.i.1 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i.1, %_0.i.i.i.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.2 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i, i64 3
  %v.i.i.i.i.i.i.i.i.i.i.i.2 = load i8, ptr %_18.i.i.i.i.i.i.i.i.i.i.i.i.1, align 1, !alias.scope !637, !noalias !640, !noundef !10
  %13 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i.2, 127
  %14 = zext nneg i8 %13 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i.2 = shl nuw i128 1, %14
  %_0.i.i.i.i.i.i.i.i.i.i.i.2 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i.2, %_0.i.i.i.i.i.i.i.i.i.i.i.1
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.3 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i, i64 4
  %v.i.i.i.i.i.i.i.i.i.i.i.3 = load i8, ptr %_18.i.i.i.i.i.i.i.i.i.i.i.i.2, align 1, !alias.scope !637, !noalias !640, !noundef !10
  %15 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i.3, 127
  %16 = zext nneg i8 %15 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i.3 = shl nuw i128 1, %16
  %_0.i.i.i.i.i.i.i.i.i.i.i.3 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i.3, %_0.i.i.i.i.i.i.i.i.i.i.i.2
  %niter.next.3 = add i64 %niter, 4
  %niter.ncmp.3 = icmp eq i64 %niter.next.3, %unroll_iter
  br i1 %niter.ncmp.3, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa, label %bb3.i.i.i.i.i.i.i.i.i.i

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i"
  %_0.i.i.i.i.i.i.i.i.i.i.i.lcssa.ph = phi i128 [ poison, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i" ], [ %_0.i.i.i.i.i.i.i.i.i.i.i.3, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i.unr = phi i128 [ 0, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i" ], [ %_0.i.i.i.i.i.i.i.i.i.i.i.3, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i.unr = phi ptr [ %_36.val.i.i.i.i.i, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i" ], [ %_18.i.i.i.i.i.i.i.i.i.i.i.i.3, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %lcmp.mod.not = icmp eq i64 %xtraiter, 0
  br i1 %lcmp.mod.not, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.epil

bb3.i.i.i.i.i.i.i.i.i.i.epil:                     ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa, %bb3.i.i.i.i.i.i.i.i.i.i.epil
  %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i.epil = phi i128 [ %_0.i.i.i.i.i.i.i.i.i.i.i.epil, %bb3.i.i.i.i.i.i.i.i.i.i.epil ], [ %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i.unr, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa ]
  %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i.epil = phi ptr [ %_18.i.i.i.i.i.i.i.i.i.i.i.i.epil, %bb3.i.i.i.i.i.i.i.i.i.i.epil ], [ %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i.unr, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa ]
  %epil.iter = phi i64 [ %epil.iter.next, %bb3.i.i.i.i.i.i.i.i.i.i.epil ], [ 0, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa ]
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.epil = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i.epil, i64 1
  %v.i.i.i.i.i.i.i.i.i.i.i.epil = load i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i.epil, align 1, !alias.scope !637, !noalias !640, !noundef !10
  %17 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i.epil, 127
  %18 = zext nneg i8 %17 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i.epil = shl nuw i128 1, %18
  %_0.i.i.i.i.i.i.i.i.i.i.i.epil = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i.epil, %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i.epil
  %epil.iter.next = add i64 %epil.iter, 1
  %epil.iter.cmp.not = icmp eq i64 %epil.iter.next, %xtraiter
  br i1 %epil.iter.cmp.not, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.epil, !llvm.loop !643

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i.epil, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa
  %_0.i.i.i.i.i.i.i.i.i.i.i.lcssa = phi i128 [ %_0.i.i.i.i.i.i.i.i.i.i.i.lcssa.ph, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i.unr-lcssa ], [ %_0.i.i.i.i.i.i.i.i.i.i.i.epil, %bb3.i.i.i.i.i.i.i.i.i.i.epil ]
  %19 = sub nsw i64 %_36.val5.i.i.i.i.i, %mid1.i.i.i.i.i.i.i
  br label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i: ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i, %bb10.i.i.i.i.i
  %_3.sroa.8.0.i5.i.i.i.i.i.i.i = phi i64 [ %_36.val5.i.i.i.i.i, %bb10.i.i.i.i.i ], [ %19, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i ]
  %_3.sroa.7.0.i4.i.i.i.i.i.i.i = phi ptr [ %_36.val.i.i.i.i.i, %bb10.i.i.i.i.i ], [ %5, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i ]
  %accum.sroa.0.0.lcssa.i.i.i.i.i.i.i.i.i.i = phi i128 [ 0, %bb10.i.i.i.i.i ], [ %_0.i.i.i.i.i.i.i.i.i.i.i.lcssa, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.loopexit.i.i.i.i.i.i.i ]
  %_7.i.i.not8.i.i2.i.i.i.i.i.i.i.i = icmp samesign eq i64 %_3.sroa.8.0.i5.i.i.i.i.i.i.i, 0
  br i1 %_7.i.i.not8.i.i2.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i, label %bb3.i.i3.i.i.i.i.i.i.i.i.preheader

bb3.i.i3.i.i.i.i.i.i.i.i.preheader:               ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i
  %xtraiter55 = and i64 %_3.sroa.8.0.i5.i.i.i.i.i.i.i, 3
  %20 = icmp ult i64 %_3.sroa.8.0.i5.i.i.i.i.i.i.i, 4
  br i1 %20, label %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, label %bb3.i.i3.i.i.i.i.i.i.i.i.preheader.new

bb3.i.i3.i.i.i.i.i.i.i.i.preheader.new:           ; preds = %bb3.i.i3.i.i.i.i.i.i.i.i.preheader
  %unroll_iter59 = and i64 %_3.sroa.8.0.i5.i.i.i.i.i.i.i, -4
  br label %bb3.i.i3.i.i.i.i.i.i.i.i

bb3.i.i3.i.i.i.i.i.i.i.i:                         ; preds = %bb3.i.i3.i.i.i.i.i.i.i.i, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader.new
  %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i = phi i128 [ 0, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader.new ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i.3, %bb3.i.i3.i.i.i.i.i.i.i.i ]
  %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i = phi ptr [ %_3.sroa.7.0.i4.i.i.i.i.i.i.i, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader.new ], [ %_18.i.i.i.i6.i.i.i.i.i.i.i.i.3, %bb3.i.i3.i.i.i.i.i.i.i.i ]
  %niter60 = phi i64 [ 0, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader.new ], [ %niter60.next.3, %bb3.i.i3.i.i.i.i.i.i.i.i ]
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i, i64 1
  %v.i.i.i7.i.i.i.i.i.i.i.i = load i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i, align 1, !alias.scope !644, !noalias !647, !noundef !10
  %21 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i, 127
  %22 = zext nneg i8 %21 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i = shl nuw i128 1, %22
  %_0.i.i.i9.i.i.i.i.i.i.i.i = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i, %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i.1 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i, i64 2
  %v.i.i.i7.i.i.i.i.i.i.i.i.1 = load i8, ptr %_18.i.i.i.i6.i.i.i.i.i.i.i.i, align 1, !alias.scope !644, !noalias !647, !noundef !10
  %23 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i.1, 127
  %24 = zext nneg i8 %23 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i.1 = shl nuw i128 1, %24
  %_0.i.i.i9.i.i.i.i.i.i.i.i.1 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i.1, %_0.i.i.i9.i.i.i.i.i.i.i.i
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i.2 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i, i64 3
  %v.i.i.i7.i.i.i.i.i.i.i.i.2 = load i8, ptr %_18.i.i.i.i6.i.i.i.i.i.i.i.i.1, align 1, !alias.scope !644, !noalias !647, !noundef !10
  %25 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i.2, 127
  %26 = zext nneg i8 %25 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i.2 = shl nuw i128 1, %26
  %_0.i.i.i9.i.i.i.i.i.i.i.i.2 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i.2, %_0.i.i.i9.i.i.i.i.i.i.i.i.1
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i.3 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i, i64 4
  %v.i.i.i7.i.i.i.i.i.i.i.i.3 = load i8, ptr %_18.i.i.i.i6.i.i.i.i.i.i.i.i.2, align 1, !alias.scope !644, !noalias !647, !noundef !10
  %27 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i.3, 127
  %28 = zext nneg i8 %27 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i.3 = shl nuw i128 1, %28
  %_0.i.i.i9.i.i.i.i.i.i.i.i.3 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i.3, %_0.i.i.i9.i.i.i.i.i.i.i.i.2
  %niter60.next.3 = add i64 %niter60, 4
  %niter60.ncmp.3 = icmp eq i64 %niter60.next.3, %unroll_iter59
  br i1 %niter60.ncmp.3, label %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, label %bb3.i.i3.i.i.i.i.i.i.i.i

_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa: ; preds = %bb3.i.i3.i.i.i.i.i.i.i.i, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader
  %_0.i.i.i9.i.i.i.i.i.i.i.i.lcssa.ph = phi i128 [ poison, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i.3, %bb3.i.i3.i.i.i.i.i.i.i.i ]
  %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i.unr = phi i128 [ 0, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i.3, %bb3.i.i3.i.i.i.i.i.i.i.i ]
  %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i.unr = phi ptr [ %_3.sroa.7.0.i4.i.i.i.i.i.i.i, %bb3.i.i3.i.i.i.i.i.i.i.i.preheader ], [ %_18.i.i.i.i6.i.i.i.i.i.i.i.i.3, %bb3.i.i3.i.i.i.i.i.i.i.i ]
  %lcmp.mod57.not = icmp eq i64 %xtraiter55, 0
  br i1 %lcmp.mod57.not, label %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i, label %bb3.i.i3.i.i.i.i.i.i.i.i.epil

bb3.i.i3.i.i.i.i.i.i.i.i.epil:                    ; preds = %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, %bb3.i.i3.i.i.i.i.i.i.i.i.epil
  %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i.epil = phi i128 [ %_0.i.i.i9.i.i.i.i.i.i.i.i.epil, %bb3.i.i3.i.i.i.i.i.i.i.i.epil ], [ %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i.unr, %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i.epil = phi ptr [ %_18.i.i.i.i6.i.i.i.i.i.i.i.i.epil, %bb3.i.i3.i.i.i.i.i.i.i.i.epil ], [ %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i.unr, %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %epil.iter56 = phi i64 [ %epil.iter56.next, %bb3.i.i3.i.i.i.i.i.i.i.i.epil ], [ 0, %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i.epil = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i.epil, i64 1
  %v.i.i.i7.i.i.i.i.i.i.i.i.epil = load i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i.epil, align 1, !alias.scope !644, !noalias !647, !noundef !10
  %29 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i.epil, 127
  %30 = zext nneg i8 %29 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i.epil = shl nuw i128 1, %30
  %_0.i.i.i9.i.i.i.i.i.i.i.i.epil = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i.epil, %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i.epil
  %epil.iter56.next = add i64 %epil.iter56, 1
  %epil.iter56.cmp.not = icmp eq i64 %epil.iter56.next, %xtraiter55
  br i1 %epil.iter56.cmp.not, label %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i, label %bb3.i.i3.i.i.i.i.i.i.i.i.epil, !llvm.loop !650

_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i: ; preds = %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, %bb3.i.i3.i.i.i.i.i.i.i.i.epil, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i
  %accum.sroa.0.0.lcssa.i.i11.i.i.i.i.i.i.i.i = phi i128 [ 0, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i.lcssa.ph, %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i.epil, %bb3.i.i3.i.i.i.i.i.i.i.i.epil ]
  %intersection.i.i.i.i.i.i.i.i = and i128 %accum.sroa.0.0.lcssa.i.i11.i.i.i.i.i.i.i.i, %accum.sroa.0.0.lcssa.i.i.i.i.i.i.i.i.i.i
  %_6.i.i.i.i.i.i.i.i = icmp eq i128 %intersection.i.i.i.i.i.i.i.i, 0
  %31 = tail call range(i128 0, 129) i128 @llvm.cttz.i128(i128 range(i128 1, 0) %intersection.i.i.i.i.i.i.i.i, i1 true)
  %32 = trunc nuw nsw i128 %31 to i32
  br i1 %_6.i.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i", label %bb3.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i
  %33 = add nsw i32 %32, -97
  %or.cond.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %33, 26
  br i1 %or.cond.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i4.i.i.i.i.i.i.i

bb3.i.i.i4.i.i.i.i.i.i.i:                         ; preds = %bb3.i.i.i.i.i.i.i.i
  %34 = add nsw i32 %32, -65
  %or.cond1.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %34, 26
  %35 = add nsw i32 %32, -38
  %spec.select.i.i.i.i.i.i.i.i.i.i = select i1 %or.cond1.i.i.i.i.i.i.i.i.i.i, i32 %35, i32 0
  br label %bb4.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i.i
  %36 = add nsw i32 %32, -96
  br label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i4.i.i.i.i.i.i.i
  %_0.sroa.3.0.i.i.ph.i.i.i.i.i.i = phi i32 [ %36, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %spec.select.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i4.i.i.i.i.i.i.i ]
  %_4.0.i.i.i.i.i.i.i = add i32 %_0.sroa.3.0.i.i.ph.i.i.i.i.i.i, %acc.sroa.0.0.i.i.i.i.i
  br label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i"

"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i": ; preds = %bb4.i.i.i.i.i.i, %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i
  %_0.sroa.0.0.i.i.i.i.i.i = phi i32 [ %_4.0.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %acc.sroa.0.0.i.i.i.i.i, %_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E.exit.i.i.i.i.i.i.i ]
  %_27.i.i.i.i.i = add nuw i64 %i.sroa.0.0.i.i.i.i.i, 1
  %_28.i.i.i.i.i = icmp eq i64 %_27.i.i.i.i.i, %data.val1
  br i1 %_28.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb10.i.i.i.i.i

funclet_bb5:                                      ; preds = %bb4.i.i.i.i.i.i.i.i
  %cleanuppad = cleanuppad within none []
; call core::ptr::drop_in_place<aoc2022::solver::day03::Rucksacks>
  call fastcc void @"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E"(ptr noalias noundef align 8 dereferenceable(24) %data) #34 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

bb3.i.i.i.i.i:                                    ; preds = %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i", %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i"
  %accum.sroa.0.010.i.i.i.i.i = phi i32 [ %_0.sroa.0.0.i5.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i" ], [ 0, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i" ]
  %37 = phi i64 [ %40, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i" ], [ %data.val1, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i" ]
  %38 = phi ptr [ %39, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i" ], [ %data.val, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hec140d935a5cb7d0E.exit.i.i.i.i.i" ]
  %..i.i.i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %37, i64 3)
  %39 = getelementptr inbounds nuw %"alloc::string::String", ptr %38, i64 %..i.i.i.i.i.i.i
  %40 = sub nuw nsw i64 %37, %..i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !651)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !654)
  %_3.i.i.i.i.i.i.i = icmp ugt i64 %37, 2
  br i1 %_3.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i"

bb2.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i
  %41 = getelementptr i8, ptr %38, i64 8
  %group.0.val.i.i.i.i.i.i.i = load ptr, ptr %41, align 8, !alias.scope !657, !noalias !658, !nonnull !10, !noundef !10
  %42 = getelementptr i8, ptr %38, i64 16
  %group.0.val5.i.i.i.i.i.i.i = load i64, ptr %42, align 8, !alias.scope !657, !noalias !658, !noundef !10
  %43 = getelementptr i8, ptr %38, i64 32
  %_10.val.i.i.i.i.i.i.i = load ptr, ptr %43, align 8, !alias.scope !657, !noalias !658, !nonnull !10, !noundef !10
  %44 = getelementptr i8, ptr %38, i64 40
  %_10.val4.i.i.i.i.i.i.i = load i64, ptr %44, align 8, !alias.scope !657, !noalias !658, !noundef !10
  %45 = getelementptr i8, ptr %38, i64 56
  %_13.val.i.i.i.i.i.i.i = load ptr, ptr %45, align 8, !alias.scope !657, !noalias !658, !nonnull !10, !noundef !10
  %46 = getelementptr i8, ptr %38, i64 64
  %_13.val3.i.i.i.i.i.i.i = load i64, ptr %46, align 8, !alias.scope !657, !noalias !658, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !667)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !670)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !672)
  %_7.i.i.not8.i.i.i.i.i.i.i.i.i.i = icmp samesign eq i64 %group.0.val5.i.i.i.i.i.i.i, 0
  br i1 %_7.i.i.not8.i.i.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12, label %bb3.i.i.i.i.i.i.i.i.i.i4.preheader

bb3.i.i.i.i.i.i.i.i.i.i4.preheader:               ; preds = %bb2.i.i.i.i.i.i.i
  %xtraiter61 = and i64 %group.0.val5.i.i.i.i.i.i.i, 3
  %47 = icmp ult i64 %group.0.val5.i.i.i.i.i.i.i, 4
  br i1 %47, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa, label %bb3.i.i.i.i.i.i.i.i.i.i4.preheader.new

bb3.i.i.i.i.i.i.i.i.i.i4.preheader.new:           ; preds = %bb3.i.i.i.i.i.i.i.i.i.i4.preheader
  %unroll_iter65 = and i64 %group.0.val5.i.i.i.i.i.i.i, -4
  br label %bb3.i.i.i.i.i.i.i.i.i.i4

bb3.i.i.i.i.i.i.i.i.i.i4:                         ; preds = %bb3.i.i.i.i.i.i.i.i.i.i4, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader.new
  %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i5 = phi i128 [ 0, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader.new ], [ %_0.i.i.i.i.i.i.i.i.i.i.i10.3, %bb3.i.i.i.i.i.i.i.i.i.i4 ]
  %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6 = phi ptr [ %group.0.val.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader.new ], [ %_18.i.i.i.i.i.i.i.i.i.i.i.i7.3, %bb3.i.i.i.i.i.i.i.i.i.i4 ]
  %niter66 = phi i64 [ 0, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader.new ], [ %niter66.next.3, %bb3.i.i.i.i.i.i.i.i.i.i4 ]
  %_18.i.i.i.i.i.i.i.i.i.i.i.i7 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6, i64 1
  %v.i.i.i.i.i.i.i.i.i.i.i8 = load i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6, align 1, !alias.scope !674, !noalias !677, !noundef !10
  %48 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i8, 127
  %49 = zext nneg i8 %48 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i9 = shl nuw i128 1, %49
  %_0.i.i.i.i.i.i.i.i.i.i.i10 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i9, %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i5
  %_18.i.i.i.i.i.i.i.i.i.i.i.i7.1 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6, i64 2
  %v.i.i.i.i.i.i.i.i.i.i.i8.1 = load i8, ptr %_18.i.i.i.i.i.i.i.i.i.i.i.i7, align 1, !alias.scope !674, !noalias !677, !noundef !10
  %50 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i8.1, 127
  %51 = zext nneg i8 %50 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i9.1 = shl nuw i128 1, %51
  %_0.i.i.i.i.i.i.i.i.i.i.i10.1 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i9.1, %_0.i.i.i.i.i.i.i.i.i.i.i10
  %_18.i.i.i.i.i.i.i.i.i.i.i.i7.2 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6, i64 3
  %v.i.i.i.i.i.i.i.i.i.i.i8.2 = load i8, ptr %_18.i.i.i.i.i.i.i.i.i.i.i.i7.1, align 1, !alias.scope !674, !noalias !677, !noundef !10
  %52 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i8.2, 127
  %53 = zext nneg i8 %52 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i9.2 = shl nuw i128 1, %53
  %_0.i.i.i.i.i.i.i.i.i.i.i10.2 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i9.2, %_0.i.i.i.i.i.i.i.i.i.i.i10.1
  %_18.i.i.i.i.i.i.i.i.i.i.i.i7.3 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6, i64 4
  %v.i.i.i.i.i.i.i.i.i.i.i8.3 = load i8, ptr %_18.i.i.i.i.i.i.i.i.i.i.i.i7.2, align 1, !alias.scope !674, !noalias !677, !noundef !10
  %54 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i8.3, 127
  %55 = zext nneg i8 %54 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i9.3 = shl nuw i128 1, %55
  %_0.i.i.i.i.i.i.i.i.i.i.i10.3 = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i9.3, %_0.i.i.i.i.i.i.i.i.i.i.i10.2
  %niter66.next.3 = add i64 %niter66, 4
  %niter66.ncmp.3 = icmp eq i64 %niter66.next.3, %unroll_iter65
  br i1 %niter66.ncmp.3, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa, label %bb3.i.i.i.i.i.i.i.i.i.i4

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i4, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader
  %_0.i.i.i.i.i.i.i.i.i.i.i10.lcssa.ph = phi i128 [ poison, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader ], [ %_0.i.i.i.i.i.i.i.i.i.i.i10.3, %bb3.i.i.i.i.i.i.i.i.i.i4 ]
  %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i5.unr = phi i128 [ 0, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader ], [ %_0.i.i.i.i.i.i.i.i.i.i.i10.3, %bb3.i.i.i.i.i.i.i.i.i.i4 ]
  %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6.unr = phi ptr [ %group.0.val.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i4.preheader ], [ %_18.i.i.i.i.i.i.i.i.i.i.i.i7.3, %bb3.i.i.i.i.i.i.i.i.i.i4 ]
  %lcmp.mod63.not = icmp eq i64 %xtraiter61, 0
  br i1 %lcmp.mod63.not, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12, label %bb3.i.i.i.i.i.i.i.i.i.i4.epil

bb3.i.i.i.i.i.i.i.i.i.i4.epil:                    ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa, %bb3.i.i.i.i.i.i.i.i.i.i4.epil
  %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i5.epil = phi i128 [ %_0.i.i.i.i.i.i.i.i.i.i.i10.epil, %bb3.i.i.i.i.i.i.i.i.i.i4.epil ], [ %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i5.unr, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa ]
  %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6.epil = phi ptr [ %_18.i.i.i.i.i.i.i.i.i.i.i.i7.epil, %bb3.i.i.i.i.i.i.i.i.i.i4.epil ], [ %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6.unr, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa ]
  %epil.iter62 = phi i64 [ %epil.iter62.next, %bb3.i.i.i.i.i.i.i.i.i.i4.epil ], [ 0, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa ]
  %_18.i.i.i.i.i.i.i.i.i.i.i.i7.epil = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6.epil, i64 1
  %v.i.i.i.i.i.i.i.i.i.i.i8.epil = load i8, ptr %self.sroa.0.09.i.i.i.i.i.i.i.i.i.i6.epil, align 1, !alias.scope !674, !noalias !677, !noundef !10
  %56 = and i8 %v.i.i.i.i.i.i.i.i.i.i.i8.epil, 127
  %57 = zext nneg i8 %56 to i128
  %_4.i.i.i.i.i.i.i.i.i.i.i9.epil = shl nuw i128 1, %57
  %_0.i.i.i.i.i.i.i.i.i.i.i10.epil = or i128 %_4.i.i.i.i.i.i.i.i.i.i.i9.epil, %accum.sroa.0.010.i.i.i.i.i.i.i.i.i.i5.epil
  %epil.iter62.next = add i64 %epil.iter62, 1
  %epil.iter62.cmp.not = icmp eq i64 %epil.iter62.next, %xtraiter61
  br i1 %epil.iter62.cmp.not, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12, label %bb3.i.i.i.i.i.i.i.i.i.i4.epil, !llvm.loop !680

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12: ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa, %bb3.i.i.i.i.i.i.i.i.i.i4.epil, %bb2.i.i.i.i.i.i.i
  %accum.sroa.0.0.lcssa.i.i.i.i.i.i.i.i.i.i13 = phi i128 [ 0, %bb2.i.i.i.i.i.i.i ], [ %_0.i.i.i.i.i.i.i.i.i.i.i10.lcssa.ph, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12.loopexit.unr-lcssa ], [ %_0.i.i.i.i.i.i.i.i.i.i.i10.epil, %bb3.i.i.i.i.i.i.i.i.i.i4.epil ]
  %_7.i.i.not8.i.i2.i.i.i.i.i.i.i.i15 = icmp samesign eq i64 %_10.val4.i.i.i.i.i.i.i, 0
  br i1 %_7.i.i.not8.i.i2.i.i.i.i.i.i.i.i15, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i, label %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader

bb3.i.i3.i.i.i.i.i.i.i.i16.preheader:             ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12
  %xtraiter67 = and i64 %_10.val4.i.i.i.i.i.i.i, 3
  %58 = icmp ult i64 %_10.val4.i.i.i.i.i.i.i, 4
  br i1 %58, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa, label %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader.new

bb3.i.i3.i.i.i.i.i.i.i.i16.preheader.new:         ; preds = %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader
  %unroll_iter71 = and i64 %_10.val4.i.i.i.i.i.i.i, -4
  br label %bb3.i.i3.i.i.i.i.i.i.i.i16

bb3.i.i3.i.i.i.i.i.i.i.i16:                       ; preds = %bb3.i.i3.i.i.i.i.i.i.i.i16, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader.new
  %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i17 = phi i128 [ 0, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader.new ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i22.3, %bb3.i.i3.i.i.i.i.i.i.i.i16 ]
  %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18 = phi ptr [ %_10.val.i.i.i.i.i.i.i, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader.new ], [ %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.3, %bb3.i.i3.i.i.i.i.i.i.i.i16 ]
  %niter72 = phi i64 [ 0, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader.new ], [ %niter72.next.3, %bb3.i.i3.i.i.i.i.i.i.i.i16 ]
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i19 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18, i64 1
  %v.i.i.i7.i.i.i.i.i.i.i.i20 = load i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18, align 1, !alias.scope !681, !noalias !684, !noundef !10
  %59 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i20, 127
  %60 = zext nneg i8 %59 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i21 = shl nuw i128 1, %60
  %_0.i.i.i9.i.i.i.i.i.i.i.i22 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i21, %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i17
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.1 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18, i64 2
  %v.i.i.i7.i.i.i.i.i.i.i.i20.1 = load i8, ptr %_18.i.i.i.i6.i.i.i.i.i.i.i.i19, align 1, !alias.scope !681, !noalias !684, !noundef !10
  %61 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i20.1, 127
  %62 = zext nneg i8 %61 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i21.1 = shl nuw i128 1, %62
  %_0.i.i.i9.i.i.i.i.i.i.i.i22.1 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i21.1, %_0.i.i.i9.i.i.i.i.i.i.i.i22
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.2 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18, i64 3
  %v.i.i.i7.i.i.i.i.i.i.i.i20.2 = load i8, ptr %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.1, align 1, !alias.scope !681, !noalias !684, !noundef !10
  %63 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i20.2, 127
  %64 = zext nneg i8 %63 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i21.2 = shl nuw i128 1, %64
  %_0.i.i.i9.i.i.i.i.i.i.i.i22.2 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i21.2, %_0.i.i.i9.i.i.i.i.i.i.i.i22.1
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.3 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18, i64 4
  %v.i.i.i7.i.i.i.i.i.i.i.i20.3 = load i8, ptr %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.2, align 1, !alias.scope !681, !noalias !684, !noundef !10
  %65 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i20.3, 127
  %66 = zext nneg i8 %65 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i21.3 = shl nuw i128 1, %66
  %_0.i.i.i9.i.i.i.i.i.i.i.i22.3 = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i21.3, %_0.i.i.i9.i.i.i.i.i.i.i.i22.2
  %niter72.next.3 = add i64 %niter72, 4
  %niter72.ncmp.3 = icmp eq i64 %niter72.next.3, %unroll_iter71
  br i1 %niter72.ncmp.3, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa, label %bb3.i.i3.i.i.i.i.i.i.i.i16

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa: ; preds = %bb3.i.i3.i.i.i.i.i.i.i.i16, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader
  %_0.i.i.i9.i.i.i.i.i.i.i.i22.lcssa.ph = phi i128 [ poison, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i22.3, %bb3.i.i3.i.i.i.i.i.i.i.i16 ]
  %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i17.unr = phi i128 [ 0, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i22.3, %bb3.i.i3.i.i.i.i.i.i.i.i16 ]
  %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18.unr = phi ptr [ %_10.val.i.i.i.i.i.i.i, %bb3.i.i3.i.i.i.i.i.i.i.i16.preheader ], [ %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.3, %bb3.i.i3.i.i.i.i.i.i.i.i16 ]
  %lcmp.mod69.not = icmp eq i64 %xtraiter67, 0
  br i1 %lcmp.mod69.not, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i, label %bb3.i.i3.i.i.i.i.i.i.i.i16.epil

bb3.i.i3.i.i.i.i.i.i.i.i16.epil:                  ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa, %bb3.i.i3.i.i.i.i.i.i.i.i16.epil
  %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i17.epil = phi i128 [ %_0.i.i.i9.i.i.i.i.i.i.i.i22.epil, %bb3.i.i3.i.i.i.i.i.i.i.i16.epil ], [ %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i17.unr, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18.epil = phi ptr [ %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.epil, %bb3.i.i3.i.i.i.i.i.i.i.i16.epil ], [ %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18.unr, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %epil.iter68 = phi i64 [ %epil.iter68.next, %bb3.i.i3.i.i.i.i.i.i.i.i16.epil ], [ 0, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %_18.i.i.i.i6.i.i.i.i.i.i.i.i19.epil = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18.epil, i64 1
  %v.i.i.i7.i.i.i.i.i.i.i.i20.epil = load i8, ptr %self.sroa.0.09.i.i5.i.i.i.i.i.i.i.i18.epil, align 1, !alias.scope !681, !noalias !684, !noundef !10
  %67 = and i8 %v.i.i.i7.i.i.i.i.i.i.i.i20.epil, 127
  %68 = zext nneg i8 %67 to i128
  %_4.i.i.i8.i.i.i.i.i.i.i.i21.epil = shl nuw i128 1, %68
  %_0.i.i.i9.i.i.i.i.i.i.i.i22.epil = or i128 %_4.i.i.i8.i.i.i.i.i.i.i.i21.epil, %accum.sroa.0.010.i.i4.i.i.i.i.i.i.i.i17.epil
  %epil.iter68.next = add i64 %epil.iter68, 1
  %epil.iter68.cmp.not = icmp eq i64 %epil.iter68.next, %xtraiter67
  br i1 %epil.iter68.cmp.not, label %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i, label %bb3.i.i3.i.i.i.i.i.i.i.i16.epil, !llvm.loop !687

_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i: ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa, %bb3.i.i3.i.i.i.i.i.i.i.i16.epil, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12
  %accum.sroa.0.0.lcssa.i.i11.i.i.i.i.i.i.i.i24 = phi i128 [ 0, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit.i.i.i.i.i.i.i.i12 ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i22.lcssa.ph, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i.loopexit.unr-lcssa ], [ %_0.i.i.i9.i.i.i.i.i.i.i.i22.epil, %bb3.i.i3.i.i.i.i.i.i.i.i16.epil ]
  %_7.i.i.not8.i.i14.i.i.i.i.i.i.i.i = icmp samesign eq i64 %_13.val3.i.i.i.i.i.i.i, 0
  br i1 %_7.i.i.not8.i.i14.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i, label %bb3.i.i15.i.i.i.i.i.i.i.i.preheader

bb3.i.i15.i.i.i.i.i.i.i.i.preheader:              ; preds = %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i
  %xtraiter73 = and i64 %_13.val3.i.i.i.i.i.i.i, 3
  %69 = icmp ult i64 %_13.val3.i.i.i.i.i.i.i, 4
  br i1 %69, label %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, label %bb3.i.i15.i.i.i.i.i.i.i.i.preheader.new

bb3.i.i15.i.i.i.i.i.i.i.i.preheader.new:          ; preds = %bb3.i.i15.i.i.i.i.i.i.i.i.preheader
  %unroll_iter77 = and i64 %_13.val3.i.i.i.i.i.i.i, -4
  br label %bb3.i.i15.i.i.i.i.i.i.i.i

bb3.i.i15.i.i.i.i.i.i.i.i:                        ; preds = %bb3.i.i15.i.i.i.i.i.i.i.i, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader.new
  %accum.sroa.0.010.i.i16.i.i.i.i.i.i.i.i = phi i128 [ 0, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader.new ], [ %_0.i.i.i21.i.i.i.i.i.i.i.i.3, %bb3.i.i15.i.i.i.i.i.i.i.i ]
  %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i = phi ptr [ %_13.val.i.i.i.i.i.i.i, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader.new ], [ %_18.i.i.i.i18.i.i.i.i.i.i.i.i.3, %bb3.i.i15.i.i.i.i.i.i.i.i ]
  %niter78 = phi i64 [ 0, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader.new ], [ %niter78.next.3, %bb3.i.i15.i.i.i.i.i.i.i.i ]
  %_18.i.i.i.i18.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i, i64 1
  %v.i.i.i19.i.i.i.i.i.i.i.i = load i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !691, !noundef !10
  %70 = and i8 %v.i.i.i19.i.i.i.i.i.i.i.i, 127
  %71 = zext nneg i8 %70 to i128
  %_4.i.i.i20.i.i.i.i.i.i.i.i = shl nuw i128 1, %71
  %_0.i.i.i21.i.i.i.i.i.i.i.i = or i128 %_4.i.i.i20.i.i.i.i.i.i.i.i, %accum.sroa.0.010.i.i16.i.i.i.i.i.i.i.i
  %_18.i.i.i.i18.i.i.i.i.i.i.i.i.1 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i, i64 2
  %v.i.i.i19.i.i.i.i.i.i.i.i.1 = load i8, ptr %_18.i.i.i.i18.i.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !691, !noundef !10
  %72 = and i8 %v.i.i.i19.i.i.i.i.i.i.i.i.1, 127
  %73 = zext nneg i8 %72 to i128
  %_4.i.i.i20.i.i.i.i.i.i.i.i.1 = shl nuw i128 1, %73
  %_0.i.i.i21.i.i.i.i.i.i.i.i.1 = or i128 %_4.i.i.i20.i.i.i.i.i.i.i.i.1, %_0.i.i.i21.i.i.i.i.i.i.i.i
  %_18.i.i.i.i18.i.i.i.i.i.i.i.i.2 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i, i64 3
  %v.i.i.i19.i.i.i.i.i.i.i.i.2 = load i8, ptr %_18.i.i.i.i18.i.i.i.i.i.i.i.i.1, align 1, !alias.scope !688, !noalias !691, !noundef !10
  %74 = and i8 %v.i.i.i19.i.i.i.i.i.i.i.i.2, 127
  %75 = zext nneg i8 %74 to i128
  %_4.i.i.i20.i.i.i.i.i.i.i.i.2 = shl nuw i128 1, %75
  %_0.i.i.i21.i.i.i.i.i.i.i.i.2 = or i128 %_4.i.i.i20.i.i.i.i.i.i.i.i.2, %_0.i.i.i21.i.i.i.i.i.i.i.i.1
  %_18.i.i.i.i18.i.i.i.i.i.i.i.i.3 = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i, i64 4
  %v.i.i.i19.i.i.i.i.i.i.i.i.3 = load i8, ptr %_18.i.i.i.i18.i.i.i.i.i.i.i.i.2, align 1, !alias.scope !688, !noalias !691, !noundef !10
  %76 = and i8 %v.i.i.i19.i.i.i.i.i.i.i.i.3, 127
  %77 = zext nneg i8 %76 to i128
  %_4.i.i.i20.i.i.i.i.i.i.i.i.3 = shl nuw i128 1, %77
  %_0.i.i.i21.i.i.i.i.i.i.i.i.3 = or i128 %_4.i.i.i20.i.i.i.i.i.i.i.i.3, %_0.i.i.i21.i.i.i.i.i.i.i.i.2
  %niter78.next.3 = add i64 %niter78, 4
  %niter78.ncmp.3 = icmp eq i64 %niter78.next.3, %unroll_iter77
  br i1 %niter78.ncmp.3, label %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, label %bb3.i.i15.i.i.i.i.i.i.i.i

_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa: ; preds = %bb3.i.i15.i.i.i.i.i.i.i.i, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader
  %_0.i.i.i21.i.i.i.i.i.i.i.i.lcssa.ph = phi i128 [ poison, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader ], [ %_0.i.i.i21.i.i.i.i.i.i.i.i.3, %bb3.i.i15.i.i.i.i.i.i.i.i ]
  %accum.sroa.0.010.i.i16.i.i.i.i.i.i.i.i.unr = phi i128 [ 0, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader ], [ %_0.i.i.i21.i.i.i.i.i.i.i.i.3, %bb3.i.i15.i.i.i.i.i.i.i.i ]
  %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i.unr = phi ptr [ %_13.val.i.i.i.i.i.i.i, %bb3.i.i15.i.i.i.i.i.i.i.i.preheader ], [ %_18.i.i.i.i18.i.i.i.i.i.i.i.i.3, %bb3.i.i15.i.i.i.i.i.i.i.i ]
  %lcmp.mod75.not = icmp eq i64 %xtraiter73, 0
  br i1 %lcmp.mod75.not, label %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i, label %bb3.i.i15.i.i.i.i.i.i.i.i.epil

bb3.i.i15.i.i.i.i.i.i.i.i.epil:                   ; preds = %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, %bb3.i.i15.i.i.i.i.i.i.i.i.epil
  %accum.sroa.0.010.i.i16.i.i.i.i.i.i.i.i.epil = phi i128 [ %_0.i.i.i21.i.i.i.i.i.i.i.i.epil, %bb3.i.i15.i.i.i.i.i.i.i.i.epil ], [ %accum.sroa.0.010.i.i16.i.i.i.i.i.i.i.i.unr, %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i.epil = phi ptr [ %_18.i.i.i.i18.i.i.i.i.i.i.i.i.epil, %bb3.i.i15.i.i.i.i.i.i.i.i.epil ], [ %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i.unr, %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %epil.iter74 = phi i64 [ %epil.iter74.next, %bb3.i.i15.i.i.i.i.i.i.i.i.epil ], [ 0, %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ]
  %_18.i.i.i.i18.i.i.i.i.i.i.i.i.epil = getelementptr inbounds nuw i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i.epil, i64 1
  %v.i.i.i19.i.i.i.i.i.i.i.i.epil = load i8, ptr %self.sroa.0.09.i.i17.i.i.i.i.i.i.i.i.epil, align 1, !alias.scope !688, !noalias !691, !noundef !10
  %78 = and i8 %v.i.i.i19.i.i.i.i.i.i.i.i.epil, 127
  %79 = zext nneg i8 %78 to i128
  %_4.i.i.i20.i.i.i.i.i.i.i.i.epil = shl nuw i128 1, %79
  %_0.i.i.i21.i.i.i.i.i.i.i.i.epil = or i128 %_4.i.i.i20.i.i.i.i.i.i.i.i.epil, %accum.sroa.0.010.i.i16.i.i.i.i.i.i.i.i.epil
  %epil.iter74.next = add i64 %epil.iter74, 1
  %epil.iter74.cmp.not = icmp eq i64 %epil.iter74.next, %xtraiter73
  br i1 %epil.iter74.cmp.not, label %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i, label %bb3.i.i15.i.i.i.i.i.i.i.i.epil, !llvm.loop !694

_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i: ; preds = %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa, %bb3.i.i15.i.i.i.i.i.i.i.i.epil, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i
  %accum.sroa.0.0.lcssa.i.i23.i.i.i.i.i.i.i.i = phi i128 [ 0, %_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E.exit12.i.i.i.i.i.i.i.i ], [ %_0.i.i.i21.i.i.i.i.i.i.i.i.lcssa.ph, %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i.loopexit.unr-lcssa ], [ %_0.i.i.i21.i.i.i.i.i.i.i.i.epil, %bb3.i.i15.i.i.i.i.i.i.i.i.epil ]
  %_5.i.i.i.i.i.i.i.i = and i128 %accum.sroa.0.0.lcssa.i.i11.i.i.i.i.i.i.i.i24, %accum.sroa.0.0.lcssa.i.i.i.i.i.i.i.i.i.i13
  %intersection.i.i.i.i.i.i.i.i25 = and i128 %_5.i.i.i.i.i.i.i.i, %accum.sroa.0.0.lcssa.i.i23.i.i.i.i.i.i.i.i
  %_9.i.i.i.i.i.i.i.i = icmp eq i128 %intersection.i.i.i.i.i.i.i.i25, 0
  %80 = tail call range(i128 0, 129) i128 @llvm.cttz.i128(i128 range(i128 1, 0) %intersection.i.i.i.i.i.i.i.i25, i1 true)
  %81 = trunc nuw nsw i128 %80 to i32
  br i1 %_9.i.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i", label %bb3.i.i.i.i.i.i.i.i26

bb3.i.i.i.i.i.i.i.i26:                            ; preds = %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i
  %82 = add nsw i32 %81, -97
  %or.cond.i.i.i.i.i.i.i.i.i.i27 = icmp ult i32 %82, 26
  br i1 %or.cond.i.i.i.i.i.i.i.i.i.i27, label %bb2.i.i.i.i.i.i.i.i.i.i32, label %bb3.i.i.i6.i.i.i.i.i.i.i

bb3.i.i.i6.i.i.i.i.i.i.i:                         ; preds = %bb3.i.i.i.i.i.i.i.i26
  %83 = add nsw i32 %81, -65
  %or.cond1.i.i.i.i.i.i.i.i.i.i28 = icmp ult i32 %83, 26
  %84 = add nsw i32 %81, -38
  %spec.select.i.i.i.i.i.i.i.i.i.i29 = select i1 %or.cond1.i.i.i.i.i.i.i.i.i.i28, i32 %84, i32 0
  br label %bb4.i.i.i.i.i.i30

bb2.i.i.i.i.i.i.i.i.i.i32:                        ; preds = %bb3.i.i.i.i.i.i.i.i26
  %85 = add nsw i32 %81, -96
  br label %bb4.i.i.i.i.i.i30

bb4.i.i.i.i.i.i30:                                ; preds = %bb2.i.i.i.i.i.i.i.i.i.i32, %bb3.i.i.i6.i.i.i.i.i.i.i
  %_0.sroa.3.0.i.ph.i.i.i.i.i.i = phi i32 [ %85, %bb2.i.i.i.i.i.i.i.i.i.i32 ], [ %spec.select.i.i.i.i.i.i.i.i.i.i29, %bb3.i.i.i6.i.i.i.i.i.i.i ]
  %_4.0.i.i.i.i.i.i.i31 = add i32 %_0.sroa.3.0.i.ph.i.i.i.i.i.i, %accum.sroa.0.010.i.i.i.i.i
  br label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i"

"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i": ; preds = %bb4.i.i.i.i.i.i30, %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i
  %_0.sroa.0.0.i5.i.i.i.i.i = phi i32 [ %_4.0.i.i.i.i.i.i.i31, %bb4.i.i.i.i.i.i30 ], [ %accum.sroa.0.010.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %accum.sroa.0.010.i.i.i.i.i, %_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE.exit.i.i.i.i.i.i.i ]
  %_2.i.i.i.i.i.i = icmp eq i64 %40, 0
  br i1 %_2.i.i.i.i.i.i, label %bb3, label %bb3.i.i.i.i.i

bb3:                                              ; preds = %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E.exit.i.i.i.i.i"
  tail call void @llvm.experimental.noalias.scope.decl(metadata !695)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !698)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !701)
  br label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb3, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i"
  %_3.sroa.0.012.i.i.i.i = phi i64 [ %86, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i" ], [ 0, %bb3 ]
  %_6.i.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %data.val, i64 %_3.sroa.0.012.i.i.i.i
  %86 = add nuw i64 %_3.sroa.0.012.i.i.i.i, 1
  %_6.val.i.i.i.i = load i64, ptr %_6.i.i.i.i, align 8, !alias.scope !701, !noalias !704
  %_6.i.i.i.i4.i.i.i.i.i.i = icmp eq i64 %_6.val.i.i.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i.i:                           ; preds = %bb5.i.i.i.i
  %87 = getelementptr i8, ptr %_6.i.i.i.i, i64 8
  %_6.val7.i.i.i.i = load ptr, ptr %87, align 8, !alias.scope !701, !noalias !704, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i, i64 noundef %_6.val.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !705
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i.i, %bb5.i.i.i.i
  %_7.i.i.i.i = icmp eq i64 %86, %data.val1
  br i1 %_7.i.i.i.i, label %bb4.i.i, label %bb5.i.i.i.i

bb4.i.i:                                          ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i", %start
  %accum.sroa.0.0.lcssa.i.i.i.i.i38 = phi i32 [ 0, %start ], [ %_0.sroa.0.0.i5.i.i.i.i.i, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i" ]
  %_0.sroa.0.0.i.i.i.i.i3437 = phi i32 [ 0, %start ], [ %_0.sroa.0.0.i.i.i.i.i.i, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i" ]
  %_1.val4.i.i = load i64, ptr %data, align 8, !range !78, !alias.scope !704, !noundef !10
  %_6.i.i.i.i6.i.i = icmp eq i64 %_1.val4.i.i, 0
  br i1 %_6.i.i.i.i6.i.i, label %"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E.exit", label %bb2.i.i.i7.i.i

bb2.i.i.i7.i.i:                                   ; preds = %bb4.i.i
  %88 = mul nuw i64 %_1.val4.i.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %data.val, i64 noundef %88, i64 noundef range(i64 1, -9223372036854775807) 8) #33, !noalias !704
  br label %"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E.exit"

"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E.exit": ; preds = %bb4.i.i, %bb2.i.i.i7.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %data)
  %89 = insertvalue { i32, i32 } poison, i32 %_0.sroa.0.0.i.i.i.i.i3437, 0
  %90 = insertvalue { i32, i32 } %89, i32 %accum.sroa.0.0.lcssa.i.i.i.i.i38, 1
  ret { i32, i32 } %90
}

; aoc2022::solver::day03::solve
; Function Attrs: uwtable
define { i32, i32 } @_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %set.i.i.i.i.i.i.i.i.i.i = alloca [48 x i8], align 8
  %third_set.i.i.i.i.i.i.i.i = alloca [48 x i8], align 8
  %second_set.i.i.i.i.i.i.i.i5 = alloca [48 x i8], align 8
  %first_set.i.i.i.i.i.i.i.i6 = alloca [48 x i8], align 8
  %second_set.i.i.i.i.i.i.i.i = alloca [48 x i8], align 8
  %first_set.i.i.i.i.i.i.i.i = alloca [48 x i8], align 8
  %data = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %data)
; call aoc2022::solver::day03::parse_input
  call void @_ZN7aoc20226solver5day0311parse_input17h5419a424c46aeee7E(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %data, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %0 = getelementptr inbounds nuw i8, ptr %data, i64 8
  %data.val = load ptr, ptr %0, align 8, !nonnull !10, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %data, i64 16
  %data.val1 = load i64, ptr %1, align 8, !noundef !10
  %2 = icmp eq i64 %data.val1, 0
  br i1 %2, label %bb2, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %start
  %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %first_set.i.i.i.i.i.i.i.i, i64 8
  %self.sink14.i.sroa.gep25.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %second_set.i.i.i.i.i.i.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %first_set.i.i.i.i.i.i.i.i, i64 24
  %4 = getelementptr inbounds nuw i8, ptr %second_set.i.i.i.i.i.i.i.i, i64 24
  br label %bb10.i.i.i.i.i

bb10.i.i.i.i.i:                                   ; preds = %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i", %bb5.i.i.i.i.i
  %i.sroa.0.0.i.i.i.i.i = phi i64 [ 0, %bb5.i.i.i.i.i ], [ %_27.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i" ]
  %acc.sroa.0.0.i.i.i.i.i = phi i32 [ 0, %bb5.i.i.i.i.i ], [ %_0.sroa.0.0.i.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i" ]
  %_36.i.i.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %data.val, i64 %i.sroa.0.0.i.i.i.i.i
  %5 = getelementptr i8, ptr %_36.i.i.i.i.i, i64 8
  %_36.val.i.i.i.i.i = load ptr, ptr %5, align 8, !nonnull !10, !noundef !10
  %6 = getelementptr i8, ptr %_36.i.i.i.i.i, i64 16
  %_36.val5.i.i.i.i.i = load i64, ptr %6, align 8, !noundef !10
  %_2.i.i.i.i.i.i.i.i = icmp sgt i64 %_36.val5.i.i.i.i.i, -1
  tail call void @llvm.assume(i1 %_2.i.i.i.i.i.i.i.i)
  %mid1.i.i.i.i.i.i.i = lshr i64 %_36.val5.i.i.i.i.i, 1
  %_5.i.i.i.i.i.i.i.i.i = icmp samesign ult i64 %_36.val5.i.i.i.i.i, 2
  br i1 %_5.i.i.i.i.i.i.i.i.i, label %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i", label %bb10.i.i.i.i.i.i.i.i.i

bb10.i.i.i.i.i.i.i.i.i:                           ; preds = %bb10.i.i.i.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_36.val.i.i.i.i.i, i64 %mid1.i.i.i.i.i.i.i
  %self.i.i.i.i.i.i.i.i.i = load i8, ptr %7, align 1, !alias.scope !706, !noalias !711, !noundef !10
  %8 = icmp sgt i8 %self.i.i.i.i.i.i.i.i.i, -65
  br i1 %8, label %bb10.split.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i

bb10.split.i.i.i.i.i.i.i.i.i:                     ; preds = %bb10.i.i.i.i.i.i.i.i.i
  %9 = sub nsw i64 %_36.val5.i.i.i.i.i, %mid1.i.i.i.i.i.i.i
  br label %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i"

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb10.i.i.i.i.i.i.i.i.i
; invoke core::str::slice_error_fail
  invoke void @_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_36.val.i.i.i.i.i, i64 noundef %_36.val5.i.i.i.i.i, i64 noundef 0, i64 noundef range(i64 0, 4611686018427387904) %mid1.i.i.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90427cacc85724e4d3b32dbfd394b367) #31
          to label %.noexc unwind label %funclet_bb5

.noexc:                                           ; preds = %bb4.i.i.i.i.i.i.i.i
  unreachable

"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i": ; preds = %bb10.split.i.i.i.i.i.i.i.i.i, %bb10.i.i.i.i.i
  %_3.sroa.7.0.i.i.i.i.i.i.i.i = phi ptr [ %7, %bb10.split.i.i.i.i.i.i.i.i.i ], [ %_36.val.i.i.i.i.i, %bb10.i.i.i.i.i ]
  %_3.sroa.8.0.i.i.i.i.i.i.i.i = phi i64 [ %9, %bb10.split.i.i.i.i.i.i.i.i.i ], [ %_36.val5.i.i.i.i.i, %bb10.i.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %first_set.i.i.i.i.i.i.i.i), !noalias !714
  %_7.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_36.val.i.i.i.i.i, i64 %mid1.i.i.i.i.i.i.i
; invoke core::iter::traits::iterator::Iterator::collect
  invoke fastcc void @_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E(ptr noalias noundef align 8 captures(address) dereferenceable(48) %first_set.i.i.i.i.i.i.i.i, ptr noundef nonnull readonly align 1 %_36.val.i.i.i.i.i, ptr noundef readonly %_7.i.i.i.i.i.i.i.i.i) #32
          to label %.noexc4 unwind label %funclet_bb5

.noexc4:                                          ; preds = %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i"
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %second_set.i.i.i.i.i.i.i.i), !noalias !714
  %_7.i8.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.sroa.7.0.i.i.i.i.i.i.i.i, i64 %_3.sroa.8.0.i.i.i.i.i.i.i.i
; invoke core::iter::traits::iterator::Iterator::collect
  invoke fastcc void @_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E(ptr noalias noundef align 8 captures(address) dereferenceable(48) %second_set.i.i.i.i.i.i.i.i, ptr noundef nonnull readonly align 1 %_3.sroa.7.0.i.i.i.i.i.i.i.i, ptr noundef readonly %_7.i8.i.i.i.i.i.i.i.i)
          to label %bb4.i4.i.i.i.i.i.i.i unwind label %funclet_bb11.i.i.i.i.i.i.i.i

funclet_bb11.i.i.i.i.i.i.i.i:                     ; preds = %.noexc4
  %cleanuppad.i.i.i.i.i.i.i.i = cleanuppad within none []
  %first_set.val.i.i.i.i.i.i.i.i = load ptr, ptr %first_set.i.i.i.i.i.i.i.i, align 8, !noalias !714
  %first_set.val2.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i, align 8, !noalias !714, !noundef !10
; call core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
  call fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %first_set.val.i.i.i.i.i.i.i.i, i64 %first_set.val2.i.i.i.i.i.i.i.i) #34 [ "funclet"(token %cleanuppad.i.i.i.i.i.i.i.i) ]
  cleanupret from %cleanuppad.i.i.i.i.i.i.i.i unwind label %funclet_bb5

bb4.i4.i.i.i.i.i.i.i:                             ; preds = %.noexc4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !718)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !721)
  %_4.i.i.i.i.i.i.i.i.i = load i64, ptr %3, align 8, !alias.scope !718, !noalias !723, !noundef !10
  %_5.i.i5.i.i.i.i.i.i.i = load i64, ptr %4, align 8, !alias.scope !721, !noalias !725, !noundef !10
  %_4.sink.i.i.i.i.i.i.i.i.i = tail call i64 @llvm.umin.i64(i64 %_4.i.i.i.i.i.i.i.i.i, i64 %_5.i.i5.i.i.i.i.i.i.i)
  %second_set.val.i.i.i.i.i.i.i.i = load ptr, ptr %second_set.i.i.i.i.i.i.i.i, align 8, !noalias !714
  %first_set.val32.i.i.i.i.i.i.i.i = load ptr, ptr %first_set.i.i.i.i.i.i.i.i, align 8, !noalias !714
  %_11.i.i.i.i.i27.i.i.i.i.i = icmp eq i64 %_4.sink.i.i.i.i.i.i.i.i.i, 0
  %second_set.val7.i.i.i.pre.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep25.i.i.i.i.i.i.i.i, align 8, !noalias !714
  br i1 %_11.i.i.i.i.i27.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.lr.ph.i.i.i.i.i

bb6.i.i.i.i.i.lr.ph.i.i.i.i.i:                    ; preds = %bb4.i4.i.i.i.i.i.i.i
  %_3.not.i.i.i.i.i.i.i.i.i = icmp ugt i64 %_4.i.i.i.i.i.i.i.i.i, %_5.i.i5.i.i.i.i.i.i.i
  %self1.i1.i.i.i.i.i.i.i.i.i = select i1 %_3.not.i.i.i.i.i.i.i.i.i, ptr %second_set.val.i.i.i.i.i.i.i.i, ptr %first_set.val32.i.i.i.i.i.i.i.i
  %next_ctrl.i.i5.sink.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i1.i.i.i.i.i.i.i.i.i, i64 16
  %_23.i.i4.sink.in.in.in.i.i.i.i.i.i.i.i.i = load <16 x i8>, ptr %self1.i1.i.i.i.i.i.i.i.i.i, align 16, !noalias !726
  %_23.i.i4.sink.in.in.i.i.i.i.i.i.i.i.i = icmp slt <16 x i8> %_23.i.i4.sink.in.in.in.i.i.i.i.i.i.i.i.i, zeroinitializer
  %_23.i.i4.sink.in.i.i.i.i.i.i.i.i.i = bitcast <16 x i1> %_23.i.i4.sink.in.in.i.i.i.i.i.i.i.i.i to i16
  %10 = or i64 %_5.i.i5.i.i.i.i.i.i.i, %_4.i.i.i.i.i.i.i.i.i
  %_3.i.i.i.i.i.i = icmp eq i64 %10, 0
  %other.sink.i.i.i.i.sroa.sel9.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v = select i1 %_3.not.i.i.i.i.i.i.i.i.i, ptr %first_set.i.i.i.i.i.i.i.i, ptr %second_set.i.i.i.i.i.i.i.i
  %other.sink.i.i.i.i.sroa.sel9.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel = getelementptr inbounds nuw i8, ptr %other.sink.i.i.i.i.sroa.sel9.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v, i64 32
  %hash_builder.val.i.i.i.i.i.i = load i64, ptr %other.sink.i.i.i.i.sroa.sel9.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel, align 8
  %other.sink.i.i.i.i.sroa.sel12.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v = select i1 %_3.not.i.i.i.i.i.i.i.i.i, ptr %first_set.i.i.i.i.i.i.i.i, ptr %second_set.i.i.i.i.i.i.i.i
  %other.sink.i.i.i.i.sroa.sel12.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel = getelementptr inbounds nuw i8, ptr %other.sink.i.i.i.i.sroa.sel12.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v, i64 40
  %hash_builder.val1.i.i.i.i.i.i = load i64, ptr %other.sink.i.i.i.i.sroa.sel12.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel, align 8
  %11 = xor i64 %hash_builder.val.i.i.i.i.i.i, 8317987319222330741
  %12 = xor i64 %hash_builder.val1.i.i.i.i.i.i, 7237128888997146477
  %13 = xor i64 %hash_builder.val.i.i.i.i.i.i, 7816392313619706465
  %_2.i.i.i.i.i.i.i.i.i.i = add i64 %12, %11
  %14 = tail call i64 @llvm.fshl.i64(i64 %12, i64 %12, i64 13)
  %15 = xor i64 %14, %_2.i.i.i.i.i.i.i.i.i.i
  %16 = tail call i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i.i.i.i, i64 32)
  %invariant.op31.i.i.i.i.i = add i64 %15, %13
  %17 = tail call i64 @llvm.fshl.i64(i64 %15, i64 %15, i64 17)
  %self.sink14.i.sroa.gep.i.i.i.val.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i, align 8
  %_26.i.i.i.i.i.i.i.i = select i1 %_3.not.i.i.i.i.i.i.i.i.i, i64 %self.sink14.i.sroa.gep.i.i.i.val.i.i.i.i.i, i64 %second_set.val7.i.i.i.pre.i.i.i.i.i
  %_29.i.i.i.i.i.i.i.i = select i1 %_3.not.i.i.i.i.i.i.i.i.i, ptr %first_set.val32.i.i.i.i.i.i.i.i, ptr %second_set.val.i.i.i.i.i.i.i.i
  %invariant.gep.i.i.i.i.i.i.i = getelementptr i8, ptr %_29.i.i.i.i.i.i.i.i, i64 -4
  br i1 %_3.i.i.i.i.i.i, label %bb6.i.i.i.i.i.us.preheader.i.i.i.i.i, label %bb6.i.i.i.i.i.preheader.i.i.i.i.i

bb6.i.i.i.i.i.preheader.i.i.i.i.i:                ; preds = %bb6.i.i.i.i.i.lr.ph.i.i.i.i.i
  %_23.i.i4.sink.i.i.i.i.i.i.i.i.i = xor i16 %_23.i.i4.sink.in.i.i.i.i.i.i.i.i.i, -1
  %invariant.op = xor i64 %hash_builder.val1.i.i.i.i.i.i, 8098989879002948979
  br label %bb6.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.us.preheader.i.i.i.i.i:             ; preds = %bb6.i.i.i.i.i.lr.ph.i.i.i.i.i
  %.not14.i.i.i.i.i.i.us.i.i.i.i.i = icmp eq i16 %_23.i.i4.sink.in.i.i.i.i.i.i.i.i.i, -1
  br i1 %.not14.i.i.i.i.i.i.us.i.i.i.i.i, label %bb9.i.i.i.i.i.i.us.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.us.i.i.i.i.i:                     ; preds = %bb6.i.i.i.i.i.us.preheader.i.i.i.i.i, %bb9.i.i.i.i.i.i.us.i.i.i.i.i
  %_1717.i.i.i.i.i.i.us.i.i.i.i.i = phi ptr [ %_17.i.i.i.i.i.i.us.i.i.i.i.i, %bb9.i.i.i.i.i.i.us.i.i.i.i.i ], [ %next_ctrl.i.i5.sink.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.us.preheader.i.i.i.i.i ]
  %18 = load <16 x i8>, ptr %_1717.i.i.i.i.i.i.us.i.i.i.i.i, align 16, !noalias !727
  %_17.i.i.i.i.i.i.us.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_1717.i.i.i.i.i.i.us.i.i.i.i.i, i64 16
  %19 = icmp sgt <16 x i8> %18, splat (i8 -1)
  %20 = bitcast <16 x i1> %19 to i16
  %.not.i.i.i.i.i.i.us.i.i.i.i.i = icmp eq i16 %20, 0
  br i1 %.not.i.i.i.i.i.i.us.i.i.i.i.i, label %bb9.i.i.i.i.i.i.us.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i.i:                          ; preds = %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i", %bb6.i.i.i.i.i.preheader.i.i.i.i.i
  %21 = phi i64 [ %30, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i" ], [ %_4.sink.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.preheader.i.i.i.i.i ]
  %.lcssa11.i.i.i.i30.i.i.i.i.i = phi ptr [ %.lcssa10.i.i.i.i.i.i.i.i.i, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i" ], [ %self1.i1.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.preheader.i.i.i.i.i ]
  %_33.i.i13.i.i.i.i29.i.i.i.i.i = phi i16 [ %_33.i.i.i.i.i.i.i.i.i.i.i, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i" ], [ %_23.i.i4.sink.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.preheader.i.i.i.i.i ]
  %_17.i.i.lcssa16.i.i.i.i28.i.i.i.i.i = phi ptr [ %_17.i.i.lcssa15.i.i.i.i.i.i.i.i.i, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i" ], [ %next_ctrl.i.i5.sink.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.preheader.i.i.i.i.i ]
  %.not14.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %_33.i.i13.i.i.i.i29.i.i.i.i.i, 0
  br i1 %.not14.i.i.i.i.i.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i

bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i:          ; preds = %bb9.i.i.i.i.i.i.i.i.i.i.i
  %_55.i.i.i.i.i.i.i.i.i.i.i = xor i16 %25, -1
  br label %bb8.i.i.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %bb6.i.i.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i.i.i.i.i
  %_1717.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_17.i.i.i.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i.i.i.i.i ], [ %_17.i.i.lcssa16.i.i.i.i28.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i ]
  %22 = phi ptr [ %26, %bb9.i.i.i.i.i.i.i.i.i.i.i ], [ %.lcssa11.i.i.i.i30.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i ]
  %23 = load <16 x i8>, ptr %_1717.i.i.i.i.i.i.i.i.i.i.i, align 16, !noalias !727
  %24 = icmp slt <16 x i8> %23, zeroinitializer
  %25 = bitcast <16 x i1> %24 to i16
  %26 = getelementptr inbounds i8, ptr %22, i64 -64
  %_17.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_1717.i.i.i.i.i.i.i.i.i.i.i, i64 16
  %.not.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %25, -1
  br i1 %.not.i.i.i.i.i.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i.i.i.i.i.i, label %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i
  %_17.i.i.lcssa15.i.i.i.i.i.i.i.i.i = phi ptr [ %_17.i.i.i.i.i.i.i.i.i.i.i, %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i ], [ %_17.i.i.lcssa16.i.i.i.i28.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i ]
  %.lcssa10.i.i.i.i.i.i.i.i.i = phi ptr [ %26, %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i ], [ %.lcssa11.i.i.i.i30.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i ]
  %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_55.i.i.i.i.i.i.i.i.i.i.i, %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i ], [ %_33.i.i13.i.i.i.i29.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i ]
  %27 = add i16 %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i, -1
  %28 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i, i1 true)
  %_24.i.i.i.i.i.i.i.i.i.i.i = zext nneg i16 %28 to i64
  %_33.i.i.i.i.i.i.i.i.i.i.i = and i16 %27, %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i
  %_45.i.i.i.i.i.i.i.i.i.i.i = sub nsw i64 0, %_24.i.i.i.i.i.i.i.i.i.i.i
  %29 = getelementptr inbounds i32, ptr %.lcssa10.i.i.i.i.i.i.i.i.i, i64 %_45.i.i.i.i.i.i.i.i.i.i.i
  %30 = add i64 %21, -1
  %31 = getelementptr inbounds i8, ptr %29, i64 -4
  %.val.i.i.i.i.i.i.i.i.i = load i32, ptr %31, align 4, !noalias !736
  %.pre.i.i.i.i.i.i.i.i.i.i = zext nneg i32 %.val.i.i.i.i.i.i.i.i.i to i64
  %b.i.i.i.i.i.i.i.i.i = or disjoint i64 %.pre.i.i.i.i.i.i.i.i.i.i, 288230376151711744
  %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass = xor i64 %.pre.i.i.i.i.i.i.i.i.i.i, %invariant.op
  %_5.i.i.i3.i.i.i.i.i.i.i = add i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, %13
  %32 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, i64 16)
  %33 = xor i64 %32, %_5.i.i.i3.i.i.i.i.i.i.i
  %_16.i.i.i.i.i.reass.i.i.i.i.i = add i64 %invariant.op31.i.i.i.i.i, %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass
  %_19.i.i.i.i.i.i.i.i.i.i = add i64 %33, %16
  %34 = xor i64 %_16.i.i.i.i.i.reass.i.i.i.i.i, %17
  %35 = tail call noundef i64 @llvm.fshl.i64(i64 %33, i64 %33, i64 21)
  %36 = xor i64 %35, %_19.i.i.i.i.i.i.i.i.i.i
  %37 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.reass.i.i.i.i.i, i64 %_16.i.i.i.i.i.reass.i.i.i.i.i, i64 32)
  %38 = xor i64 %_19.i.i.i.i.i.i.i.i.i.i, %b.i.i.i.i.i.i.i.i.i
  %39 = xor i64 %37, 255
  %_2.i3.i.i.i.i.i.i.i.i.i = add i64 %38, %34
  %_5.i6.i.i.i.i.i.i.i.i.i = add i64 %36, %39
  %40 = tail call noundef i64 @llvm.fshl.i64(i64 %34, i64 %34, i64 13)
  %41 = xor i64 %_2.i3.i.i.i.i.i.i.i.i.i, %40
  %42 = tail call noundef i64 @llvm.fshl.i64(i64 %36, i64 %36, i64 16)
  %43 = xor i64 %_5.i6.i.i.i.i.i.i.i.i.i, %42
  %44 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i.i.i.i.i = add i64 %41, %_5.i6.i.i.i.i.i.i.i.i.i
  %_19.i8.i.i.i.i.i.i.i.i.i = add i64 %43, %44
  %45 = tail call noundef i64 @llvm.fshl.i64(i64 %41, i64 %41, i64 17)
  %46 = xor i64 %_16.i7.i.i.i.i.i.i.i.i.i, %45
  %47 = tail call noundef i64 @llvm.fshl.i64(i64 %43, i64 %43, i64 21)
  %48 = xor i64 %47, %_19.i8.i.i.i.i.i.i.i.i.i
  %49 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i.i.i.i.i = add i64 %46, %_19.i8.i.i.i.i.i.i.i.i.i
  %_33.i.i.i.i.i.i.i.i.i.i = add i64 %48, %49
  %50 = tail call noundef i64 @llvm.fshl.i64(i64 %46, i64 %46, i64 13)
  %51 = xor i64 %50, %_30.i.i.i.i.i.i.i.i.i.i
  %52 = tail call noundef i64 @llvm.fshl.i64(i64 %48, i64 %48, i64 16)
  %53 = xor i64 %52, %_33.i.i.i.i.i.i.i.i.i.i
  %54 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i.i.i.i.i = add i64 %51, %_33.i.i.i.i.i.i.i.i.i.i
  %_47.i.i.i.i.i.i.i.i.i.i = add i64 %53, %54
  %55 = tail call noundef i64 @llvm.fshl.i64(i64 %51, i64 %51, i64 17)
  %56 = xor i64 %55, %_44.i.i.i.i.i.i.i.i.i.i
  %57 = tail call noundef i64 @llvm.fshl.i64(i64 %53, i64 %53, i64 21)
  %58 = xor i64 %57, %_47.i.i.i.i.i.i.i.i.i.i
  %59 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i.i.i.i.i = add i64 %56, %_47.i.i.i.i.i.i.i.i.i.i
  %_61.i.i.i.i.i.i.i.i.i.i = add i64 %58, %59
  %60 = tail call noundef i64 @llvm.fshl.i64(i64 %56, i64 %56, i64 13)
  %61 = xor i64 %60, %_58.i.i.i.i.i.i.i.i.i.i
  %62 = tail call noundef i64 @llvm.fshl.i64(i64 %58, i64 %58, i64 16)
  %63 = xor i64 %62, %_61.i.i.i.i.i.i.i.i.i.i
  %_72.i.i.i.i.i.i.i.i.i.i = add i64 %61, %_61.i.i.i.i.i.i.i.i.i.i
  %64 = tail call noundef i64 @llvm.fshl.i64(i64 %61, i64 %61, i64 17)
  %65 = tail call noundef i64 @llvm.fshl.i64(i64 %63, i64 %63, i64 21)
  %66 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i.i.i.i.i, i64 32)
  %67 = xor i64 %64, %65
  %68 = xor i64 %67, %66
  %_0.i.i.i.i.i.i.i.i.i = xor i64 %68, %_72.i.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i = lshr i64 %_0.i.i.i.i.i.i.i.i.i, 57
  %tag_hash.i.i.i.i.i.i.i.i = trunc nuw nsw i64 %_21.i.i.i.i.i.i.i.i to i8
  %.sroa.0.0.vec.insert.i.i.i.i.i.i.i.i = insertelement <16 x i8> poison, i8 %tag_hash.i.i.i.i.i.i.i.i, i64 0
  %.sroa.0.15.vec.insert.i.i.i.i.i.i.i.i = shufflevector <16 x i8> %.sroa.0.0.vec.insert.i.i.i.i.i.i.i.i, <16 x i8> poison, <16 x i32> zeroinitializer
  br label %bb1.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb20.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i
  %probe_seq.sroa.9.0.i.i.i.i.i.i.i.i = phi i64 [ 0, %bb8.i.i.i.i.i.i.i.i.i ], [ %75, %bb20.i.i.i.i.i.i.i.i ]
  %hash.pn.i.i.i.i.i.i.i = phi i64 [ %_0.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %76, %bb20.i.i.i.i.i.i.i.i ]
  %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i = and i64 %hash.pn.i.i.i.i.i.i.i, %_26.i.i.i.i.i.i.i.i
  %_27.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_29.i.i.i.i.i.i.i.i, i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i
  %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i = load <16 x i8>, ptr %_27.i.i.i.i.i.i.i.i, align 1, !noalias !737
  %69 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i, %.sroa.0.15.vec.insert.i.i.i.i.i.i.i.i
  %70 = bitcast <16 x i1> %69 to i16
  %.not.i.not23.i.i.i.i.i.i.i = icmp eq i16 %70, 0
  br i1 %.not.i.not23.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i

bb10.i.i.i.i.i.i.i.i:                             ; preds = %bb1.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i.i.i
  %iter.sroa.0.0.i24.i.i.i.i.i.i.i = phi i16 [ %_51.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i.i.i ], [ %70, %bb1.i.i.i.i.i.i.i.i ]
  %71 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %iter.sroa.0.0.i24.i.i.i.i.i.i.i, i1 true)
  %_42.i.i.i.i.i.i.i.i = zext nneg i16 %71 to i64
  %_13.i.i.i.i.i.i.i.i = add i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i, %_42.i.i.i.i.i.i.i.i
  %index5.i.i.i.i.i.i.i.i = and i64 %_13.i.i.i.i.i.i.i.i, %_26.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i = sub nsw i64 0, %index5.i.i.i.i.i.i.i.i
  %gep.i.i.i.i.i.i.i = getelementptr i32, ptr %invariant.gep.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i.i.i
  %.val.i.i.i.i.i.i.i.i = load i32, ptr %gep.i.i.i.i.i.i.i, align 4, !range !747, !noalias !748, !noundef !10
  %_0.i.i.i.i.i.i.i.i.i.i.i = icmp eq i32 %.val.i.i.i.i.i.i.i.i.i, %.val.i.i.i.i.i.i.i.i
  br i1 %_0.i.i.i.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i, label %bb17.i.i.i.i.i.i.i.i, !prof !180

bb11.i.i.i.i.i.i.i.i:                             ; preds = %bb17.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i
  %72 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i, splat (i8 -1)
  %73 = bitcast <16 x i1> %72 to i16
  %b8.not.i.i.i.i.i.i.i.i = icmp eq i16 %73, 0
  br i1 %b8.not.i.i.i.i.i.i.i.i, label %bb20.i.i.i.i.i.i.i.i, label %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i", !prof !320

bb17.i.i.i.i.i.i.i.i:                             ; preds = %bb10.i.i.i.i.i.i.i.i
  %74 = add i16 %iter.sroa.0.0.i24.i.i.i.i.i.i.i, -1
  %_51.i.i.i.i.i.i.i.i = and i16 %74, %iter.sroa.0.0.i24.i.i.i.i.i.i.i
  %.not.i.not.i.i.i.i.i.i.i = icmp eq i16 %_51.i.i.i.i.i.i.i.i, 0
  br i1 %.not.i.not.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i

bb20.i.i.i.i.i.i.i.i:                             ; preds = %bb11.i.i.i.i.i.i.i.i
  %75 = add i64 %probe_seq.sroa.9.0.i.i.i.i.i.i.i.i, 16
  %76 = add i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i, %75
  br label %bb1.i.i.i.i.i.i.i.i

"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i": ; preds = %bb11.i.i.i.i.i.i.i.i
  %_11.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %30, 0
  br i1 %_11.i.i.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i.i:                              ; preds = %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i", %bb9.i.i.i.i.i.i.us.i.i.i.i.i, %bb10.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.us.preheader.i.i.i.i.i, %bb4.i4.i.i.i.i.i.i.i
  %_0.sroa.0.0.i10.i.i.i.i.i.i.i.i = phi i32 [ 1114112, %bb4.i4.i.i.i.i.i.i.i ], [ 1114112, %bb6.i.i.i.i.i.us.preheader.i.i.i.i.i ], [ %.val.i.i.i.i.i.i.i.i.i, %bb10.i.i.i.i.i.i.i.i ], [ 1114112, %bb9.i.i.i.i.i.i.us.i.i.i.i.i ], [ 1114112, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i" ]
  %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %second_set.val7.i.i.i.pre.i.i.i.i.i, 0
  br i1 %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i:                  ; preds = %bb7.i.i.i.i.i.i.i.i
  %_10.i.i.i.i.i.i.i.i.i.i.i.i.i.i = shl i64 %second_set.val7.i.i.i.pre.i.i.i.i.i, 2
  %_32.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %_10.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 19
  %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = and i64 %_32.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -16
  %rhs5.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %second_set.val7.i.i.i.pre.i.i.i.i.i, 17
  %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_37.1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i)
  %77 = icmp ne ptr %second_set.val.i.i.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %77)
  %_4.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i:               ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %ptr.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %second_set.val.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33
  br label %bb8.i.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i:                              ; preds = %bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %second_set.i.i.i.i.i.i.i.i), !noalias !714
  %first_set.val5.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i, align 8, !noalias !714, !noundef !10
  %_4.i.i.i.i.i.i11.i.i.i.i.i.i.i.i = icmp eq i64 %first_set.val5.i.i.i.i.i.i.i.i, 0
  br i1 %_4.i.i.i.i.i.i11.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i12.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i12.i.i.i.i.i.i.i.i:                ; preds = %bb8.i.i.i.i.i.i.i.i
  %_10.i.i.i.i.i.i13.i.i.i.i.i.i.i.i = shl i64 %first_set.val5.i.i.i.i.i.i.i.i, 2
  %_32.0.i.i.i.i.i.i.i14.i.i.i.i.i.i.i.i = add i64 %_10.i.i.i.i.i.i13.i.i.i.i.i.i.i.i, 19
  %ctrl_offset.i.i.i.i.i.i.i15.i.i.i.i.i.i.i.i = and i64 %_32.0.i.i.i.i.i.i.i14.i.i.i.i.i.i.i.i, -16
  %rhs5.i.i.i.i.i.i.i16.i.i.i.i.i.i.i.i = add i64 %first_set.val5.i.i.i.i.i.i.i.i, 17
  %_37.0.i.i.i.i.i.i.i17.i.i.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i.i16.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i15.i.i.i.i.i.i.i.i
  %_37.1.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i.i17.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i15.i.i.i.i.i.i.i.i
  %_19.i.i.i.i.i.i.i19.i.i.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i.i17.i.i.i.i.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i19.i.i.i.i.i.i.i.i)
  %78 = icmp ne ptr %first_set.val32.i.i.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %78)
  %_4.not.i.i.i.i.i.i.i20.i.i.i.i.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i.i.i.i17.i.i.i.i.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i.i.i.i20.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i21.i.i.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i21.i.i.i.i.i.i.i.i:             ; preds = %bb1.i.i.i.i.i.i12.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i22.i.i.i.i.i.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i.i15.i.i.i.i.i.i.i.i
  %ptr.i.i.i.i.i.i23.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %first_set.val32.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i22.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i23.i.i.i.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i.i17.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33
  br label %_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i

_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i: ; preds = %bb1.i2.i.i.i.i.i.i21.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i12.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %first_set.i.i.i.i.i.i.i.i), !noalias !714
  %.not.i.i.i.i.i.i.i.i = icmp eq i32 %_0.sroa.0.0.i10.i.i.i.i.i.i.i.i, 1114112
  br i1 %.not.i.i.i.i.i.i.i.i, label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i", label %bb3.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i
  %79 = add nsw i32 %_0.sroa.0.0.i10.i.i.i.i.i.i.i.i, -97
  %or.cond.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %79, 26
  br i1 %or.cond.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i.i
  %80 = add nsw i32 %_0.sroa.0.0.i10.i.i.i.i.i.i.i.i, -65
  %or.cond1.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %80, 26
  %81 = add nsw i32 %_0.sroa.0.0.i10.i.i.i.i.i.i.i.i, -38
  %spec.select.i.i.i.i.i.i.i.i.i.i = select i1 %or.cond1.i.i.i.i.i.i.i.i.i.i, i32 %81, i32 0
  br label %bb4.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i.i
  %82 = add nsw i32 %_0.sroa.0.0.i10.i.i.i.i.i.i.i.i, -96
  br label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i
  %_0.sroa.3.0.i.i.ph.i.i.i.i.i.i = phi i32 [ %82, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %spec.select.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i ]
  %_4.0.i.i.i.i.i.i.i = add i32 %_0.sroa.3.0.i.i.ph.i.i.i.i.i.i, %acc.sroa.0.0.i.i.i.i.i
  br label %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i"

"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i": ; preds = %bb4.i.i.i.i.i.i, %_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i
  %_0.sroa.0.0.i.i.i.i.i.i = phi i32 [ %_4.0.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %acc.sroa.0.0.i.i.i.i.i, %_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E.exit.i.i.i.i.i.i.i ]
  %_27.i.i.i.i.i = add nuw i64 %i.sroa.0.0.i.i.i.i.i, 1
  %_28.i.i.i.i.i = icmp eq i64 %_27.i.i.i.i.i, %data.val1
  br i1 %_28.i.i.i.i.i, label %bb2, label %bb10.i.i.i.i.i

funclet_bb5:                                      ; preds = %bb2.i.i.i.i.i.i.i, %funclet_bb20.i.i.i.i.i.i.i.i, %"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E.exit.i.i.i.i.i.i.i", %bb4.i.i.i.i.i.i.i.i, %funclet_bb11.i.i.i.i.i.i.i.i
  %cleanuppad = cleanuppad within none []
; call core::ptr::drop_in_place<aoc2022::solver::day03::Rucksacks>
  call fastcc void @"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E"(ptr noalias noundef align 8 dereferenceable(24) %data) #34 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i", %start
  %_0.sroa.0.0.i.i.i.i.i = phi i32 [ 0, %start ], [ %_0.sroa.0.0.i.i.i.i.i.i, %"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h60d9d90b5a9e4fb4E.exit.i.i.i.i.i" ]
  %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i7 = getelementptr inbounds nuw i8, ptr %first_set.i.i.i.i.i.i.i.i6, i64 8
  %self.sink14.i.sroa.gep84.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %second_set.i.i.i.i.i.i.i.i5, i64 8
  %self.sink14.i23.sroa.gep85.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %third_set.i.i.i.i.i.i.i.i, i64 8
  %83 = getelementptr inbounds nuw i8, ptr %first_set.i.i.i.i.i.i.i.i6, i64 24
  %84 = getelementptr inbounds nuw i8, ptr %second_set.i.i.i.i.i.i.i.i5, i64 24
  %_7.sroa.4.0.set.sroa_idx.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %set.i.i.i.i.i.i.i.i.i.i, i64 32
  %_7.sroa.5.0.set.sroa_idx.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %set.i.i.i.i.i.i.i.i.i.i, i64 40
  %first_second.sroa.6.0.set.i.i.sroa_idx.phi.trans.insert.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %set.i.i.i.i.i.i.i.i.i.i, i64 8
  %first_second.sroa.8135.0.set.i.i.sroa_idx.phi.trans.insert.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %set.i.i.i.i.i.i.i.i.i.i, i64 24
  %85 = getelementptr inbounds nuw i8, ptr %third_set.i.i.i.i.i.i.i.i, i64 24
  %third_set.sroa.gep130.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %third_set.i.i.i.i.i.i.i.i, i64 32
  %third_set.sroa.gep127.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %third_set.i.i.i.i.i.i.i.i, i64 40
  br label %bb1.i.i.i.i.i.outer

bb1.i.i.i.i.i.outer:                              ; preds = %bb4.i.i.i.i.i.i52, %bb2
  %_4.sroa.0.0.i.i.i.i.ph = phi ptr [ %86, %bb4.i.i.i.i.i.i52 ], [ %data.val, %bb2 ]
  %self.1.i.i.i.i.i.i.ph = phi i64 [ %87, %bb4.i.i.i.i.i.i52 ], [ %data.val1, %bb2 ]
  %accum.sroa.0.0.i.i.i.i.i.ph = phi i32 [ %_4.0.i.i.i.i.i.i.i53, %bb4.i.i.i.i.i.i52 ], [ 0, %bb2 ]
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.i.i.backedge, %bb1.i.i.i.i.i.outer
  %_4.sroa.0.0.i.i.i.i = phi ptr [ %_4.sroa.0.0.i.i.i.i.ph, %bb1.i.i.i.i.i.outer ], [ %86, %bb1.i.i.i.i.i.backedge ]
  %self.1.i.i.i.i.i.i = phi i64 [ %self.1.i.i.i.i.i.i.ph, %bb1.i.i.i.i.i.outer ], [ %87, %bb1.i.i.i.i.i.backedge ]
  %_2.i.i.i.i.i.i = icmp eq i64 %self.1.i.i.i.i.i.i, 0
  br i1 %_2.i.i.i.i.i.i, label %bb3, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.i.i
  %..i.i.i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %self.1.i.i.i.i.i.i, i64 3)
  %86 = getelementptr inbounds nuw %"alloc::string::String", ptr %_4.sroa.0.0.i.i.i.i, i64 %..i.i.i.i.i.i.i
  %87 = sub nuw nsw i64 %self.1.i.i.i.i.i.i, %..i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !751)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !754)
  %_3.i.i.i.i.i.i.i = icmp ugt i64 %self.1.i.i.i.i.i.i, 2
  br i1 %_3.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.backedge

bb2.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i
  %88 = getelementptr i8, ptr %_4.sroa.0.0.i.i.i.i, i64 8
  %group.0.val.i.i.i.i.i.i.i = load ptr, ptr %88, align 8, !alias.scope !757, !noalias !758, !nonnull !10, !noundef !10
  %89 = getelementptr i8, ptr %_4.sroa.0.0.i.i.i.i, i64 16
  %group.0.val5.i.i.i.i.i.i.i = load i64, ptr %89, align 8, !alias.scope !757, !noalias !758, !noundef !10
  %90 = getelementptr i8, ptr %_4.sroa.0.0.i.i.i.i, i64 32
  %_10.val.i.i.i.i.i.i.i = load ptr, ptr %90, align 8, !alias.scope !757, !noalias !758, !nonnull !10, !noundef !10
  %91 = getelementptr i8, ptr %_4.sroa.0.0.i.i.i.i, i64 40
  %_10.val4.i.i.i.i.i.i.i = load i64, ptr %91, align 8, !alias.scope !757, !noalias !758, !noundef !10
  %92 = getelementptr i8, ptr %_4.sroa.0.0.i.i.i.i, i64 56
  %_13.val.i.i.i.i.i.i.i = load ptr, ptr %92, align 8, !alias.scope !757, !noalias !758, !nonnull !10, !noundef !10
  %93 = getelementptr i8, ptr %_4.sroa.0.0.i.i.i.i, i64 64
  %_13.val3.i.i.i.i.i.i.i = load i64, ptr %93, align 8, !alias.scope !757, !noalias !758, !noundef !10
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %first_set.i.i.i.i.i.i.i.i6), !noalias !767
  %_7.i.i.i.i.i.i.i.i.i8 = getelementptr inbounds nuw i8, ptr %group.0.val.i.i.i.i.i.i.i, i64 %group.0.val5.i.i.i.i.i.i.i
; invoke core::iter::traits::iterator::Iterator::collect
  invoke fastcc void @_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E(ptr noalias noundef align 8 captures(address) dereferenceable(48) %first_set.i.i.i.i.i.i.i.i6, ptr noundef nonnull readonly align 1 %group.0.val.i.i.i.i.i.i.i, ptr noundef readonly %_7.i.i.i.i.i.i.i.i.i8) #32
          to label %.noexc61 unwind label %funclet_bb5

.noexc61:                                         ; preds = %bb2.i.i.i.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %second_set.i.i.i.i.i.i.i.i5), !noalias !767
  %_7.i16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i.i.i.i, i64 %_10.val4.i.i.i.i.i.i.i
; invoke core::iter::traits::iterator::Iterator::collect
  invoke fastcc void @_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E(ptr noalias noundef align 8 captures(address) dereferenceable(48) %second_set.i.i.i.i.i.i.i.i5, ptr noundef nonnull readonly align 1 %_10.val.i.i.i.i.i.i.i, ptr noundef readonly %_7.i16.i.i.i.i.i.i.i.i)
          to label %bb5.i.i.i.i.i.i.i.i unwind label %funclet_bb20.i.i.i.i.i.i.i.i, !noalias !772

funclet_bb20.i.i.i.i.i.i.i.i:                     ; preds = %funclet_bb19.i.i.i.i.i.i.i.i, %.noexc61
  %cleanuppad.i.i.i.i.i.i.i.i9 = cleanuppad within none []
  %first_set.val.i.i.i.i.i.i.i.i10 = load ptr, ptr %first_set.i.i.i.i.i.i.i.i6, align 8, !noalias !767
  %first_set.val4.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i7, align 8, !noalias !767, !noundef !10
; call core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
  call fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %first_set.val.i.i.i.i.i.i.i.i10, i64 %first_set.val4.i.i.i.i.i.i.i.i) #34 [ "funclet"(token %cleanuppad.i.i.i.i.i.i.i.i9) ], !noalias !773
  cleanupret from %cleanuppad.i.i.i.i.i.i.i.i9 unwind label %funclet_bb5

funclet_bb19.i.i.i.i.i.i.i.i:                     ; preds = %funclet_bb18.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i
  %cleanuppad1.i.i.i.i.i.i.i.i = cleanuppad within none []
  %second_set.val.i.i.i.i.i.i.i.i11 = load ptr, ptr %second_set.i.i.i.i.i.i.i.i5, align 8, !noalias !767
  %second_set.val5.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep84.i.i.i.i.i.i.i.i, align 8, !noalias !767, !noundef !10
; call core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
  call fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %second_set.val.i.i.i.i.i.i.i.i11, i64 %second_set.val5.i.i.i.i.i.i.i.i) #34 [ "funclet"(token %cleanuppad1.i.i.i.i.i.i.i.i) ], !noalias !773
  cleanupret from %cleanuppad1.i.i.i.i.i.i.i.i unwind label %funclet_bb20.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i:                              ; preds = %.noexc61
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %third_set.i.i.i.i.i.i.i.i), !noalias !767
  %_7.i17.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_13.val.i.i.i.i.i.i.i, i64 %_13.val3.i.i.i.i.i.i.i
; invoke core::iter::traits::iterator::Iterator::collect
  invoke fastcc void @_ZN4core4iter6traits8iterator8Iterator7collect17h232e8279fee224e0E(ptr noalias noundef align 8 captures(address) dereferenceable(48) %third_set.i.i.i.i.i.i.i.i, ptr noundef nonnull readonly align 1 %_13.val.i.i.i.i.i.i.i, ptr noundef readonly %_7.i17.i.i.i.i.i.i.i.i)
          to label %bb6.i.i.i.i.i.i.i.i unwind label %funclet_bb19.i.i.i.i.i.i.i.i, !noalias !773

bb6.i.i.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !774)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !777)
  %_4.i.i.i.i.i.i.i.i.i12 = load i64, ptr %83, align 8, !alias.scope !774, !noalias !779, !noundef !10
  %_5.i.i.i.i.i.i.i.i.i13 = load i64, ptr %84, align 8, !alias.scope !777, !noalias !781, !noundef !10
  %_3.not.i.i.i.i.i.i.i.i.i14 = icmp ugt i64 %_4.i.i.i.i.i.i.i.i.i12, %_5.i.i.i.i.i.i.i.i.i13
  %_4.sink.i.i.i.i.i.i.i.i.i15 = tail call i64 @llvm.umin.i64(i64 %_4.i.i.i.i.i.i.i.i.i12, i64 %_5.i.i.i.i.i.i.i.i.i13)
  %second_set.val104.i.i.i.i.i.i.i.i = load ptr, ptr %second_set.i.i.i.i.i.i.i.i5, align 8, !noalias !767
  %first_set.val105.i.i.i.i.i.i.i.i = load ptr, ptr %first_set.i.i.i.i.i.i.i.i6, align 8, !noalias !767
  %self1.i1.i.i.i.i.i.i.i.i.i16 = select i1 %_3.not.i.i.i.i.i.i.i.i.i14, ptr %second_set.val104.i.i.i.i.i.i.i.i, ptr %first_set.val105.i.i.i.i.i.i.i.i
  %_23.i.i4.sink.in.in.in.i.i.i.i.i.i.i.i.i17 = load <16 x i8>, ptr %self1.i1.i.i.i.i.i.i.i.i.i16, align 16, !noalias !782
  %_23.i.i4.sink.in.in.i.i.i.i.i.i.i.i.i18 = icmp slt <16 x i8> %_23.i.i4.sink.in.in.in.i.i.i.i.i.i.i.i.i17, zeroinitializer
  %_23.i.i4.sink.in.i.i.i.i.i.i.i.i.i19 = bitcast <16 x i1> %_23.i.i4.sink.in.in.i.i.i.i.i.i.i.i.i18 to i16
  %_23.i.i4.sink.i.i.i.i.i.i.i.i.i20 = xor i16 %_23.i.i4.sink.in.i.i.i.i.i.i.i.i.i19, -1
  %next_ctrl.i.i5.sink.i.i.i.i.i.i.i.i.i21 = getelementptr inbounds nuw i8, ptr %self1.i1.i.i.i.i.i.i.i.i.i16, i64 16
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %set.i.i.i.i.i.i.i.i.i.i), !noalias !783
; invoke std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL{{tls.shim}}
  %_3.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i = invoke noundef nonnull align 8 ptr @"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$51__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$17hed5e461344c1f9f9E"()
          to label %_3.i.i.i.i.i.i.i.noexc.i.i.i.i.i.i.i.i unwind label %funclet_bb18.i.i.i.i.i.i.i.i, !noalias !773

funclet_bb18.i.i.i.i.i.i.i.i:                     ; preds = %funclet_bb1.i.i.i.i.i.i.i.i.i.i, %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i.i.i.i.i.i.i.i.i.i", %bb6.i.i.i.i.i.i.i.i
  %cleanuppad2.i.i.i.i.i.i.i.i = cleanuppad within none []
  %third_set.val.i.i.i.i.i.i.i.i = load ptr, ptr %third_set.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %third_set.val6.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i23.sroa.gep85.i.i.i.i.i.i.i.i, align 8, !noalias !767, !noundef !10
; call core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
  call fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %third_set.val.i.i.i.i.i.i.i.i, i64 %third_set.val6.i.i.i.i.i.i.i.i) #34 [ "funclet"(token %cleanuppad2.i.i.i.i.i.i.i.i) ], !noalias !773
  cleanupret from %cleanuppad2.i.i.i.i.i.i.i.i unwind label %funclet_bb19.i.i.i.i.i.i.i.i

_3.i.i.i.i.i.i.i.noexc.i.i.i.i.i.i.i.i:           ; preds = %bb6.i.i.i.i.i.i.i.i
  %_12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i, i64 16
  %94 = load i8, ptr %_12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, align 8, !range !8, !noalias !790, !noundef !10
  %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = trunc nuw i8 %94 to i1
  br i1 %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i.i.i.i.i.i.i.i.i.i", !prof !180

start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %_3.i.i.i.i.i.i.i.noexc.i.i.i.i.i.i.i.i
  %_9.i.pre.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_3.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i, align 8, !noalias !799
  %.phi.trans.insert.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i, i64 8
  %_10.i.pre.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !799
  br label %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i"

"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i.i.i.i.i.i.i.i.i.i": ; preds = %_3.i.i.i.i.i.i.i.noexc.i.i.i.i.i.i.i.i
; invoke std::sys::random::hashmap_random_keys
  %95 = invoke { i64, i64 } @_ZN3std3sys6random19hashmap_random_keys17hc3f03c6d163b2da2E()
          to label %.noexc.i.i.i.i.i.i.i.i unwind label %funclet_bb18.i.i.i.i.i.i.i.i, !noalias !773

.noexc.i.i.i.i.i.i.i.i:                           ; preds = %"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE.exit.i.i.i.i.i.i.i.i.i.i.i.i.i"
  %96 = extractvalue { i64, i64 } %95, 0
  %97 = extractvalue { i64, i64 } %95, 1
  %98 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i, i64 8
  store i64 %97, ptr %98, align 8, !noalias !800
  store i8 1, ptr %_12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !800
  br label %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i"

"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i": ; preds = %.noexc.i.i.i.i.i.i.i.i, %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i
  %hasher.1.pre-phi.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_10.i.pre.i.i.i.i.i.i.i.i.i.i.i.i.i, %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %97, %.noexc.i.i.i.i.i.i.i.i ]
  %_9.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_9.i.pre.i.i.i.i.i.i.i.i.i.i.i.i.i, %start._ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE.exit_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %96, %.noexc.i.i.i.i.i.i.i.i ]
  %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i22 = add i64 %_9.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 1
  store i64 %_4.i.i.i.i.i.i.i.i.i.i.i.i.i.i22, ptr %_3.i.i.i.i.i.i.i18.i.i.i.i.i.i.i.i, align 8, !noalias !799
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %set.i.i.i.i.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.44ffa63e8e95c400711a21744c5ea708.0, i64 32, i1 false), !noalias !783
  store i64 %_9.i.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_7.sroa.4.0.set.sroa_idx.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !783
  store i64 %hasher.1.pre-phi.i.i.i.i.i.i.i.i.i.i, ptr %_7.sroa.5.0.set.sroa_idx.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !783
  tail call void @llvm.experimental.noalias.scope.decl(metadata !803)
  %other.sink.i.sroa.sel119.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v = select i1 %_3.not.i.i.i.i.i.i.i.i.i14, ptr %first_set.i.i.i.i.i.i.i.i6, ptr %second_set.i.i.i.i.i.i.i.i5
  %other.sink.i.sroa.sel119.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel = getelementptr inbounds nuw i8, ptr %other.sink.i.sroa.sel119.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v, i64 32
  %other.sink.i.sroa.sel122.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v = select i1 %_3.not.i.i.i.i.i.i.i.i.i14, ptr %first_set.i.i.i.i.i.i.i.i6, ptr %second_set.i.i.i.i.i.i.i.i5
  %other.sink.i.sroa.sel122.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel = getelementptr inbounds nuw i8, ptr %other.sink.i.sroa.sel122.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel.v, i64 40
  %99 = or i64 %_5.i.i.i.i.i.i.i.i.i13, %_4.i.i.i.i.i.i.i.i.i12
  %_3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %99, 0
  %hash_builder.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %other.sink.i.sroa.sel119.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel, align 8, !alias.scope !803, !noalias !806
  %hash_builder.val1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %other.sink.i.sroa.sel122.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.i.sroa.sel.v.sroa.sel.v.sroa.sel, align 8, !alias.scope !803, !noalias !806
  %100 = xor i64 %hash_builder.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 8317987319222330741
  %101 = xor i64 %hash_builder.val1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 7237128888997146477
  %102 = xor i64 %hash_builder.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 7816392313619706465
  %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %101, %100
  %103 = tail call i64 @llvm.fshl.i64(i64 %101, i64 %101, i64 13)
  %104 = xor i64 %103, %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %105 = tail call i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %invariant.op4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %104, %102
  %106 = tail call i64 @llvm.fshl.i64(i64 %104, i64 %104, i64 17)
  %self.sink14.i.sroa.gep.val.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep.i.i.i.i.i.i.i.i7, align 8, !noalias !767
  %self.sink14.i.sroa.gep84.val.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i.sroa.gep84.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %_26.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = select i1 %_3.not.i.i.i.i.i.i.i.i.i14, i64 %self.sink14.i.sroa.gep.val.i.i.i.i.i.i.i.i, i64 %self.sink14.i.sroa.gep84.val.i.i.i.i.i.i.i.i
  %_29.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = select i1 %_3.not.i.i.i.i.i.i.i.i.i14, ptr %first_set.val105.i.i.i.i.i.i.i.i, ptr %second_set.val104.i.i.i.i.i.i.i.i
  %invariant.gep.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr i8, ptr %_29.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 -4
  br i1 %_3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader

bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader: ; preds = %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i"
  %invariant.op224 = xor i64 %hash_builder.val1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 8098989879002948979
  br label %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i", %bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_23.lcssa29.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.lcssa28.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %next_ctrl.i.i5.sink.i.i.i.i.i.i.i.i.i21, %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i" ]
  %_4024.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_40.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_23.i.i4.sink.i.i.i.i.i.i.i.i.i20, %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i" ]
  %n.sroa.0.0.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %111, %bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_4.sink.i.i.i.i.i.i.i.i.i15, %"_ZN73_$LT$std..hash..random..RandomState$u20$as$u20$core..default..Default$GT$7default17h1558c7956153b486E.exit.i.i.i.i.i.i.i.i.i.i" ]
  %.not18.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %_4024.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not18.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_14.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %n.sroa.0.0.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_14.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i.i.i, label %bb12.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb12.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:    ; preds = %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_2321.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_23.lcssa29.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %107 = load <16 x i8>, ptr %_2321.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, align 16, !noalias !827
  %108 = icmp slt <16 x i8> %107, zeroinitializer
  %109 = bitcast <16 x i1> %108 to i16
  %_23.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2321.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 16
  %.not.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %109, -1
  br i1 %.not.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb12.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.bb11_crit_edge.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.bb11_crit_edge.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %bb12.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_62.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = xor i16 %109, -1
  br label %bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb11.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:    ; preds = %bb1.bb11_crit_edge.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_23.lcssa28.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.bb11_crit_edge.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_23.lcssa29.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %self3.lcssa.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_62.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.bb11_crit_edge.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_4024.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %110 = add i16 %self3.lcssa.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -1
  %_40.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = and i16 %110, %self3.lcssa.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %111 = add i64 %n.sroa.0.0.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -1
  br label %bb1.outer.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:  ; preds = %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader, %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i"
  %_23.lcssa29.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.lcssa28.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i" ], [ %next_ctrl.i.i5.sink.i.i.i.i.i.i.i.i.i21, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader ]
  %.lcssa2227.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %.lcssa2226.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i" ], [ %self1.i1.i.i.i.i.i.i.i.i.i16, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader ]
  %_4024.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_40.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i" ], [ %_23.i.i4.sink.i.i.i.i.i.i.i.i.i20, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader ]
  %n.sroa.0.0.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %161, %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i" ], [ %_4.sink.i.i.i.i.i.i.i.i.i15, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.preheader ]
  %.not18.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %_4024.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not18.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb12.lr.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb12.lr.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_14.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %n.sroa.0.0.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_14.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb9.loopexit108.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.bb11_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_62.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = xor i16 %165, -1
  br label %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:       ; preds = %bb1.bb11_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_23.lcssa28.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.bb11_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_23.lcssa29.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %.lcssa2226.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %166, %bb1.bb11_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %.lcssa2227.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_62.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.bb11_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_4024.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %112 = add i16 %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -1
  %113 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i1 true)
  %_31.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = zext nneg i16 %113 to i64
  %_40.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = and i16 %112, %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_52.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = sub nsw i64 0, %_31.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %114 = getelementptr inbounds i32, ptr %.lcssa2226.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_52.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %115 = getelementptr i8, ptr %114, i64 -4
  %.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load i32, ptr %115, align 4, !noalias !836
  %.pre.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = zext nneg i32 %.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i to i64
  %b.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = or disjoint i64 %.pre.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 288230376151711744
  %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass = xor i64 %.pre.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %invariant.op224
  %_5.i.i.i3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, %102
  %116 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, i64 16)
  %117 = xor i64 %116, %_5.i.i.i3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_16.i.i.i.i.i.i.reass.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %invariant.op4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass
  %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %117, %105
  %118 = xor i64 %_16.i.i.i.i.i.i.reass.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %106
  %119 = tail call noundef i64 @llvm.fshl.i64(i64 %117, i64 %117, i64 21)
  %120 = xor i64 %119, %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %121 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.i.reass.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_16.i.i.i.i.i.i.reass.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %122 = xor i64 %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %b.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %123 = xor i64 %121, 255
  %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %122, %118
  %_5.i6.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %120, %123
  %124 = tail call noundef i64 @llvm.fshl.i64(i64 %118, i64 %118, i64 13)
  %125 = xor i64 %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %124
  %126 = tail call noundef i64 @llvm.fshl.i64(i64 %120, i64 %120, i64 16)
  %127 = xor i64 %_5.i6.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %126
  %128 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %125, %_5.i6.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_19.i8.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %127, %128
  %129 = tail call noundef i64 @llvm.fshl.i64(i64 %125, i64 %125, i64 17)
  %130 = xor i64 %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %129
  %131 = tail call noundef i64 @llvm.fshl.i64(i64 %127, i64 %127, i64 21)
  %132 = xor i64 %131, %_19.i8.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %133 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %130, %_19.i8.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_33.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %132, %133
  %134 = tail call noundef i64 @llvm.fshl.i64(i64 %130, i64 %130, i64 13)
  %135 = xor i64 %134, %_30.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %136 = tail call noundef i64 @llvm.fshl.i64(i64 %132, i64 %132, i64 16)
  %137 = xor i64 %136, %_33.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %138 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %135, %_33.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_47.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %137, %138
  %139 = tail call noundef i64 @llvm.fshl.i64(i64 %135, i64 %135, i64 17)
  %140 = xor i64 %139, %_44.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %141 = tail call noundef i64 @llvm.fshl.i64(i64 %137, i64 %137, i64 21)
  %142 = xor i64 %141, %_47.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %143 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %140, %_47.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_61.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %142, %143
  %144 = tail call noundef i64 @llvm.fshl.i64(i64 %140, i64 %140, i64 13)
  %145 = xor i64 %144, %_58.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %146 = tail call noundef i64 @llvm.fshl.i64(i64 %142, i64 %142, i64 16)
  %147 = xor i64 %146, %_61.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_72.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %145, %_61.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %148 = tail call noundef i64 @llvm.fshl.i64(i64 %145, i64 %145, i64 17)
  %149 = tail call noundef i64 @llvm.fshl.i64(i64 %147, i64 %147, i64 21)
  %150 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %151 = xor i64 %148, %149
  %152 = xor i64 %151, %150
  %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = xor i64 %152, %_72.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = lshr i64 %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 57
  %tag_hash.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = trunc nuw nsw i64 %_21.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i to i8
  %.sroa.0.0.vec.insert.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = insertelement <16 x i8> poison, i8 %tag_hash.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 0
  %.sroa.0.15.vec.insert.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = shufflevector <16 x i8> %.sroa.0.0.vec.insert.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, <16 x i8> poison, <16 x i32> zeroinitializer
  br label %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:    ; preds = %bb20.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %probe_seq.sroa.9.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ 0, %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %159, %bb20.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %hash.pn.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %160, %bb20.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = and i64 %hash.pn.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %_26.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_27.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_29.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load <16 x i8>, ptr %_27.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !837
  %153 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %.sroa.0.15.vec.insert.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %154 = bitcast <16 x i1> %153 to i16
  %.not.i.not23.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %154, 0
  br i1 %.not.i.not23.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb10.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:   ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_51.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %154, %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %155 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i1 true)
  %_42.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = zext nneg i16 %155 to i64
  %_13.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %_42.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %index5.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = and i64 %_13.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %_26.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = sub nsw i64 0, %index5.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %gep.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr i32, ptr %invariant.gep.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = load i32, ptr %gep.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, align 4, !range !747, !noalias !847, !noundef !10
  %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i32 %.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, !prof !180

bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:   ; preds = %bb17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %156 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, splat (i8 -1)
  %157 = bitcast <16 x i1> %156 to i16
  %b8.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %157, 0
  br i1 %b8.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb20.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i", !prof !320

bb17.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:   ; preds = %bb10.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %158 = add i16 %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -1
  %_51.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = and i16 %158, %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %.not.i.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %_51.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not.i.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb20.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:   ; preds = %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %159 = add i64 %probe_seq.sroa.9.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, 16
  %160 = add i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %159
  br label %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:  ; preds = %bb10.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
; invoke hashbrown::map::HashMap<K,V,S,A>::insert
  invoke fastcc void @"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE"(ptr noalias noundef nonnull align 8 dereferenceable(48) %set.i.i.i.i.i.i.i.i.i.i, i32 noundef range(i32 0, 1114112) %.val.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i)
          to label %"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i" unwind label %funclet_bb1.i.i.i.i.i.i.i.i.i.i

"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h38e2f5c22c6c3945E.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i": ; preds = %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %161 = add i64 %n.sroa.0.0.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, -1
  br label %bb1.outer.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:       ; preds = %bb12.lr.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_2321.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %_23.lcssa29.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.lr.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %162 = phi ptr [ %166, %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ], [ %.lcssa2227.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.lr.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %163 = load <16 x i8>, ptr %_2321.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, align 16, !noalias !827
  %164 = icmp slt <16 x i8> %163, zeroinitializer
  %165 = bitcast <16 x i1> %164 to i16
  %166 = getelementptr inbounds i8, ptr %162, i64 -64
  %_23.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2321.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 16
  %.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %165, -1
  br i1 %.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.bb11_crit_edge.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

funclet_bb1.i.i.i.i.i.i.i.i.i.i:                  ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %cleanuppad2.i.i.i.i.i.i.i.i.i.i = cleanuppad within none []
  %set.val.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %set.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !783
  %set.val3.i.i.i.i.i.i.i.i.i.i = load i64, ptr %first_second.sroa.6.0.set.i.i.sroa_idx.phi.trans.insert.i.i.i.i.i.i.i.i, align 8, !noalias !783, !noundef !10
; call core::ptr::drop_in_place<std::collections::hash::set::HashSet<char>>
  call fastcc void @"_ZN4core3ptr69drop_in_place$LT$std..collections..hash..set..HashSet$LT$char$GT$$GT$17haa0807705d9da249E"(ptr %set.val.i.i.i.i.i.i.i.i.i.i, i64 %set.val3.i.i.i.i.i.i.i.i.i.i) #34 [ "funclet"(token %cleanuppad2.i.i.i.i.i.i.i.i.i.i) ], !noalias !850
  cleanupret from %cleanuppad2.i.i.i.i.i.i.i.i.i.i unwind label %funclet_bb18.i.i.i.i.i.i.i.i

bb9.loopexit108.i.i.i.i.i.i.i.i:                  ; preds = %bb12.lr.ph.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i
  %first_second.sroa.0.0.copyload.pre.i.i.i.i.i.i.i.i = load i64, ptr %set.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !851
  %first_second.sroa.6.0.copyload.pre.i.i.i.i.i.i.i.i = load i64, ptr %first_second.sroa.6.0.set.i.i.sroa_idx.phi.trans.insert.i.i.i.i.i.i.i.i, align 8, !noalias !851
  %first_second.sroa.8135.0.copyload.pre.i.i.i.i.i.i.i.i = load i64, ptr %first_second.sroa.8135.0.set.i.i.sroa_idx.phi.trans.insert.i.i.i.i.i.i.i.i, align 8, !noalias !851
  %first_second.sroa.10.0.copyload.pre.i.i.i.i.i.i.i.i = load i64, ptr %_7.sroa.4.0.set.sroa_idx.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !851
  %first_second.sroa.11.0.copyload.pre.i.i.i.i.i.i.i.i = load i64, ptr %_7.sroa.5.0.set.sroa_idx.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !851
  %167 = inttoptr i64 %first_second.sroa.0.0.copyload.pre.i.i.i.i.i.i.i.i to ptr
  br label %bb9.i.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.i.i:                              ; preds = %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb9.loopexit108.i.i.i.i.i.i.i.i
  %first_second.sroa.11.0.copyload.i.i.i.i.i.i.i.i = phi i64 [ %first_second.sroa.11.0.copyload.pre.i.i.i.i.i.i.i.i, %bb9.loopexit108.i.i.i.i.i.i.i.i ], [ %hasher.1.pre-phi.i.i.i.i.i.i.i.i.i.i, %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %first_second.sroa.10.0.copyload.i.i.i.i.i.i.i.i = phi i64 [ %first_second.sroa.10.0.copyload.pre.i.i.i.i.i.i.i.i, %bb9.loopexit108.i.i.i.i.i.i.i.i ], [ %_9.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %first_second.sroa.8135.0.copyload.i.i.i.i.i.i.i.i = phi i64 [ %first_second.sroa.8135.0.copyload.pre.i.i.i.i.i.i.i.i, %bb9.loopexit108.i.i.i.i.i.i.i.i ], [ 0, %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %first_second.sroa.6.0.copyload.i.i.i.i.i.i.i.i = phi i64 [ %first_second.sroa.6.0.copyload.pre.i.i.i.i.i.i.i.i, %bb9.loopexit108.i.i.i.i.i.i.i.i ], [ 0, %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %first_second.sroa.0.0.copyload.i.i.i.i.i.i.i.i = phi ptr [ %167, %bb9.loopexit108.i.i.i.i.i.i.i.i ], [ @alloc_d0776666182ad032bd1011cf266e2f3a, %bb12.lr.ph.i.i.us.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %set.i.i.i.i.i.i.i.i.i.i), !noalias !783
  tail call void @llvm.experimental.noalias.scope.decl(metadata !852)
  %_5.i20.i.i.i.i.i.i.i.i = load i64, ptr %85, align 8, !alias.scope !852, !noalias !855, !noundef !10
  %_4.sink.i24.i.i.i.i.i.i.i.i = tail call i64 @llvm.umin.i64(i64 %first_second.sroa.8135.0.copyload.i.i.i.i.i.i.i.i, i64 %_5.i20.i.i.i.i.i.i.i.i)
  %_11.i.i99.i.i.i.i.i.i.i.i = icmp eq i64 %_4.sink.i24.i.i.i.i.i.i.i.i, 0
  br i1 %_11.i.i99.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i, label %bb6.i.i.lr.ph.i.i.i.i.i.i.i.i

bb6.i.i.lr.ph.i.i.i.i.i.i.i.i:                    ; preds = %bb9.i.i.i.i.i.i.i.i
  %_3.not.i21.i.i.i.i.i.i.i.i = icmp ugt i64 %first_second.sroa.8135.0.copyload.i.i.i.i.i.i.i.i, %_5.i20.i.i.i.i.i.i.i.i
  %third_set.val106.i.i.i.i.i.i.i.i = load ptr, ptr %third_set.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %self1.i1.i26.i.i.i.i.i.i.i.i = select i1 %_3.not.i21.i.i.i.i.i.i.i.i, ptr %third_set.val106.i.i.i.i.i.i.i.i, ptr %first_second.sroa.0.0.copyload.i.i.i.i.i.i.i.i
  %next_ctrl.i.i5.sink.i33.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i1.i26.i.i.i.i.i.i.i.i, i64 16
  %_23.i.i4.sink.in.in.in.i28.i.i.i.i.i.i.i.i = load <16 x i8>, ptr %self1.i1.i26.i.i.i.i.i.i.i.i, align 16, !noalias !858
  %_23.i.i4.sink.in.in.i29.i.i.i.i.i.i.i.i = icmp slt <16 x i8> %_23.i.i4.sink.in.in.in.i28.i.i.i.i.i.i.i.i, zeroinitializer
  %_23.i.i4.sink.in.i30.i.i.i.i.i.i.i.i = bitcast <16 x i1> %_23.i.i4.sink.in.in.i29.i.i.i.i.i.i.i.i to i16
  %168 = or i64 %_5.i20.i.i.i.i.i.i.i.i, %first_second.sroa.8135.0.copyload.i.i.i.i.i.i.i.i
  %_3.i.i.i.i.i.i.i.i.i = icmp eq i64 %168, 0
  %hash_builder.val.i.sroa.speculate.load.false.i.i.i.i.i.i.i.i = load i64, ptr %third_set.sroa.gep130.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %hash_builder.val.i.sroa.speculated.i.i.i.i.i.i.i.i = select i1 %_3.not.i21.i.i.i.i.i.i.i.i, i64 %first_second.sroa.10.0.copyload.i.i.i.i.i.i.i.i, i64 %hash_builder.val.i.sroa.speculate.load.false.i.i.i.i.i.i.i.i
  %hash_builder.val1.i.sroa.speculate.load.false.i.i.i.i.i.i.i.i = load i64, ptr %third_set.sroa.gep127.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %hash_builder.val1.i.sroa.speculated.i.i.i.i.i.i.i.i = select i1 %_3.not.i21.i.i.i.i.i.i.i.i, i64 %first_second.sroa.11.0.copyload.i.i.i.i.i.i.i.i, i64 %hash_builder.val1.i.sroa.speculate.load.false.i.i.i.i.i.i.i.i
  %169 = xor i64 %hash_builder.val.i.sroa.speculated.i.i.i.i.i.i.i.i, 8317987319222330741
  %170 = xor i64 %hash_builder.val1.i.sroa.speculated.i.i.i.i.i.i.i.i, 7237128888997146477
  %171 = xor i64 %hash_builder.val.i.sroa.speculated.i.i.i.i.i.i.i.i, 7816392313619706465
  %_2.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %170, %169
  %172 = tail call i64 @llvm.fshl.i64(i64 %170, i64 %170, i64 13)
  %173 = xor i64 %172, %_2.i.i.i.i.i.i.i.i.i.i.i.i.i
  %174 = tail call i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %invariant.op103.i.i.i.i.i.i.i.i = add i64 %173, %171
  %175 = tail call i64 @llvm.fshl.i64(i64 %173, i64 %173, i64 17)
  %_26.i.i.i.sroa.speculate.load.false.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i23.sroa.gep85.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %_26.i.i.i.sroa.speculated.i.i.i.i.i.i.i.i = select i1 %_3.not.i21.i.i.i.i.i.i.i.i, i64 %first_second.sroa.6.0.copyload.i.i.i.i.i.i.i.i, i64 %_26.i.i.i.sroa.speculate.load.false.i.i.i.i.i.i.i.i
  %_29.i.i.i.sroa.speculated.i.i.i.i.i.i.i.i = select i1 %_3.not.i21.i.i.i.i.i.i.i.i, ptr %first_second.sroa.0.0.copyload.i.i.i.i.i.i.i.i, ptr %third_set.val106.i.i.i.i.i.i.i.i
  %invariant.gep.i.i.i.i.i.i.i.i.i.i = getelementptr i8, ptr %_29.i.i.i.sroa.speculated.i.i.i.i.i.i.i.i, i64 -4
  br i1 %_3.i.i.i.i.i.i.i.i.i, label %bb6.i.i.us.i.preheader.i.i.i.i.i.i.i, label %bb6.i.i.i.preheader.i.i.i.i.i.i.i

bb6.i.i.us.i.preheader.i.i.i.i.i.i.i:             ; preds = %bb6.i.i.lr.ph.i.i.i.i.i.i.i.i
  %.not14.i.i.i.us.i.i.i.i.i.i.i.i = icmp eq i16 %_23.i.i4.sink.in.i30.i.i.i.i.i.i.i.i, -1
  br i1 %.not14.i.i.i.us.i.i.i.i.i.i.i.i, label %bb9.i.i.i.us.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i

bb6.i.i.i.preheader.i.i.i.i.i.i.i:                ; preds = %bb6.i.i.lr.ph.i.i.i.i.i.i.i.i
  %_23.i.i4.sink.i31.i.i.i.i.i.i.i.i = xor i16 %_23.i.i4.sink.in.i30.i.i.i.i.i.i.i.i, -1
  %invariant.op225 = xor i64 %hash_builder.val1.i.sroa.speculated.i.i.i.i.i.i.i.i, 8098989879002948979
  br label %bb6.i.i.i.i.i.i.i.i.i.i23

bb9.i.i.i.us.i.i.i.i.i.i.i.i:                     ; preds = %bb6.i.i.us.i.preheader.i.i.i.i.i.i.i, %bb9.i.i.i.us.i.i.i.i.i.i.i.i
  %_1717.i.i.i.us.i.i.i.i.i.i.i.i = phi ptr [ %_17.i.i.i.us.i.i.i.i.i.i.i.i, %bb9.i.i.i.us.i.i.i.i.i.i.i.i ], [ %next_ctrl.i.i5.sink.i33.i.i.i.i.i.i.i.i, %bb6.i.i.us.i.preheader.i.i.i.i.i.i.i ]
  %176 = load <16 x i8>, ptr %_1717.i.i.i.us.i.i.i.i.i.i.i.i, align 16, !noalias !859
  %_17.i.i.i.us.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_1717.i.i.i.us.i.i.i.i.i.i.i.i, i64 16
  %177 = icmp sgt <16 x i8> %176, splat (i8 -1)
  %178 = bitcast <16 x i1> %177 to i16
  %.not.i.i.i.us.i.i.i.i.i.i.i.i = icmp eq i16 %178, 0
  br i1 %.not.i.i.i.us.i.i.i.i.i.i.i.i, label %bb9.i.i.i.us.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i.i23:                        ; preds = %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i", %bb6.i.i.i.preheader.i.i.i.i.i.i.i
  %179 = phi i64 [ %188, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i" ], [ %_4.sink.i24.i.i.i.i.i.i.i.i, %bb6.i.i.i.preheader.i.i.i.i.i.i.i ]
  %.lcssa11.i102.i.i.i.i.i.i.i.i = phi ptr [ %.lcssa10.i.i.i.i.i.i.i.i.i27, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i" ], [ %self1.i1.i26.i.i.i.i.i.i.i.i, %bb6.i.i.i.preheader.i.i.i.i.i.i.i ]
  %_33.i.i13.i101.i.i.i.i.i.i.i.i = phi i16 [ %_33.i.i.i.i.i.i.i.i.i.i.i30, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i" ], [ %_23.i.i4.sink.i31.i.i.i.i.i.i.i.i, %bb6.i.i.i.preheader.i.i.i.i.i.i.i ]
  %_17.i.i.lcssa16.i100.i.i.i.i.i.i.i.i = phi ptr [ %_17.i.i.lcssa15.i.i.i.i.i.i.i.i.i26, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i" ], [ %next_ctrl.i.i5.sink.i33.i.i.i.i.i.i.i.i, %bb6.i.i.i.preheader.i.i.i.i.i.i.i ]
  %.not14.i.i.i.i.i.i.i.i.i.i.i24 = icmp eq i16 %_33.i.i13.i101.i.i.i.i.i.i.i.i, 0
  br i1 %.not14.i.i.i.i.i.i.i.i.i.i.i24, label %bb9.i.i.i.i.i.i.i.i.i.i.i55, label %bb8.i.i.i.i.i.i.i.i.i25

bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i59:        ; preds = %bb9.i.i.i.i.i.i.i.i.i.i.i55
  %_55.i.i.i.i.i.i.i.i.i.i.i60 = xor i16 %183, -1
  br label %bb8.i.i.i.i.i.i.i.i.i25

bb9.i.i.i.i.i.i.i.i.i.i.i55:                      ; preds = %bb6.i.i.i.i.i.i.i.i.i.i23, %bb9.i.i.i.i.i.i.i.i.i.i.i55
  %_1717.i.i.i.i.i.i.i.i.i.i.i56 = phi ptr [ %_17.i.i.i.i.i.i.i.i.i.i.i57, %bb9.i.i.i.i.i.i.i.i.i.i.i55 ], [ %_17.i.i.lcssa16.i100.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i23 ]
  %180 = phi ptr [ %184, %bb9.i.i.i.i.i.i.i.i.i.i.i55 ], [ %.lcssa11.i102.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i23 ]
  %181 = load <16 x i8>, ptr %_1717.i.i.i.i.i.i.i.i.i.i.i56, align 16, !noalias !859
  %182 = icmp slt <16 x i8> %181, zeroinitializer
  %183 = bitcast <16 x i1> %182 to i16
  %184 = getelementptr inbounds i8, ptr %180, i64 -64
  %_17.i.i.i.i.i.i.i.i.i.i.i57 = getelementptr inbounds nuw i8, ptr %_1717.i.i.i.i.i.i.i.i.i.i.i56, i64 16
  %.not.i.i.i.i.i.i.i.i.i.i.i58 = icmp eq i16 %183, -1
  br i1 %.not.i.i.i.i.i.i.i.i.i.i.i58, label %bb9.i.i.i.i.i.i.i.i.i.i.i55, label %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i59

bb8.i.i.i.i.i.i.i.i.i25:                          ; preds = %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i59, %bb6.i.i.i.i.i.i.i.i.i.i23
  %_17.i.i.lcssa15.i.i.i.i.i.i.i.i.i26 = phi ptr [ %_17.i.i.i.i.i.i.i.i.i.i.i57, %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i59 ], [ %_17.i.i.lcssa16.i100.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i23 ]
  %.lcssa10.i.i.i.i.i.i.i.i.i27 = phi ptr [ %184, %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i59 ], [ %.lcssa11.i102.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i23 ]
  %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i28 = phi i16 [ %_55.i.i.i.i.i.i.i.i.i.i.i60, %bb1.bb8_crit_edge.i.i.i.i.i.i.i.i.i.i.i59 ], [ %_33.i.i13.i101.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i.i23 ]
  %185 = add i16 %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i28, -1
  %186 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i28, i1 true)
  %_24.i.i.i.i.i.i.i.i.i.i.i29 = zext nneg i16 %186 to i64
  %_33.i.i.i.i.i.i.i.i.i.i.i30 = and i16 %185, %self3.lcssa.i.i.i.i.i.i.i.i.i.i.i28
  %_45.i.i.i.i.i.i.i.i.i.i.i31 = sub nsw i64 0, %_24.i.i.i.i.i.i.i.i.i.i.i29
  %187 = getelementptr inbounds i32, ptr %.lcssa10.i.i.i.i.i.i.i.i.i27, i64 %_45.i.i.i.i.i.i.i.i.i.i.i31
  %188 = add i64 %179, -1
  %189 = getelementptr inbounds i8, ptr %187, i64 -4
  %.val.i.i.i.i.i.i.i.i.i32 = load i32, ptr %189, align 4, !noalias !868
  %.pre.i.i.i.i.i.i.i.i.i.i.i.i.i = zext nneg i32 %.val.i.i.i.i.i.i.i.i.i32 to i64
  %b.i.i.i.i.i.i.i.i.i.i.i.i = or disjoint i64 %.pre.i.i.i.i.i.i.i.i.i.i.i.i.i, 288230376151711744
  %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass = xor i64 %.pre.i.i.i.i.i.i.i.i.i.i.i.i.i, %invariant.op225
  %_5.i.i.i3.i.i.i.i.i.i.i.i.i.i = add i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, %171
  %190 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, i64 %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass, i64 16)
  %191 = xor i64 %190, %_5.i.i.i3.i.i.i.i.i.i.i.i.i.i
  %_16.i.i.i.i.i.reass.i.i.i.i.i.i.i.i = add i64 %invariant.op103.i.i.i.i.i.i.i.i, %.reass.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.i.reass.reass
  %_19.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %191, %174
  %192 = xor i64 %_16.i.i.i.i.i.reass.i.i.i.i.i.i.i.i, %175
  %193 = tail call noundef i64 @llvm.fshl.i64(i64 %191, i64 %191, i64 21)
  %194 = xor i64 %193, %_19.i.i.i.i.i.i.i.i.i.i.i.i.i
  %195 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.reass.i.i.i.i.i.i.i.i, i64 %_16.i.i.i.i.i.reass.i.i.i.i.i.i.i.i, i64 32)
  %196 = xor i64 %_19.i.i.i.i.i.i.i.i.i.i.i.i.i, %b.i.i.i.i.i.i.i.i.i.i.i.i
  %197 = xor i64 %195, 255
  %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %196, %192
  %_5.i6.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %194, %197
  %198 = tail call noundef i64 @llvm.fshl.i64(i64 %192, i64 %192, i64 13)
  %199 = xor i64 %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i, %198
  %200 = tail call noundef i64 @llvm.fshl.i64(i64 %194, i64 %194, i64 16)
  %201 = xor i64 %_5.i6.i.i.i.i.i.i.i.i.i.i.i.i, %200
  %202 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %199, %_5.i6.i.i.i.i.i.i.i.i.i.i.i.i
  %_19.i8.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %201, %202
  %203 = tail call noundef i64 @llvm.fshl.i64(i64 %199, i64 %199, i64 17)
  %204 = xor i64 %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i, %203
  %205 = tail call noundef i64 @llvm.fshl.i64(i64 %201, i64 %201, i64 21)
  %206 = xor i64 %205, %_19.i8.i.i.i.i.i.i.i.i.i.i.i.i
  %207 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %204, %_19.i8.i.i.i.i.i.i.i.i.i.i.i.i
  %_33.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %206, %207
  %208 = tail call noundef i64 @llvm.fshl.i64(i64 %204, i64 %204, i64 13)
  %209 = xor i64 %208, %_30.i.i.i.i.i.i.i.i.i.i.i.i.i
  %210 = tail call noundef i64 @llvm.fshl.i64(i64 %206, i64 %206, i64 16)
  %211 = xor i64 %210, %_33.i.i.i.i.i.i.i.i.i.i.i.i.i
  %212 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %209, %_33.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_47.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %211, %212
  %213 = tail call noundef i64 @llvm.fshl.i64(i64 %209, i64 %209, i64 17)
  %214 = xor i64 %213, %_44.i.i.i.i.i.i.i.i.i.i.i.i.i
  %215 = tail call noundef i64 @llvm.fshl.i64(i64 %211, i64 %211, i64 21)
  %216 = xor i64 %215, %_47.i.i.i.i.i.i.i.i.i.i.i.i.i
  %217 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %214, %_47.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_61.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %216, %217
  %218 = tail call noundef i64 @llvm.fshl.i64(i64 %214, i64 %214, i64 13)
  %219 = xor i64 %218, %_58.i.i.i.i.i.i.i.i.i.i.i.i.i
  %220 = tail call noundef i64 @llvm.fshl.i64(i64 %216, i64 %216, i64 16)
  %221 = xor i64 %220, %_61.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_72.i.i.i.i.i.i.i.i.i.i.i.i.i = add i64 %219, %_61.i.i.i.i.i.i.i.i.i.i.i.i.i
  %222 = tail call noundef i64 @llvm.fshl.i64(i64 %219, i64 %219, i64 17)
  %223 = tail call noundef i64 @llvm.fshl.i64(i64 %221, i64 %221, i64 21)
  %224 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 32)
  %225 = xor i64 %222, %223
  %226 = xor i64 %225, %224
  %_0.i.i.i.i.i.i.i.i.i.i.i.i = xor i64 %226, %_72.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i.i.i = lshr i64 %_0.i.i.i.i.i.i.i.i.i.i.i.i, 57
  %tag_hash.i.i.i.i.i.i.i.i.i.i.i = trunc nuw nsw i64 %_21.i.i.i.i.i.i.i.i.i.i.i to i8
  %.sroa.0.0.vec.insert.i.i.i.i.i.i.i.i.i.i.i = insertelement <16 x i8> poison, i8 %tag_hash.i.i.i.i.i.i.i.i.i.i.i, i64 0
  %.sroa.0.15.vec.insert.i.i.i.i.i.i.i.i.i.i.i = shufflevector <16 x i8> %.sroa.0.0.vec.insert.i.i.i.i.i.i.i.i.i.i.i, <16 x i8> poison, <16 x i32> zeroinitializer
  br label %bb1.i.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %bb20.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i25
  %probe_seq.sroa.9.0.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ 0, %bb8.i.i.i.i.i.i.i.i.i25 ], [ %233, %bb20.i.i.i.i.i.i.i.i.i.i.i ]
  %hash.pn.i.i.i.i.i.i.i.i.i.i = phi i64 [ %_0.i.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i25 ], [ %234, %bb20.i.i.i.i.i.i.i.i.i.i.i ]
  %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i = and i64 %hash.pn.i.i.i.i.i.i.i.i.i.i, %_26.i.i.i.sroa.speculated.i.i.i.i.i.i.i.i
  %_27.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_29.i.i.i.sroa.speculated.i.i.i.i.i.i.i.i, i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i
  %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i.i.i.i = load <16 x i8>, ptr %_27.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !869
  %227 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i.i.i.i, %.sroa.0.15.vec.insert.i.i.i.i.i.i.i.i.i.i.i
  %228 = bitcast <16 x i1> %227 to i16
  %.not.i.not23.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %228, 0
  br i1 %.not.i.not23.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i.i.i.i

bb10.i.i.i.i.i.i.i.i.i.i.i:                       ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i = phi i16 [ %_51.i.i.i.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i.i.i.i.i.i ], [ %228, %bb1.i.i.i.i.i.i.i.i.i.i.i ]
  %229 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i, i1 true)
  %_42.i.i.i.i.i.i.i.i.i.i.i = zext nneg i16 %229 to i64
  %_13.i.i.i.i.i.i.i.i.i.i.i = add i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i, %_42.i.i.i.i.i.i.i.i.i.i.i
  %index5.i.i.i.i.i.i.i.i.i.i.i = and i64 %_13.i.i.i.i.i.i.i.i.i.i.i, %_26.i.i.i.sroa.speculated.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i.i.i = sub nsw i64 0, %index5.i.i.i.i.i.i.i.i.i.i.i
  %gep.i.i.i.i.i.i.i.i.i.i = getelementptr i32, ptr %invariant.gep.i.i.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i.i.i.i.i.i
  %.val.i.i.i.i.i.i.i.i.i.i.i = load i32, ptr %gep.i.i.i.i.i.i.i.i.i.i, align 4, !range !747, !noalias !879, !noundef !10
  %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i32 %.val.i.i.i.i.i.i.i.i.i32, %.val.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_0.i.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i, label %bb17.i.i.i.i.i.i.i.i.i.i.i, !prof !180

bb11.i.i.i.i.i.i.i.i.i.i.i:                       ; preds = %bb17.i.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i.i
  %230 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i17.i.i.i.i.i.i.i.i.i.i, splat (i8 -1)
  %231 = bitcast <16 x i1> %230 to i16
  %b8.not.i.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %231, 0
  br i1 %b8.not.i.i.i.i.i.i.i.i.i.i.i, label %bb20.i.i.i.i.i.i.i.i.i.i.i, label %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i", !prof !320

bb17.i.i.i.i.i.i.i.i.i.i.i:                       ; preds = %bb10.i.i.i.i.i.i.i.i.i.i.i
  %232 = add i16 %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i, -1
  %_51.i.i.i.i.i.i.i.i.i.i.i = and i16 %232, %iter.sroa.0.0.i24.i.i.i.i.i.i.i.i.i.i
  %.not.i.not.i.i.i.i.i.i.i.i.i.i = icmp eq i16 %_51.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not.i.not.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i.i.i.i, label %bb10.i.i.i.i.i.i.i.i.i.i.i

bb20.i.i.i.i.i.i.i.i.i.i.i:                       ; preds = %bb11.i.i.i.i.i.i.i.i.i.i.i
  %233 = add i64 %probe_seq.sroa.9.0.i.i.i.i.i.i.i.i.i.i.i, 16
  %234 = add i64 %probe_seq.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i, %233
  br label %bb1.i.i.i.i.i.i.i.i.i.i.i

"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i": ; preds = %bb11.i.i.i.i.i.i.i.i.i.i.i
  %_11.i.i.i.i.i.i.i.i.i.i33 = icmp eq i64 %188, 0
  br i1 %_11.i.i.i.i.i.i.i.i.i.i33, label %bb12.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i.i23

bb12.i.i.i.i.i.i.i.i:                             ; preds = %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i", %bb9.i.i.i.us.i.i.i.i.i.i.i.i, %bb10.i.i.i.i.i.i.i.i.i.i.i, %bb6.i.i.us.i.preheader.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i.i
  %_0.sroa.0.0.i37.i.i.i.i.i.i.i.i = phi i32 [ 1114112, %bb9.i.i.i.i.i.i.i.i ], [ 1114112, %bb6.i.i.us.i.preheader.i.i.i.i.i.i.i ], [ %.val.i.i.i.i.i.i.i.i.i32, %bb10.i.i.i.i.i.i.i.i.i.i.i ], [ 1114112, %bb9.i.i.i.us.i.i.i.i.i.i.i.i ], [ 1114112, %"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE.exit.loopexit.i.i.i.i.i.i.i.i" ]
  %_4.i.i.i.i.i.i38.i.i.i.i.i.i.i.i = icmp eq i64 %first_second.sroa.6.0.copyload.i.i.i.i.i.i.i.i, 0
  br i1 %_4.i.i.i.i.i.i38.i.i.i.i.i.i.i.i, label %bb13.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i34

bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i34:                ; preds = %bb12.i.i.i.i.i.i.i.i
  %_10.i.i.i.i.i.i.i.i.i.i.i.i.i.i35 = shl i64 %first_second.sroa.6.0.copyload.i.i.i.i.i.i.i.i, 2
  %_32.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i36 = add i64 %_10.i.i.i.i.i.i.i.i.i.i.i.i.i.i35, 19
  %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i37 = and i64 %_32.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i36, -16
  %rhs5.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i38 = add i64 %first_second.sroa.6.0.copyload.i.i.i.i.i.i.i.i, 17
  %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i39 = add i64 %rhs5.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i38, %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i37
  %_37.1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i40 = icmp uge i64 %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i39, %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i37
  %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i41 = icmp ult i64 %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i39, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i40)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i41)
  %235 = icmp ne ptr %first_second.sroa.0.0.copyload.i.i.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %235)
  %_4.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i42 = icmp eq i64 %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i39, 0
  br i1 %_4.not.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i42, label %bb13.i.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i43

bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i43:             ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i34
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.i.i44 = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i37
  %ptr.i.i.i.i.i.i.i.i.i.i.i.i.i.i45 = getelementptr inbounds i8, ptr %first_second.sroa.0.0.copyload.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i.i.i.i.i.i.i.i.i44
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i.i.i.i.i.i.i.i.i45, i64 noundef %_37.0.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i39, i64 noundef range(i64 1, -9223372036854775807) 16) #33, !noalias !773
  br label %bb13.i.i.i.i.i.i.i.i

bb13.i.i.i.i.i.i.i.i:                             ; preds = %bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i.i.i43, %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i.i34, %bb12.i.i.i.i.i.i.i.i
  %third_set.val12.i.i.i.i.i.i.i.i = load ptr, ptr %third_set.i.i.i.i.i.i.i.i, align 8, !noalias !767
  %third_set.val13.i.i.i.i.i.i.i.i = load i64, ptr %self.sink14.i23.sroa.gep85.i.i.i.i.i.i.i.i, align 8, !noalias !767, !noundef !10
  %_4.i.i.i.i.i.i39.i.i.i.i.i.i.i.i = icmp eq i64 %third_set.val13.i.i.i.i.i.i.i.i, 0
  br i1 %_4.i.i.i.i.i.i39.i.i.i.i.i.i.i.i, label %bb14.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i40.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i40.i.i.i.i.i.i.i.i:                ; preds = %bb13.i.i.i.i.i.i.i.i
  %_10.i.i.i.i.i.i41.i.i.i.i.i.i.i.i = shl i64 %third_set.val13.i.i.i.i.i.i.i.i, 2
  %_32.0.i.i.i.i.i.i.i42.i.i.i.i.i.i.i.i = add i64 %_10.i.i.i.i.i.i41.i.i.i.i.i.i.i.i, 19
  %ctrl_offset.i.i.i.i.i.i.i43.i.i.i.i.i.i.i.i = and i64 %_32.0.i.i.i.i.i.i.i42.i.i.i.i.i.i.i.i, -16
  %rhs5.i.i.i.i.i.i.i44.i.i.i.i.i.i.i.i = add i64 %third_set.val13.i.i.i.i.i.i.i.i, 17
  %_37.0.i.i.i.i.i.i.i45.i.i.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i.i44.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i43.i.i.i.i.i.i.i.i
  %_37.1.i.i.i.i.i.i.i46.i.i.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i.i45.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i43.i.i.i.i.i.i.i.i
  %_19.i.i.i.i.i.i.i47.i.i.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i.i45.i.i.i.i.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i.i46.i.i.i.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i47.i.i.i.i.i.i.i.i)
  %236 = icmp ne ptr %third_set.val12.i.i.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %236)
  %_4.not.i.i.i.i.i.i.i48.i.i.i.i.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i.i.i.i45.i.i.i.i.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i.i.i.i48.i.i.i.i.i.i.i.i, label %bb14.i.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i49.i.i.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i49.i.i.i.i.i.i.i.i:             ; preds = %bb1.i.i.i.i.i.i40.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i50.i.i.i.i.i.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i.i43.i.i.i.i.i.i.i.i
  %ptr.i.i.i.i.i.i51.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %third_set.val12.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i50.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i51.i.i.i.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i.i45.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33, !noalias !773
  br label %bb14.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i:                             ; preds = %bb1.i2.i.i.i.i.i.i49.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i40.i.i.i.i.i.i.i.i, %bb13.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %third_set.i.i.i.i.i.i.i.i), !noalias !767
  %_4.i.i.i.i.i.i53.i.i.i.i.i.i.i.i = icmp eq i64 %self.sink14.i.sroa.gep84.val.i.i.i.i.i.i.i.i, 0
  br i1 %_4.i.i.i.i.i.i53.i.i.i.i.i.i.i.i, label %bb15.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i54.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i54.i.i.i.i.i.i.i.i:                ; preds = %bb14.i.i.i.i.i.i.i.i
  %_10.i.i.i.i.i.i55.i.i.i.i.i.i.i.i = shl i64 %self.sink14.i.sroa.gep84.val.i.i.i.i.i.i.i.i, 2
  %_32.0.i.i.i.i.i.i.i56.i.i.i.i.i.i.i.i = add i64 %_10.i.i.i.i.i.i55.i.i.i.i.i.i.i.i, 19
  %ctrl_offset.i.i.i.i.i.i.i57.i.i.i.i.i.i.i.i = and i64 %_32.0.i.i.i.i.i.i.i56.i.i.i.i.i.i.i.i, -16
  %rhs5.i.i.i.i.i.i.i58.i.i.i.i.i.i.i.i = add i64 %self.sink14.i.sroa.gep84.val.i.i.i.i.i.i.i.i, 17
  %_37.0.i.i.i.i.i.i.i59.i.i.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i.i58.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i57.i.i.i.i.i.i.i.i
  %_37.1.i.i.i.i.i.i.i60.i.i.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i.i59.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i57.i.i.i.i.i.i.i.i
  %_19.i.i.i.i.i.i.i61.i.i.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i.i59.i.i.i.i.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i.i60.i.i.i.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i61.i.i.i.i.i.i.i.i)
  %237 = icmp ne ptr %second_set.val104.i.i.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %237)
  %_4.not.i.i.i.i.i.i.i62.i.i.i.i.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i.i.i.i59.i.i.i.i.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i.i.i.i62.i.i.i.i.i.i.i.i, label %bb15.i.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i63.i.i.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i63.i.i.i.i.i.i.i.i:             ; preds = %bb1.i.i.i.i.i.i54.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i64.i.i.i.i.i.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i.i57.i.i.i.i.i.i.i.i
  %ptr.i.i.i.i.i.i65.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %second_set.val104.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i64.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i65.i.i.i.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i.i59.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33, !noalias !773
  br label %bb15.i.i.i.i.i.i.i.i

bb15.i.i.i.i.i.i.i.i:                             ; preds = %bb1.i2.i.i.i.i.i.i63.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i54.i.i.i.i.i.i.i.i, %bb14.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %second_set.i.i.i.i.i.i.i.i5), !noalias !767
  %_4.i.i.i.i.i.i67.i.i.i.i.i.i.i.i = icmp eq i64 %self.sink14.i.sroa.gep.val.i.i.i.i.i.i.i.i, 0
  br i1 %_4.i.i.i.i.i.i67.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E.exit.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i68.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i68.i.i.i.i.i.i.i.i:                ; preds = %bb15.i.i.i.i.i.i.i.i
  %_10.i.i.i.i.i.i69.i.i.i.i.i.i.i.i = shl i64 %self.sink14.i.sroa.gep.val.i.i.i.i.i.i.i.i, 2
  %_32.0.i.i.i.i.i.i.i70.i.i.i.i.i.i.i.i = add i64 %_10.i.i.i.i.i.i69.i.i.i.i.i.i.i.i, 19
  %ctrl_offset.i.i.i.i.i.i.i71.i.i.i.i.i.i.i.i = and i64 %_32.0.i.i.i.i.i.i.i70.i.i.i.i.i.i.i.i, -16
  %rhs5.i.i.i.i.i.i.i72.i.i.i.i.i.i.i.i = add i64 %self.sink14.i.sroa.gep.val.i.i.i.i.i.i.i.i, 17
  %_37.0.i.i.i.i.i.i.i73.i.i.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i.i72.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i71.i.i.i.i.i.i.i.i
  %_37.1.i.i.i.i.i.i.i74.i.i.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i.i73.i.i.i.i.i.i.i.i, %ctrl_offset.i.i.i.i.i.i.i71.i.i.i.i.i.i.i.i
  %_19.i.i.i.i.i.i.i75.i.i.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i.i73.i.i.i.i.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i.i74.i.i.i.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i75.i.i.i.i.i.i.i.i)
  %238 = icmp ne ptr %first_set.val105.i.i.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %238)
  %_4.not.i.i.i.i.i.i.i76.i.i.i.i.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i.i.i.i73.i.i.i.i.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i.i.i.i76.i.i.i.i.i.i.i.i, label %_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E.exit.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i77.i.i.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i77.i.i.i.i.i.i.i.i:             ; preds = %bb1.i.i.i.i.i.i68.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i78.i.i.i.i.i.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i.i.i.i71.i.i.i.i.i.i.i.i
  %ptr.i.i.i.i.i.i79.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %first_set.val105.i.i.i.i.i.i.i.i, i64 %_18.i.i.i.i.i.i78.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i79.i.i.i.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i.i73.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33, !noalias !773
  br label %_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E.exit.i.i.i.i.i.i.i

_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E.exit.i.i.i.i.i.i.i: ; preds = %bb1.i2.i.i.i.i.i.i77.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i68.i.i.i.i.i.i.i.i, %bb15.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %first_set.i.i.i.i.i.i.i.i6), !noalias !767
  %.not.i.i.i.i.i.i.i.i46 = icmp eq i32 %_0.sroa.0.0.i37.i.i.i.i.i.i.i.i, 1114112
  br i1 %.not.i.i.i.i.i.i.i.i46, label %bb1.i.i.i.i.i.backedge, label %bb3.i.i.i.i.i.i.i.i47

bb1.i.i.i.i.i.backedge:                           ; preds = %_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i
  br label %bb1.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i47:                            ; preds = %_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E.exit.i.i.i.i.i.i.i
  %239 = add nsw i32 %_0.sroa.0.0.i37.i.i.i.i.i.i.i.i, -97
  %or.cond.i.i.i.i.i.i.i.i.i.i48 = icmp ult i32 %239, 26
  br i1 %or.cond.i.i.i.i.i.i.i.i.i.i48, label %bb2.i.i.i.i.i.i.i.i.i.i54, label %bb3.i.i.i.i.i.i.i.i.i.i49

bb3.i.i.i.i.i.i.i.i.i.i49:                        ; preds = %bb3.i.i.i.i.i.i.i.i47
  %240 = add nsw i32 %_0.sroa.0.0.i37.i.i.i.i.i.i.i.i, -65
  %or.cond1.i.i.i.i.i.i.i.i.i.i50 = icmp ult i32 %240, 26
  %241 = add nsw i32 %_0.sroa.0.0.i37.i.i.i.i.i.i.i.i, -38
  %spec.select.i.i.i.i.i.i.i.i.i.i51 = select i1 %or.cond1.i.i.i.i.i.i.i.i.i.i50, i32 %241, i32 0
  br label %bb4.i.i.i.i.i.i52

bb2.i.i.i.i.i.i.i.i.i.i54:                        ; preds = %bb3.i.i.i.i.i.i.i.i47
  %242 = add nsw i32 %_0.sroa.0.0.i37.i.i.i.i.i.i.i.i, -96
  br label %bb4.i.i.i.i.i.i52

bb4.i.i.i.i.i.i52:                                ; preds = %bb2.i.i.i.i.i.i.i.i.i.i54, %bb3.i.i.i.i.i.i.i.i.i.i49
  %_0.sroa.3.0.i.ph.i.i.i.i.i.i = phi i32 [ %242, %bb2.i.i.i.i.i.i.i.i.i.i54 ], [ %spec.select.i.i.i.i.i.i.i.i.i.i51, %bb3.i.i.i.i.i.i.i.i.i.i49 ]
  %_4.0.i.i.i.i.i.i.i53 = add i32 %_0.sroa.3.0.i.ph.i.i.i.i.i.i, %accum.sroa.0.0.i.i.i.i.i.ph
  br label %bb1.i.i.i.i.i.outer

bb3:                                              ; preds = %bb1.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !882)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !885)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !888)
  br i1 %2, label %bb4.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb3, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i"
  %_3.sroa.0.012.i.i.i.i = phi i64 [ %243, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i" ], [ 0, %bb3 ]
  %_6.i.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %data.val, i64 %_3.sroa.0.012.i.i.i.i
  %243 = add nuw i64 %_3.sroa.0.012.i.i.i.i, 1
  %_6.val.i.i.i.i = load i64, ptr %_6.i.i.i.i, align 8, !alias.scope !888, !noalias !891
  %_6.i.i.i.i4.i.i.i.i.i.i = icmp eq i64 %_6.val.i.i.i.i, 0
  br i1 %_6.i.i.i.i4.i.i.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i", label %bb2.i.i.i5.i.i.i.i.i.i

bb2.i.i.i5.i.i.i.i.i.i:                           ; preds = %bb5.i.i.i.i
  %244 = getelementptr i8, ptr %_6.i.i.i.i, i64 8
  %_6.val7.i.i.i.i = load ptr, ptr %244, align 8, !alias.scope !888, !noalias !891, !nonnull !10, !noundef !10
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i, i64 noundef %_6.val.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33, !noalias !892
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i": ; preds = %bb2.i.i.i5.i.i.i.i.i.i, %bb5.i.i.i.i
  %_7.i.i.i.i = icmp eq i64 %243, %data.val1
  br i1 %_7.i.i.i.i, label %bb4.i.i, label %bb5.i.i.i.i

bb4.i.i:                                          ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit.i.i.i.i", %bb3
  %_1.val4.i.i = load i64, ptr %data, align 8, !range !78, !alias.scope !891, !noundef !10
  %_6.i.i.i.i6.i.i = icmp eq i64 %_1.val4.i.i, 0
  br i1 %_6.i.i.i.i6.i.i, label %"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E.exit", label %bb2.i.i.i7.i.i

bb2.i.i.i7.i.i:                                   ; preds = %bb4.i.i
  %245 = mul nuw i64 %_1.val4.i.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %data.val, i64 noundef %245, i64 noundef range(i64 1, -9223372036854775807) 8) #33, !noalias !891
  br label %"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E.exit"

"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E.exit": ; preds = %bb4.i.i, %bb2.i.i.i7.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %data)
  %246 = insertvalue { i32, i32 } poison, i32 %_0.sroa.0.0.i.i.i.i.i, 0
  %247 = insertvalue { i32, i32 } %246, i32 %accum.sroa.0.0.i.i.i.i.i.ph, 1
  ret { i32, i32 } %247
}

; aoc2022::solver::run_day
; Function Attrs: uwtable
define void @_ZN7aoc20226solver7run_day17h0d314465522a10edE(ptr dead_on_unwind noalias noundef writable writeonly sret([48 x i8]) align 8 captures(none) dereferenceable(48) %_0, i64 noundef %0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %buf.i.i53 = alloca [10 x i8], align 1
  %buf.i.i44 = alloca [10 x i8], align 1
  %buf.i.i32 = alloca [20 x i8], align 1
  %buf.i.i22 = alloca [20 x i8], align 1
  %buf.i.i11 = alloca [20 x i8], align 1
  %buf.i.i = alloca [20 x i8], align 1
  %args = alloca [16 x i8], align 8
  %day = alloca [8 x i8], align 8
  store i64 %0, ptr %day, align 8
  switch i64 %0, label %bb1 [
    i64 1, label %bb4
    i64 2, label %bb3
    i64 3, label %bb2
  ], !prof !96

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %day, ptr %args, align 8
  %_31.sroa.4.0.args.sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hbcf79f68ff2d61d8E", ptr %_31.sroa.4.0.args.sroa_idx, align 8
; call anyhow::__private::format_err
  %error = call fastcc noundef nonnull ptr @_ZN6anyhow9__private10format_err17h9d02632e9c6caa4dE(ptr noundef nonnull %args) #32
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %1 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %error, ptr %1, align 8
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb17

bb4:                                              ; preds = %start
; call aoc2022::solver::day01::solve
  %2 = tail call { i64, i64 } @_ZN7aoc20226solver5day015solve17hc5fefd61c6cf4600E(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %_5.0 = extractvalue { i64, i64 } %2, 0
  %_5.1 = extractvalue { i64, i64 } %2, 1
  call void @llvm.lifetime.start.p0(i64 20, ptr nonnull %buf.i.i), !noalias !893
; call core::fmt::num::imp::<impl usize>::_fmt
  %3 = call { ptr, i64 } @"_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE"(i64 noundef %_5.0, ptr noalias noundef nonnull align 1 %buf.i.i, i64 noundef 20), !noalias !893
  %self.1.i.i = extractvalue { ptr, i64 } %3, 1
  %_8.i.i.i.i.i = icmp eq i64 %self.1.i.i, 0
  br i1 %_8.i.i.i.i.i, label %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i": ; preds = %bb4
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !898
; call __rustc::__rust_alloc
  %4 = call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.1.i.i, i64 noundef range(i64 1, 9) 1) #33, !noalias !898
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb3.i.i.i.i, label %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit"

bb3.i.i.i.i:                                      ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i"
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.1.i.i) #36, !noalias !904
  unreachable

"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit": ; preds = %bb4, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i"
  %_4.sroa.10.0.i.i.i.i = phi ptr [ inttoptr (i64 1 to ptr), %bb4 ], [ %4, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i" ]
  %self.0.i.i = extractvalue { ptr, i64 } %3, 0
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i, ptr nonnull readonly align 1 %self.0.i.i, i64 range(i64 0, -9223372036854775808) %self.1.i.i, i1 false), !noalias !905
  call void @llvm.lifetime.end.p0(i64 20, ptr nonnull %buf.i.i), !noalias !893
  call void @llvm.lifetime.start.p0(i64 20, ptr nonnull %buf.i.i11), !noalias !906
; invoke core::fmt::num::imp::<impl usize>::_fmt
  %6 = invoke { ptr, i64 } @"_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE"(i64 noundef %_5.1, ptr noalias noundef nonnull align 1 %buf.i.i11, i64 noundef 20)
          to label %.noexc unwind label %funclet_bb20

.noexc:                                           ; preds = %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit"
  %self.1.i.i12 = extractvalue { ptr, i64 } %6, 1
  %_8.i.i.i.i.i13 = icmp eq i64 %self.1.i.i12, 0
  br i1 %_8.i.i.i.i.i13, label %bb7, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i14"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i14": ; preds = %.noexc
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !911
; call __rustc::__rust_alloc
  %7 = call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.1.i.i12, i64 noundef range(i64 1, 9) 1) #33, !noalias !911
  %8 = icmp eq ptr %7, null
  br i1 %8, label %bb3.i.i.i.i19, label %bb7

bb3.i.i.i.i19:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i14"
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.1.i.i12) #36
          to label %.noexc20 unwind label %funclet_bb20

.noexc20:                                         ; preds = %bb3.i.i.i.i19
  unreachable

bb3:                                              ; preds = %start
; call aoc2022::solver::day02::solve
  %9 = tail call { i64, i64 } @_ZN7aoc20226solver5day025solve17h15f04a137efee39bE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %_13.0 = extractvalue { i64, i64 } %9, 0
  %_13.1 = extractvalue { i64, i64 } %9, 1
  call void @llvm.lifetime.start.p0(i64 20, ptr nonnull %buf.i.i22), !noalias !917
; call core::fmt::num::imp::<impl usize>::_fmt
  %10 = call { ptr, i64 } @"_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE"(i64 noundef %_13.0, ptr noalias noundef nonnull align 1 %buf.i.i22, i64 noundef 20), !noalias !917
  %self.1.i.i23 = extractvalue { ptr, i64 } %10, 1
  %_8.i.i.i.i.i24 = icmp eq i64 %self.1.i.i23, 0
  br i1 %_8.i.i.i.i.i24, label %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit31", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i25"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i25": ; preds = %bb3
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !922
; call __rustc::__rust_alloc
  %11 = call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.1.i.i23, i64 noundef range(i64 1, 9) 1) #33, !noalias !922
  %12 = icmp eq ptr %11, null
  br i1 %12, label %bb3.i.i.i.i30, label %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit31"

bb3.i.i.i.i30:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i25"
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.1.i.i23) #36, !noalias !928
  unreachable

"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit31": ; preds = %bb3, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i25"
  %_4.sroa.10.0.i.i.i.i26 = phi ptr [ inttoptr (i64 1 to ptr), %bb3 ], [ %11, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i25" ]
  %self.0.i.i27 = extractvalue { ptr, i64 } %10, 0
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i26, ptr nonnull readonly align 1 %self.0.i.i27, i64 range(i64 0, -9223372036854775808) %self.1.i.i23, i1 false), !noalias !929
  call void @llvm.lifetime.end.p0(i64 20, ptr nonnull %buf.i.i22), !noalias !917
  call void @llvm.lifetime.start.p0(i64 20, ptr nonnull %buf.i.i32), !noalias !930
; invoke core::fmt::num::imp::<impl usize>::_fmt
  %13 = invoke { ptr, i64 } @"_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE"(i64 noundef %_13.1, ptr noalias noundef nonnull align 1 %buf.i.i32, i64 noundef 20)
          to label %.noexc41 unwind label %funclet_bb19

.noexc41:                                         ; preds = %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit31"
  %self.1.i.i33 = extractvalue { ptr, i64 } %13, 1
  %_8.i.i.i.i.i34 = icmp eq i64 %self.1.i.i33, 0
  br i1 %_8.i.i.i.i.i34, label %bb10, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i35"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i35": ; preds = %.noexc41
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !935
; call __rustc::__rust_alloc
  %14 = call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.1.i.i33, i64 noundef range(i64 1, 9) 1) #33, !noalias !935
  %15 = icmp eq ptr %14, null
  br i1 %15, label %bb3.i.i.i.i40, label %bb10

bb3.i.i.i.i40:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i35"
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.1.i.i33) #36
          to label %.noexc42 unwind label %funclet_bb19

.noexc42:                                         ; preds = %bb3.i.i.i.i40
  unreachable

bb2:                                              ; preds = %start
; call aoc2022::solver::day03::solve
  %16 = tail call { i32, i32 } @_ZN7aoc20226solver5day035solve17hff6b3e95cb3ead9aE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %input.0, i64 noundef %input.1)
  %_21.0 = extractvalue { i32, i32 } %16, 0
  %_21.1 = extractvalue { i32, i32 } %16, 1
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %buf.i.i44), !noalias !941
; call core::fmt::num::imp::<impl u32>::_fmt
  %17 = call { ptr, i64 } @"_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h68bd8f419e61f018E"(i32 noundef %_21.0, ptr noalias noundef nonnull align 1 %buf.i.i44, i64 noundef 10), !noalias !941
  %self.1.i.i45 = extractvalue { ptr, i64 } %17, 1
  %_8.i.i.i.i.i46 = icmp eq i64 %self.1.i.i45, 0
  br i1 %_8.i.i.i.i.i46, label %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E.exit", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i47"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i47": ; preds = %bb2
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !946
; call __rustc::__rust_alloc
  %18 = call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.1.i.i45, i64 noundef range(i64 1, 9) 1) #33, !noalias !946
  %19 = icmp eq ptr %18, null
  br i1 %19, label %bb3.i.i.i.i52, label %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E.exit"

bb3.i.i.i.i52:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i47"
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.1.i.i45) #36, !noalias !952
  unreachable

"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E.exit": ; preds = %bb2, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i47"
  %_4.sroa.10.0.i.i.i.i48 = phi ptr [ inttoptr (i64 1 to ptr), %bb2 ], [ %18, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i47" ]
  %self.0.i.i49 = extractvalue { ptr, i64 } %17, 0
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i48, ptr nonnull readonly align 1 %self.0.i.i49, i64 range(i64 0, -9223372036854775808) %self.1.i.i45, i1 false), !noalias !953
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %buf.i.i44), !noalias !941
  call void @llvm.lifetime.start.p0(i64 10, ptr nonnull %buf.i.i53), !noalias !954
; invoke core::fmt::num::imp::<impl u32>::_fmt
  %20 = invoke { ptr, i64 } @"_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h68bd8f419e61f018E"(i32 noundef %_21.1, ptr noalias noundef nonnull align 1 %buf.i.i53, i64 noundef 10)
          to label %.noexc62 unwind label %funclet_bb18

.noexc62:                                         ; preds = %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E.exit"
  %self.1.i.i54 = extractvalue { ptr, i64 } %20, 1
  %_8.i.i.i.i.i55 = icmp eq i64 %self.1.i.i54, 0
  br i1 %_8.i.i.i.i.i55, label %bb13, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i56"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i56": ; preds = %.noexc62
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !959
; call __rustc::__rust_alloc
  %21 = call noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.1.i.i54, i64 noundef range(i64 1, 9) 1) #33, !noalias !959
  %22 = icmp eq ptr %21, null
  br i1 %22, label %bb3.i.i.i.i61, label %bb13

bb3.i.i.i.i61:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i56"
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.1.i.i54) #36
          to label %.noexc63 unwind label %funclet_bb18

.noexc63:                                         ; preds = %bb3.i.i.i.i61
  unreachable

funclet_bb20:                                     ; preds = %bb3.i.i.i.i19, %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit"
  %cleanuppad = cleanuppad within none []
  br i1 %_8.i.i.i.i.i, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit", label %bb2.i.i.i5.i.i

bb2.i.i.i5.i.i:                                   ; preds = %funclet_bb20
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i.i.i.i, i64 noundef %self.1.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad) ]
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit": ; preds = %funclet_bb20, %bb2.i.i.i5.i.i
  cleanupret from %cleanuppad unwind to caller

bb7:                                              ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i14", %.noexc
  %_4.sroa.10.0.i.i.i.i15 = phi ptr [ inttoptr (i64 1 to ptr), %.noexc ], [ %7, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i14" ]
  %self.0.i.i16 = extractvalue { ptr, i64 } %6, 0
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i15, ptr nonnull readonly align 1 %self.0.i.i16, i64 range(i64 0, -9223372036854775808) %self.1.i.i12, i1 false), !noalias !965
  call void @llvm.lifetime.end.p0(i64 20, ptr nonnull %buf.i.i11), !noalias !906
  store i64 %self.1.i.i, ptr %_0, align 8
  %_6.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_4.sroa.10.0.i.i.i.i, ptr %_6.sroa.4.0._0.sroa_idx, align 8
  %_6.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %self.1.i.i, ptr %_6.sroa.5.0._0.sroa_idx, align 8
  %_6.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %self.1.i.i12, ptr %_6.sroa.6.0._0.sroa_idx, align 8
  %_6.sroa.6.sroa.4.0._6.sroa.6.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %_4.sroa.10.0.i.i.i.i15, ptr %_6.sroa.6.sroa.4.0._6.sroa.6.0._0.sroa_idx.sroa_idx, align 8
  %_6.sroa.6.sroa.5.0._6.sroa.6.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %self.1.i.i12, ptr %_6.sroa.6.sroa.5.0._6.sroa.6.0._0.sroa_idx.sroa_idx, align 8
  br label %bb17

bb17:                                             ; preds = %bb1, %bb13, %bb10, %bb7
  ret void

funclet_bb19:                                     ; preds = %bb3.i.i.i.i40, %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E.exit31"
  %cleanuppad6 = cleanuppad within none []
  br i1 %_8.i.i.i.i.i24, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit67", label %bb2.i.i.i5.i.i66

bb2.i.i.i5.i.i66:                                 ; preds = %funclet_bb19
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i.i.i.i26, i64 noundef %self.1.i.i23, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad6) ]
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit67"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit67": ; preds = %funclet_bb19, %bb2.i.i.i5.i.i66
  cleanupret from %cleanuppad6 unwind to caller

bb10:                                             ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i35", %.noexc41
  %_4.sroa.10.0.i.i.i.i36 = phi ptr [ inttoptr (i64 1 to ptr), %.noexc41 ], [ %14, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i35" ]
  %self.0.i.i37 = extractvalue { ptr, i64 } %13, 0
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i36, ptr nonnull readonly align 1 %self.0.i.i37, i64 range(i64 0, -9223372036854775808) %self.1.i.i33, i1 false), !noalias !966
  call void @llvm.lifetime.end.p0(i64 20, ptr nonnull %buf.i.i32), !noalias !930
  store i64 %self.1.i.i23, ptr %_0, align 8
  %_14.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_4.sroa.10.0.i.i.i.i26, ptr %_14.sroa.4.0._0.sroa_idx, align 8
  %_14.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %self.1.i.i23, ptr %_14.sroa.5.0._0.sroa_idx, align 8
  %_14.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %self.1.i.i33, ptr %_14.sroa.6.0._0.sroa_idx, align 8
  %_14.sroa.6.sroa.4.0._14.sroa.6.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %_4.sroa.10.0.i.i.i.i36, ptr %_14.sroa.6.sroa.4.0._14.sroa.6.0._0.sroa_idx.sroa_idx, align 8
  %_14.sroa.6.sroa.5.0._14.sroa.6.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %self.1.i.i33, ptr %_14.sroa.6.sroa.5.0._14.sroa.6.0._0.sroa_idx.sroa_idx, align 8
  br label %bb17

funclet_bb18:                                     ; preds = %bb3.i.i.i.i61, %"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E.exit"
  %cleanuppad7 = cleanuppad within none []
  br i1 %_8.i.i.i.i.i46, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit70", label %bb2.i.i.i5.i.i69

bb2.i.i.i5.i.i69:                                 ; preds = %funclet_bb18
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i.i.i.i48, i64 noundef %self.1.i.i45, i64 noundef range(i64 1, -9223372036854775807) 1) #33 [ "funclet"(token %cleanuppad7) ]
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit70"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hafad7ea7b1c6cafaE.exit70": ; preds = %funclet_bb18, %bb2.i.i.i5.i.i69
  cleanupret from %cleanuppad7 unwind to caller

bb13:                                             ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i56", %.noexc62
  %_4.sroa.10.0.i.i.i.i57 = phi ptr [ inttoptr (i64 1 to ptr), %.noexc62 ], [ %21, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i.i.i.i56" ]
  %self.0.i.i58 = extractvalue { ptr, i64 } %20, 0
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i57, ptr nonnull readonly align 1 %self.0.i.i58, i64 range(i64 0, -9223372036854775808) %self.1.i.i54, i1 false), !noalias !967
  call void @llvm.lifetime.end.p0(i64 10, ptr nonnull %buf.i.i53), !noalias !954
  store i64 %self.1.i.i45, ptr %_0, align 8
  %_22.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_4.sroa.10.0.i.i.i.i48, ptr %_22.sroa.4.0._0.sroa_idx, align 8
  %_22.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %self.1.i.i45, ptr %_22.sroa.5.0._0.sroa_idx, align 8
  %_22.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %self.1.i.i54, ptr %_22.sroa.6.0._0.sroa_idx, align 8
  %_22.sroa.6.sroa.4.0._22.sroa.6.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %_4.sroa.10.0.i.i.i.i57, ptr %_22.sroa.6.sroa.4.0._22.sroa.6.0._0.sroa_idx.sroa_idx, align 8
  %_22.sroa.6.sroa.5.0._22.sroa.6.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %self.1.i.i54, ptr %_22.sroa.6.sroa.5.0._22.sroa.6.0._0.sroa_idx.sroa_idx, align 8
  br label %bb17
}

; <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @"_ZN81_$LT$core..str..iter..Lines$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha5afe9ba5eb60990E"(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !968)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !971)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !974)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !8, !alias.scope !977, !noundef !10
  %_2.i.i.i = trunc nuw i8 %1 to i1
  br i1 %_2.i.i.i, label %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2096107455fc8217E.exit", label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %start
  %_4.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val.i.i.i = load ptr, ptr %_4.i.i.i, align 8, !alias.scope !977, !nonnull !10, !align !11, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.val1.i.i.i = load i64, ptr %2, align 8, !alias.scope !977, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !978)
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %index2.i.i.i.i = load i64, ptr %4, align 8, !alias.scope !981, !noalias !982, !noundef !10
  %_37.not.i.i.i.i = icmp ugt i64 %index2.i.i.i.i, %_4.val1.i.i.i
  %.promoted.i.i.i.i = load i64, ptr %3, align 8, !alias.scope !981, !noalias !982
  %_4224.i.i.i.i = icmp ult i64 %index2.i.i.i.i, %.promoted.i.i.i.i
  %or.cond25.i.i.i.i = or i1 %_37.not.i.i.i.i, %_4224.i.i.i.i
  br i1 %or.cond25.i.i.i.i, label %bb1.i.i.i.i, label %bb12.lr.ph.i.i.i.i

bb12.lr.ph.i.i.i.i:                               ; preds = %bb2.i.i.i
  %_10.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_47.i.i.i.i = load i8, ptr %5, align 8, !alias.scope !981, !noalias !982, !noundef !10
  %_12.i.i.i.i = zext i8 %_47.i.i.i.i to i64
  %6 = getelementptr i8, ptr %_10.i.i.i.i, i64 %_12.i.i.i.i
  %_48.i.i.i.i = getelementptr i8, ptr %6, i64 -1
  %_65.i.i.i.i = icmp ult i8 %_47.i.i.i.i, 5
  %last_byte.us.pre.i.i.i.i = load i8, ptr %_48.i.i.i.i, align 1, !alias.scope !981, !noalias !982
  br i1 %_65.i.i.i.i, label %bb12.us.i.i.i.i, label %bb12.i.i.i.i, !prof !984

bb12.us.i.i.i.i:                                  ; preds = %bb12.lr.ph.i.i.i.i, %bb9.us.i.i.i.i
  %7 = phi i64 [ %16, %bb9.us.i.i.i.i ], [ %.promoted.i.i.i.i, %bb12.lr.ph.i.i.i.i ]
  %new_len.us.i.i.i.i = sub nuw i64 %index2.i.i.i.i, %7
  %_45.us.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i, i64 %7
  %_3.i.us.i.i.i.i = icmp samesign ult i64 %new_len.us.i.i.i.i, 16
  br i1 %_3.i.us.i.i.i.i, label %bb5.preheader.i.us.i.i.i.i, label %bb2.i.us.i.i.i.i

bb2.i.us.i.i.i.i:                                 ; preds = %bb12.us.i.i.i.i
; call core::slice::memchr::memchr_aligned
  %8 = tail call { i64, i64 } @_ZN4core5slice6memchr14memchr_aligned17he27ef990a57e50bcE(i8 noundef %last_byte.us.pre.i.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_45.us.i.i.i.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i.i.i.i), !noalias !985
  br label %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.us.i.i.i.i

bb5.preheader.i.us.i.i.i.i:                       ; preds = %bb12.us.i.i.i.i
  %_64.not.i.us.i.i.i.i = icmp eq i64 %new_len.us.i.i.i.i, 0
  br i1 %_64.not.i.us.i.i.i.i, label %bb4.i.us.i.i.i.i, label %bb7.i.us.i.i.i.i

bb7.i.us.i.i.i.i:                                 ; preds = %bb5.preheader.i.us.i.i.i.i, %bb9.i.us.i.i.i.i
  %i.sroa.0.05.i.us.i.i.i.i = phi i64 [ %10, %bb9.i.us.i.i.i.i ], [ 0, %bb5.preheader.i.us.i.i.i.i ]
  %9 = getelementptr inbounds nuw i8, ptr %_45.us.i.i.i.i, i64 %i.sroa.0.05.i.us.i.i.i.i
  %_9.i.us.i.i.i.i = load i8, ptr %9, align 1, !alias.scope !986, !noalias !985, !noundef !10
  %_8.i.us.i.i.i.i = icmp eq i8 %_9.i.us.i.i.i.i, %last_byte.us.pre.i.i.i.i
  br i1 %_8.i.us.i.i.i.i, label %bb4.i.us.i.i.i.i, label %bb9.i.us.i.i.i.i

bb9.i.us.i.i.i.i:                                 ; preds = %bb7.i.us.i.i.i.i
  %10 = add nuw nsw i64 %i.sroa.0.05.i.us.i.i.i.i, 1
  %exitcond.not.i.us.i.i.i.i = icmp eq i64 %10, %new_len.us.i.i.i.i
  br i1 %exitcond.not.i.us.i.i.i.i, label %bb4.i.us.i.i.i.i, label %bb7.i.us.i.i.i.i

bb4.i.us.i.i.i.i:                                 ; preds = %bb9.i.us.i.i.i.i, %bb7.i.us.i.i.i.i, %bb5.preheader.i.us.i.i.i.i
  %i.sroa.0.0.lcssa.i.us.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.us.i.i.i.i ], [ %new_len.us.i.i.i.i, %bb9.i.us.i.i.i.i ], [ %i.sroa.0.05.i.us.i.i.i.i, %bb7.i.us.i.i.i.i ]
  %_0.sroa.0.1.i.us.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.us.i.i.i.i ], [ 0, %bb9.i.us.i.i.i.i ], [ 1, %bb7.i.us.i.i.i.i ]
  %11 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.us.i.i.i.i, 0
  %12 = insertvalue { i64, i64 } %11, i64 %i.sroa.0.0.lcssa.i.us.i.i.i.i, 1
  br label %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.us.i.i.i.i

_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.us.i.i.i.i: ; preds = %bb4.i.us.i.i.i.i, %bb2.i.us.i.i.i.i
  %.merged.i.us.i.i.i.i = phi { i64, i64 } [ %12, %bb4.i.us.i.i.i.i ], [ %8, %bb2.i.us.i.i.i.i ]
  %13 = extractvalue { i64, i64 } %.merged.i.us.i.i.i.i, 0
  %14 = trunc nuw i64 %13 to i1
  br i1 %14, label %bb4.us.i.i.i.i, label %bb10.i.i.i.i

bb4.us.i.i.i.i:                                   ; preds = %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.us.i.i.i.i
  %15 = extractvalue { i64, i64 } %.merged.i.us.i.i.i.i, 1
  %_16.us.i.i.i.i = add i64 %7, 1
  %16 = add i64 %_16.us.i.i.i.i, %15
  store i64 %16, ptr %3, align 8, !alias.scope !981, !noalias !982
  %_17.not.us.i.i.i.i = icmp ult i64 %16, %_12.i.i.i.i
  %_54.not.us.i.i.i.i = icmp ugt i64 %16, %_4.val1.i.i.i
  %or.cond.i.i.i.i = or i1 %_17.not.us.i.i.i.i, %_54.not.us.i.i.i.i
  br i1 %or.cond.i.i.i.i, label %bb9.us.i.i.i.i, label %bb19.us.i.i.i.i

bb19.us.i.i.i.i:                                  ; preds = %bb4.us.i.i.i.i
  %found_char.us.i.i.i.i = sub nuw i64 %16, %_12.i.i.i.i
  %_62.us.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i, i64 %found_char.us.i.i.i.i
  %17 = tail call i32 @memcmp(ptr nonnull %_62.us.i.i.i.i, ptr nonnull %_10.i.i.i.i, i64 %_12.i.i.i.i), !noalias !982
  %_27.us.i.i.i.i = icmp eq i32 %17, 0
  br i1 %_27.us.i.i.i.i, label %bb7.i.i.i, label %bb9.us.i.i.i.i

bb9.us.i.i.i.i:                                   ; preds = %bb19.us.i.i.i.i, %bb4.us.i.i.i.i
  %_42.us.i.i.i.i = icmp ult i64 %index2.i.i.i.i, %16
  br i1 %_42.us.i.i.i.i, label %bb1.i.i.i.i, label %bb12.us.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %bb12.lr.ph.i.i.i.i, %bb9.i.i.i.i
  %18 = phi i64 [ %27, %bb9.i.i.i.i ], [ %.promoted.i.i.i.i, %bb12.lr.ph.i.i.i.i ]
  %new_len.i.i.i.i = sub nuw i64 %index2.i.i.i.i, %18
  %_45.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i, i64 %18
  %_3.i.i.i.i.i = icmp samesign ult i64 %new_len.i.i.i.i, 16
  br i1 %_3.i.i.i.i.i, label %bb5.preheader.i.i.i.i.i, label %bb2.i.i.i.i.i

bb5.preheader.i.i.i.i.i:                          ; preds = %bb12.i.i.i.i
  %_64.not.i.i.i.i.i = icmp eq i64 %new_len.i.i.i.i, 0
  br i1 %_64.not.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb7.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
; call core::slice::memchr::memchr_aligned
  %19 = tail call { i64, i64 } @_ZN4core5slice6memchr14memchr_aligned17he27ef990a57e50bcE(i8 noundef %last_byte.us.pre.i.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_45.i.i.i.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i.i.i.i), !noalias !985
  br label %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb9.i.i.i.i.i, %bb7.i.i.i.i.i, %bb5.preheader.i.i.i.i.i
  %i.sroa.0.0.lcssa.i.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.i.i.i.i ], [ %new_len.i.i.i.i, %bb9.i.i.i.i.i ], [ %i.sroa.0.05.i.i.i.i.i, %bb7.i.i.i.i.i ]
  %_0.sroa.0.1.i.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.i.i.i.i ], [ 0, %bb9.i.i.i.i.i ], [ 1, %bb7.i.i.i.i.i ]
  %20 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.i.i.i.i, 0
  %21 = insertvalue { i64, i64 } %20, i64 %i.sroa.0.0.lcssa.i.i.i.i.i, 1
  br label %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.i.i.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb5.preheader.i.i.i.i.i, %bb9.i.i.i.i.i
  %i.sroa.0.05.i.i.i.i.i = phi i64 [ %23, %bb9.i.i.i.i.i ], [ 0, %bb5.preheader.i.i.i.i.i ]
  %22 = getelementptr inbounds nuw i8, ptr %_45.i.i.i.i, i64 %i.sroa.0.05.i.i.i.i.i
  %_9.i.i.i.i.i = load i8, ptr %22, align 1, !alias.scope !986, !noalias !985, !noundef !10
  %_8.i.i.i.i.i = icmp eq i8 %_9.i.i.i.i.i, %last_byte.us.pre.i.i.i.i
  br i1 %_8.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb9.i.i.i.i.i

bb9.i.i.i.i.i:                                    ; preds = %bb7.i.i.i.i.i
  %23 = add nuw nsw i64 %i.sroa.0.05.i.i.i.i.i, 1
  %exitcond.not.i.i.i.i.i = icmp eq i64 %23, %new_len.i.i.i.i
  br i1 %exitcond.not.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb7.i.i.i.i.i

_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.i.i.i.i: ; preds = %bb4.i.i.i.i.i, %bb2.i.i.i.i.i
  %.merged.i.i.i.i.i = phi { i64, i64 } [ %21, %bb4.i.i.i.i.i ], [ %19, %bb2.i.i.i.i.i ]
  %24 = extractvalue { i64, i64 } %.merged.i.i.i.i.i, 0
  %25 = trunc nuw i64 %24 to i1
  br i1 %25, label %bb4.i.i.i.i, label %bb10.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.i.i.i.i
  %26 = extractvalue { i64, i64 } %.merged.i.i.i.i.i, 1
  %_16.i.i.i.i = add i64 %18, 1
  %27 = add i64 %_16.i.i.i.i, %26
  store i64 %27, ptr %3, align 8, !alias.scope !981, !noalias !982
  %_17.not.i.i.i.i = icmp ult i64 %27, %_12.i.i.i.i
  %_54.not.i.i.i.i = icmp ugt i64 %27, %_4.val1.i.i.i
  %or.cond69.i.i.i.i = or i1 %_17.not.i.i.i.i, %_54.not.i.i.i.i
  br i1 %or.cond69.i.i.i.i, label %bb9.i.i.i.i, label %bb25.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.i.i.i.i, %_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E.exit.us.i.i.i.i
  store i64 %index2.i.i.i.i, ptr %3, align 8, !alias.scope !981, !noalias !982
  br label %bb1.i.i.i.i

bb9.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  %_42.i.i.i.i = icmp ult i64 %index2.i.i.i.i, %27
  br i1 %_42.i.i.i.i, label %bb1.i.i.i.i, label %bb12.i.i.i.i

bb25.i.i.i.i:                                     ; preds = %bb4.i.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_ZN4core5slice5index16slice_index_fail17h30ecc7bdca4bc32bE(i64 noundef 0, i64 noundef %_12.i.i.i.i, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aa47418bb16f08c24f7eacc7bfd02189) #31, !noalias !985
  unreachable

bb7.i.i.i:                                        ; preds = %bb19.us.i.i.i.i
  %i.i.i.i = load i64, ptr %self, align 8, !alias.scope !977, !noundef !10
  %new_len.i.i.i = sub nuw i64 %16, %i.i.i.i
  store i64 %16, ptr %self, align 8, !alias.scope !977
  br label %bb5.i

bb1.i.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb9.us.i.i.i.i, %bb10.i.i.i.i, %bb2.i.i.i
  store i8 1, ptr %0, align 1, !alias.scope !989
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %29 = load i8, ptr %28, align 8, !range !8, !alias.scope !989, !noundef !10
  %_3.i.i.i.i = trunc nuw i8 %29 to i1
  %i.pre.i.i.i.i = load i64, ptr %self, align 8, !alias.scope !989
  %.phi.trans.insert.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i.i, align 8, !alias.scope !989
  %_4.not.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i, %i.pre.i.i.i.i
  %or.cond.not.i.i.i.i = select i1 %_3.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i
  %new_len.i4.i.i.i = sub nuw i64 %i1.pre.i.i.i.i, %i.pre.i.i.i.i
  br i1 %or.cond.not.i.i.i.i, label %bb5.i, label %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2096107455fc8217E.exit"

bb5.i:                                            ; preds = %bb1.i.i.i.i, %bb7.i.i.i
  %_0.sroa.4.1.i.i.i = phi i64 [ %new_len.i.i.i, %bb7.i.i.i ], [ %new_len.i4.i.i.i, %bb1.i.i.i.i ]
  %i.i.i.pn.i = phi i64 [ %i.i.i.i, %bb7.i.i.i ], [ %i.pre.i.i.i.i, %bb1.i.i.i.i ]
  %_0.sroa.0.1.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i, i64 %i.i.i.pn.i
  %30 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.1.i.i.i, 0
  %31 = insertvalue { ptr, i64 } %30, i64 %_0.sroa.4.1.i.i.i, 1
  %_5.not.i.i.i.i.i = icmp eq i64 %_0.sroa.4.1.i.i.i, 0
  br i1 %_5.not.i.i.i.i.i, label %"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i", label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i.i.i.i"

"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i.i.i.i": ; preds = %bb5.i
  %32 = getelementptr i8, ptr %_0.sroa.0.1.i.i.i, i64 %_0.sroa.4.1.i.i.i
  %_16.i.i.i.i.i = getelementptr i8, ptr %32, i64 -1
  %_16.val.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !alias.scope !992, !noalias !1001
  %_16.val.i.i.fr.i.i.i = freeze i8 %_16.val.i.i.i.i.i
  %33 = icmp eq i8 %_16.val.i.i.fr.i.i.i, 10
  %i.i.i.i.i = add i64 %_0.sroa.4.1.i.i.i, -1
  br i1 %33, label %bb1.i.i.i, label %"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i"

bb1.i.i.i:                                        ; preds = %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i.i.i.i"
  %34 = insertvalue { ptr, i64 } %31, i64 %i.i.i.i.i, 1
  %_5.not.i.i12.i.i.i = icmp eq i64 %i.i.i.i.i, 0
  br i1 %_5.not.i.i12.i.i.i, label %"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E.exit21.i.i.i", label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i13.i.i.i"

"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i13.i.i.i": ; preds = %bb1.i.i.i
  %35 = getelementptr i8, ptr %_0.sroa.0.1.i.i.i, i64 %i.i.i.i.i
  %_16.i.i14.i.i.i = getelementptr i8, ptr %35, i64 -1
  %_16.val.i.i16.i.i.i = load i8, ptr %_16.i.i14.i.i.i, align 1, !alias.scope !1004, !noalias !1009
  %_16.val.i.i16.fr.i.i.i = freeze i8 %_16.val.i.i16.i.i.i
  %36 = icmp eq i8 %_16.val.i.i16.fr.i.i.i, 13
  %i.i17.i.i.i = add i64 %_0.sroa.4.1.i.i.i, -2
  %spec.select.i19.i.i.i = select i1 %36, ptr %_0.sroa.0.1.i.i.i, ptr null
  br label %"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E.exit21.i.i.i"

"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E.exit21.i.i.i": ; preds = %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i13.i.i.i", %bb1.i.i.i
  %i4.i20.i.i.i = phi i64 [ %i.i17.i.i.i, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i13.i.i.i" ], [ -1, %bb1.i.i.i ]
  %37 = phi ptr [ %spec.select.i19.i.i.i, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i13.i.i.i" ], [ null, %bb1.i.i.i ]
  %38 = insertvalue { ptr, i64 } poison, ptr %37, 0
  %39 = insertvalue { ptr, i64 } %38, i64 %i4.i20.i.i.i, 1
  %.not11.i.i.i = icmp eq ptr %37, null
  %..i.i.i = select i1 %.not11.i.i.i, { ptr, i64 } %34, { ptr, i64 } %39
  br label %"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i"

"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i": ; preds = %"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E.exit21.i.i.i", %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i.i.i.i", %bb5.i
  %.merged.i.i.i = phi { ptr, i64 } [ %..i.i.i, %"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E.exit21.i.i.i" ], [ %31, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E.exit.i.i.i.i" ], [ %31, %bb5.i ]
  %_7.0.i = extractvalue { ptr, i64 } %.merged.i.i.i, 0
  %_7.1.i = extractvalue { ptr, i64 } %.merged.i.i.i, 1
  br label %"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2096107455fc8217E.exit"

"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2096107455fc8217E.exit": ; preds = %start, %bb1.i.i.i.i, %"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i"
  %_0.sroa.3.0.i = phi i64 [ %_7.1.i, %"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i" ], [ undef, %start ], [ undef, %bb1.i.i.i.i ]
  %_0.sroa.0.0.i = phi ptr [ %_7.0.i, %"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E.exit.i" ], [ null, %start ], [ null, %bb1.i.i.i.i ]
  %40 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i, 0
  %41 = insertvalue { ptr, i64 } %40, i64 %_0.sroa.3.0.i, 1
  ret { ptr, i64 } %41
}

; hashbrown::map::HashMap<K,V,S,A>::insert
; Function Attrs: uwtable
define internal fastcc void @"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h246fc1cc5c01f60cE"(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(48) %self, i32 noundef range(i32 0, 1114112) %0) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %hash_builder = getelementptr inbounds nuw i8, ptr %self, i64 32
  %hash_builder.val = load i64, ptr %hash_builder, align 8, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %hash_builder.val2 = load i64, ptr %1, align 8, !noundef !10
  %2 = xor i64 %hash_builder.val, 8317987319222330741
  %3 = xor i64 %hash_builder.val2, 7237128888997146477
  %4 = xor i64 %hash_builder.val, 7816392313619706465
  %.pre.i.i.i.i = zext nneg i32 %0 to i64
  %b.i.i.i = or disjoint i64 %.pre.i.i.i.i, 288230376151711744
  %5 = xor i64 %hash_builder.val2, %.pre.i.i.i.i
  %6 = xor i64 %5, 8098989879002948979
  %_2.i.i.i.i = add i64 %3, %2
  %_5.i.i.i3.i = add i64 %6, %4
  %7 = tail call noundef i64 @llvm.fshl.i64(i64 %3, i64 %3, i64 13)
  %8 = xor i64 %7, %_2.i.i.i.i
  %9 = tail call noundef i64 @llvm.fshl.i64(i64 %6, i64 %6, i64 16)
  %10 = xor i64 %9, %_5.i.i.i3.i
  %11 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i, i64 %_2.i.i.i.i, i64 32)
  %_16.i.i.i.i = add i64 %8, %_5.i.i.i3.i
  %_19.i.i.i.i = add i64 %10, %11
  %12 = tail call noundef i64 @llvm.fshl.i64(i64 %8, i64 %8, i64 17)
  %13 = xor i64 %_16.i.i.i.i, %12
  %14 = tail call noundef i64 @llvm.fshl.i64(i64 %10, i64 %10, i64 21)
  %15 = xor i64 %14, %_19.i.i.i.i
  %16 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i, i64 %_16.i.i.i.i, i64 32)
  %17 = xor i64 %_19.i.i.i.i, %b.i.i.i
  %18 = xor i64 %16, 255
  %_2.i3.i.i.i = add i64 %17, %13
  %_5.i6.i.i.i = add i64 %18, %15
  %19 = tail call noundef i64 @llvm.fshl.i64(i64 %13, i64 %13, i64 13)
  %20 = xor i64 %_2.i3.i.i.i, %19
  %21 = tail call noundef i64 @llvm.fshl.i64(i64 %15, i64 %15, i64 16)
  %22 = xor i64 %_5.i6.i.i.i, %21
  %23 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i, i64 %_2.i3.i.i.i, i64 32)
  %_16.i7.i.i.i = add i64 %20, %_5.i6.i.i.i
  %_19.i8.i.i.i = add i64 %22, %23
  %24 = tail call noundef i64 @llvm.fshl.i64(i64 %20, i64 %20, i64 17)
  %25 = xor i64 %_16.i7.i.i.i, %24
  %26 = tail call noundef i64 @llvm.fshl.i64(i64 %22, i64 %22, i64 21)
  %27 = xor i64 %26, %_19.i8.i.i.i
  %28 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i, i64 %_16.i7.i.i.i, i64 32)
  %_30.i.i.i.i = add i64 %25, %_19.i8.i.i.i
  %_33.i.i.i.i = add i64 %27, %28
  %29 = tail call noundef i64 @llvm.fshl.i64(i64 %25, i64 %25, i64 13)
  %30 = xor i64 %29, %_30.i.i.i.i
  %31 = tail call noundef i64 @llvm.fshl.i64(i64 %27, i64 %27, i64 16)
  %32 = xor i64 %31, %_33.i.i.i.i
  %33 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i, i64 %_30.i.i.i.i, i64 32)
  %_44.i.i.i.i = add i64 %30, %_33.i.i.i.i
  %_47.i.i.i.i = add i64 %32, %33
  %34 = tail call noundef i64 @llvm.fshl.i64(i64 %30, i64 %30, i64 17)
  %35 = xor i64 %34, %_44.i.i.i.i
  %36 = tail call noundef i64 @llvm.fshl.i64(i64 %32, i64 %32, i64 21)
  %37 = xor i64 %36, %_47.i.i.i.i
  %38 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i, i64 %_44.i.i.i.i, i64 32)
  %_58.i.i.i.i = add i64 %35, %_47.i.i.i.i
  %_61.i.i.i.i = add i64 %37, %38
  %39 = tail call noundef i64 @llvm.fshl.i64(i64 %35, i64 %35, i64 13)
  %40 = xor i64 %39, %_58.i.i.i.i
  %41 = tail call noundef i64 @llvm.fshl.i64(i64 %37, i64 %37, i64 16)
  %42 = xor i64 %41, %_61.i.i.i.i
  %_72.i.i.i.i = add i64 %40, %_61.i.i.i.i
  %43 = tail call noundef i64 @llvm.fshl.i64(i64 %40, i64 %40, i64 17)
  %44 = tail call noundef i64 @llvm.fshl.i64(i64 %42, i64 %42, i64 21)
  %45 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i, i64 %_72.i.i.i.i, i64 32)
  %46 = xor i64 %43, %44
  %47 = xor i64 %46, %45
  %_0.i.i.i = xor i64 %47, %_72.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1012)
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_5.i.i = load i64, ptr %48, align 8, !alias.scope !1015, !noalias !1018, !noundef !10
  %b.i.i = icmp eq i64 %_5.i.i, 0
  br i1 %b.i.i, label %bb8.i.i, label %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i", !prof !320

bb8.i.i:                                          ; preds = %start
; call hashbrown::raw::RawTable<T,A>::reserve_rehash
  %49 = tail call { i64, i64 } @"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E"(ptr noalias noundef nonnull align 8 dereferenceable(32) %self, i64 noundef 1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %hash_builder, i1 noundef zeroext true), !noalias !1022
  %_8.0.i.i = extractvalue { i64, i64 } %49, 0
  %50 = icmp eq i64 %_8.0.i.i, -9223372036854775807
  tail call void @llvm.assume(i1 %50)
  br label %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i"

"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i": ; preds = %bb8.i.i, %start
  %self.val.i = load ptr, ptr %self, align 8, !alias.scope !1012, !noalias !1023, !nonnull !10, !noundef !10
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val4.i = load i64, ptr %51, align 8, !alias.scope !1012, !noalias !1023, !noundef !10
  %_30.i.i = lshr i64 %_0.i.i.i, 57
  %tag_hash.i.i = trunc nuw nsw i64 %_30.i.i to i8
  %.sroa.0.0.vec.insert.i.i.i = insertelement <16 x i8> poison, i8 %tag_hash.i.i, i64 0
  %.sroa.0.15.vec.insert.i.i.i = shufflevector <16 x i8> %.sroa.0.0.vec.insert.i.i.i, <16 x i8> poison, <16 x i32> zeroinitializer
  %invariant.gep.i.i = getelementptr i8, ptr %self.val.i, i64 -4
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb9.i.i, %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i"
  %hash.pn.i.i = phi i64 [ %_0.i.i.i, %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i" ], [ %63, %bb9.i.i ]
  %insert_index.sroa.4.0.i.i = phi i64 [ undef, %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i" ], [ %insert_index.sroa.4.117.i.i, %bb9.i.i ]
  %insert_index.sroa.0.0.i.i = phi i64 [ 0, %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i" ], [ %insert_index.sroa.0.119.i.i, %bb9.i.i ]
  %52 = phi i64 [ 0, %"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E.exit.i" ], [ %62, %bb9.i.i ]
  %probe_seq.sroa.0.0.i.i = and i64 %hash.pn.i.i, %self.val4.i
  %_36.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %probe_seq.sroa.0.0.i.i
  %dst.sroa.0.0.copyload.i24.i.i = load <16 x i8>, ptr %_36.i.i, align 1, !noalias !1024
  %53 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i24.i.i, %.sroa.0.15.vec.insert.i.i.i
  %54 = bitcast <16 x i1> %53 to i16
  %.not25.i.i = icmp eq i16 %54, 0
  br i1 %.not25.i.i, label %bb16.i.i, label %bb15.i.i

bb15.i.i:                                         ; preds = %bb1.i.i, %bb19.i.i
  %iter.sroa.0.026.i.i = phi i16 [ %_60.i.i, %bb19.i.i ], [ %54, %bb1.i.i ]
  %55 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %iter.sroa.0.026.i.i, i1 true)
  %_51.i.i = zext nneg i16 %55 to i64
  %_14.i.i = add i64 %probe_seq.sroa.0.0.i.i, %_51.i.i
  %index6.i.i = and i64 %_14.i.i, %self.val4.i
  %_18.i.i.i = sub nsw i64 0, %index6.i.i
  %gep.i.i = getelementptr i32, ptr %invariant.gep.i.i, i64 %_18.i.i.i
  %.val.i.i.i = load i32, ptr %gep.i.i, align 4, !range !747, !noalias !1027, !noundef !10
  %_0.i.i.i.i.i.i = icmp eq i32 %0, %.val.i.i.i
  br i1 %_0.i.i.i.i.i.i, label %bb4, label %bb19.i.i, !prof !180

bb16.i.i:                                         ; preds = %bb19.i.i, %bb1.i.i
  %_64.not.i.i = icmp eq i64 %insert_index.sroa.0.0.i.i, 1
  br i1 %_64.not.i.i, label %bb7.i.i, label %bb21.i.i, !prof !320

bb19.i.i:                                         ; preds = %bb15.i.i
  %56 = add i16 %iter.sroa.0.026.i.i, -1
  %_60.i.i = and i16 %56, %iter.sroa.0.026.i.i
  %.not.i.i = icmp eq i16 %_60.i.i, 0
  br i1 %.not.i.i, label %bb16.i.i, label %bb15.i.i

bb21.i.i:                                         ; preds = %bb16.i.i
  %57 = icmp slt <16 x i8> %dst.sroa.0.0.copyload.i24.i.i, zeroinitializer
  %58 = bitcast <16 x i1> %57 to i16
  %.not.i.i.i = icmp eq i16 %58, 0
  br i1 %.not.i.i.i, label %bb9.i.i, label %bb6.thread21.i.i, !prof !320

bb6.thread21.i.i:                                 ; preds = %bb21.i.i
  %59 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %58, i1 true)
  %_17.i.i.i = zext nneg i16 %59 to i64
  %_8.i.i.i = add i64 %probe_seq.sroa.0.0.i.i, %_17.i.i.i
  %_7.i.i.i = and i64 %_8.i.i.i, %self.val4.i
  br label %bb7.i.i

bb7.i.i:                                          ; preds = %bb6.thread21.i.i, %bb16.i.i
  %insert_index.sroa.4.118.i.i = phi i64 [ %_7.i.i.i, %bb6.thread21.i.i ], [ %insert_index.sroa.4.0.i.i, %bb16.i.i ]
  %60 = icmp eq <16 x i8> %dst.sroa.0.0.copyload.i24.i.i, splat (i8 -1)
  %61 = bitcast <16 x i1> %60 to i16
  %b11.not.i.i = icmp eq i16 %61, 0
  br i1 %b11.not.i.i, label %bb9.i.i, label %bb27.i.i, !prof !320

bb9.i.i:                                          ; preds = %bb7.i.i, %bb21.i.i
  %insert_index.sroa.0.119.i.i = phi i64 [ 1, %bb7.i.i ], [ 0, %bb21.i.i ]
  %insert_index.sroa.4.117.i.i = phi i64 [ %insert_index.sroa.4.118.i.i, %bb7.i.i ], [ undef, %bb21.i.i ]
  %62 = add i64 %52, 16
  %63 = add i64 %62, %probe_seq.sroa.0.0.i.i
  br label %bb1.i.i

bb27.i.i:                                         ; preds = %bb7.i.i
  %_8.i20.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %insert_index.sroa.4.118.i.i
  %_12.i21.i.i = load i8, ptr %_8.i20.i.i, align 1, !noalias !1030, !noundef !10
  %b.i.i.i3 = icmp sgt i8 %_12.i21.i.i, -1
  br i1 %b.i.i.i3, label %bb2.i.i.i, label %bb2, !prof !320

bb2.i.i.i:                                        ; preds = %bb27.i.i
  %64 = load <16 x i8>, ptr %self.val.i, align 16, !noalias !1031
  %65 = icmp slt <16 x i8> %64, zeroinitializer
  %66 = bitcast <16 x i1> %65 to i16
  %67 = icmp ne i16 %66, 0
  tail call void @llvm.assume(i1 %67)
  %68 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %66, i1 true)
  %_25.i.i.i = zext nneg i16 %68 to i64
  %_9.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %_25.i.i.i
  %old_ctrl.i.pre = load i8, ptr %_9.i.phi.trans.insert, align 1, !noalias !1034
  br label %bb2

bb2:                                              ; preds = %bb27.i.i, %bb2.i.i.i
  %old_ctrl.i = phi i8 [ %_12.i21.i.i, %bb27.i.i ], [ %old_ctrl.i.pre, %bb2.i.i.i ]
  %_0.sroa.3.0.i.ph.i = phi i64 [ %insert_index.sroa.4.118.i.i, %bb27.i.i ], [ %_25.i.i.i, %bb2.i.i.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1034)
  %_9.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %_0.sroa.3.0.i.ph.i
  %_14.i = and i8 %old_ctrl.i, 1
  %_12.i = zext nneg i8 %_14.i to i64
  %_17.i = add i64 %_0.sroa.3.0.i.ph.i, -16
  %_16.i = and i64 %_17.i, %self.val4.i
  store i8 %tag_hash.i.i, ptr %_9.i, align 1, !noalias !1034
  %69 = getelementptr i8, ptr %self.val.i, i64 %_16.i
  %_24.i = getelementptr i8, ptr %69, i64 16
  store i8 %tag_hash.i.i, ptr %_24.i, align 1, !noalias !1034
  %70 = load <2 x i64>, ptr %48, align 8, !alias.scope !1034
  %71 = insertelement <2 x i64> <i64 poison, i64 -1>, i64 %_12.i, i64 0
  %72 = sub <2 x i64> %70, %71
  store <2 x i64> %72, ptr %48, align 8, !alias.scope !1034
  %_37.i = sub nsw i64 0, %_0.sroa.3.0.i.ph.i
  %73 = getelementptr inbounds i32, ptr %self.val.i, i64 %_37.i
  %74 = getelementptr inbounds i8, ptr %73, i64 -4
  store i32 %0, ptr %74, align 4, !noalias !1034
  br label %bb4

bb4:                                              ; preds = %bb15.i.i, %bb2
  ret void
}

; hashbrown::raw::RawTable<T,A>::reserve_rehash
; Function Attrs: cold noinline uwtable
define { i64, i64 } @"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h7cea8a49a31e8956E"(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self, i64 noundef %additional, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %0, i1 noundef zeroext %fallibility) unnamed_addr #14 personality ptr @__CxxFrameHandler3 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1037)
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.i = load i64, ptr %1, align 8, !alias.scope !1037, !noalias !1040, !noundef !10
  %_21.0.i = add i64 %self1.i, %additional
  %_21.1.i = icmp ult i64 %_21.0.i, %self1.i
  br i1 %_21.1.i, label %bb9.i, label %bb11.i, !prof !320

bb11.i:                                           ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %3 = load i64, ptr %2, align 8, !alias.scope !1037, !noalias !1040, !noundef !10
  %_24.i = icmp ult i64 %3, 8
  %_26.i = add i64 %3, 1
  %_255.i = lshr i64 %_26.i, 3
  %4 = mul nuw i64 %_255.i, 7
  %full_capacity.sroa.0.0.i = select i1 %_24.i, i64 %3, i64 %4
  %_146.i = lshr i64 %full_capacity.sroa.0.0.i, 1
  %_13.not.i = icmp ugt i64 %_21.0.i, %_146.i
  br i1 %_13.not.i, label %bb4.i, label %bb2.i

bb9.i:                                            ; preds = %start
; call hashbrown::raw::Fallibility::capacity_overflow
  %5 = tail call { i64, i64 } @_ZN9hashbrown3raw11Fallibility17capacity_overflow17h167bbd1a91e4964eE(i1 noundef zeroext %fallibility), !noalias !1042
  %_11.0.i = extractvalue { i64, i64 } %5, 0
  %_11.1.i = extractvalue { i64, i64 } %5, 1
  br label %_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit

bb4.i:                                            ; preds = %bb11.i
  %_19.i = add nuw i64 %full_capacity.sroa.0.0.i, 1
  %..i = tail call noundef i64 @llvm.umax.i64(i64 %_19.i, i64 %_21.0.i)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1043)
  %_6.i.i = icmp ult i64 %..i, 15
  br i1 %_6.i.i, label %bb10.thread.i, label %bb16.i.i6

bb16.i.i6:                                        ; preds = %bb4.i
  %_33.1.i.i = icmp ugt i64 %..i, 2305843009213693951
  br i1 %_33.1.i.i, label %bb9.i9, label %bb10.i, !prof !320

bb10.thread.i:                                    ; preds = %bb4.i
  %_15.i.i = icmp samesign ult i64 %..i, 4
  %6 = and i64 %..i, 8
  %..i.i = add nuw nsw i64 %6, 8
  %buckets.sroa.0.0.i.i = select i1 %_15.i.i, i64 4, i64 %..i.i
  br label %bb11.i.i.i

bb10.i:                                           ; preds = %bb16.i.i6
  %_33.0.i.i = shl nuw i64 %..i, 3
  %adjusted_cap.i.i = udiv i64 %_33.0.i.i, 7
  %p.i.i = add nsw i64 %adjusted_cap.i.i, -1
  %7 = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %p.i.i, i1 true)
  %8 = lshr i64 -1, %7
  %9 = add nuw nsw i64 %8, 1
  %_32.1.i.i.i = icmp samesign ugt i64 %8, 4611686018427387899
  br i1 %_32.1.i.i.i, label %bb3.i.i8, label %bb11.i.i.i, !prof !1046

bb11.i.i.i:                                       ; preds = %bb10.i, %bb10.thread.i
  %_0.sroa.4.0.i.ph8.i = phi i64 [ %buckets.sroa.0.0.i.i, %bb10.thread.i ], [ %9, %bb10.i ]
  %_25.0.i.i.i = shl nuw i64 %_0.sroa.4.0.i.ph8.i, 2
  %_32.0.i.i.i = add nuw i64 %_25.0.i.i.i, 15
  %ctrl_offset.i.i.i = and i64 %_32.0.i.i.i, -16
  %rhs5.i.i.i = add nuw nsw i64 %_0.sroa.4.0.i.ph8.i, 16
  %_37.0.i.i.i = add i64 %ctrl_offset.i.i.i, %rhs5.i.i.i
  %_37.1.i.i.i = icmp ult i64 %_37.0.i.i.i, %ctrl_offset.i.i.i
  %_19.i.i.i = icmp ugt i64 %_37.0.i.i.i, 9223372036854775792
  %or.cond.i.i = or i1 %_37.1.i.i.i, %_19.i.i.i
  br i1 %or.cond.i.i, label %bb3.i.i8, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i", !prof !306

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i": ; preds = %bb11.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #33, !noalias !1047
; call __rustc::__rust_alloc
  %10 = tail call noundef align 16 ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef %_37.0.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33, !noalias !1047
  %11 = icmp eq ptr %10, null
  br i1 %11, label %bb15.i.i7, label %_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit

bb3.i.i8:                                         ; preds = %bb11.i.i.i, %bb10.i
; call hashbrown::raw::Fallibility::capacity_overflow
  %12 = tail call { i64, i64 } @_ZN9hashbrown3raw11Fallibility17capacity_overflow17h167bbd1a91e4964eE(i1 noundef zeroext %fallibility), !noalias !1047
  br label %bb14.i.i

bb15.i.i7:                                        ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i"
; call hashbrown::raw::Fallibility::alloc_err
  %13 = tail call { i64, i64 } @_ZN9hashbrown3raw11Fallibility9alloc_err17h452573c969b6d13aE(i1 noundef zeroext %fallibility, i64 noundef 16, i64 noundef %_37.0.i.i.i), !noalias !1047
  br label %bb14.i.i

bb9.i9:                                           ; preds = %bb16.i.i6
; call hashbrown::raw::Fallibility::capacity_overflow
  %14 = tail call { i64, i64 } @_ZN9hashbrown3raw11Fallibility17capacity_overflow17h167bbd1a91e4964eE(i1 noundef zeroext %fallibility), !noalias !1052
  br label %bb14.i.i

_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h9100bbc99652c246E.exit.i.i"
  %_26.i.i = add nsw i64 %_0.sroa.4.0.i.ph8.i, -1
  %_35.i.i = icmp samesign ult i64 %_26.i.i, 8
  %_367.i.i = lshr i64 %_0.sroa.4.0.i.ph8.i, 3
  %15 = mul nuw nsw i64 %_367.i.i, 7
  %bucket_mask.sroa.0.0.i.i = select i1 %_35.i.i, i64 %_26.i.i, i64 %15
  %ptr.i.i = getelementptr inbounds nuw i8, ptr %10, i64 %ctrl_offset.i.i.i
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 16 dereferenceable(1) %ptr.i.i, i8 -1, i64 %rhs5.i.i.i, i1 false), !noalias !1052
  %16 = ptrtoint ptr %ptr.i.i to i64
  %invariant.gep = getelementptr i8, ptr %ptr.i.i, i64 16
  %_69.not.i.i59 = icmp eq i64 %self1.i, 0
  br i1 %_69.not.i.i59, label %_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit.bb4.i.i_crit_edge, label %bb1.i.preheader.lr.ph

_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit.bb4.i.i_crit_edge: ; preds = %_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit
  %_3.sroa.0.0.copyload.i.i.i.i.i.pre = load i64, ptr %self, align 8, !alias.scope !1053, !noalias !1056
  %_3.sroa.0.0.copyload.i.i.i.i.i.pre.ptr = inttoptr i64 %_3.sroa.0.0.copyload.i.i.i.i.i.pre to ptr
  br label %bb4.i.i

bb1.i.preheader.lr.ph:                            ; preds = %_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit
  %_58.i.i = load ptr, ptr %self, align 8, !alias.scope !1058, !noalias !1059, !nonnull !10, !noundef !10
  %17 = load <16 x i8>, ptr %_58.i.i, align 16, !noalias !1061
  %18 = icmp slt <16 x i8> %17, zeroinitializer
  %19 = bitcast <16 x i1> %18 to i16
  %_67.i.i = xor i16 %19, -1
  %invariant.gep64 = getelementptr i8, ptr %_58.i.i, i64 -4
  %hash_builder.val.i.i = load i64, ptr %0, align 8, !noalias !1064, !noundef !10
  %20 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %hash_builder.val1.i.i = load i64, ptr %20, align 8, !noalias !1064, !noundef !10
  %21 = xor i64 %hash_builder.val.i.i, 8317987319222330741
  %22 = xor i64 %hash_builder.val1.i.i, 7237128888997146477
  %23 = xor i64 %hash_builder.val.i.i, 7816392313619706465
  %_2.i.i.i.i.i.i = add i64 %22, %21
  %24 = tail call noundef i64 @llvm.fshl.i64(i64 %22, i64 %22, i64 13)
  %25 = xor i64 %24, %_2.i.i.i.i.i.i
  %26 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i, i64 32)
  %invariant.op68 = add i64 %23, %25
  %27 = tail call noundef i64 @llvm.fshl.i64(i64 %25, i64 %25, i64 17)
  %invariant.op93 = xor i64 %hash_builder.val1.i.i, 8098989879002948979
  br label %bb1.i.preheader

bb14.i.i:                                         ; preds = %bb3.i.i8, %bb15.i.i7, %bb9.i9
  %.pn = phi { i64, i64 } [ %14, %bb9.i9 ], [ %13, %bb15.i.i7 ], [ %12, %bb3.i.i8 ]
  %self1.sroa.7.0.i.i.ph = extractvalue { i64, i64 } %.pn, 0
  %self1.sroa.9.0.i.i.ph = extractvalue { i64, i64 } %.pn, 1
  br label %_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit

bb1.i.preheader:                                  ; preds = %bb1.i.preheader.lr.ph, %bb22.i.i
  %iter.i.i.sroa.0.063 = phi ptr [ %_58.i.i, %bb1.i.preheader.lr.ph ], [ %iter.i.i.sroa.0.2.lcssa, %bb22.i.i ]
  %iter.i.i.sroa.5.062 = phi i64 [ 0, %bb1.i.preheader.lr.ph ], [ %iter.i.i.sroa.5.2.lcssa, %bb22.i.i ]
  %iter.i.i.sroa.9.061 = phi i64 [ %self1.i, %bb1.i.preheader.lr.ph ], [ %34, %bb22.i.i ]
  %iter.i.i.sroa.13.060 = phi i16 [ %_67.i.i, %bb1.i.preheader.lr.ph ], [ %_31.i, %bb22.i.i ]
  %.not.i54 = icmp eq i16 %iter.i.i.sroa.13.060, 0
  br i1 %.not.i54, label %_42.i.noexc, label %bb18.i.i

_42.i.noexc:                                      ; preds = %bb1.i.preheader, %_42.i.noexc
  %iter.i.i.sroa.0.256 = phi ptr [ %ptr.i, %_42.i.noexc ], [ %iter.i.i.sroa.0.063, %bb1.i.preheader ]
  %iter.i.i.sroa.5.255 = phi i64 [ %31, %_42.i.noexc ], [ %iter.i.i.sroa.5.062, %bb1.i.preheader ]
  %ptr.i = getelementptr inbounds nuw i8, ptr %iter.i.i.sroa.0.256, i64 16
  %28 = load <16 x i8>, ptr %ptr.i, align 16, !noalias !1068
  %29 = icmp slt <16 x i8> %28, zeroinitializer
  %30 = bitcast <16 x i1> %29 to i16
  %31 = add i64 %iter.i.i.sroa.5.255, 16
  %.not.i = icmp eq i16 %30, -1
  br i1 %.not.i, label %_42.i.noexc, label %bb18.i.i.loopexit

bb18.i.i.loopexit:                                ; preds = %_42.i.noexc
  %_43.i = xor i16 %30, -1
  br label %bb18.i.i

bb18.i.i:                                         ; preds = %bb18.i.i.loopexit, %bb1.i.preheader
  %iter.i.i.sroa.13.2.lcssa = phi i16 [ %iter.i.i.sroa.13.060, %bb1.i.preheader ], [ %_43.i, %bb18.i.i.loopexit ]
  %iter.i.i.sroa.5.2.lcssa = phi i64 [ %iter.i.i.sroa.5.062, %bb1.i.preheader ], [ %31, %bb18.i.i.loopexit ]
  %iter.i.i.sroa.0.2.lcssa = phi ptr [ %iter.i.i.sroa.0.063, %bb1.i.preheader ], [ %ptr.i, %bb18.i.i.loopexit ]
  %32 = add i16 %iter.i.i.sroa.13.2.lcssa, -1
  %33 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %iter.i.i.sroa.13.2.lcssa, i1 true)
  %_22.i = zext nneg i16 %33 to i64
  %_31.i = and i16 %32, %iter.i.i.sroa.13.2.lcssa
  %_5.i = add i64 %iter.i.i.sroa.5.2.lcssa, %_22.i
  %34 = add i64 %iter.i.i.sroa.9.061, -1
  %_18.i = sub nsw i64 0, %_5.i
  %gep65 = getelementptr i32, ptr %invariant.gep64, i64 %_18.i
  %.val.i = load i32, ptr %gep65, align 4, !range !747, !alias.scope !1071, !noalias !1074, !noundef !10
  %.pre.i.i.i.i.i.i = zext nneg i32 %.val.i to i64
  %b.i.i.i.i.i = or disjoint i64 %.pre.i.i.i.i.i.i, 288230376151711744
  %.reass67.reass.reass = xor i64 %.pre.i.i.i.i.i.i, %invariant.op93
  %_5.i.i.i3.i.i.i = add i64 %.reass67.reass.reass, %23
  %35 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass67.reass.reass, i64 %.reass67.reass.reass, i64 16)
  %36 = xor i64 %35, %_5.i.i.i3.i.i.i
  %_16.i.i.i.i.i.i.reass = add i64 %.reass67.reass.reass, %invariant.op68
  %_19.i.i.i.i.i.i = add i64 %36, %26
  %37 = xor i64 %_16.i.i.i.i.i.i.reass, %27
  %38 = tail call noundef i64 @llvm.fshl.i64(i64 %36, i64 %36, i64 21)
  %39 = xor i64 %38, %_19.i.i.i.i.i.i
  %40 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.i.reass, i64 %_16.i.i.i.i.i.i.reass, i64 32)
  %41 = xor i64 %_19.i.i.i.i.i.i, %b.i.i.i.i.i
  %42 = xor i64 %40, 255
  %_2.i3.i.i.i.i.i = add i64 %41, %37
  %_5.i6.i.i.i.i.i = add i64 %42, %39
  %43 = tail call noundef i64 @llvm.fshl.i64(i64 %37, i64 %37, i64 13)
  %44 = xor i64 %_2.i3.i.i.i.i.i, %43
  %45 = tail call noundef i64 @llvm.fshl.i64(i64 %39, i64 %39, i64 16)
  %46 = xor i64 %_5.i6.i.i.i.i.i, %45
  %47 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i = add i64 %44, %_5.i6.i.i.i.i.i
  %_19.i8.i.i.i.i.i = add i64 %46, %47
  %48 = tail call noundef i64 @llvm.fshl.i64(i64 %44, i64 %44, i64 17)
  %49 = xor i64 %_16.i7.i.i.i.i.i, %48
  %50 = tail call noundef i64 @llvm.fshl.i64(i64 %46, i64 %46, i64 21)
  %51 = xor i64 %50, %_19.i8.i.i.i.i.i
  %52 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i = add i64 %49, %_19.i8.i.i.i.i.i
  %_33.i.i.i.i.i.i = add i64 %51, %52
  %53 = tail call noundef i64 @llvm.fshl.i64(i64 %49, i64 %49, i64 13)
  %54 = xor i64 %53, %_30.i.i.i.i.i.i
  %55 = tail call noundef i64 @llvm.fshl.i64(i64 %51, i64 %51, i64 16)
  %56 = xor i64 %55, %_33.i.i.i.i.i.i
  %57 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i = add i64 %54, %_33.i.i.i.i.i.i
  %_47.i.i.i.i.i.i = add i64 %56, %57
  %58 = tail call noundef i64 @llvm.fshl.i64(i64 %54, i64 %54, i64 17)
  %59 = xor i64 %58, %_44.i.i.i.i.i.i
  %60 = tail call noundef i64 @llvm.fshl.i64(i64 %56, i64 %56, i64 21)
  %61 = xor i64 %60, %_47.i.i.i.i.i.i
  %62 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i = add i64 %59, %_47.i.i.i.i.i.i
  %_61.i.i.i.i.i.i = add i64 %61, %62
  %63 = tail call noundef i64 @llvm.fshl.i64(i64 %59, i64 %59, i64 13)
  %64 = xor i64 %63, %_58.i.i.i.i.i.i
  %65 = tail call noundef i64 @llvm.fshl.i64(i64 %61, i64 %61, i64 16)
  %66 = xor i64 %65, %_61.i.i.i.i.i.i
  %_72.i.i.i.i.i.i = add i64 %64, %_61.i.i.i.i.i.i
  %67 = tail call noundef i64 @llvm.fshl.i64(i64 %64, i64 %64, i64 17)
  %68 = tail call noundef i64 @llvm.fshl.i64(i64 %66, i64 %66, i64 21)
  %69 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i, i64 32)
  %70 = xor i64 %67, %68
  %71 = xor i64 %70, %69
  %_0.i.i.i.i.i = xor i64 %71, %_72.i.i.i.i.i.i
  %probe_seq.sroa.0.06.i = and i64 %_0.i.i.i.i.i, %_26.i.i
  %_187.i = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %probe_seq.sroa.0.06.i
  %dst.sroa.0.0.copyload.i58.i = load <16 x i8>, ptr %_187.i, align 1, !noalias !1077
  %72 = icmp slt <16 x i8> %dst.sroa.0.0.copyload.i58.i, zeroinitializer
  %73 = bitcast <16 x i1> %72 to i16
  %.not.i9.i = icmp eq i16 %73, 0
  br i1 %.not.i9.i, label %bb6.i, label %bb9.i15, !prof !1080

bb4.i.i:                                          ; preds = %bb22.i.i, %_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit.bb4.i.i_crit_edge
  %_3.sroa.0.0.copyload.i.i.i.i.i.ptr = phi ptr [ %_3.sroa.0.0.copyload.i.i.i.i.i.pre.ptr, %_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E.exit.bb4.i.i_crit_edge ], [ %_58.i.i, %bb22.i.i ]
  %74 = sub i64 %bucket_mask.sroa.0.0.i.i, %self1.i
  store i64 %16, ptr %self, align 8, !alias.scope !1053, !noalias !1056
  store i64 %_26.i.i, ptr %2, align 8, !alias.scope !1081, !noalias !1083
  %_11.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  store i64 %74, ptr %_11.i.i.2.i.i, align 8, !alias.scope !1085, !noalias !1087
  %_3.i.i.i = icmp eq i64 %3, 0
  br i1 %_3.i.i.i, label %_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb4.i.i
  %_10.i.i.i = shl i64 %3, 2
  %75 = add i64 %_10.i.i.i, 4
  %_32.0.i.i.i.i = add i64 %_10.i.i.i, 19
  %_32.1.i.i.i.i = icmp uge i64 %_32.0.i.i.i.i, %75
  tail call void @llvm.assume(i1 %_32.1.i.i.i.i)
  %ctrl_offset.i.i.i.i = and i64 %_32.0.i.i.i.i, -16
  %rhs5.i.i.i.i = add i64 %3, 17
  %_37.0.i.i.i.i = add i64 %rhs5.i.i.i.i, %ctrl_offset.i.i.i.i
  %_37.1.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i, %ctrl_offset.i.i.i.i
  %_19.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i, 9223372036854775793
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i)
  %76 = icmp ne ptr %_3.sroa.0.0.copyload.i.i.i.i.i.ptr, null
  tail call void @llvm.assume(i1 %76)
  %_4.not.i.i.i.i = icmp eq i64 %_37.0.i.i.i.i, 0
  br i1 %_4.not.i.i.i.i, label %_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit, label %bb1.i1.i.i.i

bb1.i1.i.i.i:                                     ; preds = %bb1.i.i.i
  %_18.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i
  %ptr.i.i.i = getelementptr inbounds i8, ptr %_3.sroa.0.0.copyload.i.i.i.i.i.ptr, i64 %_18.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i, i64 noundef %_37.0.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 16) #33, !noalias !1089
  br label %_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit

bb6.i:                                            ; preds = %bb18.i.i, %bb6.i
  %probe_seq.sroa.0.010.i = phi i64 [ %probe_seq.sroa.0.0.i, %bb6.i ], [ %probe_seq.sroa.0.06.i, %bb18.i.i ]
  %77 = phi i64 [ %78, %bb6.i ], [ 0, %bb18.i.i ]
  %78 = add i64 %77, 16
  %79 = add i64 %78, %probe_seq.sroa.0.010.i
  %probe_seq.sroa.0.0.i = and i64 %79, %_26.i.i
  %_18.i17 = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %probe_seq.sroa.0.0.i
  %dst.sroa.0.0.copyload.i5.i = load <16 x i8>, ptr %_18.i17, align 1, !noalias !1077
  %80 = icmp slt <16 x i8> %dst.sroa.0.0.copyload.i5.i, zeroinitializer
  %81 = bitcast <16 x i1> %80 to i16
  %.not.i.i = icmp eq i16 %81, 0
  br i1 %.not.i.i, label %bb6.i, label %bb9.i15, !prof !1094

bb9.i15:                                          ; preds = %bb6.i, %bb18.i.i
  %probe_seq.sroa.0.0.lcssa.i = phi i64 [ %probe_seq.sroa.0.06.i, %bb18.i.i ], [ %probe_seq.sroa.0.0.i, %bb6.i ]
  %.lcssa.i = phi i16 [ %73, %bb18.i.i ], [ %81, %bb6.i ]
  %82 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %.lcssa.i, i1 true)
  %_17.i.i = zext nneg i16 %82 to i64
  %_8.i.i = add nuw nsw i64 %probe_seq.sroa.0.0.lcssa.i, %_17.i.i
  %_7.i.i = and i64 %_8.i.i, %_26.i.i
  %_8.i5.i = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %_7.i.i
  %_12.i.i = load i8, ptr %_8.i5.i, align 1, !noundef !10
  %b.i.i = icmp sgt i8 %_12.i.i, -1
  br i1 %b.i.i, label %bb2.i.i16, label %bb22.i.i, !prof !320

bb2.i.i16:                                        ; preds = %bb9.i15
  %83 = load <16 x i8>, ptr %ptr.i.i, align 16, !noalias !1095
  %84 = icmp slt <16 x i8> %83, zeroinitializer
  %85 = bitcast <16 x i1> %84 to i16
  %86 = icmp ne i16 %85, 0
  tail call void @llvm.assume(i1 %86)
  %87 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %85, i1 true)
  %_25.i.i = zext nneg i16 %87 to i64
  br label %bb22.i.i

bb22.i.i:                                         ; preds = %bb2.i.i16, %bb9.i15
  %index.sroa.0.0.i.i = phi i64 [ %_25.i.i, %bb2.i.i16 ], [ %_7.i.i, %bb9.i15 ]
  %_75.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %index.sroa.0.0.i.i
  %_79.i.i = lshr i64 %_0.i.i.i.i.i, 57
  %_80.i.i = trunc nuw nsw i64 %_79.i.i to i8
  %_84.i.i = add nsw i64 %index.sroa.0.0.i.i, -16
  %_83.i.i = and i64 %_84.i.i, %_26.i.i
  store i8 %_80.i.i, ptr %_75.i.i, align 1
  %gep = getelementptr i8, ptr %invariant.gep, i64 %_83.i.i
  store i8 %_80.i.i, ptr %gep, align 1
  %_96.i.i = shl i64 %_5.i, 2
  %_98.i.i = sub nuw nsw i64 -4, %_96.i.i
  %_23.i.i = getelementptr inbounds i8, ptr %_58.i.i, i64 %_98.i.i
  %_102.i.i = shl nuw i64 %index.sroa.0.0.i.i, 2
  %_104.i.i = sub nuw nsw i64 -4, %_102.i.i
  %dst.i.i = getelementptr inbounds i8, ptr %ptr.i.i, i64 %_104.i.i
  %88 = load i32, ptr %_23.i.i, align 1
  store i32 %88, ptr %dst.i.i, align 4
  %_69.not.i.i = icmp eq i64 %34, 0
  br i1 %_69.not.i.i, label %bb4.i.i, label %bb1.i.preheader

bb2.i:                                            ; preds = %bb11.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1098)
  %self.val.i = load ptr, ptr %self, align 8, !alias.scope !1098
  %_27.not4.i.i = icmp eq i64 %_26.i, 0
  br i1 %_27.not4.i.i, label %_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.thread19.i, label %bb6.lr.ph.i.i

_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.thread19.i: ; preds = %bb2.i
  %89 = icmp ne ptr %self.val.i, null
  tail call void @llvm.assume(i1 %89)
  br label %_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E.exit

bb6.lr.ph.i.i:                                    ; preds = %bb2.i
  %d9.i.i.i.i = lshr i64 %_26.i, 4
  %r2.i.i.i.i = and i64 %_26.i, 15
  %_19.not.i.i.i.i = icmp ne i64 %r2.i.i.i.i, 0
  %90 = zext i1 %_19.not.i.i.i.i to i64
  %yield_count.sroa.0.0.i.i.i.i = add nuw nsw i64 %d9.i.i.i.i, %90
  %91 = icmp ne ptr %self.val.i, null
  tail call void @llvm.assume(i1 %91)
  %xtraiter = and i64 %yield_count.sroa.0.0.i.i.i.i, 1
  %92 = icmp eq i64 %yield_count.sroa.0.0.i.i.i.i, 1
  br i1 %92, label %bb7.i.i.unr-lcssa, label %bb6.lr.ph.i.i.new

bb6.lr.ph.i.i.new:                                ; preds = %bb6.lr.ph.i.i
  %unroll_iter = and i64 %yield_count.sroa.0.0.i.i.i.i, 2305843009213693950
  %invariant.gep91 = getelementptr i8, ptr %self.val.i, i64 16
  br label %bb6.i.i

bb7.i.i.unr-lcssa:                                ; preds = %bb6.i.i, %bb6.lr.ph.i.i
  %iter.sroa.0.06.i.i.unr = phi i64 [ 0, %bb6.lr.ph.i.i ], [ %_29.i.i.1, %bb6.i.i ]
  %lcmp.mod.not = icmp eq i64 %xtraiter, 0
  br i1 %lcmp.mod.not, label %bb7.i.i, label %bb6.i.i.epil

bb6.i.i.epil:                                     ; preds = %bb7.i.i.unr-lcssa
  %_34.i.i.epil = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %iter.sroa.0.06.i.i.unr
  %93 = load <16 x i8>, ptr %_34.i.i.epil, align 16, !noalias !1101
  %.lobit.i.i.i.epil = ashr <16 x i8> %93, splat (i8 7)
  %94 = bitcast <16 x i8> %.lobit.i.i.i.epil to <2 x i64>
  %95 = or <2 x i64> %94, splat (i64 -9187201950435737472)
  store <2 x i64> %95, ptr %_34.i.i.epil, align 16, !noalias !1104
  br label %bb7.i.i

bb7.i.i:                                          ; preds = %bb7.i.i.unr-lcssa, %bb6.i.i.epil
  %b.i.i18 = icmp ult i64 %_26.i, 16
  br i1 %b.i.i18, label %_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.i, label %bb4.lr.ph.i, !prof !1107

bb6.i.i:                                          ; preds = %bb6.i.i, %bb6.lr.ph.i.i.new
  %iter.sroa.0.06.i.i = phi i64 [ 0, %bb6.lr.ph.i.i.new ], [ %_29.i.i.1, %bb6.i.i ]
  %niter = phi i64 [ 0, %bb6.lr.ph.i.i.new ], [ %niter.next.1, %bb6.i.i ]
  %_34.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %iter.sroa.0.06.i.i
  %96 = load <16 x i8>, ptr %_34.i.i, align 16, !noalias !1101
  %.lobit.i.i.i = ashr <16 x i8> %96, splat (i8 7)
  %97 = bitcast <16 x i8> %.lobit.i.i.i to <2 x i64>
  %98 = or <2 x i64> %97, splat (i64 -9187201950435737472)
  store <2 x i64> %98, ptr %_34.i.i, align 16, !noalias !1104
  %_29.i.i.1 = add i64 %iter.sroa.0.06.i.i, 32
  %gep92 = getelementptr i8, ptr %invariant.gep91, i64 %iter.sroa.0.06.i.i
  %99 = load <16 x i8>, ptr %gep92, align 16, !noalias !1101
  %.lobit.i.i.i.1 = ashr <16 x i8> %99, splat (i8 7)
  %100 = bitcast <16 x i8> %.lobit.i.i.i.1 to <2 x i64>
  %101 = or <2 x i64> %100, splat (i64 -9187201950435737472)
  store <2 x i64> %101, ptr %gep92, align 16, !noalias !1104
  %niter.next.1 = add i64 %niter, 2
  %niter.ncmp.1 = icmp eq i64 %niter.next.1, %unroll_iter
  br i1 %niter.ncmp.1, label %bb7.i.i.unr-lcssa, label %bb6.i.i

_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.i: ; preds = %bb7.i.i
  br label %bb4.lr.ph.i

bb4.lr.ph.i:                                      ; preds = %_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.i, %bb7.i.i
  %.sink.i = phi i64 [ 16, %_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.i ], [ %_26.i, %bb7.i.i ]
  %_4.i.sink.i = phi i64 [ %_26.i, %_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.i ], [ 16, %bb7.i.i ]
  %_53.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %.sink.i
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %_53.i.i, ptr nonnull align 1 %self.val.i, i64 %_4.i.sink.i, i1 false), !noalias !1098
  %invariant.gep8.i = getelementptr i8, ptr %self.val.i, i64 -4
  %invariant.gep.i = getelementptr i8, ptr %self.val.i, i64 16
  %hash_builder.val.i.i.i = load i64, ptr %0, align 8
  %102 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %hash_builder.val1.i.i.i = load i64, ptr %102, align 8
  %103 = xor i64 %hash_builder.val.i.i.i, 8317987319222330741
  %104 = xor i64 %hash_builder.val1.i.i.i, 7237128888997146477
  %105 = xor i64 %hash_builder.val.i.i.i, 7816392313619706465
  %_2.i.i.i.i.i.i.i = add i64 %104, %103
  %106 = tail call i64 @llvm.fshl.i64(i64 %104, i64 %104, i64 13)
  %107 = xor i64 %106, %_2.i.i.i.i.i.i.i
  %108 = tail call i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i, i64 32)
  %invariant.op53 = add i64 %105, %107
  %109 = tail call i64 @llvm.fshl.i64(i64 %107, i64 %107, i64 17)
  %invariant.op = xor i64 %hash_builder.val1.i.i.i, 8098989879002948979
  br label %bb4.i19

bb4.i19:                                          ; preds = %bb14.i, %bb4.lr.ph.i
  %iter.sroa.0.17.i = phi i64 [ 1, %bb4.lr.ph.i ], [ %iter.sroa.0.1.i, %bb14.i ]
  %iter.sroa.0.06.i = phi i64 [ 0, %bb4.lr.ph.i ], [ %iter.sroa.0.17.i, %bb14.i ]
  %_69.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %iter.sroa.0.06.i
  %_73.i = load i8, ptr %_69.i, align 1, !noalias !1098, !noundef !10
  %_72.not.i = icmp eq i8 %_73.i, -128
  br i1 %_72.not.i, label %bb7.i, label %bb14.i

bb7.i:                                            ; preds = %bb4.i19
  %_76.i = shl i64 %iter.sroa.0.06.i, 2
  %_78.i = sub nuw nsw i64 -4, %_76.i
  %i_p.i = getelementptr inbounds i8, ptr %self.val.i, i64 %_78.i
  %_18.i.i = sub nsw i64 0, %iter.sroa.0.06.i
  %gep9.i = getelementptr i32, ptr %invariant.gep8.i, i64 %_18.i.i
  br label %bb9.i20

bb9.i20:                                          ; preds = %bb12.i22, %bb7.i
  %.val.i.i = load i32, ptr %gep9.i, align 4, !range !747, !alias.scope !1108, !noalias !1111, !noundef !10
  %.pre.i.i.i.i.i.i.i = zext nneg i32 %.val.i.i to i64
  %b.i.i.i.i.i.i = or disjoint i64 %.pre.i.i.i.i.i.i.i, 288230376151711744
  %.reass.reass.reass = xor i64 %.pre.i.i.i.i.i.i.i, %invariant.op
  %_5.i.i.i3.i.i.i.i = add i64 %.reass.reass.reass, %105
  %110 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass.reass.reass, i64 %.reass.reass.reass, i64 16)
  %111 = xor i64 %110, %_5.i.i.i3.i.i.i.i
  %_16.i.i.i.i.i.i.i.reass = add i64 %.reass.reass.reass, %invariant.op53
  %_19.i.i.i.i.i.i.i = add i64 %111, %108
  %112 = xor i64 %_16.i.i.i.i.i.i.i.reass, %109
  %113 = tail call noundef i64 @llvm.fshl.i64(i64 %111, i64 %111, i64 21)
  %114 = xor i64 %113, %_19.i.i.i.i.i.i.i
  %115 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.i.i.reass, i64 %_16.i.i.i.i.i.i.i.reass, i64 32)
  %116 = xor i64 %_19.i.i.i.i.i.i.i, %b.i.i.i.i.i.i
  %117 = xor i64 %115, 255
  %_2.i3.i.i.i.i.i.i = add i64 %116, %112
  %_5.i6.i.i.i.i.i.i = add i64 %117, %114
  %118 = tail call noundef i64 @llvm.fshl.i64(i64 %112, i64 %112, i64 13)
  %119 = xor i64 %_2.i3.i.i.i.i.i.i, %118
  %120 = tail call noundef i64 @llvm.fshl.i64(i64 %114, i64 %114, i64 16)
  %121 = xor i64 %_5.i6.i.i.i.i.i.i, %120
  %122 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i.i = add i64 %119, %_5.i6.i.i.i.i.i.i
  %_19.i8.i.i.i.i.i.i = add i64 %121, %122
  %123 = tail call noundef i64 @llvm.fshl.i64(i64 %119, i64 %119, i64 17)
  %124 = xor i64 %_16.i7.i.i.i.i.i.i, %123
  %125 = tail call noundef i64 @llvm.fshl.i64(i64 %121, i64 %121, i64 21)
  %126 = xor i64 %125, %_19.i8.i.i.i.i.i.i
  %127 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i.i = add i64 %124, %_19.i8.i.i.i.i.i.i
  %_33.i.i.i.i.i.i.i = add i64 %126, %127
  %128 = tail call noundef i64 @llvm.fshl.i64(i64 %124, i64 %124, i64 13)
  %129 = xor i64 %128, %_30.i.i.i.i.i.i.i
  %130 = tail call noundef i64 @llvm.fshl.i64(i64 %126, i64 %126, i64 16)
  %131 = xor i64 %130, %_33.i.i.i.i.i.i.i
  %132 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i.i = add i64 %129, %_33.i.i.i.i.i.i.i
  %_47.i.i.i.i.i.i.i = add i64 %131, %132
  %133 = tail call noundef i64 @llvm.fshl.i64(i64 %129, i64 %129, i64 17)
  %134 = xor i64 %133, %_44.i.i.i.i.i.i.i
  %135 = tail call noundef i64 @llvm.fshl.i64(i64 %131, i64 %131, i64 21)
  %136 = xor i64 %135, %_47.i.i.i.i.i.i.i
  %137 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i.i = add i64 %134, %_47.i.i.i.i.i.i.i
  %_61.i.i.i.i.i.i.i = add i64 %136, %137
  %138 = tail call noundef i64 @llvm.fshl.i64(i64 %134, i64 %134, i64 13)
  %139 = xor i64 %138, %_58.i.i.i.i.i.i.i
  %140 = tail call noundef i64 @llvm.fshl.i64(i64 %136, i64 %136, i64 16)
  %141 = xor i64 %140, %_61.i.i.i.i.i.i.i
  %_72.i.i.i.i.i.i.i = add i64 %139, %_61.i.i.i.i.i.i.i
  %142 = tail call noundef i64 @llvm.fshl.i64(i64 %139, i64 %139, i64 17)
  %143 = tail call noundef i64 @llvm.fshl.i64(i64 %141, i64 %141, i64 21)
  %144 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i.i, i64 32)
  %145 = xor i64 %142, %143
  %146 = xor i64 %145, %144
  %_0.i.i.i.i.i.i = xor i64 %146, %_72.i.i.i.i.i.i.i
  %probe_seq.sroa.0.06.i.i = and i64 %_0.i.i.i.i.i.i, %3
  %_187.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %probe_seq.sroa.0.06.i.i
  %dst.sroa.0.0.copyload.i58.i.i = load <16 x i8>, ptr %_187.i.i, align 1, !noalias !1117
  %147 = icmp slt <16 x i8> %dst.sroa.0.0.copyload.i58.i.i, zeroinitializer
  %148 = bitcast <16 x i1> %147 to i16
  %.not.i9.i.i = icmp eq i16 %148, 0
  br i1 %.not.i9.i.i, label %bb6.i21.i, label %bb9.i.i, !prof !1080

bb6.i21.i:                                        ; preds = %bb9.i20, %bb6.i21.i
  %probe_seq.sroa.0.010.i.i = phi i64 [ %probe_seq.sroa.0.0.i.i, %bb6.i21.i ], [ %probe_seq.sroa.0.06.i.i, %bb9.i20 ]
  %149 = phi i64 [ %150, %bb6.i21.i ], [ 0, %bb9.i20 ]
  %150 = add i64 %149, 16
  %151 = add i64 %150, %probe_seq.sroa.0.010.i.i
  %probe_seq.sroa.0.0.i.i = and i64 %151, %3
  %_18.i22.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %probe_seq.sroa.0.0.i.i
  %dst.sroa.0.0.copyload.i5.i.i = load <16 x i8>, ptr %_18.i22.i, align 1, !noalias !1117
  %152 = icmp slt <16 x i8> %dst.sroa.0.0.copyload.i5.i.i, zeroinitializer
  %153 = bitcast <16 x i1> %152 to i16
  %.not.i.i.i = icmp eq i16 %153, 0
  br i1 %.not.i.i.i, label %bb6.i21.i, label %bb9.i.i, !prof !1094

bb9.i.i:                                          ; preds = %bb6.i21.i, %bb9.i20
  %probe_seq.sroa.0.0.lcssa.i.i = phi i64 [ %probe_seq.sroa.0.06.i.i, %bb9.i20 ], [ %probe_seq.sroa.0.0.i.i, %bb6.i21.i ]
  %.lcssa.i.i = phi i16 [ %148, %bb9.i20 ], [ %153, %bb6.i21.i ]
  %154 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %.lcssa.i.i, i1 true)
  %_17.i.i.i = zext nneg i16 %154 to i64
  %_8.i.i.i = add i64 %probe_seq.sroa.0.0.lcssa.i.i, %_17.i.i.i
  %_7.i.i.i = and i64 %_8.i.i.i, %3
  %_8.i5.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %_7.i.i.i
  %_12.i.i.i = load i8, ptr %_8.i5.i.i, align 1, !noalias !1098, !noundef !10
  %b.i.i.i = icmp sgt i8 %_12.i.i.i, -1
  br i1 %b.i.i.i, label %bb2.i.i.i, label %bb10.i21, !prof !320

bb2.i.i.i:                                        ; preds = %bb9.i.i
  %155 = load <16 x i8>, ptr %self.val.i, align 16, !noalias !1120
  %156 = icmp slt <16 x i8> %155, zeroinitializer
  %157 = bitcast <16 x i1> %156 to i16
  %158 = icmp ne i16 %157, 0
  tail call void @llvm.assume(i1 %158)
  %159 = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %157, i1 true)
  %_25.i.i.i = zext nneg i16 %159 to i64
  br label %bb10.i21

bb10.i21:                                         ; preds = %bb2.i.i.i, %bb9.i.i
  %index.sroa.0.0.i.i.i = phi i64 [ %_25.i.i.i, %bb2.i.i.i ], [ %_7.i.i.i, %bb9.i.i ]
  %_86.i = sub i64 %iter.sroa.0.06.i, %probe_seq.sroa.0.06.i.i
  %_88.i = sub i64 %index.sroa.0.0.i.i.i, %probe_seq.sroa.0.06.i.i
  %_8518.i = xor i64 %_88.i, %_86.i
  %b.unshifted.i = and i64 %_8518.i, %3
  %b.i = icmp ult i64 %b.unshifted.i, 16
  br i1 %b.i, label %bb20.i, label %bb21.i, !prof !180

bb21.i:                                           ; preds = %bb10.i21
  %_108.i = shl i64 %index.sroa.0.0.i.i.i, 2
  %_110.i = sub nuw nsw i64 -4, %_108.i
  %new_i_p.i = getelementptr inbounds i8, ptr %self.val.i, i64 %_110.i
  %_113.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %index.sroa.0.0.i.i.i
  %prev_ctrl.i = load i8, ptr %_113.i, align 1, !noalias !1098, !noundef !10
  %_117.i = lshr i64 %_0.i.i.i.i.i.i, 57
  %_118.i = trunc nuw nsw i64 %_117.i to i8
  %_122.i = add i64 %index.sroa.0.0.i.i.i, -16
  %_121.i = and i64 %_122.i, %3
  store i8 %_118.i, ptr %_113.i, align 1, !noalias !1098
  %gep.i = getelementptr i8, ptr %invariant.gep.i, i64 %_121.i
  store i8 %_118.i, ptr %gep.i, align 1, !noalias !1098
  %_33.i = icmp eq i8 %prev_ctrl.i, -1
  br i1 %_33.i, label %bb11.i23, label %bb12.i22

bb20.i:                                           ; preds = %bb10.i21
  %_91.i = lshr i64 %_0.i.i.i.i.i.i, 57
  %_92.i = trunc nuw nsw i64 %_91.i to i8
  %_96.i = add i64 %iter.sroa.0.06.i, -16
  %_95.i = and i64 %_96.i, %3
  store i8 %_92.i, ptr %_69.i, align 1, !noalias !1098
  %gep13.i = getelementptr i8, ptr %invariant.gep.i, i64 %_95.i
  store i8 %_92.i, ptr %gep13.i, align 1, !noalias !1098
  br label %bb14.i

bb12.i22:                                         ; preds = %bb21.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1123)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1126)
  %_3.sroa.0.0.copyload.i.i6.i.i.i = load i32, ptr %i_p.i, align 1, !alias.scope !1123, !noalias !1128
  %_4.sroa.0.0.copyload.i.i7.i.i.i = load i32, ptr %new_i_p.i, align 1, !alias.scope !1126, !noalias !1129
  store i32 %_4.sroa.0.0.copyload.i.i7.i.i.i, ptr %i_p.i, align 1, !alias.scope !1123, !noalias !1128
  store i32 %_3.sroa.0.0.copyload.i.i6.i.i.i, ptr %new_i_p.i, align 1, !alias.scope !1126, !noalias !1129
  br label %bb9.i20

bb11.i23:                                         ; preds = %bb21.i
  %_135.i = add i64 %iter.sroa.0.06.i, -16
  %_134.i = and i64 %_135.i, %3
  store i8 -1, ptr %_69.i, align 1, !noalias !1098
  %gep11.i = getelementptr i8, ptr %invariant.gep.i, i64 %_134.i
  store i8 -1, ptr %gep11.i, align 1, !noalias !1098
  %160 = load i32, ptr %i_p.i, align 1, !noalias !1098
  store i32 %160, ptr %new_i_p.i, align 1, !noalias !1098
  br label %bb14.i

bb14.i:                                           ; preds = %bb11.i23, %bb20.i, %bb4.i19
  %_62.i = icmp ult i64 %iter.sroa.0.17.i, %_26.i
  %_66.i = zext i1 %_62.i to i64
  %iter.sroa.0.1.i = add nuw i64 %iter.sroa.0.17.i, %_66.i
  br i1 %_62.i, label %bb4.i19, label %_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E.exit

_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E.exit: ; preds = %bb14.i, %_ZN9hashbrown3raw13RawTableInner23prepare_rehash_in_place17h43508b23312966cbE.exit.thread19.i
  %161 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %162 = sub i64 %full_capacity.sroa.0.0.i, %self1.i
  store i64 %162, ptr %161, align 8, !alias.scope !1098
  br label %_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit

_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE.exit: ; preds = %bb1.i1.i.i.i, %bb1.i.i.i, %bb4.i.i, %bb14.i.i, %bb9.i, %_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E.exit
  %_0.sroa.4.0.i = phi i64 [ %_11.1.i, %bb9.i ], [ undef, %_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E.exit ], [ %self1.sroa.9.0.i.i.ph, %bb14.i.i ], [ undef, %bb4.i.i ], [ undef, %bb1.i.i.i ], [ undef, %bb1.i1.i.i.i ]
  %_0.sroa.0.0.i = phi i64 [ %_11.0.i, %bb9.i ], [ -9223372036854775807, %_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E.exit ], [ %self1.sroa.7.0.i.i.ph, %bb14.i.i ], [ -9223372036854775807, %bb4.i.i ], [ -9223372036854775807, %bb1.i.i.i ], [ -9223372036854775807, %bb1.i1.i.i.i ]
  %163 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0.i, 0
  %164 = insertvalue { i64, i64 } %163, i64 %_0.sroa.4.0.i, 1
  ret { i64, i64 } %164
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #15

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #16

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #15

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #17

declare i32 @__CxxFrameHandler3(...) unnamed_addr #18

; std::sys::random::hashmap_random_keys
; Function Attrs: uwtable
declare { i64, i64 } @_ZN3std3sys6random19hashmap_random_keys17hc3f03c6d163b2da2E() unnamed_addr #1

; std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL{{tls.shim}}
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$51__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$17hed5e461344c1f9f9E"() unnamed_addr #1

; <str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hdd75f4eba36c23acE"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h2e02e0ff298d12e0E"(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.fshl.i64(i64, i64, i64) #19

; core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hbcf79f68ff2d61d8E"(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i128 @llvm.cttz.i128(i128, i1 immarg) #19

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #19

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #16

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17hdddacd639c98ccdaE(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #20

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_ZN4core9panicking18panic_bounds_check17hd953c611c26672caE(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #21

; core::str::slice_error_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core3str16slice_error_fail17hfa16a7e04e1d89dbE(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #20

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core5slice5index16slice_index_fail17h30ecc7bdca4bc32bE(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #20

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #22

; core::slice::sort::shared::smallsort::panic_on_ord_violation
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core5slice4sort6shared9smallsort22panic_on_ord_violation17ha8ac69acadf1c3c7E() unnamed_addr #20

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctlz.i64(i64, i1 immarg) #19

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_ZN4core5slice6memchr14memchr_aligned17he27ef990a57e50bcE(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; core::fmt::num::imp::<impl u32>::_fmt
; Function Attrs: uwtable
declare { ptr, i64 } @"_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h68bd8f419e61f018E"(i32 noundef, ptr noalias noundef nonnull align 1, i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #23

; core::fmt::num::imp::<impl usize>::_fmt
; Function Attrs: uwtable
declare { ptr, i64 } @"_ZN4core3fmt3num3imp23_$LT$impl$u20$usize$GT$4_fmt17ha4599271dde7c38eE"(i64 noundef, ptr noalias noundef nonnull align 1, i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; core::str::pattern::StrSearcher::new
; Function Attrs: uwtable
declare void @_ZN4core3str7pattern11StrSearcher3new17h068a94d23c181adaE(ptr dead_on_unwind noalias noundef writable sret([104 x i8]) align 8 captures(address) dereferenceable(104), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; alloc::fmt::format::format_inner
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format12format_inner17hbb70ff8f9f00ea6cE(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #1

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h8d2b010e90e04388E(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #24

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #2

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #25

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #26

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCshXwFllX56pT_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #27

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h8738464738de9066E(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #24

; std::backtrace::Backtrace::capture
; Function Attrs: noinline uwtable
declare void @_ZN3std9backtrace9Backtrace7capture17hf7fc842bd7b2c58bE(ptr dead_on_unwind noalias noundef writable sret([48 x i8]) align 8 captures(address) dereferenceable(48)) unnamed_addr #8

; anyhow::fmt::<impl anyhow::error::ErrorImpl>::debug
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN6anyhow3fmt42_$LT$impl$u20$anyhow..error..ErrorImpl$GT$5debug17h7876388f4297a68bE"(ptr noundef nonnull, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; anyhow::error::ErrorImpl::error
; Function Attrs: uwtable
declare { ptr, ptr } @_ZN6anyhow5error9ErrorImpl5error17h21f16d1503d56ce3E(ptr noundef nonnull) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #28

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i16 @llvm.cttz.i16(i16, i1 immarg) #19

; hashbrown::raw::Fallibility::capacity_overflow
; Function Attrs: uwtable
declare { i64, i64 } @_ZN9hashbrown3raw11Fallibility17capacity_overflow17h167bbd1a91e4964eE(i1 noundef zeroext) unnamed_addr #1

; hashbrown::raw::Fallibility::alloc_err
; Function Attrs: uwtable
declare { i64, i64 } @_ZN9hashbrown3raw11Fallibility9alloc_err17h452573c969b6d13aE(i1 noundef zeroext, i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #29

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #30

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #30

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #1 = { uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #2 = { nounwind uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #3 = { inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: readwrite) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #4 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #5 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #6 = { nofree nosync nounwind memory(read, inaccessiblemem: none) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #7 = { nounwind memory(argmem: readwrite, inaccessiblemem: write) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #8 = { noinline uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #9 = { nofree noinline norecurse nosync nounwind memory(argmem: readwrite, inaccessiblemem: readwrite) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #10 = { cold nounwind uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #11 = { cold uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #12 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #13 = { cold inlinehint uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #14 = { cold noinline uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #15 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #16 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #17 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #18 = { "target-cpu"="x86-64" }
attributes #19 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #20 = { cold noinline noreturn uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #21 = { cold minsize noinline noreturn optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #22 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #23 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #24 = { cold minsize noreturn optsize uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #25 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCshXwFllX56pT_7___rustc19___rust_alloc_zeroed" "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #26 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #27 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #28 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #29 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #30 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #31 = { noinline noreturn }
attributes #32 = { inlinehint }
attributes #33 = { nounwind }
attributes #34 = { cold }
attributes #35 = { noinline }
attributes #36 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.93.0 (254b59607 2026-01-19)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E: %self"}
!4 = distinct !{!4, !"_ZN90_$LT$core..str..iter..Split$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haad4a3100ba097c7E"}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$4next17h35ea55aa757c4212E: %self"}
!7 = distinct !{!7, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$4next17h35ea55aa757c4212E"}
!8 = !{i8 0, i8 2}
!9 = !{!6, !3}
!10 = !{}
!11 = !{i64 1}
!12 = !{!13}
!13 = distinct !{!13, !14, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE: %_0"}
!14 = distinct !{!14, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE"}
!15 = !{!16}
!16 = distinct !{!16, !14, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h50647d9f594408afE: %self"}
!17 = !{i64 0, i64 2}
!18 = !{!16, !6, !3}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$4next17h9fe395199a43b525E: %self"}
!21 = distinct !{!21, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$4next17h9fe395199a43b525E"}
!22 = !{!20, !16, !6, !3}
!23 = !{!24, !13}
!24 = distinct !{!24, !21, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$4next17h9fe395199a43b525E: %otherwise"}
!25 = !{!26}
!26 = distinct !{!26, !27, !"_ZN4core3str6traits112_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeFrom$LT$usize$GT$$GT$3get17h4aa639c683cbc268E: %slice.0"}
!27 = distinct !{!27, !"_ZN4core3str6traits112_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeFrom$LT$usize$GT$$GT$3get17h4aa639c683cbc268E"}
!28 = !{!24, !29, !13, !16, !6, !3}
!29 = distinct !{!29, !21, !"_ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$4next17h9fe395199a43b525E: %self:Peel0"}
!30 = !{!31, !24, !29, !13, !16, !6, !3}
!31 = distinct !{!31, !32, !"_ZN4core3str11validations15next_code_point17h5561dc5c3c9990b4E: %bytes"}
!32 = distinct !{!32, !"_ZN4core3str11validations15next_code_point17h5561dc5c3c9990b4E"}
!33 = !{!29, !16, !6, !3}
!34 = !{!29}
!35 = !{!24, !20, !13, !16, !6, !3}
!36 = !{!31, !24, !20, !13, !16, !6, !3}
!37 = !{!38, !6, !3}
!38 = distinct !{!38, !39, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$7get_end17ha39ce76d1f9a1eb5E: %self"}
!39 = distinct !{!39, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$7get_end17ha39ce76d1f9a1eb5E"}
!40 = !{!41, !43, !45, !47}
!41 = distinct !{!41, !42, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9b0f4c69e5ebf78cE: %self"}
!42 = distinct !{!42, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9b0f4c69e5ebf78cE"}
!43 = distinct !{!43, !44, !"_ZN56_$LT$usize$u20$as$u20$core..iter..traits..accum..Sum$GT$3sum17h2b76d8a9eedada70E: %iter"}
!44 = distinct !{!44, !"_ZN56_$LT$usize$u20$as$u20$core..iter..traits..accum..Sum$GT$3sum17h2b76d8a9eedada70E"}
!45 = distinct !{!45, !46, !"_ZN4core4iter6traits8iterator8Iterator3sum17h32eba6b2fbcbc055E: %self"}
!46 = distinct !{!46, !"_ZN4core4iter6traits8iterator8Iterator3sum17h32eba6b2fbcbc055E"}
!47 = distinct !{!47, !48, !"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE: %group.0"}
!48 = distinct !{!48, !"_ZN7aoc20226solver5day0111parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h13e102e820df4aabE"}
!49 = !{!47}
!50 = !{!41, !43, !45}
!51 = !{!52, !41, !43, !45}
!52 = distinct !{!52, !53, !"_ZN4core4iter6traits8iterator8Iterator4fold17h8965113cbf0279a6E: %self"}
!53 = distinct !{!53, !"_ZN4core4iter6traits8iterator8Iterator4fold17h8965113cbf0279a6E"}
!54 = !{!55, !57, !59}
!55 = distinct !{!55, !56, !"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h00f062525874c398E: argument 1"}
!56 = distinct !{!56, !"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h00f062525874c398E"}
!57 = distinct !{!57, !58, !"_ZN4core3num62_$LT$impl$u20$core..str..traits..FromStr$u20$for$u20$usize$GT$8from_str17h3d9e44fab7f3fe06E: %src.0"}
!58 = distinct !{!58, !"_ZN4core3num62_$LT$impl$u20$core..str..traits..FromStr$u20$for$u20$usize$GT$8from_str17h3d9e44fab7f3fe06E"}
!59 = distinct !{!59, !60, !"_ZN4core3str21_$LT$impl$u20$str$GT$5parse17hea8722e402001f3dE: %self.0"}
!60 = distinct !{!60, !"_ZN4core3str21_$LT$impl$u20$str$GT$5parse17hea8722e402001f3dE"}
!61 = !{!62, !63, !64, !52, !41, !43, !45}
!62 = distinct !{!62, !56, !"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h00f062525874c398E: %_0"}
!63 = distinct !{!63, !58, !"_ZN4core3num62_$LT$impl$u20$core..str..traits..FromStr$u20$for$u20$usize$GT$8from_str17h3d9e44fab7f3fe06E: %_0"}
!64 = distinct !{!64, !60, !"_ZN4core3str21_$LT$impl$u20$str$GT$5parse17hea8722e402001f3dE: %_0"}
!65 = !{!66}
!66 = distinct !{!66, !67, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E: %_1"}
!67 = distinct !{!67, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E"}
!68 = !{!69}
!69 = distinct !{!69, !70, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E: %_1"}
!70 = distinct !{!70, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E"}
!71 = !{!72}
!72 = distinct !{!72, !73, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E: %_1"}
!73 = distinct !{!73, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E"}
!74 = !{!75}
!75 = distinct !{!75, !76, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE: %_1.0"}
!76 = distinct !{!76, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE"}
!77 = !{!75, !72}
!78 = !{i64 0, i64 -9223372036854775808}
!79 = !{!80}
!80 = distinct !{!80, !81, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE: %_1.0"}
!81 = distinct !{!81, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE"}
!82 = !{i64 0, i64 4}
!83 = !{!84}
!84 = distinct !{!84, !85, !"_ZN4core3ptr46drop_in_place$LT$std..backtrace..Backtrace$GT$17h9fbd18306c52b20aE: %_1"}
!85 = distinct !{!85, !"_ZN4core3ptr46drop_in_place$LT$std..backtrace..Backtrace$GT$17h9fbd18306c52b20aE"}
!86 = !{!87}
!87 = distinct !{!87, !88, !"_ZN4core3ptr42drop_in_place$LT$std..backtrace..Inner$GT$17h461bb7857c437637E: %_1"}
!88 = distinct !{!88, !"_ZN4core3ptr42drop_in_place$LT$std..backtrace..Inner$GT$17h461bb7857c437637E"}
!89 = !{!90}
!90 = distinct !{!90, !91, !"_ZN4core3ptr150drop_in_place$LT$std..sync..lazy_lock..LazyLock$LT$std..backtrace..Capture$C$std..backtrace..helper..lazy_resolve..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$17h4473e320a31435b3E: %_1"}
!91 = distinct !{!91, !"_ZN4core3ptr150drop_in_place$LT$std..sync..lazy_lock..LazyLock$LT$std..backtrace..Capture$C$std..backtrace..helper..lazy_resolve..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$17h4473e320a31435b3E"}
!92 = !{!93}
!93 = distinct !{!93, !94, !"_ZN85_$LT$std..sync..lazy_lock..LazyLock$LT$T$C$F$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h7288de8c374d1edcE: %self"}
!94 = distinct !{!94, !"_ZN85_$LT$std..sync..lazy_lock..LazyLock$LT$T$C$F$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h7288de8c374d1edcE"}
!95 = !{!93, !90, !87, !84}
!96 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000}
!97 = !{!98}
!98 = distinct !{!98, !99, !"_ZN4core3ptr44drop_in_place$LT$std..backtrace..Capture$GT$17h4458a92cddc1a6c3E: %_1"}
!99 = distinct !{!99, !"_ZN4core3ptr44drop_in_place$LT$std..backtrace..Capture$GT$17h4458a92cddc1a6c3E"}
!100 = !{!101}
!101 = distinct !{!101, !102, !"_ZN4core3ptr74drop_in_place$LT$alloc..vec..Vec$LT$std..backtrace..BacktraceFrame$GT$$GT$17h9ae867c80455c9d3E: %_1"}
!102 = distinct !{!102, !"_ZN4core3ptr74drop_in_place$LT$alloc..vec..Vec$LT$std..backtrace..BacktraceFrame$GT$$GT$17h9ae867c80455c9d3E"}
!103 = !{!101, !98, !90, !87, !84}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE: %_1"}
!106 = distinct !{!106, !"_ZN4core3ptr51drop_in_place$LT$std..backtrace..BacktraceFrame$GT$17h2288851fdfdcf37aE"}
!107 = !{!108}
!108 = distinct !{!108, !109, !"_ZN4core3ptr75drop_in_place$LT$alloc..vec..Vec$LT$std..backtrace..BacktraceSymbol$GT$$GT$17h772deac12af1287fE: %_1"}
!109 = distinct !{!109, !"_ZN4core3ptr75drop_in_place$LT$alloc..vec..Vec$LT$std..backtrace..BacktraceSymbol$GT$$GT$17h772deac12af1287fE"}
!110 = !{!108, !105}
!111 = !{!112}
!112 = distinct !{!112, !113, !"_ZN4core3ptr62drop_in_place$LT$$u5b$std..backtrace..BacktraceSymbol$u5d$$GT$17h58d1563909e2f7a5E: %_1.0"}
!113 = distinct !{!113, !"_ZN4core3ptr62drop_in_place$LT$$u5b$std..backtrace..BacktraceSymbol$u5d$$GT$17h58d1563909e2f7a5E"}
!114 = !{!115}
!115 = distinct !{!115, !116, !"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE: %_1"}
!116 = distinct !{!116, !"_ZN4core3ptr52drop_in_place$LT$std..backtrace..BacktraceSymbol$GT$17hfd3530ba13715d2dE"}
!117 = !{i64 0, i64 -9223372036854775807}
!118 = !{!115, !112}
!119 = !{!108, !105, !101, !98, !90, !87, !84}
!120 = !{!115, !112, !108, !105, !101, !98, !90, !87, !84}
!121 = !{!122}
!122 = distinct !{!122, !123, !"_ZN4core3ptr76drop_in_place$LT$core..option..Option$LT$std..backtrace..BytesOrWide$GT$$GT$17hca53ae33fd78f48fE: %_1"}
!123 = distinct !{!123, !"_ZN4core3ptr76drop_in_place$LT$core..option..Option$LT$std..backtrace..BytesOrWide$GT$$GT$17hca53ae33fd78f48fE"}
!124 = !{i64 0, i64 3}
!125 = !{!122, !115, !112}
!126 = !{!127}
!127 = distinct !{!127, !128, !"_ZN4core3ptr48drop_in_place$LT$std..backtrace..BytesOrWide$GT$17h6a6a26af94130113E: %_1"}
!128 = distinct !{!128, !"_ZN4core3ptr48drop_in_place$LT$std..backtrace..BytesOrWide$GT$17h6a6a26af94130113E"}
!129 = !{!127, !122, !115, !112}
!130 = !{!127, !122, !115, !112, !108, !105, !101, !98, !90, !87, !84}
!131 = !{!132}
!132 = distinct !{!132, !133, !"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE: %self.0"}
!133 = distinct !{!133, !"_ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17he4c6ac997f7bfd1fE"}
!134 = !{!135, !137, !139, !141, !142, !144, !145, !147}
!135 = distinct !{!135, !136, !"_ZN4core3str11validations15next_code_point17h5561dc5c3c9990b4E: %bytes"}
!136 = distinct !{!136, !"_ZN4core3str11validations15next_code_point17h5561dc5c3c9990b4E"}
!137 = distinct !{!137, !138, !"_ZN87_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h8e653cbaca80a467E: %self"}
!138 = distinct !{!138, !"_ZN87_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h8e653cbaca80a467E"}
!139 = distinct !{!139, !140, !"_ZN97_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..Searcher$GT$4next17h8f1f9226f9a306d8E: %_0"}
!140 = distinct !{!140, !"_ZN97_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..Searcher$GT$4next17h8f1f9226f9a306d8E"}
!141 = distinct !{!141, !140, !"_ZN97_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..Searcher$GT$4next17h8f1f9226f9a306d8E: %self"}
!142 = distinct !{!142, !143, !"_ZN4core3str7pattern8Searcher11next_reject17hc9a3df5d47d6016cE: %_0"}
!143 = distinct !{!143, !"_ZN4core3str7pattern8Searcher11next_reject17hc9a3df5d47d6016cE"}
!144 = distinct !{!144, !143, !"_ZN4core3str7pattern8Searcher11next_reject17hc9a3df5d47d6016cE: %self"}
!145 = distinct !{!145, !146, !"_ZN99_$LT$core..str..pattern..CharPredicateSearcher$LT$F$GT$$u20$as$u20$core..str..pattern..Searcher$GT$11next_reject17he98360e0e35430ccE: %_0"}
!146 = distinct !{!146, !"_ZN99_$LT$core..str..pattern..CharPredicateSearcher$LT$F$GT$$u20$as$u20$core..str..pattern..Searcher$GT$11next_reject17he98360e0e35430ccE"}
!147 = distinct !{!147, !146, !"_ZN99_$LT$core..str..pattern..CharPredicateSearcher$LT$F$GT$$u20$as$u20$core..str..pattern..Searcher$GT$11next_reject17he98360e0e35430ccE: %self"}
!148 = !{!139, !141, !142, !144, !145, !147, !132}
!149 = !{!150, !152, !154, !156, !157, !159, !160, !162}
!150 = distinct !{!150, !151, !"_ZN4core3str11validations23next_code_point_reverse17hbbf8184e4443a296E: %bytes"}
!151 = distinct !{!151, !"_ZN4core3str11validations23next_code_point_reverse17hbbf8184e4443a296E"}
!152 = distinct !{!152, !153, !"_ZN102_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hfd6f30013b138968E: %self"}
!153 = distinct !{!153, !"_ZN102_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hfd6f30013b138968E"}
!154 = distinct !{!154, !155, !"_ZN104_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$9next_back17h27f4e343bc4d8221E: %_0"}
!155 = distinct !{!155, !"_ZN104_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$9next_back17h27f4e343bc4d8221E"}
!156 = distinct !{!156, !155, !"_ZN104_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$9next_back17h27f4e343bc4d8221E: %self"}
!157 = distinct !{!157, !158, !"_ZN4core3str7pattern15ReverseSearcher16next_reject_back17h74b2b777ba2b82bbE: %_0"}
!158 = distinct !{!158, !"_ZN4core3str7pattern15ReverseSearcher16next_reject_back17h74b2b777ba2b82bbE"}
!159 = distinct !{!159, !158, !"_ZN4core3str7pattern15ReverseSearcher16next_reject_back17h74b2b777ba2b82bbE: %self"}
!160 = distinct !{!160, !161, !"_ZN106_$LT$core..str..pattern..CharPredicateSearcher$LT$F$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$16next_reject_back17hffca25e6faabb203E: %_0"}
!161 = distinct !{!161, !"_ZN106_$LT$core..str..pattern..CharPredicateSearcher$LT$F$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$16next_reject_back17hffca25e6faabb203E"}
!162 = distinct !{!162, !161, !"_ZN106_$LT$core..str..pattern..CharPredicateSearcher$LT$F$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$16next_reject_back17hffca25e6faabb203E: %self"}
!163 = !{!154, !156, !157, !159, !160, !162, !132}
!164 = !{!165}
!165 = distinct !{!165, !166, !"_ZN84_$LT$core..str..pattern..MatchOnly$u20$as$u20$core..str..pattern..TwoWayStrategy$GT$8matching17hc88e17ffd09cc42bE: %_0"}
!166 = distinct !{!166, !"_ZN84_$LT$core..str..pattern..MatchOnly$u20$as$u20$core..str..pattern..TwoWayStrategy$GT$8matching17hc88e17ffd09cc42bE"}
!167 = !{!168}
!168 = distinct !{!168, !169, !"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1b209ee349209c18E: %_0"}
!169 = distinct !{!169, !"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1b209ee349209c18E"}
!170 = !{!171, !173, !175, !168}
!171 = distinct !{!171, !172, !"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h43984239def11c07E: %__rust_std_internal_init"}
!172 = distinct !{!172, !"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h43984239def11c07E"}
!173 = distinct !{!173, !174, !"_ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE: argument 0"}
!174 = distinct !{!174, !"_ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE"}
!175 = distinct !{!175, !176, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17h45080ca56f2a806cE: %_0"}
!176 = distinct !{!176, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17h45080ca56f2a806cE"}
!177 = !{!178, !171, !173, !175, !168}
!178 = distinct !{!178, !179, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$11get_or_init17h1cf4cb6d83cb1c7bE: %i"}
!179 = distinct !{!179, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$11get_or_init17h1cf4cb6d83cb1c7bE"}
!180 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!181 = !{!175, !168}
!182 = !{!183, !175, !168}
!183 = distinct !{!183, !184, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE: argument 0"}
!184 = distinct !{!184, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE"}
!185 = !{!186, !188, !190, !192, !194, !196, !198, !168}
!186 = distinct !{!186, !187, !"_ZN4core3str11validations15next_code_point17h5561dc5c3c9990b4E: %bytes"}
!187 = distinct !{!187, !"_ZN4core3str11validations15next_code_point17h5561dc5c3c9990b4E"}
!188 = distinct !{!188, !189, !"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE: %self"}
!189 = distinct !{!189, !"_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h829e98dd688a3e6aE"}
!190 = distinct !{!190, !191, !"_ZN4core4iter6traits8iterator8Iterator4fold17h764b805098f0c82eE: argument 0"}
!191 = distinct !{!191, !"_ZN4core4iter6traits8iterator8Iterator4fold17h764b805098f0c82eE"}
!192 = distinct !{!192, !193, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17haa6d169da494022fE: %g"}
!193 = distinct !{!193, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17haa6d169da494022fE"}
!194 = distinct !{!194, !195, !"_ZN4core4iter6traits8iterator8Iterator8for_each17h4ae01196c2a12590E: %f"}
!195 = distinct !{!195, !"_ZN4core4iter6traits8iterator8Iterator8for_each17h4ae01196c2a12590E"}
!196 = distinct !{!196, !197, !"_ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h44018a7efad917aeE: %self"}
!197 = distinct !{!197, !"_ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h44018a7efad917aeE"}
!198 = distinct !{!198, !199, !"_ZN105_$LT$hashbrown..set..HashSet$LT$T$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$6extend17h90f2b29039cd235aE: %self"}
!199 = distinct !{!199, !"_ZN105_$LT$hashbrown..set..HashSet$LT$T$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$6extend17h90f2b29039cd235aE"}
!200 = !{!201, !203}
!201 = distinct !{!201, !202, !"_ZN99_$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h322189dedef8dc78E: %self"}
!202 = distinct !{!202, !"_ZN99_$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h322189dedef8dc78E"}
!203 = distinct !{!203, !204, !"_ZN4core3ptr82drop_in_place$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$usize$GT$$GT$17h2a22e8fc00276fcbE: %_1"}
!204 = distinct !{!204, !"_ZN4core3ptr82drop_in_place$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$usize$GT$$GT$17h2a22e8fc00276fcbE"}
!205 = !{!206}
!206 = distinct !{!206, !207, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse7revswap17he402cb984c996450E: %a.0"}
!207 = distinct !{!207, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse7revswap17he402cb984c996450E"}
!208 = !{!209}
!209 = distinct !{!209, !207, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse7revswap17he402cb984c996450E: %b.0"}
!210 = !{!206, !211}
!211 = distinct !{!211, !212, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17hc57e4f034bec5172E: %self.0"}
!212 = distinct !{!212, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17hc57e4f034bec5172E"}
!213 = !{!209, !211}
!214 = distinct !{!214, !215, !216}
!215 = !{!"llvm.loop.isvectorized", i32 1}
!216 = !{!"llvm.loop.unroll.runtime.disable"}
!217 = distinct !{!217, !216, !215}
!218 = !{!219}
!219 = distinct !{!219, !220, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %x"}
!220 = distinct !{!220, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E"}
!221 = !{!222}
!222 = distinct !{!222, !220, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %y"}
!223 = !{!224}
!224 = distinct !{!224, !225, !"_ZN4core5slice4sort6shared9smallsort18small_sort_network17h9f91f8845c016121E: %v.0"}
!225 = distinct !{!225, !"_ZN4core5slice4sort6shared9smallsort18small_sort_network17h9f91f8845c016121E"}
!226 = !{!227, !224}
!227 = distinct !{!227, !228, !"_ZN4core5slice4sort6shared9smallsort14sort13_optimal17h204b3751435faf6cE: %v.0"}
!228 = distinct !{!228, !"_ZN4core5slice4sort6shared9smallsort14sort13_optimal17h204b3751435faf6cE"}
!229 = !{!230, !224}
!230 = distinct !{!230, !231, !"_ZN4core5slice4sort6shared9smallsort13sort9_optimal17ha5187a5b6def4eceE: %v.0"}
!231 = distinct !{!231, !"_ZN4core5slice4sort6shared9smallsort13sort9_optimal17ha5187a5b6def4eceE"}
!232 = !{!233, !224}
!233 = distinct !{!233, !234, !"_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E: %v.0"}
!234 = distinct !{!234, !"_ZN4core5slice4sort6shared9smallsort25insertion_sort_shift_left17ha03a838940890e89E"}
!235 = !{!236, !238}
!236 = distinct !{!236, !237, !"_ZN99_$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h322189dedef8dc78E: %self"}
!237 = distinct !{!237, !"_ZN99_$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h322189dedef8dc78E"}
!238 = distinct !{!238, !239, !"_ZN4core3ptr82drop_in_place$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$usize$GT$$GT$17h2a22e8fc00276fcbE: %_1"}
!239 = distinct !{!239, !"_ZN4core3ptr82drop_in_place$LT$core..slice..sort..shared..smallsort..CopyOnDrop$LT$usize$GT$$GT$17h2a22e8fc00276fcbE"}
!240 = !{!241}
!241 = distinct !{!241, !242, !"_ZN4core5slice4sort6shared9smallsort19bidirectional_merge17hd46cedbb47ad2770E: %v.0"}
!242 = distinct !{!242, !"_ZN4core5slice4sort6shared9smallsort19bidirectional_merge17hd46cedbb47ad2770E"}
!243 = !{!241, !224}
!244 = !{!245, !247, !241, !224}
!245 = distinct !{!245, !246, !"_ZN4core5slice4sort6shared9smallsort8merge_up17h63f27df35256ba4bE: %_0"}
!246 = distinct !{!246, !"_ZN4core5slice4sort6shared9smallsort8merge_up17h63f27df35256ba4bE"}
!247 = distinct !{!247, !246, !"_ZN4core5slice4sort6shared9smallsort8merge_up17h63f27df35256ba4bE: %is_less"}
!248 = !{!249, !251, !241, !224}
!249 = distinct !{!249, !250, !"_ZN4core5slice4sort6shared9smallsort10merge_down17hc43b3d0187c2b635E: %_0"}
!250 = distinct !{!250, !"_ZN4core5slice4sort6shared9smallsort10merge_down17hc43b3d0187c2b635E"}
!251 = distinct !{!251, !250, !"_ZN4core5slice4sort6shared9smallsort10merge_down17hc43b3d0187c2b635E: %is_less"}
!252 = !{!"branch_weights", i32 4001, i32 4000000}
!253 = !{!254}
!254 = distinct !{!254, !255, !"_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E: %v.0"}
!255 = distinct !{!255, !"_ZN4core5slice4sort6shared5pivot12choose_pivot17h50516133c64d0056E"}
!256 = !{!257, !259}
!257 = distinct !{!257, !258, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E: %self.0"}
!258 = distinct !{!258, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E"}
!259 = distinct !{!259, !260, !"_ZN4core5slice4sort8unstable9quicksort9partition17h7ae5ad0e3b277121E: %v.0"}
!260 = distinct !{!260, !"_ZN4core5slice4sort8unstable9quicksort9partition17h7ae5ad0e3b277121E"}
!261 = !{!262}
!262 = distinct !{!262, !263, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE: %v.0"}
!263 = distinct !{!263, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE"}
!264 = !{!265}
!265 = distinct !{!265, !263, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17h3db3f9645a0bb7bdE: %pivot"}
!266 = !{!262, !259}
!267 = !{!265, !259}
!268 = !{!269, !262}
!269 = distinct !{!269, !270, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hee4516bd1940e8f3E: %state"}
!270 = distinct !{!270, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hee4516bd1940e8f3E"}
!271 = !{!269, !265}
!272 = !{!273, !265}
!273 = distinct !{!273, !274, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hee4516bd1940e8f3E: %state"}
!274 = distinct !{!274, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hee4516bd1940e8f3E"}
!275 = !{!276, !265}
!276 = distinct !{!276, !277, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hee4516bd1940e8f3E: %state"}
!277 = distinct !{!277, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hee4516bd1940e8f3E"}
!278 = !{!279, !259}
!279 = distinct !{!279, !280, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E: %self.0"}
!280 = distinct !{!280, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E"}
!281 = !{!282, !284}
!282 = distinct !{!282, !283, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E: %self.0"}
!283 = distinct !{!283, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E"}
!284 = distinct !{!284, !285, !"_ZN4core5slice4sort8unstable9quicksort9partition17h87ff59f1696c53cbE: %v.0"}
!285 = distinct !{!285, !"_ZN4core5slice4sort8unstable9quicksort9partition17h87ff59f1696c53cbE"}
!286 = !{!287}
!287 = distinct !{!287, !288, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE: %v.0"}
!288 = distinct !{!288, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE"}
!289 = !{!290}
!290 = distinct !{!290, !288, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic17ha307fcf0c89c97edE: %pivot"}
!291 = !{!287, !284}
!292 = !{!290, !284}
!293 = !{!294, !287}
!294 = distinct !{!294, !295, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hb7e4d8c1da77b040E: %state"}
!295 = distinct !{!295, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hb7e4d8c1da77b040E"}
!296 = !{!294, !290}
!297 = !{!298, !290}
!298 = distinct !{!298, !299, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hb7e4d8c1da77b040E: %state"}
!299 = distinct !{!299, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hb7e4d8c1da77b040E"}
!300 = !{!301, !290}
!301 = distinct !{!301, !302, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hb7e4d8c1da77b040E: %state"}
!302 = distinct !{!302, !"_ZN4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclic28_$u7b$$u7b$closure$u7d$$u7d$17hb7e4d8c1da77b040E"}
!303 = !{!304, !284}
!304 = distinct !{!304, !305, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E: %self.0"}
!305 = distinct !{!305, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$14swap_unchecked17h94759969c7a5fbe7E"}
!306 = !{!"branch_weights", i32 2002, i32 2000}
!307 = !{!308}
!308 = distinct !{!308, !309, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17hee3ea99e491cdd6bE: %self"}
!309 = distinct !{!309, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17hee3ea99e491cdd6bE"}
!310 = !{!311}
!311 = distinct !{!311, !312, !"_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E: %_1"}
!312 = distinct !{!312, !"_ZN4core3ptr111drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$17h7b8d5462fe1b8ed9E"}
!313 = !{!314, !311}
!314 = distinct !{!314, !315, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E: %_1"}
!315 = distinct !{!315, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E"}
!316 = !{!317}
!317 = distinct !{!317, !318, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E: %_1"}
!318 = distinct !{!318, !"_ZN4core3ptr79drop_in_place$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$17h9dec66c5d7b34317E"}
!319 = !{!317, !311}
!320 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!321 = !{!322}
!322 = distinct !{!322, !323, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h1ec63019a87b54f9E: %x.0"}
!323 = distinct !{!323, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h1ec63019a87b54f9E"}
!324 = !{!325}
!325 = distinct !{!325, !326, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h78b42933f774d27fE: %x"}
!326 = distinct !{!326, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h78b42933f774d27fE"}
!327 = !{!328}
!328 = distinct !{!328, !329, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h3e21f0fb660f3a91E: %x"}
!329 = distinct !{!329, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h3e21f0fb660f3a91E"}
!330 = !{!331}
!331 = distinct !{!331, !332, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17hcb17a0cacde96825E: %x"}
!332 = distinct !{!332, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17hcb17a0cacde96825E"}
!333 = !{!334}
!334 = distinct !{!334, !335, !"_ZN4core6option15Option$LT$T$GT$11map_or_else17hbd451feb1fcd170bE: argument 1"}
!335 = distinct !{!335, !"_ZN4core6option15Option$LT$T$GT$11map_or_else17hbd451feb1fcd170bE"}
!336 = !{!337}
!337 = distinct !{!337, !338, !"_ZN4core3str21_$LT$impl$u20$str$GT$5split17h5b1cfc84e68f2da2E: %_0"}
!338 = distinct !{!338, !"_ZN4core3str21_$LT$impl$u20$str$GT$5split17h5b1cfc84e68f2da2E"}
!339 = !{!340}
!340 = distinct !{!340, !338, !"_ZN4core3str21_$LT$impl$u20$str$GT$5split17h5b1cfc84e68f2da2E: %self.0"}
!341 = !{!342, !344, !345, !347}
!342 = distinct !{!342, !343, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1a66d60baa0c921dE: %_0"}
!343 = distinct !{!343, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1a66d60baa0c921dE"}
!344 = distinct !{!344, !343, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h1a66d60baa0c921dE: %iter"}
!345 = distinct !{!345, !346, !"_ZN4core4iter6traits8iterator8Iterator7collect17h591a7ad1524e9751E: %_0"}
!346 = distinct !{!346, !"_ZN4core4iter6traits8iterator8Iterator7collect17h591a7ad1524e9751E"}
!347 = distinct !{!347, !346, !"_ZN4core4iter6traits8iterator8Iterator7collect17h591a7ad1524e9751E: %self"}
!348 = !{!349, !351, !352, !354, !342, !344, !345, !347}
!349 = distinct !{!349, !350, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h3d926b6e8fea9002E: %_0"}
!350 = distinct !{!350, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h3d926b6e8fea9002E"}
!351 = distinct !{!351, !350, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h3d926b6e8fea9002E: %iterator"}
!352 = distinct !{!352, !353, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h3d602ee9c58c08d4E: %_0"}
!353 = distinct !{!353, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h3d602ee9c58c08d4E"}
!354 = distinct !{!354, !353, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h3d602ee9c58c08d4E: %iterator"}
!355 = !{!349, !352, !342, !344, !345, !347}
!356 = !{!357, !349, !351, !352, !354, !342, !344, !345, !347}
!357 = distinct !{!357, !358, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!358 = distinct !{!358, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!359 = !{!360}
!360 = distinct !{!360, !361, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hc839dae2a147be16E: %self"}
!361 = distinct !{!361, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hc839dae2a147be16E"}
!362 = !{!363}
!363 = distinct !{!363, !364, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hc3e8fcf8c7849b2aE: %self"}
!364 = distinct !{!364, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hc3e8fcf8c7849b2aE"}
!365 = !{!363, !360}
!366 = !{!367, !368, !349, !351, !352, !354, !342, !344, !345, !347}
!367 = distinct !{!367, !364, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hc3e8fcf8c7849b2aE: %iterator"}
!368 = distinct !{!368, !361, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hc839dae2a147be16E: %iter"}
!369 = !{!363, !367, !360, !368, !349, !351, !352, !354, !342, !344, !345, !347}
!370 = !{!351, !354, !344, !347}
!371 = !{!372}
!372 = distinct !{!372, !373, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h1f23c083347f96c6E: %init"}
!373 = distinct !{!373, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h1f23c083347f96c6E"}
!374 = !{!375, !377, !372}
!375 = distinct !{!375, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0"}
!376 = distinct !{!376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E"}
!377 = distinct !{!377, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x"}
!378 = distinct !{!378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E"}
!379 = !{!380, !381}
!380 = distinct !{!380, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1"}
!381 = distinct !{!381, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y"}
!382 = !{!377}
!383 = !{!381}
!384 = !{!375}
!385 = !{!380}
!386 = !{!387, !388}
!387 = distinct !{!387, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1:It1"}
!388 = distinct !{!388, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y:It1"}
!389 = !{!390, !391, !372}
!390 = distinct !{!390, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0:It1"}
!391 = distinct !{!391, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x:It1"}
!392 = !{!393, !394}
!393 = distinct !{!393, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1:It2"}
!394 = distinct !{!394, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y:It2"}
!395 = !{!396, !397, !372}
!396 = distinct !{!396, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0:It2"}
!397 = distinct !{!397, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x:It2"}
!398 = !{!399, !400}
!399 = distinct !{!399, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1:It3"}
!400 = distinct !{!400, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y:It3"}
!401 = !{!402, !403, !372}
!402 = distinct !{!402, !376, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0:It3"}
!403 = distinct !{!403, !378, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x:It3"}
!404 = distinct !{!404, !405}
!405 = !{!"llvm.loop.unroll.disable"}
!406 = !{!407, !409, !411, !412}
!407 = distinct !{!407, !408, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!408 = distinct !{!408, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!409 = distinct !{!409, !410, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h4e8048263505f88dE: %v"}
!410 = distinct !{!410, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h4e8048263505f88dE"}
!411 = distinct !{!411, !410, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h4e8048263505f88dE: %s.0"}
!412 = distinct !{!412, !413, !"_ZN67_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h22077266838cdeb2E: %_0"}
!413 = distinct !{!413, !"_ZN67_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h22077266838cdeb2E"}
!414 = !{!409, !411, !412}
!415 = !{!409, !412}
!416 = !{!417}
!417 = distinct !{!417, !418, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$16sort_unstable_by17h5c425dcba92c83fcE: %self.0"}
!418 = distinct !{!418, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$16sort_unstable_by17h5c425dcba92c83fcE"}
!419 = !{!"branch_weights", !"expected", i32 2145766519, i32 1717129}
!420 = !{!421, !423, !425, !427}
!421 = distinct !{!421, !422, !"_ZN98_$LT$core..iter..adapters..take..Take$LT$I$GT$$u20$as$u20$core..iter..adapters..take..SpecTake$GT$9spec_fold17hebf72f94a048087aE: %self"}
!422 = distinct !{!422, !"_ZN98_$LT$core..iter..adapters..take..Take$LT$I$GT$$u20$as$u20$core..iter..adapters..take..SpecTake$GT$9spec_fold17hebf72f94a048087aE"}
!423 = distinct !{!423, !424, !"_ZN100_$LT$core..iter..adapters..take..Take$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h18b4262cff6d4401E: %self"}
!424 = distinct !{!424, !"_ZN100_$LT$core..iter..adapters..take..Take$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h18b4262cff6d4401E"}
!425 = distinct !{!425, !426, !"_ZN73_$LT$usize$u20$as$u20$core..iter..traits..accum..Sum$LT$$RF$usize$GT$$GT$3sum17hf5cfc8a2efd775d1E: %iter"}
!426 = distinct !{!426, !"_ZN73_$LT$usize$u20$as$u20$core..iter..traits..accum..Sum$LT$$RF$usize$GT$$GT$3sum17hf5cfc8a2efd775d1E"}
!427 = distinct !{!427, !428, !"_ZN4core4iter6traits8iterator8Iterator3sum17h0d98b806dfa6676dE: %self"}
!428 = distinct !{!428, !"_ZN4core4iter6traits8iterator8Iterator3sum17h0d98b806dfa6676dE"}
!429 = distinct !{!429, !216, !215}
!430 = !{!431}
!431 = distinct !{!431, !432, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h1f23c083347f96c6E: %init"}
!432 = distinct !{!432, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h1f23c083347f96c6E"}
!433 = !{!434, !436, !431}
!434 = distinct !{!434, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0"}
!435 = distinct !{!435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E"}
!436 = distinct !{!436, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x"}
!437 = distinct !{!437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E"}
!438 = !{!439, !440}
!439 = distinct !{!439, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1"}
!440 = distinct !{!440, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y"}
!441 = !{!436}
!442 = !{!440}
!443 = !{!434}
!444 = !{!439}
!445 = !{!446, !447}
!446 = distinct !{!446, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1:It1"}
!447 = distinct !{!447, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y:It1"}
!448 = !{!449, !450, !431}
!449 = distinct !{!449, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0:It1"}
!450 = distinct !{!450, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x:It1"}
!451 = !{!452, !453}
!452 = distinct !{!452, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1:It2"}
!453 = distinct !{!453, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y:It2"}
!454 = !{!455, !456, !431}
!455 = distinct !{!455, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0:It2"}
!456 = distinct !{!456, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x:It2"}
!457 = !{!458, !459}
!458 = distinct !{!458, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 1:It3"}
!459 = distinct !{!459, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %y:It3"}
!460 = !{!461, !462, !431}
!461 = distinct !{!461, !435, !"_ZN4core3cmp6max_by17hd5771e1bcbf6b5c2E: argument 0:It3"}
!462 = distinct !{!462, !437, !"_ZN4core4iter6traits8iterator8Iterator6max_by4fold28_$u7b$$u7b$closure$u7d$$u7d$17h9a658731e65df522E: %x:It3"}
!463 = distinct !{!463, !405}
!464 = !{!465, !467, !468, !470}
!465 = distinct !{!465, !466, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h32d8ade7fe76e622E: %_0"}
!466 = distinct !{!466, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h32d8ade7fe76e622E"}
!467 = distinct !{!467, !466, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h32d8ade7fe76e622E: %iter"}
!468 = distinct !{!468, !469, !"_ZN4core4iter6traits8iterator8Iterator7collect17h1a19bbf95a9ccf9cE: %_0"}
!469 = distinct !{!469, !"_ZN4core4iter6traits8iterator8Iterator7collect17h1a19bbf95a9ccf9cE"}
!470 = distinct !{!470, !469, !"_ZN4core4iter6traits8iterator8Iterator7collect17h1a19bbf95a9ccf9cE: %self"}
!471 = !{!472, !474}
!472 = distinct !{!472, !473, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h00a233a0b17ba244E: %_0"}
!473 = distinct !{!473, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h00a233a0b17ba244E"}
!474 = distinct !{!474, !473, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h00a233a0b17ba244E: %self"}
!475 = !{!465, !468}
!476 = !{!477, !479, !480, !482, !465, !467, !468, !470}
!477 = distinct !{!477, !478, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h07dd33f5a79e4ae4E: %_0"}
!478 = distinct !{!478, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h07dd33f5a79e4ae4E"}
!479 = distinct !{!479, !478, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h07dd33f5a79e4ae4E: %iterator"}
!480 = distinct !{!480, !481, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h79f3a0699c0d09b5E: %_0"}
!481 = distinct !{!481, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h79f3a0699c0d09b5E"}
!482 = distinct !{!482, !481, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h79f3a0699c0d09b5E: %iterator"}
!483 = !{!477, !480, !465, !467, !468, !470}
!484 = !{!485, !487, !489, !491, !477, !479, !480, !482, !465, !467, !468, !470}
!485 = distinct !{!485, !486, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h0b5726fde583dc33E: %self"}
!486 = distinct !{!486, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h0b5726fde583dc33E"}
!487 = distinct !{!487, !488, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h64418506f7f007d7E: %self"}
!488 = distinct !{!488, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h64418506f7f007d7E"}
!489 = distinct !{!489, !490, !"_ZN4core4iter6traits8iterator8Iterator8find_map17h6b96b2503103b68cE: %self"}
!490 = distinct !{!490, !"_ZN4core4iter6traits8iterator8Iterator8find_map17h6b96b2503103b68cE"}
!491 = distinct !{!491, !492, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E: %self"}
!492 = distinct !{!492, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E"}
!493 = !{!494, !496, !498, !500}
!494 = distinct !{!494, !495, !"_ZN7aoc20226solver5day0211parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h25e77ddfd26c3a3eE: %line.0"}
!495 = distinct !{!495, !"_ZN7aoc20226solver5day0211parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h25e77ddfd26c3a3eE"}
!496 = distinct !{!496, !497, !"_ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17ha70f1eafaa7bf8cdE: argument 0"}
!497 = distinct !{!497, !"_ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17ha70f1eafaa7bf8cdE"}
!498 = distinct !{!498, !499, !"_ZN4core4iter6traits8iterator8Iterator8find_map5check28_$u7b$$u7b$closure$u7d$$u7d$17hb1848c38965677d4E: %x.0"}
!499 = distinct !{!499, !"_ZN4core4iter6traits8iterator8Iterator8find_map5check28_$u7b$$u7b$closure$u7d$$u7d$17hb1848c38965677d4E"}
!500 = distinct !{!500, !501, !"_ZN4core4iter8adapters6filter15filter_try_fold28_$u7b$$u7b$closure$u7d$$u7d$17h079ff259f98b3bd0E: argument 0"}
!501 = distinct !{!501, !"_ZN4core4iter8adapters6filter15filter_try_fold28_$u7b$$u7b$closure$u7d$$u7d$17h079ff259f98b3bd0E"}
!502 = !{!503, !477, !479, !480, !482, !465, !467, !468, !470}
!503 = distinct !{!503, !504, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!504 = distinct !{!504, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!505 = !{!506}
!506 = distinct !{!506, !507, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h91547bd48c0d0becE: %self"}
!507 = distinct !{!507, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h91547bd48c0d0becE"}
!508 = !{!509}
!509 = distinct !{!509, !510, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hafb0852a898d8bd8E: %self"}
!510 = distinct !{!510, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hafb0852a898d8bd8E"}
!511 = !{!512, !514, !516, !518, !509, !520, !506, !521, !477, !479, !480, !482, !465, !467, !468, !470}
!512 = distinct !{!512, !513, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h0b5726fde583dc33E: %self"}
!513 = distinct !{!513, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h0b5726fde583dc33E"}
!514 = distinct !{!514, !515, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h64418506f7f007d7E: %self"}
!515 = distinct !{!515, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h64418506f7f007d7E"}
!516 = distinct !{!516, !517, !"_ZN4core4iter6traits8iterator8Iterator8find_map17h6b96b2503103b68cE: %self"}
!517 = distinct !{!517, !"_ZN4core4iter6traits8iterator8Iterator8find_map17h6b96b2503103b68cE"}
!518 = distinct !{!518, !519, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E: %self"}
!519 = distinct !{!519, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0c0a692ee86c79d6E"}
!520 = distinct !{!520, !510, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hafb0852a898d8bd8E: %iterator"}
!521 = distinct !{!521, !507, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h91547bd48c0d0becE: %iter"}
!522 = !{!523, !525, !527, !529}
!523 = distinct !{!523, !524, !"_ZN7aoc20226solver5day0211parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h25e77ddfd26c3a3eE: %line.0"}
!524 = distinct !{!524, !"_ZN7aoc20226solver5day0211parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h25e77ddfd26c3a3eE"}
!525 = distinct !{!525, !526, !"_ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17ha70f1eafaa7bf8cdE: argument 0"}
!526 = distinct !{!526, !"_ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17ha70f1eafaa7bf8cdE"}
!527 = distinct !{!527, !528, !"_ZN4core4iter6traits8iterator8Iterator8find_map5check28_$u7b$$u7b$closure$u7d$$u7d$17hb1848c38965677d4E: %x.0"}
!528 = distinct !{!528, !"_ZN4core4iter6traits8iterator8Iterator8find_map5check28_$u7b$$u7b$closure$u7d$$u7d$17hb1848c38965677d4E"}
!529 = distinct !{!529, !530, !"_ZN4core4iter8adapters6filter15filter_try_fold28_$u7b$$u7b$closure$u7d$$u7d$17h079ff259f98b3bd0E: argument 0"}
!530 = distinct !{!530, !"_ZN4core4iter8adapters6filter15filter_try_fold28_$u7b$$u7b$closure$u7d$$u7d$17h079ff259f98b3bd0E"}
!531 = !{!509, !506}
!532 = !{!520, !521, !477, !479, !480, !482, !465, !467, !468, !470}
!533 = !{!509, !520, !506, !521, !477, !479, !480, !482, !465, !467, !468, !470}
!534 = !{!479, !482, !467, !470}
!535 = !{!536, !538, !539, !541}
!536 = distinct !{!536, !537, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17hffcd7115112f552dE: %_0"}
!537 = distinct !{!537, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17hffcd7115112f552dE"}
!538 = distinct !{!538, !537, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17hffcd7115112f552dE: %iter"}
!539 = distinct !{!539, !540, !"_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E: %_0"}
!540 = distinct !{!540, !"_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E"}
!541 = distinct !{!541, !540, !"_ZN4core4iter6traits8iterator8Iterator7collect17h599791d53ff78ad9E: %self"}
!542 = !{!543, !545}
!543 = distinct !{!543, !544, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hef314d87c9471ba8E: %_0"}
!544 = distinct !{!544, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hef314d87c9471ba8E"}
!545 = distinct !{!545, !544, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hef314d87c9471ba8E: %self"}
!546 = !{!536, !539}
!547 = !{!548, !550, !551, !553, !536, !538, !539, !541}
!548 = distinct !{!548, !549, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17hd6b1cd4adc375bd4E: %_0"}
!549 = distinct !{!549, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17hd6b1cd4adc375bd4E"}
!550 = distinct !{!550, !549, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17hd6b1cd4adc375bd4E: %iterator"}
!551 = distinct !{!551, !552, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h2a1a4bf93478cf05E: %_0"}
!552 = distinct !{!552, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h2a1a4bf93478cf05E"}
!553 = distinct !{!553, !552, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17h2a1a4bf93478cf05E: %iterator"}
!554 = !{!555, !548, !551, !536, !538, !539, !541}
!555 = distinct !{!555, !556, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E: %_0"}
!556 = distinct !{!556, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E"}
!557 = !{!558, !560, !562, !555, !564, !548, !550, !551, !553, !536, !538, !539, !541}
!558 = distinct !{!558, !559, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h7ba5e51ab03952c3E: %self"}
!559 = distinct !{!559, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h7ba5e51ab03952c3E"}
!560 = distinct !{!560, !561, !"_ZN4core4iter6traits8iterator8Iterator4find17h760dddbf069dd629E: %self"}
!561 = distinct !{!561, !"_ZN4core4iter6traits8iterator8Iterator4find17h760dddbf069dd629E"}
!562 = distinct !{!562, !563, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2df1d476115a649aE: %self"}
!563 = distinct !{!563, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2df1d476115a649aE"}
!564 = distinct !{!564, !556, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E: %self"}
!565 = !{!566, !568, !570, !571, !573, !574, !576, !577, !579, !555, !564, !548, !550, !551, !553, !536, !538, !539, !541}
!566 = distinct !{!566, !567, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!567 = distinct !{!567, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!568 = distinct !{!568, !569, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!569 = distinct !{!569, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!570 = distinct !{!570, !569, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!571 = distinct !{!571, !572, !"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h6c3604c0f52dd81fE: %_0"}
!572 = distinct !{!572, !"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h6c3604c0f52dd81fE"}
!573 = distinct !{!573, !572, !"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h6c3604c0f52dd81fE: %self.0"}
!574 = distinct !{!574, !575, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17hc85475c5a72e0922E: %_0"}
!575 = distinct !{!575, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17hc85475c5a72e0922E"}
!576 = distinct !{!576, !575, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17hc85475c5a72e0922E: %self.0"}
!577 = distinct !{!577, !578, !"_ZN7aoc20226solver5day0311parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h05f0b82ddca920aeE: %_0"}
!578 = distinct !{!578, !"_ZN7aoc20226solver5day0311parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h05f0b82ddca920aeE"}
!579 = distinct !{!579, !578, !"_ZN7aoc20226solver5day0311parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h05f0b82ddca920aeE: %line.0"}
!580 = !{!568, !570, !571, !573, !574, !576, !577, !579, !555, !564, !548, !550, !551, !553, !536, !538, !539, !541}
!581 = !{!568, !571, !574, !577, !555, !564, !548, !550, !551, !553, !536, !538, !539, !541}
!582 = !{!583, !548, !550, !551, !553, !536, !538, !539, !541}
!583 = distinct !{!583, !584, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!584 = distinct !{!584, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!585 = !{!548, !551, !536, !538, !539, !541}
!586 = !{!587}
!587 = distinct !{!587, !588, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h4ff1026247ef221dE: %self"}
!588 = distinct !{!588, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h4ff1026247ef221dE"}
!589 = !{!590}
!590 = distinct !{!590, !591, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hd927f3793b36aa77E: %self"}
!591 = distinct !{!591, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hd927f3793b36aa77E"}
!592 = !{!593, !595, !597, !599, !601, !590, !602, !587, !603, !548, !550, !551, !553, !536, !538, !539, !541}
!593 = distinct !{!593, !594, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h7ba5e51ab03952c3E: %self"}
!594 = distinct !{!594, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h7ba5e51ab03952c3E"}
!595 = distinct !{!595, !596, !"_ZN4core4iter6traits8iterator8Iterator4find17h760dddbf069dd629E: %self"}
!596 = distinct !{!596, !"_ZN4core4iter6traits8iterator8Iterator4find17h760dddbf069dd629E"}
!597 = distinct !{!597, !598, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2df1d476115a649aE: %self"}
!598 = distinct !{!598, !"_ZN108_$LT$core..iter..adapters..filter..Filter$LT$I$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2df1d476115a649aE"}
!599 = distinct !{!599, !600, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E: %_0"}
!600 = distinct !{!600, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E"}
!601 = distinct !{!601, !600, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he5e4ae9857db64d7E: %self"}
!602 = distinct !{!602, !591, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hd927f3793b36aa77E: %iterator"}
!603 = distinct !{!603, !588, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h4ff1026247ef221dE: %iter"}
!604 = !{!605, !607, !609, !610, !612, !613, !615, !616, !618, !599, !601, !590, !602, !587, !603, !548, !550, !551, !553, !536, !538, !539, !541}
!605 = distinct !{!605, !606, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!606 = distinct !{!606, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!607 = distinct !{!607, !608, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!608 = distinct !{!608, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!609 = distinct !{!609, !608, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!610 = distinct !{!610, !611, !"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h6c3604c0f52dd81fE: %_0"}
!611 = distinct !{!611, !"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h6c3604c0f52dd81fE"}
!612 = distinct !{!612, !611, !"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h6c3604c0f52dd81fE: %self.0"}
!613 = distinct !{!613, !614, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17hc85475c5a72e0922E: %_0"}
!614 = distinct !{!614, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17hc85475c5a72e0922E"}
!615 = distinct !{!615, !614, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17hc85475c5a72e0922E: %self.0"}
!616 = distinct !{!616, !617, !"_ZN7aoc20226solver5day0311parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h05f0b82ddca920aeE: %_0"}
!617 = distinct !{!617, !"_ZN7aoc20226solver5day0311parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h05f0b82ddca920aeE"}
!618 = distinct !{!618, !617, !"_ZN7aoc20226solver5day0311parse_input28_$u7b$$u7b$closure$u7d$$u7d$17h05f0b82ddca920aeE: %line.0"}
!619 = !{!607, !610, !613, !616, !599, !601, !590, !602, !587, !603, !548, !550, !551, !553, !536, !538, !539, !541}
!620 = !{!590, !587}
!621 = !{!602, !603, !548, !550, !551, !553, !536, !538, !539, !541}
!622 = !{!590, !602, !587, !603, !548, !550, !551, !553, !536, !538, !539, !541}
!623 = !{!550, !553, !538, !541}
!624 = !{!625, !627}
!625 = distinct !{!625, !626, !"_ZN4core3str21_$LT$impl$u20$str$GT$16split_at_checked17he7be85c0547726c0E: %self.0"}
!626 = distinct !{!626, !"_ZN4core3str21_$LT$impl$u20$str$GT$16split_at_checked17he7be85c0547726c0E"}
!627 = distinct !{!627, !628, !"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E: %self.0"}
!628 = distinct !{!628, !"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E"}
!629 = !{!630, !631}
!630 = distinct !{!630, !626, !"_ZN4core3str21_$LT$impl$u20$str$GT$16split_at_checked17he7be85c0547726c0E: %_0"}
!631 = distinct !{!631, !628, !"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E: %pair"}
!632 = !{!633}
!633 = distinct !{!633, !634, !"_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E: %first.0"}
!634 = distinct !{!634, !"_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E"}
!635 = !{!636}
!636 = distinct !{!636, !634, !"_ZN7aoc20226solver5day0323find_common_item_bitset17h7075cdf086536b87E: %second.0"}
!637 = !{!638, !633}
!638 = distinct !{!638, !639, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E: %s.0"}
!639 = distinct !{!639, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E"}
!640 = !{!641, !636}
!641 = distinct !{!641, !642, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE: %self"}
!642 = distinct !{!642, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE"}
!643 = distinct !{!643, !405}
!644 = !{!645, !636}
!645 = distinct !{!645, !646, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E: %s.0"}
!646 = distinct !{!646, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E"}
!647 = !{!648, !633}
!648 = distinct !{!648, !649, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE: %self"}
!649 = distinct !{!649, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE"}
!650 = distinct !{!650, !405}
!651 = !{!652}
!652 = distinct !{!652, !653, !"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E: %item.0"}
!653 = distinct !{!653, !"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17hbd322adcd82b4385E"}
!654 = !{!655}
!655 = distinct !{!655, !656, !"_ZN7aoc20226solver5day0318solve_part2_bitset28_$u7b$$u7b$closure$u7d$$u7d$17h88bcbbeda308bcafE: %group.0"}
!656 = distinct !{!656, !"_ZN7aoc20226solver5day0318solve_part2_bitset28_$u7b$$u7b$closure$u7d$$u7d$17h88bcbbeda308bcafE"}
!657 = !{!655, !652}
!658 = !{!659, !661, !663, !665}
!659 = distinct !{!659, !660, !"_ZN4core4iter6traits8iterator8Iterator4fold17h7ce6814dd7bedb7fE: %self"}
!660 = distinct !{!660, !"_ZN4core4iter6traits8iterator8Iterator4fold17h7ce6814dd7bedb7fE"}
!661 = distinct !{!661, !662, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h35031672675708adE: %self"}
!662 = distinct !{!662, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h35031672675708adE"}
!663 = distinct !{!663, !664, !"_ZN54_$LT$u32$u20$as$u20$core..iter..traits..accum..Sum$GT$3sum17hdffaac89595898edE: %iter"}
!664 = distinct !{!664, !"_ZN54_$LT$u32$u20$as$u20$core..iter..traits..accum..Sum$GT$3sum17hdffaac89595898edE"}
!665 = distinct !{!665, !666, !"_ZN4core4iter6traits8iterator8Iterator3sum17h6048b74665a90d5dE: %self"}
!666 = distinct !{!666, !"_ZN4core4iter6traits8iterator8Iterator3sum17h6048b74665a90d5dE"}
!667 = !{!668}
!668 = distinct !{!668, !669, !"_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE: %first.0"}
!669 = distinct !{!669, !"_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE"}
!670 = !{!671}
!671 = distinct !{!671, !669, !"_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE: %second.0"}
!672 = !{!673}
!673 = distinct !{!673, !669, !"_ZN7aoc20226solver5day0322find_badge_item_bitset17he562ce550176777cE: %third.0"}
!674 = !{!675, !668}
!675 = distinct !{!675, !676, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E: %s.0"}
!676 = distinct !{!676, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E"}
!677 = !{!678, !671, !673, !655, !652, !659, !661, !663, !665}
!678 = distinct !{!678, !679, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE: %self"}
!679 = distinct !{!679, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE"}
!680 = distinct !{!680, !405}
!681 = !{!682, !671}
!682 = distinct !{!682, !683, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E: %s.0"}
!683 = distinct !{!683, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E"}
!684 = !{!685, !668, !673, !655, !652, !659, !661, !663, !665}
!685 = distinct !{!685, !686, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE: %self"}
!686 = distinct !{!686, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE"}
!687 = distinct !{!687, !405}
!688 = !{!689, !673}
!689 = distinct !{!689, !690, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E: %s.0"}
!690 = distinct !{!690, !"_ZN7aoc20226solver5day0315items_to_bitset17ha25488c699d41572E"}
!691 = !{!692, !668, !671, !655, !652, !659, !661, !663, !665}
!692 = distinct !{!692, !693, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE: %self"}
!693 = distinct !{!693, !"_ZN81_$LT$core..str..iter..Bytes$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hb769c602cf1b3f0cE"}
!694 = distinct !{!694, !405}
!695 = !{!696}
!696 = distinct !{!696, !697, !"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E: %_1"}
!697 = distinct !{!697, !"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E"}
!698 = !{!699}
!699 = distinct !{!699, !700, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E: %_1"}
!700 = distinct !{!700, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E"}
!701 = !{!702}
!702 = distinct !{!702, !703, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE: %_1.0"}
!703 = distinct !{!703, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE"}
!704 = !{!699, !696}
!705 = !{!702, !699, !696}
!706 = !{!707, !709}
!707 = distinct !{!707, !708, !"_ZN4core3str21_$LT$impl$u20$str$GT$16split_at_checked17he7be85c0547726c0E: %self.0"}
!708 = distinct !{!708, !"_ZN4core3str21_$LT$impl$u20$str$GT$16split_at_checked17he7be85c0547726c0E"}
!709 = distinct !{!709, !710, !"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E: %self.0"}
!710 = distinct !{!710, !"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E"}
!711 = !{!712, !713}
!712 = distinct !{!712, !708, !"_ZN4core3str21_$LT$impl$u20$str$GT$16split_at_checked17he7be85c0547726c0E: %_0"}
!713 = distinct !{!713, !710, !"_ZN4core3str21_$LT$impl$u20$str$GT$8split_at17h9d8f8cf796f2e575E: %pair"}
!714 = !{!715, !717}
!715 = distinct !{!715, !716, !"_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E: %first.0"}
!716 = distinct !{!716, !"_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E"}
!717 = distinct !{!717, !716, !"_ZN7aoc20226solver5day0316find_common_item17h3b572e1d1656f828E: %second.0"}
!718 = !{!719}
!719 = distinct !{!719, !720, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %self"}
!720 = distinct !{!720, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E"}
!721 = !{!722}
!722 = distinct !{!722, !720, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %other"}
!723 = !{!724, !722, !715, !717}
!724 = distinct !{!724, !720, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %_0"}
!725 = !{!724, !719, !715, !717}
!726 = !{!724, !719, !722}
!727 = !{!728, !730, !732, !734}
!728 = distinct !{!728, !729, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!729 = distinct !{!729, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!730 = distinct !{!730, !731, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9next_impl17h35c27a136cc69b92E: %self"}
!731 = distinct !{!731, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9next_impl17h35c27a136cc69b92E"}
!732 = distinct !{!732, !733, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haf516787cead4a0fE: %self"}
!733 = distinct !{!733, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haf516787cead4a0fE"}
!734 = distinct !{!734, !735, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc80d5fe8583f88c9E: %self"}
!735 = distinct !{!735, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc80d5fe8583f88c9E"}
!736 = !{!734}
!737 = !{!738, !740, !742, !744, !745, !734}
!738 = distinct !{!738, !739, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE: %_0"}
!739 = distinct !{!739, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE"}
!740 = distinct !{!740, !741, !"_ZN9hashbrown3raw13RawTableInner10find_inner17h09a04a9b2a617436E: %self"}
!741 = distinct !{!741, !"_ZN9hashbrown3raw13RawTableInner10find_inner17h09a04a9b2a617436E"}
!742 = distinct !{!742, !743, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE: %self"}
!743 = distinct !{!743, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE"}
!744 = distinct !{!744, !743, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE: argument 1"}
!745 = distinct !{!745, !746, !"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE: %self"}
!746 = distinct !{!746, !"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE"}
!747 = !{i32 0, i32 1114112}
!748 = !{!749, !740, !742, !744, !745, !734}
!749 = distinct !{!749, !750, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find28_$u7b$$u7b$closure$u7d$$u7d$17h85ca384617373b11E: %_1"}
!750 = distinct !{!750, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find28_$u7b$$u7b$closure$u7d$$u7d$17h85ca384617373b11E"}
!751 = !{!752}
!752 = distinct !{!752, !753, !"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h0a46ad3df04f37daE: %item.0"}
!753 = distinct !{!753, !"_ZN4core4iter8adapters10filter_map15filter_map_fold28_$u7b$$u7b$closure$u7d$$u7d$17h0a46ad3df04f37daE"}
!754 = !{!755}
!755 = distinct !{!755, !756, !"_ZN7aoc20226solver5day0316solve_part2_impl28_$u7b$$u7b$closure$u7d$$u7d$17h1255efea226947e3E: %group.0"}
!756 = distinct !{!756, !"_ZN7aoc20226solver5day0316solve_part2_impl28_$u7b$$u7b$closure$u7d$$u7d$17h1255efea226947e3E"}
!757 = !{!755, !752}
!758 = !{!759, !761, !763, !765}
!759 = distinct !{!759, !760, !"_ZN4core4iter6traits8iterator8Iterator4fold17hfdb8dfb5f4d4cec8E: %self"}
!760 = distinct !{!760, !"_ZN4core4iter6traits8iterator8Iterator4fold17hfdb8dfb5f4d4cec8E"}
!761 = distinct !{!761, !762, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17he08a2a5a238f1ce8E: %self"}
!762 = distinct !{!762, !"_ZN115_$LT$core..iter..adapters..filter_map..FilterMap$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17he08a2a5a238f1ce8E"}
!763 = distinct !{!763, !764, !"_ZN54_$LT$u32$u20$as$u20$core..iter..traits..accum..Sum$GT$3sum17hf32516ff36e1dbadE: %iter"}
!764 = distinct !{!764, !"_ZN54_$LT$u32$u20$as$u20$core..iter..traits..accum..Sum$GT$3sum17hf32516ff36e1dbadE"}
!765 = distinct !{!765, !766, !"_ZN4core4iter6traits8iterator8Iterator3sum17hd953e8b6c9ea0f65E: %self"}
!766 = distinct !{!766, !"_ZN4core4iter6traits8iterator8Iterator3sum17hd953e8b6c9ea0f65E"}
!767 = !{!768, !770, !771, !755, !752, !759, !761, !763, !765}
!768 = distinct !{!768, !769, !"_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E: %first.0"}
!769 = distinct !{!769, !"_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E"}
!770 = distinct !{!770, !769, !"_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E: %second.0"}
!771 = distinct !{!771, !769, !"_ZN7aoc20226solver5day0315find_badge_item17h322962ddc8193a26E: %third.0"}
!772 = !{!771, !755, !752, !759, !761, !763, !765}
!773 = !{!755, !752, !759, !761, !763, !765}
!774 = !{!775}
!775 = distinct !{!775, !776, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %self"}
!776 = distinct !{!776, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E"}
!777 = !{!778}
!778 = distinct !{!778, !776, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %other"}
!779 = !{!780, !778, !768, !770, !771, !755, !752, !759, !761, !763, !765}
!780 = distinct !{!780, !776, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %_0"}
!781 = !{!780, !775, !768, !770, !771, !755, !752, !759, !761, !763, !765}
!782 = !{!780, !775, !778, !755, !752, !759, !761, !763, !765}
!783 = !{!784, !786, !787, !789, !768, !770, !771, !755, !752, !759, !761, !763, !765}
!784 = distinct !{!784, !785, !"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h49c5b7e8e2d7c7d0E: %_0"}
!785 = distinct !{!785, !"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h49c5b7e8e2d7c7d0E"}
!786 = distinct !{!786, !785, !"_ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h49c5b7e8e2d7c7d0E: %iter"}
!787 = distinct !{!787, !788, !"_ZN4core4iter6traits8iterator8Iterator7collect17h32150e8bfecccf73E: %_0"}
!788 = distinct !{!788, !"_ZN4core4iter6traits8iterator8Iterator7collect17h32150e8bfecccf73E"}
!789 = distinct !{!789, !788, !"_ZN4core4iter6traits8iterator8Iterator7collect17h32150e8bfecccf73E: %self"}
!790 = !{!791, !793, !795, !797, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!791 = distinct !{!791, !792, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$11get_or_init17h1cf4cb6d83cb1c7bE: %i"}
!792 = distinct !{!792, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$11get_or_init17h1cf4cb6d83cb1c7bE"}
!793 = distinct !{!793, !794, !"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h43984239def11c07E: %__rust_std_internal_init"}
!794 = distinct !{!794, !"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h43984239def11c07E"}
!795 = distinct !{!795, !796, !"_ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE: argument 0"}
!796 = distinct !{!796, !"_ZN4core3ops8function6FnOnce9call_once17hb5ad51aaf3e9b80dE"}
!797 = distinct !{!797, !798, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17h45080ca56f2a806cE: %_0"}
!798 = distinct !{!798, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17h45080ca56f2a806cE"}
!799 = !{!797, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!800 = !{!801, !797, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!801 = distinct !{!801, !802, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE: argument 0"}
!802 = distinct !{!802, !"_ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$16get_or_init_slow17h02117a00d0612e3dE"}
!803 = !{!804}
!804 = distinct !{!804, !805, !"_ZN92_$LT$hashbrown..map..Keys$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hbf4970b127920a26E: argument 1"}
!805 = distinct !{!805, !"_ZN92_$LT$hashbrown..map..Keys$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hbf4970b127920a26E"}
!806 = !{!807, !808, !809, !811, !812, !814, !815, !817, !818, !820, !821, !823, !824, !826, !784, !786, !787, !789, !768, !770, !771, !755, !752, !759, !761, !763, !765}
!807 = distinct !{!807, !805, !"_ZN92_$LT$hashbrown..map..Keys$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hbf4970b127920a26E: %self"}
!808 = distinct !{!808, !805, !"_ZN92_$LT$hashbrown..map..Keys$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hbf4970b127920a26E: argument 2"}
!809 = distinct !{!809, !810, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h5b402558c82c0444E: %self"}
!810 = distinct !{!810, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h5b402558c82c0444E"}
!811 = distinct !{!811, !810, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h5b402558c82c0444E: argument 1"}
!812 = distinct !{!812, !813, !"_ZN104_$LT$core..iter..adapters..copied..Copied$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2e33d3cb10bb931aE: %self"}
!813 = distinct !{!813, !"_ZN104_$LT$core..iter..adapters..copied..Copied$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2e33d3cb10bb931aE"}
!814 = distinct !{!814, !813, !"_ZN104_$LT$core..iter..adapters..copied..Copied$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2e33d3cb10bb931aE: %f"}
!815 = distinct !{!815, !816, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17he787c39c875b2cfaE: %self"}
!816 = distinct !{!816, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17he787c39c875b2cfaE"}
!817 = distinct !{!817, !816, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17he787c39c875b2cfaE: %g"}
!818 = distinct !{!818, !819, !"_ZN4core4iter6traits8iterator8Iterator8for_each17h51f3be2813e27851E: %self"}
!819 = distinct !{!819, !"_ZN4core4iter6traits8iterator8Iterator8for_each17h51f3be2813e27851E"}
!820 = distinct !{!820, !819, !"_ZN4core4iter6traits8iterator8Iterator8for_each17h51f3be2813e27851E: %f"}
!821 = distinct !{!821, !822, !"_ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hd5418a412294258cE: %self"}
!822 = distinct !{!822, !"_ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hd5418a412294258cE"}
!823 = distinct !{!823, !822, !"_ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hd5418a412294258cE: %iter"}
!824 = distinct !{!824, !825, !"_ZN105_$LT$hashbrown..set..HashSet$LT$T$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$6extend17hc63884e07f92b351E: %self"}
!825 = distinct !{!825, !"_ZN105_$LT$hashbrown..set..HashSet$LT$T$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$6extend17hc63884e07f92b351E"}
!826 = distinct !{!826, !825, !"_ZN105_$LT$hashbrown..set..HashSet$LT$T$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$6extend17hc63884e07f92b351E: %iter"}
!827 = !{!828, !830, !832, !833, !835, !807, !804, !808, !809, !811, !812, !814, !815, !817, !818, !820, !821, !823, !824, !826, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!828 = distinct !{!828, !829, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!829 = distinct !{!829, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!830 = distinct !{!830, !831, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9fold_impl17hb5bb9e336cec7ad9E: %self"}
!831 = distinct !{!831, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9fold_impl17hb5bb9e336cec7ad9E"}
!832 = distinct !{!832, !831, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9fold_impl17hb5bb9e336cec7ad9E: argument 1"}
!833 = distinct !{!833, !834, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hb6ea8b29ba942e57E: %self"}
!834 = distinct !{!834, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hb6ea8b29ba942e57E"}
!835 = distinct !{!835, !834, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hb6ea8b29ba942e57E: argument 1"}
!836 = !{!830, !832, !833, !835, !807, !804, !808, !809, !811, !812, !814, !815, !817, !818, !820, !821, !823, !824, !826, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!837 = !{!838, !840, !842, !844, !845, !830, !832, !833, !835, !807, !804, !808, !809, !811, !812, !814, !815, !817, !818, !820, !821, !823, !824, !826, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!838 = distinct !{!838, !839, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE: %_0"}
!839 = distinct !{!839, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE"}
!840 = distinct !{!840, !841, !"_ZN9hashbrown3raw13RawTableInner10find_inner17h09a04a9b2a617436E: %self"}
!841 = distinct !{!841, !"_ZN9hashbrown3raw13RawTableInner10find_inner17h09a04a9b2a617436E"}
!842 = distinct !{!842, !843, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE: %self"}
!843 = distinct !{!843, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE"}
!844 = distinct !{!844, !843, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE: argument 1"}
!845 = distinct !{!845, !846, !"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE: %self"}
!846 = distinct !{!846, !"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE"}
!847 = !{!848, !840, !842, !844, !845, !830, !832, !833, !835, !807, !804, !808, !809, !811, !812, !814, !815, !817, !818, !820, !821, !823, !824, !826, !784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!848 = distinct !{!848, !849, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find28_$u7b$$u7b$closure$u7d$$u7d$17h85ca384617373b11E: %_1"}
!849 = distinct !{!849, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find28_$u7b$$u7b$closure$u7d$$u7d$17h85ca384617373b11E"}
!850 = !{!784, !786, !787, !789, !755, !752, !759, !761, !763, !765}
!851 = !{!786, !789, !768, !770, !771, !755, !752, !759, !761, !763, !765}
!852 = !{!853}
!853 = distinct !{!853, !854, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %other"}
!854 = distinct !{!854, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E"}
!855 = !{!856, !857, !768, !770, !771, !755, !752, !759, !761, !763, !765}
!856 = distinct !{!856, !854, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %_0"}
!857 = distinct !{!857, !854, !"_ZN3std11collections4hash3set20HashSet$LT$T$C$S$GT$12intersection17h162a2b37d8f89cf6E: %self"}
!858 = !{!856, !857, !853, !755, !752, !759, !761, !763, !765}
!859 = !{!860, !862, !864, !866, !755, !752, !759, !761, !763, !765}
!860 = distinct !{!860, !861, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!861 = distinct !{!861, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!862 = distinct !{!862, !863, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9next_impl17h35c27a136cc69b92E: %self"}
!863 = distinct !{!863, !"_ZN9hashbrown3raw21RawIterRange$LT$T$GT$9next_impl17h35c27a136cc69b92E"}
!864 = distinct !{!864, !865, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haf516787cead4a0fE: %self"}
!865 = distinct !{!865, !"_ZN92_$LT$hashbrown..map..Iter$LT$K$C$V$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17haf516787cead4a0fE"}
!866 = distinct !{!866, !867, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc80d5fe8583f88c9E: %self"}
!867 = distinct !{!867, !"_ZN113_$LT$std..collections..hash..set..Intersection$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc80d5fe8583f88c9E"}
!868 = !{!866, !755, !752, !759, !761, !763, !765}
!869 = !{!870, !872, !874, !876, !877, !866, !755, !752, !759, !761, !763, !765}
!870 = distinct !{!870, !871, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE: %_0"}
!871 = distinct !{!871, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE"}
!872 = distinct !{!872, !873, !"_ZN9hashbrown3raw13RawTableInner10find_inner17h09a04a9b2a617436E: %self"}
!873 = distinct !{!873, !"_ZN9hashbrown3raw13RawTableInner10find_inner17h09a04a9b2a617436E"}
!874 = distinct !{!874, !875, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE: %self"}
!875 = distinct !{!875, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE"}
!876 = distinct !{!876, !875, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find17hc2c49d373df902baE: argument 1"}
!877 = distinct !{!877, !878, !"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE: %self"}
!878 = distinct !{!878, !"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17he45b9247ecc5d38dE"}
!879 = !{!880, !872, !874, !876, !877, !866, !755, !752, !759, !761, !763, !765}
!880 = distinct !{!880, !881, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find28_$u7b$$u7b$closure$u7d$$u7d$17h85ca384617373b11E: %_1"}
!881 = distinct !{!881, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$4find28_$u7b$$u7b$closure$u7d$$u7d$17h85ca384617373b11E"}
!882 = !{!883}
!883 = distinct !{!883, !884, !"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E: %_1"}
!884 = distinct !{!884, !"_ZN4core3ptr54drop_in_place$LT$aoc2022..solver..day03..Rucksacks$GT$17h5740ff9396045d15E"}
!885 = !{!886}
!886 = distinct !{!886, !887, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E: %_1"}
!887 = distinct !{!887, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h9d99ae088e1ba3f1E"}
!888 = !{!889}
!889 = distinct !{!889, !890, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE: %_1.0"}
!890 = distinct !{!890, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h24a37f6c99bb6c1dE"}
!891 = !{!886, !883}
!892 = !{!889, !886, !883}
!893 = !{!894, !896}
!894 = distinct !{!894, !895, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E: %_0"}
!895 = distinct !{!895, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E"}
!896 = distinct !{!896, !897, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E: %_0"}
!897 = distinct !{!897, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E"}
!898 = !{!899, !901, !903, !894, !896}
!899 = distinct !{!899, !900, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!900 = distinct !{!900, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!901 = distinct !{!901, !902, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!902 = distinct !{!902, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!903 = distinct !{!903, !902, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!904 = !{!901, !903, !894, !896}
!905 = !{!901, !894, !896}
!906 = !{!907, !909}
!907 = distinct !{!907, !908, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E: %_0"}
!908 = distinct !{!908, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E"}
!909 = distinct !{!909, !910, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E: %_0"}
!910 = distinct !{!910, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E"}
!911 = !{!912, !914, !916, !907, !909}
!912 = distinct !{!912, !913, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!913 = distinct !{!913, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!914 = distinct !{!914, !915, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!915 = distinct !{!915, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!916 = distinct !{!916, !915, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!917 = !{!918, !920}
!918 = distinct !{!918, !919, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E: %_0"}
!919 = distinct !{!919, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E"}
!920 = distinct !{!920, !921, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E: %_0"}
!921 = distinct !{!921, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E"}
!922 = !{!923, !925, !927, !918, !920}
!923 = distinct !{!923, !924, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!924 = distinct !{!924, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!925 = distinct !{!925, !926, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!926 = distinct !{!926, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!927 = distinct !{!927, !926, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!928 = !{!925, !927, !918, !920}
!929 = !{!925, !918, !920}
!930 = !{!931, !933}
!931 = distinct !{!931, !932, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E: %_0"}
!932 = distinct !{!932, !"_ZN53_$LT$usize$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h4ee552fe1c4d9338E"}
!933 = distinct !{!933, !934, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E: %_0"}
!934 = distinct !{!934, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17ha3b354abbc2e5100E"}
!935 = !{!936, !938, !940, !931, !933}
!936 = distinct !{!936, !937, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!937 = distinct !{!937, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!938 = distinct !{!938, !939, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!939 = distinct !{!939, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!940 = distinct !{!940, !939, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!941 = !{!942, !944}
!942 = distinct !{!942, !943, !"_ZN51_$LT$u32$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h1925bb469bafafc5E: %_0"}
!943 = distinct !{!943, !"_ZN51_$LT$u32$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h1925bb469bafafc5E"}
!944 = distinct !{!944, !945, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E: %_0"}
!945 = distinct !{!945, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E"}
!946 = !{!947, !949, !951, !942, !944}
!947 = distinct !{!947, !948, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!948 = distinct !{!948, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!949 = distinct !{!949, !950, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!950 = distinct !{!950, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!951 = distinct !{!951, !950, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!952 = !{!949, !951, !942, !944}
!953 = !{!949, !942, !944}
!954 = !{!955, !957}
!955 = distinct !{!955, !956, !"_ZN51_$LT$u32$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h1925bb469bafafc5E: %_0"}
!956 = distinct !{!956, !"_ZN51_$LT$u32$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h1925bb469bafafc5E"}
!957 = distinct !{!957, !958, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E: %_0"}
!958 = distinct !{!958, !"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h469c49da91750733E"}
!959 = !{!960, !962, !964, !955, !957}
!960 = distinct !{!960, !961, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E: %_0"}
!961 = distinct !{!961, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h05c16547e5b351c7E"}
!962 = distinct !{!962, !963, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %v"}
!963 = distinct !{!963, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E"}
!964 = distinct !{!964, !963, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17h30cee15ad1776a76E: %s.0"}
!965 = !{!914, !907, !909}
!966 = !{!938, !931, !933}
!967 = !{!962, !955, !957}
!968 = !{!969}
!969 = distinct !{!969, !970, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2096107455fc8217E: %self"}
!970 = distinct !{!970, !"_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2096107455fc8217E"}
!971 = !{!972}
!972 = distinct !{!972, !973, !"_ZN99_$LT$core..str..iter..SplitInclusive$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h7007be8a14e48f6aE: %self"}
!973 = distinct !{!973, !"_ZN99_$LT$core..str..iter..SplitInclusive$LT$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h7007be8a14e48f6aE"}
!974 = !{!975}
!975 = distinct !{!975, !976, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$14next_inclusive17h58104a3b9c3e2db0E: %self"}
!976 = distinct !{!976, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$14next_inclusive17h58104a3b9c3e2db0E"}
!977 = !{!975, !972, !969}
!978 = !{!979}
!979 = distinct !{!979, !980, !"_ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hb8758048a7afb167E: %self"}
!980 = distinct !{!980, !"_ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hb8758048a7afb167E"}
!981 = !{!979, !975, !972, !969}
!982 = !{!983}
!983 = distinct !{!983, !980, !"_ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hb8758048a7afb167E: %_0"}
!984 = !{!"branch_weights", i32 4000000, i32 4001}
!985 = !{!983, !979, !975, !972, !969}
!986 = !{!987}
!987 = distinct !{!987, !988, !"_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E: %text.0"}
!988 = distinct !{!988, !"_ZN4core5slice6memchr6memchr17h394f9b31f7c2b9e6E"}
!989 = !{!990, !975, !972, !969}
!990 = distinct !{!990, !991, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$7get_end17h0d4b64fa5ce54b67E: %self"}
!991 = distinct !{!991, !"_ZN4core3str4iter22SplitInternal$LT$P$GT$7get_end17h0d4b64fa5ce54b67E"}
!992 = !{!993, !995, !997, !999}
!993 = distinct !{!993, !994, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E: %self.0"}
!994 = distinct !{!994, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E"}
!995 = distinct !{!995, !996, !"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E: %haystack.0"}
!996 = distinct !{!996, !"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E"}
!997 = distinct !{!997, !998, !"_ZN89_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..Fn$LT$$LP$$RF$str$C$$RP$$GT$$GT$4call17h4594de12ac10aaedE: argument 0"}
!998 = distinct !{!998, !"_ZN89_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..Fn$LT$$LP$$RF$str$C$$RP$$GT$$GT$4call17h4594de12ac10aaedE"}
!999 = distinct !{!999, !1000, !"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E: argument 0"}
!1000 = distinct !{!1000, !"_ZN92_$LT$core..str..LinesMap$u20$as$u20$core..ops..function..FnMut$LT$$LP$$RF$str$C$$RP$$GT$$GT$8call_mut17h39069d3678f0ff40E"}
!1001 = !{!1002, !1003, !969}
!1002 = distinct !{!1002, !994, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E: %needle.0"}
!1003 = distinct !{!1003, !996, !"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E: %self.0"}
!1004 = !{!1005, !1007, !997, !999}
!1005 = distinct !{!1005, !1006, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E: %self.0"}
!1006 = distinct !{!1006, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E"}
!1007 = distinct !{!1007, !1008, !"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E: %haystack.0"}
!1008 = distinct !{!1008, !"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E"}
!1009 = !{!1010, !1011, !969}
!1010 = distinct !{!1010, !1006, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9ends_with17h8285db8322eec876E: %needle.0"}
!1011 = distinct !{!1011, !1008, !"_ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15strip_suffix_of17h0c61911c692e6a89E: %self.0"}
!1012 = !{!1013}
!1013 = distinct !{!1013, !1014, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$25find_or_find_insert_index17hbdee965a32244296E: %self"}
!1014 = distinct !{!1014, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$25find_or_find_insert_index17hbdee965a32244296E"}
!1015 = !{!1016, !1013}
!1016 = distinct !{!1016, !1017, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E: %self"}
!1017 = distinct !{!1017, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E"}
!1018 = !{!1019, !1020, !1021}
!1019 = distinct !{!1019, !1017, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$7reserve17hfb686a516719da24E: %hasher"}
!1020 = distinct !{!1020, !1014, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$25find_or_find_insert_index17hbdee965a32244296E: argument 1"}
!1021 = distinct !{!1021, !1014, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$25find_or_find_insert_index17hbdee965a32244296E: %hasher"}
!1022 = !{!1020}
!1023 = !{!1020, !1021}
!1024 = !{!1025, !1013, !1020, !1021}
!1025 = distinct !{!1025, !1026, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE: %_0"}
!1026 = distinct !{!1026, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE"}
!1027 = !{!1028, !1013, !1020, !1021}
!1028 = distinct !{!1028, !1029, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$25find_or_find_insert_index28_$u7b$$u7b$closure$u7d$$u7d$17h32adbd9b25aed85dE: %_1"}
!1029 = distinct !{!1029, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$25find_or_find_insert_index28_$u7b$$u7b$closure$u7d$$u7d$17h32adbd9b25aed85dE"}
!1030 = !{!1013, !1020, !1021}
!1031 = !{!1032, !1013, !1020, !1021}
!1032 = distinct !{!1032, !1033, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!1033 = distinct !{!1033, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!1034 = !{!1035}
!1035 = distinct !{!1035, !1036, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$22insert_tagged_at_index17h6a5f589d65d019d2E: %self"}
!1036 = distinct !{!1036, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$22insert_tagged_at_index17h6a5f589d65d019d2E"}
!1037 = !{!1038}
!1038 = distinct !{!1038, !1039, !"_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE: %self"}
!1039 = distinct !{!1039, !"_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE"}
!1040 = !{!1041}
!1041 = distinct !{!1041, !1039, !"_ZN9hashbrown3raw13RawTableInner20reserve_rehash_inner17he6e621ada850df6fE: %alloc"}
!1042 = !{!1038, !1041}
!1043 = !{!1044}
!1044 = distinct !{!1044, !1045, !"_ZN9hashbrown3raw13RawTableInner12resize_inner17h536bd935418c97a0E: %self"}
!1045 = distinct !{!1045, !"_ZN9hashbrown3raw13RawTableInner12resize_inner17h536bd935418c97a0E"}
!1046 = !{!"branch_weights", i32 4292820, i32 2143190828}
!1047 = !{!1048, !1050}
!1048 = distinct !{!1048, !1049, !"_ZN9hashbrown3raw13RawTableInner17new_uninitialized17hd0b06a6ee7a87ac4E: %_0"}
!1049 = distinct !{!1049, !"_ZN9hashbrown3raw13RawTableInner17new_uninitialized17hd0b06a6ee7a87ac4E"}
!1050 = distinct !{!1050, !1051, !"_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E: %_0"}
!1051 = distinct !{!1051, !"_ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17h49efc48a683e26d0E"}
!1052 = !{!1050}
!1053 = !{!1054}
!1054 = distinct !{!1054, !1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %x"}
!1055 = distinct !{!1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E"}
!1056 = !{!1057}
!1057 = distinct !{!1057, !1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %y"}
!1058 = !{!1044, !1038}
!1059 = !{!1060, !1041}
!1060 = distinct !{!1060, !1045, !"_ZN9hashbrown3raw13RawTableInner12resize_inner17h536bd935418c97a0E: %alloc"}
!1061 = !{!1062, !1044, !1038}
!1062 = distinct !{!1062, !1063, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!1063 = distinct !{!1063, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!1064 = !{!1065, !1067}
!1065 = distinct !{!1065, !1066, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash28_$u7b$$u7b$closure$u7d$$u7d$17h750b758265279954E: %_1"}
!1066 = distinct !{!1066, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash28_$u7b$$u7b$closure$u7d$$u7d$17h750b758265279954E"}
!1067 = distinct !{!1067, !1066, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash28_$u7b$$u7b$closure$u7d$$u7d$17h750b758265279954E: %table"}
!1068 = !{!1069}
!1069 = distinct !{!1069, !1070, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!1070 = distinct !{!1070, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!1071 = !{!1072}
!1072 = distinct !{!1072, !1073, !"_ZN4core4hash11BuildHasher8hash_one17h92fd872fa3e55b4dE: argument 0"}
!1073 = distinct !{!1073, !"_ZN4core4hash11BuildHasher8hash_one17h92fd872fa3e55b4dE"}
!1074 = !{!1075, !1065, !1067}
!1075 = distinct !{!1075, !1076, !"_ZN4core4hash5impls52_$LT$impl$u20$core..hash..Hash$u20$for$u20$$RF$T$GT$4hash17h949a96ecd4fad6f1E: %state"}
!1076 = distinct !{!1076, !"_ZN4core4hash5impls52_$LT$impl$u20$core..hash..Hash$u20$for$u20$$RF$T$GT$4hash17h949a96ecd4fad6f1E"}
!1077 = !{!1078}
!1078 = distinct !{!1078, !1079, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE: %_0"}
!1079 = distinct !{!1079, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE"}
!1080 = !{!"branch_weights", i32 1, i32 1999}
!1081 = !{!1082}
!1082 = distinct !{!1082, !1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %x:It1"}
!1083 = !{!1084}
!1084 = distinct !{!1084, !1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %y:It1"}
!1085 = !{!1086}
!1086 = distinct !{!1086, !1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %x:It2"}
!1087 = !{!1088}
!1088 = distinct !{!1088, !1055, !"_ZN4core3ptr10swap_chunk17h97d8c3e5ffb62e47E: %y:It2"}
!1089 = !{!1090, !1092}
!1090 = distinct !{!1090, !1091, !"_ZN88_$LT$hashbrown..scopeguard..ScopeGuard$LT$T$C$F$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1ced3369f5b3143bE: %self"}
!1091 = distinct !{!1091, !"_ZN88_$LT$hashbrown..scopeguard..ScopeGuard$LT$T$C$F$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1ced3369f5b3143bE"}
!1092 = distinct !{!1092, !1093, !"_ZN4core3ptr196drop_in_place$LT$hashbrown..scopeguard..ScopeGuard$LT$hashbrown..raw..RawTableInner$C$hashbrown..raw..RawTableInner..prepare_resize$LT$alloc..alloc..Global$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$17h4b305965ac15b6e0E: %_1"}
!1093 = distinct !{!1093, !"_ZN4core3ptr196drop_in_place$LT$hashbrown..scopeguard..ScopeGuard$LT$hashbrown..raw..RawTableInner$C$hashbrown..raw..RawTableInner..prepare_resize$LT$alloc..alloc..Global$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$17h4b305965ac15b6e0E"}
!1094 = !{!"branch_weights", i32 0, i32 1}
!1095 = !{!1096}
!1096 = distinct !{!1096, !1097, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!1097 = distinct !{!1097, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!1098 = !{!1099}
!1099 = distinct !{!1099, !1100, !"_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E: %self"}
!1100 = distinct !{!1100, !"_ZN9hashbrown3raw13RawTableInner15rehash_in_place17h47d519993a2060c2E"}
!1101 = !{!1102, !1099}
!1102 = distinct !{!1102, !1103, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!1103 = distinct !{!1103, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!1104 = !{!1105, !1099}
!1105 = distinct !{!1105, !1106, !"_ZN4core9core_arch3x864sse215_mm_store_si12817h457b15f89698a781E: %a"}
!1106 = distinct !{!1106, !"_ZN4core9core_arch3x864sse215_mm_store_si12817h457b15f89698a781E"}
!1107 = !{!"branch_weights", !"expected", i32 0, i32 -2147483648}
!1108 = !{!1109}
!1109 = distinct !{!1109, !1110, !"_ZN4core4hash11BuildHasher8hash_one17h92fd872fa3e55b4dE: argument 0"}
!1110 = distinct !{!1110, !"_ZN4core4hash11BuildHasher8hash_one17h92fd872fa3e55b4dE"}
!1111 = !{!1112, !1114, !1116, !1099}
!1112 = distinct !{!1112, !1113, !"_ZN4core4hash5impls52_$LT$impl$u20$core..hash..Hash$u20$for$u20$$RF$T$GT$4hash17h949a96ecd4fad6f1E: %state"}
!1113 = distinct !{!1113, !"_ZN4core4hash5impls52_$LT$impl$u20$core..hash..Hash$u20$for$u20$$RF$T$GT$4hash17h949a96ecd4fad6f1E"}
!1114 = distinct !{!1114, !1115, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash28_$u7b$$u7b$closure$u7d$$u7d$17h750b758265279954E: %_1"}
!1115 = distinct !{!1115, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash28_$u7b$$u7b$closure$u7d$$u7d$17h750b758265279954E"}
!1116 = distinct !{!1116, !1115, !"_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash28_$u7b$$u7b$closure$u7d$$u7d$17h750b758265279954E: %table"}
!1117 = !{!1118, !1099}
!1118 = distinct !{!1118, !1119, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE: %_0"}
!1119 = distinct !{!1119, !"_ZN4core9core_arch3x864sse215_mm_loadu_si12817h55c195dea49014bfE"}
!1120 = !{!1121, !1099}
!1121 = distinct !{!1121, !1122, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E: %_0"}
!1122 = distinct !{!1122, !"_ZN4core9core_arch3x864sse214_mm_load_si12817h007cc598ccd3c888E"}
!1123 = !{!1124}
!1124 = distinct !{!1124, !1125, !"_ZN4core3ptr10swap_chunk17hd3ab3415b330d385E: %x"}
!1125 = distinct !{!1125, !"_ZN4core3ptr10swap_chunk17hd3ab3415b330d385E"}
!1126 = !{!1127}
!1127 = distinct !{!1127, !1125, !"_ZN4core3ptr10swap_chunk17hd3ab3415b330d385E: %y"}
!1128 = !{!1127, !1099}
!1129 = !{!1124, !1099}
