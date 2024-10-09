.config
    VEC MASCARAS 8
.code
PLAYER:
	CS ZERO

    INDEX X
    MASK MASCARAS

    INDEX Y
    CA PANT

BOTONES:
    CA BTNUP
    BZF SIG1
    INCR Y
SIG1:
    CA BTNDWN
    BZF SIG2
    EXTEND
    DIM Y
SIG2:
    CA BTNDGT
    BZF SIG3
    INCR X
SIG3:
    CA BTNLFT
    BZF CHECKS
    EXTEND
    DIM X
CHECKS:
    CA X
    EXTEND
    SU MAX
    BZMF SIG4
    CA MAX
    TS X
SIG4:
    CA Y
    EXTEND
    SU MAX
    BZMF SIG5
    CA MAX
    TS Y
SIG5:
    CA X
    EXTEND
    AUG X
    BZMF NEG1
    EXTEND
    DIM X
NEG1:
    CA ZERO
    TS X

    CA Y
    EXTEND
    AUG Y
    BZMF NEG2
    EXTEND
    DIM Y
NEG2:
    CA ZERO
    TS Y

    TCF PLAYER

.data
MAX:
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

