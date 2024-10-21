.code
MOVIMIENTO:
    CCS BTNUP

    INCR Y

    CA BTNDWN
    BZF B1
    EXTEND
    DIM Y
B1:
    CCS BTNRGT

    INCR X

    CA BTNLFT
    BZF B2
    EXTEND
    DIM X
B2:
    CA X
    EXTEND
    SU MAXXY
    BZMF B3
    CA MAX
    TS X
B3:
    CA Y
    EXTEND
    SU MAXXY
    BZMF B4
    CA MAX
    TS Y
B4:
    CA X
    EXTEND
    AUG X
    BZMF B5
    EXTEND
    DIM X
B5:
    CA ZERO
    TS X

    CA Y
    EXTEND
    AUG Y
    BZMF B6
    EXTEND
    DIM Y
B6:
    CA ZERO
    TS Y

    RETURN

