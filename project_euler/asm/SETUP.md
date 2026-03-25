# Running x86 MASM Assembly on Windows

Steps taken to compile and run 32-bit x86 MASM code on Windows 11 with VS 2022.

---

## Prerequisites

MASM (`ml.exe`) ships with the **C++ build tools** workload in Visual Studio 2022.
If you already fixed the Rust linker issue, MASM is already installed.

Verify:
```
find "C:/Program Files/Microsoft Visual Studio" -name "ml.exe"
```
You should see paths ending in `Hostx64\x86\ml.exe` and `Hostx86\x86\ml.exe`.

---

## Key Tool Paths (VS 2022 Community)

| Tool | Path |
|------|------|
| Assembler (32-bit) | `C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x86\ml.exe` |
| Linker (64-bit, can target x86) | `C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe` |
| MSVC libs (x86) | `C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\lib\x86` |
| Windows SDK libs (x86) | `C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x86` |

The MSVC version number (`14.44.35207`) and SDK version (`10.0.26100.0`) may differ — check with `find`.

---

## MASM Source File Structure (32-bit)

```asm
    .386                        ; target instruction set
    .model flat, stdcall        ; flat memory model, stdcall calling convention
    option casemap:none         ; case-sensitive symbol names

; Declare external Windows API functions
ExitProcess PROTO :DWORD
WriteFile   PROTO :DWORD, :DWORD, :DWORD, :DWORD, :DWORD

STD_OUTPUT_HANDLE EQU -11      ; constant for GetStdHandle

    includelib kernel32.lib    ; tell linker which import lib to use

.data
    ; variables and constants here

.code

start PROC
    ; your code here
    INVOKE ExitProcess, 0
start ENDP

END start                      ; END label sets the program entry point
```

**Key points:**
- `INVOKE` is a MASM macro that handles argument pushing and calling conventions
- `PROTO` declares external functions so INVOKE knows the signature
- `ADDR var` in INVOKE generates a `LEA` to get a variable's address
- `SIZEOF label` gives the byte size of a data label
- `END start` (not just `END`) tells the linker the entry point — no `/entry:` flag needed

---

## Build Script (PowerShell)

**Use PowerShell, not a `.bat` file called from Git Bash.**
Git Bash converts paths in `cmd.exe` arguments (e.g. `/c` → `C:\`), which breaks batch files.

```powershell
# build.ps1
$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot

$ml   = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x86\ml.exe'
$link = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe'
$msvcLib = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\lib\x86'
$sdkLib  = 'C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x86'

# Assemble: /c = compile only (no link), /Fo = output .obj file
& $ml /c /Fo myfile.obj myfile.asm

# Link: /machine:x86 = 32-bit output, /subsystem:console = console app
& $link /subsystem:console /machine:x86 /out:myfile.exe `
    myfile.obj `
    "/LIBPATH:$msvcLib" "/LIBPATH:$sdkLib" `
    kernel32.lib
```

Run it from Git Bash:
```bash
powershell.exe -NoProfile -ExecutionPolicy Bypass -File build.ps1
```

---

## Assembly → Executable: What Happens

```
p018_brute.asm
    ↓  ml.exe /c /Fo p018_brute.obj p018_brute.asm
p018_brute.obj   (COFF object file, x86 machine code + relocation info)
    ↓  link.exe /subsystem:console /machine:x86 ... kernel32.lib
p018_brute.exe   (PE32 executable, imports from kernel32.dll)
```

---

## Pitfalls Encountered

### 1. Git Bash path conversion breaks `cmd.exe` invocation
`cmd /c "..."` from Git Bash converts `/c` flags to `C:\` paths.
**Fix:** Use a PowerShell script (`build.ps1`) instead of a `.bat` file.

### 2. ECX is a volatile register — destroyed by function calls
After calling any Windows API function (e.g. `WriteFile`), `EAX`, `ECX`, `EDX` are trashed.
**Fix:** Compute and use `ECX`/`EDI` values *after* the last API call that precedes them.

```asm
; WRONG — WriteFile destroys ECX before the second WriteFile uses it
lea ecx, [buf + 12]
sub ecx, edi                      ; ecx = digit count
INVOKE WriteFile, hout, ADDR prefix, SIZEOF prefix, ADDR nw, 0  ; ECX trashed!
INVOKE WriteFile, hout, edi, ecx, ADDR nw, 0  ; ecx is garbage

; RIGHT — format AFTER the prefix is printed
INVOKE WriteFile, hout, ADDR prefix, SIZEOF prefix, ADDR nw, 0
lea ecx, [buf + 12]               ; ECX freshly set, no more calls before use
sub ecx, edi
INVOKE WriteFile, hout, edi, ecx, ADDR nw, 0
```

### 3. Avoid `printf` from msvcrt without CRT startup
When `start` (not `main`) is the entry point, the C runtime hasn't initialised.
`printf` depends on CRT init (stdout setup). **Fix:** Use kernel32-only API calls —
`GetStdHandle` + `WriteFile` — and write your own int-to-string conversion.

---

## Output via kernel32 Only (No CRT)

```asm
GetStdHandle    PROTO :DWORD
WriteFile       PROTO :DWORD, :DWORD, :DWORD, :DWORD, :DWORD
STD_OUTPUT_HANDLE EQU -11

; get stdout handle once at startup
INVOKE GetStdHandle, STD_OUTPUT_HANDLE
mov hout, eax

; print a string literal
INVOKE WriteFile, hout, ADDR my_string, SIZEOF my_string, ADDR nw, 0

; convert uint in eax to decimal (right-to-left into a 12-byte buffer)
lea edi, [num_buf + 11]
mov ebx, 10
fmt: xor edx, edx
     div ebx
     add dl, '0'
     mov [edi], dl
     dec edi
     test eax, eax
     jnz fmt
inc edi                          ; edi = first digit
lea ecx, [num_buf + 12]
sub ecx, edi                     ; ecx = digit count
INVOKE WriteFile, hout, edi, ecx, ADDR nw, 0
```

---

## Volatile vs Callee-Saved Registers (x86 stdcall / Windows ABI)

| Volatile (caller-saved) | Callee-saved |
|------------------------|--------------|
| EAX, ECX, EDX          | EBX, ESI, EDI, EBP, ESP |

After any `CALL` or `INVOKE`, assume `EAX`, `ECX`, `EDX` have changed.
`EBX`, `ESI`, `EDI`, `EBP` are preserved across calls (callee must save/restore them).

---

**See also**: [Session notes](../../zettelkasten/Daily%20Notes/2026-03-01.md)
