# Step 2 Compilation Process: From Source to Executable

This directory contains all intermediate files showing how `step2_push_front.rs` compiles into an executable.

## 🔄 Compilation Pipeline

```
step2_push_front.rs (Source)
    ↓ [Parsing & AST]
1. ast_representation.txt
    ↓ [HIR - High-level IR]
2. hir_representation.txt  
    ↓ [MIR - Mid-level IR]
3. mir_representation.txt
    ↓ [LLVM IR Generation]
4. llvm_ir.ll
    ↓ [Assembly Generation]
5. assembly_output.s
    ↓ [Object File]
6. object_file_info.txt
    ↓ [Linking]
7. executable_analysis.txt
```

## 🚀 How to Generate These Files

```powershell
# Navigate to the tutorial directory
cd D:\repos\rust_study\Mission4_tut

# Generate each intermediate stage
.\generate_compilation_stages.ps1
```

## 📊 Files Generated

- **`ast_representation.txt`** - Abstract Syntax Tree structure
- **`hir_representation.txt`** - High-level Intermediate Representation
- **`mir_representation.txt`** - Mid-level Intermediate Representation  
- **`llvm_ir.ll`** - LLVM Intermediate Representation
- **`assembly_output.s`** - Platform-specific assembly code
- **`object_file_info.txt`** - Object file structure analysis
- **`executable_analysis.txt`** - Final executable breakdown

Each file shows a different stage of compilation with explanations!