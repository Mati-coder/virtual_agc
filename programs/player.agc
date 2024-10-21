.config
    VEC MASCARAS 8
.code
PLAYER:
	CS ZERO

    INDEX X
    MASK MASCARAS

    INDEX Y
    TS PANT

    CCS BTNUP

    INCR Y

    CA BTNDWN
    BZF A1
    EXTEND
    DIM Y
A1:
    CCS BTNRGT

    INCR X

    CA BTNLFT
    BZF A2
    EXTEND
    DIM X
A2:
    CCS BTN2

    RETURN
    
    CA X
    EXTEND
    SU MAXXY
    BZMF A3
    CA MAX
    TS X
A3:
    CA Y
    EXTEND
    SU MAXXY
    BZMF A4
    CA MAX
    TS Y
A4:
    CA X
    EXTEND
    AUG X
    BZMF A5
    EXTEND
    DIM X
A5:
    CA ZERO
    TS X

    CA Y
    EXTEND
    AUG Y
    BZMF A6
    EXTEND
    DIM Y
A6:
    CA ZERO
    TS Y

    TCF PLAYER

.data
MAXXY:
    DEC 7
MASCARAS:
    DEC 1
    DEC 2
    DEC 4
    DEC 8
    DEC 16
    DEC 32
    DEC 64
    DEC 128

