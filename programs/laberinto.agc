.config
    VEC MASCNEG 8
.code
LABERINTO:
    CA ZERO
    TS X
    TS Y
LABB:
	CS ZERO

    INDEX X
    MASK MASC

    CA ANCHOPANT
IMPR:
    TS I
    
    CA Y
    INCR ACC
    EXTEND
    SU I
    EXTEND
    DIM ACC

    EXTEND
    BZF D2

    INDEX I
    CA MAPA
    INDEX I
    TS PANT
    TCF D3
D2:
    INDEX X
    CA MASCNEG

    INDEX I
    TS PANT
D3:
    CCS I

    TCF IMPR

    CA CICLOS3
    TS CICLOS
    TC DELAY

    TC MOVIMIENTO

    CA BTN1
    EXTEND
    BZMF D1
    TC LIMPPANT
    TCF INICIO
D1:
    TCF LABB

.data
MASCNEG:
    DEC 32766
    DEC 32765
    DEC 32763
    DEC 32759
    DEC 32751
    DEC 32735
    DEC 32703
    DEC 32639