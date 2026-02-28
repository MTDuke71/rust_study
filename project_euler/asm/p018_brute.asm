; p018_brute.asm — bitRAKE's brute-force max triangle path (P018, 15 rows)
;
; Original: bitRAKE, projecteuler.net thread #18, Jan 5 2005
; Adapted:  25-row → 15-row P018 triangle; console output via kernel32 only.
;
; Algorithm:
;   Enumerate all 2^14 = 16,384 paths through the 15-row triangle.
;   Path encoded as 14 bits in ecx (MSB-first): 0 = go left, 1 = go right.
;   Flat triangle offset advanced by:  adc edx, esi  (esi = 1-indexed row)
;   Left child sits +esi cells ahead; right child sits +esi+1 cells ahead —
;   the carry flag from SHL naturally selects between them.
;
; Build: build.bat

    .386
    .model flat, stdcall
    option casemap:none

GetStdHandle    PROTO :DWORD
WriteFile       PROTO :DWORD, :DWORD, :DWORD, :DWORD, :DWORD
ExitProcess     PROTO :DWORD

STD_OUTPUT_HANDLE EQU -11

    includelib kernel32.lib

.data

TriTable LABEL BYTE
    BYTE 75
    BYTE 95, 64
    BYTE 17, 47, 82
    BYTE 18, 35, 87, 10
    BYTE 20, 04, 82, 47, 65
    BYTE 19, 01, 23, 75, 03, 34
    BYTE 88, 02, 77, 73, 07, 63, 67
    BYTE 99, 65, 04, 28, 06, 16, 70, 92
    BYTE 41, 41, 26, 56, 83, 40, 80, 70, 33
    BYTE 41, 48, 72, 33, 47, 32, 37, 16, 94, 29
    BYTE 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14
    BYTE 70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57
    BYTE 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48
    BYTE 63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31
    BYTE 04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23

prefix  BYTE "P018 brute force: "
MAX_TOT DWORD 0
hout    DWORD 0
nw      DWORD 0
num_buf BYTE 12 DUP(' ')   ; number string, filled right-to-left
CRLF    BYTE 13, 10

.code

start PROC
    ; ── get stdout handle ─────────────────────────────────────────────────
    INVOKE GetStdHandle, STD_OUTPUT_HANDLE
    mov hout, eax

    ; ── bitRAKE's algorithm (15 rows, 2^14 = 16,384 paths) ───────────────
    mov ebx, 03FFFh     ; path counter: 16383 down to 0
    mov MAX_TOT, 0

next:
    mov ecx, ebx        ; ecx = current path bits
    xor edx, edx        ; edx = flat table offset (start at 0)
    xor edi, edi        ; edi = sum for this path
    shl ecx, 18         ; align 14-bit path to bit 31  (32 - 14 = 18)
    xor esi, esi        ; esi = row counter

@@: inc esi                     ; esi: 1 → 15  (1-indexed destination row)
    shl ecx, 1                  ; direction bit → carry  (0=left, 1=right)
    movzx eax, TriTable[edx]    ; current triangle cell value
    adc edx, esi                ; edx += esi + carry  (left=+esi, right=+esi+1)
    add edi, eax                ; accumulate path sum
    cmp esi, 15
    jne @B

    mov eax, MAX_TOT
    cmp eax, edi
    jae no_update
    mov MAX_TOT, edi
no_update:
    dec ebx
    jns next               ; loop while ebx >= 0  (processes all 2^14 paths)

    ; ── print prefix first (before format loop so WriteFile can't destroy ECX) ──
    INVOKE WriteFile, hout, ADDR prefix, SIZEOF prefix, ADDR nw, 0

    ; ── convert MAX_TOT to decimal string (right-to-left into num_buf) ───
    ; Note: ECX and EDI are freshly set here, after all prior function calls.
    mov eax, MAX_TOT
    lea edi, [num_buf + 11]   ; start at last byte
    mov ebx, 10
fmt_loop:
    xor edx, edx
    div ebx                   ; eax = quotient, edx = remainder digit
    add dl, '0'
    mov [edi], dl
    dec edi
    test eax, eax
    jnz fmt_loop

    ; edi+1 = first digit, (num_buf+12) - (edi+1) = digit count
    inc edi
    lea ecx, [num_buf + 12]
    sub ecx, edi              ; ecx = number of digit characters

    ; ── print number then newline ─────────────────────────────────────────
    INVOKE WriteFile, hout, edi, ecx, ADDR nw, 0
    INVOKE WriteFile, hout, ADDR CRLF, 2, ADDR nw, 0

    INVOKE ExitProcess, 0
start ENDP

END start
