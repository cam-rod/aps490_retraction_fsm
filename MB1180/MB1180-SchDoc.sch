EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr B 17000 11000
encoding utf-8
Sheet 1 1
Title "MB1180-SchDoc"
Date "22 01 2024"
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Notes 10160 10980 0    60   ~ 0
1
Text Notes 10410 10980 0    60   ~ 0
3
Text Notes 8560 10520 0    60   ~ 12
TOP
Text Notes 9280 10830 0    60   ~ 0
MB1180
Text Notes 10280 10830 0    60   ~ 0
C.1
Text Notes 8550 10980 0    60   ~ 0
*
Wire Notes Line
	8300 10550 10700 10550
Wire Notes Line
	10700 10700 8300 10700
Wire Notes Line
	8800 10850 8800 10700
Wire Notes Line
	9850 11000 9850 10700
Wire Notes Line
	10700 11000 10700 10400
Text Notes 8330 10520 0    60   ~ 0
Title:
Text Notes 8330 10830 0    60   ~ 0
Size:
Text Notes 8850 10830 0    60   ~ 0
Reference:
Text Notes 8330 10980 0    60   ~ 0
Date:
Text Notes 9900 10980 0    60   ~ 0
Sheet:
Text Notes 10290 10980 0    60   ~ 0
of
Text Notes 8550 10830 0    60   ~ 0
A4
Wire Notes Line
	10700 10850 8300 10850
Text Notes 9900 10830 0    60   ~ 0
Revision:
Text Notes 8640 10670 0    60   ~ 12
NUCLEO32
Text Notes 8330 10670 0    60   ~ 0
Project:
Wire Notes Line
	8300 11000 8300 10400
Wire Notes Line
	8300 10400 11500 10400
Text Label 8300 10000 0 60 ~
REV B: SB14 changed to JP1 Jumper for easy IDD measurement, and enlarge board length; CN1 USB PN changed to Micro-B for Device.  REV C: Add SB18/SB16 for connecting D4/D5 to A4/A5  
Text HLabel 4100 7700 0 60 BiDi ~
MCO
Text HLabel 4100 7200 0 60 Output ~
VCP_TX
Text HLabel 4100 7500 0 60 BiDi ~
SWCLK
Text HLabel 4100 7600 0 60 BiDi ~
SWDIO
Text HLabel 4100 7300 0 60 Input ~
VCP_RX
Text HLabel 4100 7900 0 60 BiDi ~
NRST
$Sheet
S 2600 7100 1500 1000
F0 "U_MCU_32" 60
F1 "MCU_32-SchDoc.sch" 60
$EndSheet
Text HLabel 5400 7600 2 60 BiDi ~
TMS
Text HLabel 5400 7500 2 60 BiDi ~
TCK
Text HLabel 5400 7700 2 60 BiDi ~
MCO
Text HLabel 5400 7900 2 60 BiDi ~
NRST
Text HLabel 5400 7200 2 60 Input ~
STLK_RX
Text HLabel 5400 7300 2 60 Output ~
STLK_TX
Text HLabel 6900 7500 0 60 BiDi ~
SWO
$Sheet
S 5400 7100 1500 1000
F0 "U_ST_LINK_V2-1" 60
F1 "ST_LINK_V2-1-SCHDOC.sch" 60
$EndSheet
Wire Wire Line
	5400 7200 4100 7200
Wire Wire Line
	5400 7300 4100 7300
Wire Wire Line
	5400 7500 4100 7500
Wire Wire Line
	5400 7700 4100 7700
Wire Wire Line
	5400 7900 4100 7900
Wire Wire Line
	5400 7600 4100 7600
NoConn ~ 6900 7500
$EndSCHEMATC