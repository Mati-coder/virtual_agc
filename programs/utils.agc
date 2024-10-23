.code
MOVIMIENTO:
    CCS BTNUP

    INCR Y

    CA BTNDWN
    EXTEND
    BZF B1
    EXTEND
    DIM Y
B1:
    CCS BTNRGT

    INCR X

    CA BTNLFT
    EXTEND
    BZF B2
    EXTEND
    DIM X
B2:
    CS X
    AD MAXXY
    COM
    EXTEND
    BZMF B3
    CA MAXXY
    TS X
B3:
    CS Y
    AD MAXXY
    COM
    EXTEND
    BZMF B4
    CA MAXXY
    TS Y
B4:
    CA X
    EXTEND
    BZMF B5
    TCF B6
B5:
    CA ZERO
    TS X
B6:
    CA Y
    EXTEND
    BZMF B7
    TCF B8
B7:
    CA ZERO
    TS Y
B8:
    RETURN

DELAY:
    CA CICLOS
DELAYL:
    TS I
    CCS I

    TCF DELAYL

    RETURN

LIMPPANT:
    CA ANCHOPANT
BUCLELP:
    TS I
    
    CA ZERO
    INDEX I
    TS PANT
    CCS I

    TCF BUCLELP

    RETURN

.data
ANCHOPANT:
    DEC 7
MAXXY:
    DEC 7

